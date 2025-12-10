use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::oal::device_lifecycle;
use wgpu::util::DeviceExt;
// Re-exported types for low-level image copy/layout structures are
// provided by the `wgpu-types` crate (aliased here as `wgt`).
use wgt;
// ab_glyph usage removed; prefer rusttype-based rasterization via bytes.
use rusttype::{Font as RtFont, Scale as RtScale, point as rt_point};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal glyph atlas skeleton.
///
/// This is a lightweight placeholder that registers a reupload callback
/// with the `DeviceLifecycle` module so that when a device is recreated
/// the atlas records the new device id and marks that it has reuploaded
/// (in a real implementation this is where GPU texture recreation and
/// uploads would be scheduled).
pub struct GlyphAtlas {
	last_device: Arc<Mutex<Option<usize>>>,
	reuploaded: Arc<Mutex<bool>>,
	callback_handle: Option<usize>,
	device_reupload_handle: Option<usize>,
	// Simple in-memory glyph cache: char -> (width, height, bitmap)
	glyphs: Arc<Mutex<HashMap<char, (u32, u32, Vec<u8>)>>>,
	// Glyph placements inside the last-packed atlas: char -> (x, y, w, h) in pixels
	glyph_positions: Arc<Mutex<HashMap<char, (u32, u32, u32, u32)>>>,
	// Glyphs that have been rasterized but not yet uploaded to the GPU atlas.
	pending_glyphs: Arc<Mutex<std::collections::HashSet<char>>>,
	// Simple packer state (reset on perform_reupload). Use interior
	// mutability so we can grow the atlas size on demand.
	atlas_size: Arc<Mutex<(u32, u32)>>,
	// Persisted GPU handles created on reupload so the renderer can use them.
	atlas_texture: Arc<Mutex<Option<wgpu::Texture>>>,
	atlas_view: Arc<Mutex<Option<wgpu::TextureView>>>,
	// Monotonic version counter incremented on each perform_reupload so
	// external consumers can detect when the atlas has changed.
	atlas_version: Arc<Mutex<u64>>,
	// Background rasterization worker.
	raster_sender: Option<Sender<RasterRequest>>,
	worker_handle: Option<JoinHandle<()>>,
}

/// Request to rasterize a glyph on the background worker.
struct RasterRequest {
	font_bytes: Vec<u8>,
	scale: f32,
	ch: char,
}

impl GlyphAtlas {
	/// Create a new GlyphAtlas and register a reupload callback.
	pub fn new() -> Self {
		let last = Arc::new(Mutex::new(None));
		let reup = Arc::new(Mutex::new(false));
		let last_weak = Arc::downgrade(&last);
		let reup_weak = Arc::downgrade(&reup);

		// Use Weak references in the id-only callback so we don't hold a
		// strong reference to internal state (avoids retention if
		// unregistering is missed).
		let cb = Arc::new(move |_old: usize, new: usize| {
			if let (Some(last_arc), Some(reup_arc)) = (last_weak.upgrade(), reup_weak.upgrade()) {
				if let Ok(mut g) = last_arc.lock() {
					*g = Some(new);
				}
				if let Ok(mut r) = reup_arc.lock() {
					*r = true;
				}
			}
			// NOTE: real GPU uploads should be scheduled here rather
			// than performed synchronously inside the callback.
		});

		let handle = device_lifecycle::register_device_recreated_callback(cb);

		// Register a device/queue reupload callback that will perform a
		// placeholder atlas upload using the provided `Device` and `Queue`.
		let last_weak_for_reupload = Arc::downgrade(&last);
		let reup_weak_for_reupload = Arc::downgrade(&reup);
		let dev_cb = Arc::new(move |device: &wgpu::Device, queue: &wgpu::Queue| {
			// Small default atlas size for a placeholder.
			let width = 256u32;
			let height = 256u32;
			let size = wgpu::Extent3d {
				width,
				height,
				depth_or_array_layers: 1,
			};

			let desc = wgpu::TextureDescriptor {
				label: Some("glyph_atlas_texture"),
				size,
				mip_level_count: 1,
				sample_count: 1,
				dimension: wgpu::TextureDimension::D2,
				format: wgpu::TextureFormat::Rgba8UnormSrgb,
				usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				view_formats: &[],
			};

			let texture = device.create_texture(&desc);

			let pixel_count = (width as usize) * (height as usize);
			let data = vec![0xFFu8; pixel_count * 4];

			let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("atlas_upload_buffer"),
				contents: &data,
				usage: wgpu::BufferUsages::COPY_SRC,
			});

			let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("atlas_upload_encoder"),
			});

			// Use write_texture for a direct upload of the atlas contents.
			// In wgpu 0.27 the buffer/texture copy types were renamed to
			// `TexelCopyBufferLayout` and `TexelCopyTextureInfo` in the
			// `wgpu-types` crate (aliased here as `wgt`).
			let layout = wgt::TexelCopyBufferLayout {
				offset: 0,
				bytes_per_row: Some((4 * width) as u32),
				rows_per_image: Some(height as u32),
			};
			let dst = wgt::TexelCopyTextureInfo {
				texture: &texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
				aspect: wgpu::TextureAspect::All,
			};
			queue.write_texture(dst, &data, layout, size);

			// Update internal state to reflect reupload (if still live).
			if let Some(last_arc) = last_weak_for_reupload.upgrade() {
				let new_id = (&*device as *const _) as usize;
				if let Ok(mut g) = last_arc.lock() {
					*g = Some(new_id);
				}
			}
			if let Some(reup_arc) = reup_weak_for_reupload.upgrade() {
				if let Ok(mut r) = reup_arc.lock() {
					*r = true;
				}
			}
			drop(texture);
		});

		let reup_handle = device_lifecycle::register_device_reupload_callback(dev_cb);

		// Prepare the CPU-side glyph cache and start a background worker
		// to rasterize glyphs into that cache.
		let glyphs_arc = Arc::new(Mutex::new(HashMap::new()));
		let (tx, rx) = mpsc::channel::<RasterRequest>();
		let glyphs_clone = Arc::clone(&glyphs_arc);
		let worker = thread::spawn(move || {
			while let Ok(req) = rx.recv() {
				// Rasterize on background thread; use rusttype helper.
				if let Some(rt_font) = RtFont::try_from_bytes(&req.font_bytes) {
					let scale = RtScale::uniform(req.scale);
					let v_metrics = rt_font.v_metrics(scale);
					let glyph = rt_font
						.glyph(req.ch)
						.scaled(scale)
						.positioned(rt_point(0.0, v_metrics.ascent));
					if let Some(bb) = glyph.pixel_bounding_box() {
						let width = bb.width() as u32;
						let height = bb.height() as u32;
						let mut pixel_data = vec![0u8; (width * height) as usize];
						glyph.draw(|x, y, v| {
							let px = x as usize;
							let py = y as usize;
							let idx = py * width as usize + px;
							if idx < pixel_data.len() {
								pixel_data[idx] = (v * 255.0).max(0.0).min(255.0) as u8;
							}
						});
						let mut rgba = Vec::with_capacity((width * height * 4) as usize);
						for a in pixel_data.iter() {
							rgba.push(0xFF);
							rgba.push(0xFF);
							rgba.push(0xFF);
							rgba.push(*a);
						}
						if let Ok(mut gmap) = glyphs_clone.lock() {
							gmap.insert(req.ch, (width, height, rgba));
						}
					}
				}
			}
		});

		Self {
			last_device: last,
			reuploaded: reup,
			callback_handle: Some(handle),
			device_reupload_handle: Some(reup_handle),
			glyphs: glyphs_arc,
			glyph_positions: Arc::new(Mutex::new(HashMap::new())),
			pending_glyphs: Arc::new(Mutex::new(std::collections::HashSet::new())),
			atlas_size: Arc::new(Mutex::new((1024, 1024))),
			atlas_texture: Arc::new(Mutex::new(None)),
			atlas_view: Arc::new(Mutex::new(None)),
			atlas_version: Arc::new(Mutex::new(0)),
			raster_sender: Some(tx),
			worker_handle: Some(worker),
		}
	}

	/// Rasterize a single character using the provided font and scale and
	/// store it in the atlas's CPU-side cache. This does not upload to the
	/// GPU immediately; call `perform_reupload` (or wait for device reupload)
	/// to pack and upload cached glyphs.
	/// Rasterize a single character by providing raw font bytes.
	///
	/// Prefer callers to use `rasterize_char_from_bytes` directly. This
	/// convenience method simply forwards to that implementation.
	pub fn rasterize_char(&self, font_bytes: &[u8], scale: f32, ch: char) {
		self.rasterize_char_from_bytes(font_bytes, scale, ch);
	}

	/// Rasterize a glyph from raw font bytes using `rusttype` and cache it.
	/// This is a convenience helper for callers who have font bytes available.
	pub fn rasterize_char_from_bytes(&self, font_bytes: &[u8], scale: f32, ch: char) {
		if let Some(rt_font) = RtFont::try_from_bytes(font_bytes) {
			let scale = RtScale::uniform(scale);
			let v_metrics = rt_font.v_metrics(scale);
			let glyph = rt_font
				.glyph(ch)
				.scaled(scale)
				.positioned(rt_point(0.0, v_metrics.ascent));
			if let Some(bb) = glyph.pixel_bounding_box() {
				let width = bb.width() as u32;
				let height = bb.height() as u32;
				let mut pixel_data = vec![0u8; (width * height) as usize];
				glyph.draw(|x, y, v| {
					let px = x as usize;
					let py = y as usize;
					let idx = py * width as usize + px;
					if idx < pixel_data.len() {
						pixel_data[idx] = (v * 255.0).max(0.0).min(255.0) as u8;
					}
				});
				// expand to RGBA
				let mut rgba = Vec::with_capacity((width * height * 4) as usize);
				for a in pixel_data.iter() {
					rgba.push(0xFF);
					rgba.push(0xFF);
					rgba.push(0xFF);
					rgba.push(*a);
				}
				if let Ok(mut gmap) = self.glyphs.lock() {
					gmap.insert(ch, (width, height, rgba));
					// mark glyph pending for upload
					if let Ok(mut pending) = self.pending_glyphs.lock() {
						pending.insert(ch);
					}
				}
			}
		}
	}

	fn pack_and_upload_from(
		glyphs: &Arc<Mutex<HashMap<char, (u32, u32, Vec<u8>)>>>,
		atlas_size: (u32, u32),
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> Result<(wgpu::Texture, HashMap<char, (u32, u32, u32, u32)>), String> {
		let (aw, ah) = atlas_size;
		let mut atlas = vec![0u8; aw as usize * ah as usize * 4];
		let gmap = glyphs.lock().map_err(|_| "glyphs lock poisoned")?;
		let mut placements: HashMap<char, (u32, u32, u32, u32)> = HashMap::new();
		let mut x: u32 = 0;
		let mut y: u32 = 0;
		let mut row_h: u32 = 0;
		let padding: u32 = 1;
		for (_ch, (gw, gh, bitmap)) in gmap.iter() {
			if *gw == 0 || *gh == 0 {
				continue;
			}
			if x + gw + padding > aw {
				x = 0;
				y = y + row_h + padding;
				row_h = 0;
			}
			if y + gh > ah {
				return Err("atlas full".into());
			}
			for row in 0..*gh {
				let dst_start = (((y + row) * aw + x) * 4) as usize;
				let src_start = (row * gw * 4) as usize;
				let len = (*gw as usize) * 4;
				atlas[dst_start..dst_start + len]
					.copy_from_slice(&bitmap[src_start..src_start + len]);
			}

			// record placement x,y,w,h for this glyph
			// x,y are pixel offsets within the atlas
			placements.insert(*_ch, (x, y, *gw, *gh));
			x += gw + padding;
			if *gh > row_h {
				row_h = *gh;
			}
		}

		let size = wgpu::Extent3d {
			width: aw,
			height: ah,
			depth_or_array_layers: 1,
		};
		let desc = wgpu::TextureDescriptor {
			label: Some("glyph_atlas_texture"),
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		};

		let texture = device.create_texture(&desc);

		let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("atlas_upload_buffer"),
			contents: &atlas,
			usage: wgpu::BufferUsages::COPY_SRC,
		});

		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("atlas_upload_encoder"),
		});
		// Upload packed atlas data using write_texture.
		let layout = wgt::TexelCopyBufferLayout {
			offset: 0,
			bytes_per_row: Some((4 * aw) as u32),
			rows_per_image: Some(ah as u32),
		};
		let dst = wgt::TexelCopyTextureInfo {
			texture: &texture,
			mip_level: 0,
			origin: wgpu::Origin3d::ZERO,
			aspect: wgpu::TextureAspect::All,
		};
		queue.write_texture(dst, &atlas, layout, size);

		Ok((texture, placements))
	}

	// Compute placements for all glyphs using the same packing algorithm as
	// `pack_and_upload_from` but only return the placements map. This is
	// useful for incremental uploads where we want to know where each
	// glyph should be placed inside the atlas without building a full
	// atlas buffer.
	fn compute_placements(
		glyphs: &Arc<Mutex<HashMap<char, (u32, u32, Vec<u8>)>>>,
		atlas_size: (u32, u32),
	) -> Result<HashMap<char, (u32, u32, u32, u32)>, String> {
		let (aw, ah) = atlas_size;
		let gmap = glyphs.lock().map_err(|_| "glyphs lock poisoned")?;
		let mut placements: HashMap<char, (u32, u32, u32, u32)> = HashMap::new();
		let mut x: u32 = 0;
		let mut y: u32 = 0;
		let mut row_h: u32 = 0;
		let padding: u32 = 1;
		for (ch, (gw, gh, _bitmap)) in gmap.iter() {
			if *gw == 0 || *gh == 0 {
				continue;
			}
			if x + gw + padding > aw {
				x = 0;
				y = y + row_h + padding;
				row_h = 0;
			}
			if y + gh > ah {
				return Err("atlas full".into());
			}
			placements.insert(*ch, (x, y, *gw, *gh));
			x += gw + padding;
			if *gh > row_h {
				row_h = *gh;
			}
		}
		Ok(placements)
	}

	/// Perform an incremental upload for newly rasterized glyphs. This will
	/// compute placements and copy only the newly required glyph regions
	/// into the existing atlas texture when possible. If the atlas texture
	/// does not exist or the atlas would overflow, it will fall back to a
	/// full reupload via `pack_and_upload_from`.
	pub fn perform_incremental_reupload(
		&self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> Result<(), String> {
		// If there are no pending glyphs, nothing to do.
		let pending: Vec<char> = {
			let p = self
				.pending_glyphs
				.lock()
				.map_err(|_| "pending lock poisoned")?;
			p.iter().copied().collect()
		};
		if pending.is_empty() {
			return Ok(());
		}

		// Compute placements for the current glyph set.
		let atlas_sz = {
			let s = self
				.atlas_size
				.lock()
				.map_err(|_| "atlas_size lock poisoned")?;
			*s
		};
		let placements = match Self::compute_placements(&self.glyphs, atlas_sz) {
			Ok(p) => p,
			Err(e) => {
				// Atlas full or packing failed; fallback to full reupload
				let (tex, plats) =
					Self::pack_and_upload_from(&self.glyphs, atlas_sz, device, queue)?;
				if let Ok(mut at) = self.atlas_texture.lock() {
					*at = Some(tex);
				}
				if let Ok(mut gp) = self.glyph_positions.lock() {
					*gp = plats;
				}
				if let Ok(mut ver) = self.atlas_version.lock() {
					*ver = ver.wrapping_add(1);
				}
				if let Ok(mut pset) = self.pending_glyphs.lock() {
					pset.clear();
				}
				return Ok(());
			}
		};

		// Ensure we have an existing atlas texture to write into. If not,
		// create a fresh texture and upload full atlas.
		let mut need_full = false;
		let atlas_exists = {
			let guard = self
				.atlas_texture
				.lock()
				.map_err(|_| "atlas_texture lock poisoned")?;
			guard.is_some()
		};
		if !atlas_exists {
			need_full = true;
		}
		if need_full {
			let (tex, plats) = Self::pack_and_upload_from(&self.glyphs, atlas_sz, device, queue)?;
			if let Ok(mut at) = self.atlas_texture.lock() {
				*at = Some(tex);
			}
			if let Ok(mut gp) = self.glyph_positions.lock() {
				*gp = plats;
			}
			if let Ok(mut ver) = self.atlas_version.lock() {
				*ver = ver.wrapping_add(1);
			}
			if let Ok(mut pset) = self.pending_glyphs.lock() {
				pset.clear();
			}
			return Ok(());
		}

		// We have an atlas texture and computed placements. For each glyph
		// that is pending and not present in glyph_positions, upload its
		// bitmap into the atlas at the computed origin.
		let atlas = self
			.atlas_texture
			.lock()
			.map_err(|_| "atlas lock poisoned")?;
		let atlas_ref = atlas
			.as_ref()
			.ok_or_else(|| "missing atlas texture".to_string())?;

		let mut gp = self
			.glyph_positions
			.lock()
			.map_err(|_| "glyph_positions lock poisoned")?;
		let gmap = self.glyphs.lock().map_err(|_| "glyphs lock poisoned")?;

		// For each pending glyph, if it's not already in gp, copy its bitmap into the atlas texture at placements[&ch]
		for ch in pending.iter() {
			if gp.contains_key(ch) {
				// already uploaded
				continue;
			}
			if let Some((x, y, w, h)) = placements.get(ch) {
				if let Some((_gw, _gh, bitmap)) = gmap.get(ch) {
					// create a small buffer for this glyph and copy into the atlas
					let row_bytes = (*w as usize) * 4;
					let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
						label: Some("glyph-sub-upload"),
						contents: &bitmap,
						usage: wgpu::BufferUsages::COPY_SRC,
					});
					let mut encoder =
						device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
							label: Some("glyph-sub-encoder"),
						});
					// Sub-upload the glyph bitmap into the existing atlas texture.
					let layout = wgt::TexelCopyBufferLayout {
						offset: 0,
						bytes_per_row: Some(row_bytes as u32),
						rows_per_image: Some(*h as u32),
					};
					let dst = wgt::TexelCopyTextureInfo {
						texture: atlas_ref,
						mip_level: 0,
						origin: wgpu::Origin3d { x: *x, y: *y, z: 0 },
						aspect: wgpu::TextureAspect::All,
					};
					queue.write_texture(
						dst,
						&bitmap,
						layout,
						wgpu::Extent3d {
							width: *w,
							height: *h,
							depth_or_array_layers: 1,
						},
					);
					// record placement
					gp.insert(*ch, (*x, *y, *w, *h));
				}
			}
		}

		// mark reuploaded and bump version
		if let Ok(mut r) = self.reuploaded.lock() {
			*r = true;
		}
		if let Ok(mut ver) = self.atlas_version.lock() {
			*ver = ver.wrapping_add(1);
		}
		if let Ok(mut pset) = self.pending_glyphs.lock() {
			pset.clear();
		}

		Ok(())
	}

	/// Return the last device id recorded by the atlas (if any).
	pub fn last_device_id(&self) -> Option<usize> {
		if let Ok(g) = self.last_device.lock() {
			*g
		} else {
			None
		}
	}

	/// Whether the atlas has performed (or would perform) a reupload
	/// after the last device recreation.
	pub fn reuploaded(&self) -> bool {
		if let Ok(r) = self.reuploaded.lock() {
			*r
		} else {
			false
		}
	}

	/// Clear the reuploaded flag (useful for tests).
	pub fn clear_reuploaded(&self) {
		if let Ok(mut r) = self.reuploaded.lock() {
			*r = false;
		}
	}

	/// Perform an actual GPU reupload using the provided device and queue.
	///
	/// This creates a small RGBA texture and uploads placeholder data into
	/// it. Real implementations should replace the placeholder data with
	/// rasterized glyph bitmaps and manage persistent atlas textures.
	pub fn perform_reupload(
		&self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> Result<(), String> {
		// Pack CPU-side cached glyph bitmaps into an atlas and upload it.
		let atlas_sz = {
			let s = self
				.atlas_size
				.lock()
				.map_err(|_| "atlas_size lock poisoned")?;
			*s
		};
		let (texture, placements) =
			match Self::pack_and_upload_from(&self.glyphs, atlas_sz, device, queue) {
				Ok(t) => t,
				Err(e) => return Err(e),
			};

		// Create a texture view for sampling
		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

		// Persist the GPU handles so renderers can access them later.
		if let Ok(mut at) = self.atlas_texture.lock() {
			*at = Some(texture);
		}
		if let Ok(mut av) = self.atlas_view.lock() {
			*av = Some(view);
		}

		// Bump the atlas version so callers can detect the change and
		// recreate bind-groups if necessary.
		if let Ok(mut ver) = self.atlas_version.lock() {
			*ver = ver.wrapping_add(1);
		}

		// store placements into glyph_positions map
		if let Ok(mut gp) = self.glyph_positions.lock() {
			*gp = placements;
		}

		// Record that a reupload occurred and update last device id using
		// the active device if available.
		if let Some(active) = crate::oal::device_lifecycle::get_active_device() {
			if let Ok(mut g) = self.last_device.lock() {
				*g = Some(active);
			}
		}
		if let Ok(mut r) = self.reuploaded.lock() {
			*r = true;
		}

		// Note: we intentionally don't hold onto the `texture` handle here.
		// A real implementation would store it in the atlas state for use
		// by the renderer.

		Ok(())
	}

	/// Queue an asynchronous rasterization request for a glyph. This will
	/// rasterize the glyph on a background thread and store the CPU bitmap
	/// in the atlas's glyph cache. The actual GPU upload still happens
	/// during `perform_reupload` when a `wgpu::Device`/`Queue` are
	/// available.
	pub fn request_rasterize(
		&self,
		font_bytes: Vec<u8>,
		scale: f32,
		ch: char,
	) -> Result<(), String> {
		if let Some(tx) = &self.raster_sender {
			tx.send(RasterRequest {
				font_bytes,
				scale,
				ch,
			})
			.map_err(|e| format!("raster request send failed: {}", e))?;
			Ok(())
		} else {
			Err("raster worker not available".into())
		}
	}

	/// Return a freshly-created `TextureView` for the current atlas texture
	/// if one exists. Callers should create bind groups from this view or
	/// otherwise sample it. This clones the view by creating a new view from
	/// the stored `wgpu::Texture` (cheap operation).
	pub fn get_atlas_view(&self) -> Option<wgpu::TextureView> {
		if let Ok(at) = self.atlas_texture.lock() {
			if let Some(ref tex) = *at {
				return Some(tex.create_view(&wgpu::TextureViewDescriptor::default()));
			}
		}
		None
	}

	/// Return a monotonic version token that increments whenever the atlas
	/// texture/view is replaced via `perform_reupload`.
	pub fn atlas_version(&self) -> u64 {
		if let Ok(v) = self.atlas_version.lock() {
			*v
		} else {
			0
		}
	}

	/// Create a bind group for sampling the atlas using the provided
	/// `BindGroupLayout` and `Sampler`. The layout is expected to have a
	/// texture binding at `binding=0` and a sampler at `binding=1` (caller
	/// responsibility to ensure layout matches shader expectations).
	pub fn create_bind_group_for_atlas(
		&self,
		device: &wgpu::Device,
		layout: &wgpu::BindGroupLayout,
		sampler: &wgpu::Sampler,
	) -> Option<wgpu::BindGroup> {
		if let Ok(at) = self.atlas_texture.lock() {
			if let Some(ref tex) = *at {
				let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
				let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
					label: Some("glyph_atlas_bind_group"),
					layout,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::TextureView(&view),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::Sampler(sampler),
						},
					],
				});
				return Some(bg);
			}
		}
		None
	}
}

// A process-global glyph atlas singleton convenience for simple apps
// and the renderer path. Callers may still construct their own
// `GlyphAtlas` instances if they prefer. We expose a getter that lazily
// initializes a shared atlas.
static GLOBAL_GLYPH_ATLAS: OnceLock<Arc<Mutex<GlyphAtlas>>> = OnceLock::new();

/// Return a shared global `GlyphAtlas` instance (lazy-initialized).
pub fn global_glyph_atlas() -> Arc<Mutex<GlyphAtlas>> {
	GLOBAL_GLYPH_ATLAS
		.get_or_init(|| Arc::new(Mutex::new(GlyphAtlas::new())))
		.clone()
}

impl GlyphAtlas {
	/// Return pixel placement for a glyph if available: (x,y,w,h) in pixels.
	pub fn get_glyph_placement(&self, ch: char) -> Option<(u32, u32, u32, u32)> {
		if let Ok(gp) = self.glyph_positions.lock() {
			gp.get(&ch).cloned()
		} else {
			None
		}
	}

	/// Return configured atlas size in pixels.
	pub fn atlas_size(&self) -> (u32, u32) {
		if let Ok(s) = self.atlas_size.lock() {
			*s
		} else {
			(0, 0)
		}
	}
}

impl Drop for GlyphAtlas {
	fn drop(&mut self) {
		if let Some(id) = self.callback_handle.take() {
			device_lifecycle::unregister_device_recreated_callback(id);
		}
		if let Some(id) = self.device_reupload_handle.take() {
			device_lifecycle::unregister_device_reupload_callback(id);
		}
		// Shut down the raster worker cleanly.
		// Dropping the sender will cause the worker loop to exit.
		self.raster_sender.take();
		if let Some(handle) = self.worker_handle.take() {
			let _ = handle.join();
		}
	}
}
