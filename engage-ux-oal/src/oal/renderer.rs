use crate::errors::Result;
use crate::oal::{Canvas, DeviceMetrics, Unit};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Called when a device is replaced/ recreated. This hook allows the
/// renderer module to invalidate any GPU caches tied to a previous
/// device instance. Currently this is a no-op placeholder; backends and
/// caches that hold device-local handles should implement more precise
/// invalidation here in the future.
pub fn invalidate_gpu_caches_for_device(old_device_id: usize) {
	// Remove cached GPU resources associated with the old device id so
	// they may be re-created when the new device becomes available.
	if let Some(lock) = GPU_DEVICE_CACHE.get() {
		if let Ok(mut map) = lock.lock() {
			map.remove(&old_device_id);
		}
	}
}

// Per-device cached GPU resources (pipelines, bind group layouts, etc.).
struct CachedDeviceResources {
	text_pipeline: Option<wgpu::RenderPipeline>,
	atlas_bgl: Option<wgpu::BindGroupLayout>,
	// Cached atlas bind-group + the atlas version it was created for.
	atlas_bind_group: Option<(u64, wgpu::BindGroup)>,
	// Per-device sampler for sampling the atlas. Cached to avoid
	// recreating a sampler each frame.
	atlas_sampler: Option<wgpu::Sampler>,
	// Cached shader module for text pipeline to avoid recompilation.
	text_shader: Option<wgpu::ShaderModule>,
	// Reusable vertex/index buffers for batched text geometry.
	vertex_buf: Option<wgpu::Buffer>,
	vertex_capacity: usize,
	index_buf: Option<wgpu::Buffer>,
	index_capacity: usize,
}

static GPU_DEVICE_CACHE: OnceLock<Mutex<HashMap<usize, CachedDeviceResources>>> = OnceLock::new();

/// Renderer trait - minimal contract for OAL renderers.
pub trait Renderer: Send {
	/// Render the given canvas and view into the backing surface.
	fn render(&mut self, canvas: &Canvas, metrics: &DeviceMetrics, unit: Unit) -> Result<()>;

	/// Optional GPU render submission hook.
	///
	/// Implementors may return a boxed `RenderCallback` which will be invoked
	/// on the backend's GPU/context-owning thread with a backend-specific
	/// context provided as `&mut dyn Any` (for example a `WgpuRenderContext`).
	/// The default implementation returns `None`, indicating no GPU submission
	/// is available and the OAL should fall back to software rendering via
	/// `render`.
	fn try_gpu_render(
		&mut self,
		canvas: &Canvas,
		metrics: &DeviceMetrics,
		unit: Unit,
	) -> Option<Box<dyn crate::oal::backend::RenderCallback>> {
		let _ = (canvas, metrics, unit);
		None
	}
}

/// A noop software renderer for tests and headless operation.
pub struct NoopRenderer;

impl NoopRenderer {
	/// Create a new `NoopRenderer` instance. Provided for tests and
	/// headless operation where rendering is intentionally a no-op.
	pub fn new() -> Self {
		Self
	}
}

impl Renderer for NoopRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		// Intentionally no-op.
		Ok(())
	}
}

/// Choose the best available renderer: placeholder; keep simple for now.
#[allow(dead_code)]
pub fn choose_renderer() -> Box<dyn Renderer> {
	Box::new(NoopRenderer::new())
}

/// A simple Skia-based software renderer fallback.
#[cfg(feature = "skia")]
pub struct SkiaSoftwareRenderer {
	// Placeholder for renderer configuration (fonts, caching, etc.)
}

#[cfg(feature = "skia")]
impl SkiaSoftwareRenderer {
	pub fn new() -> Self {
		Self {}
	}
}

#[cfg(feature = "skia")]
impl Renderer for SkiaSoftwareRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		// Simple software path using skia-safe would go here. For now
		// this is a placeholder that indicates software rendering occurred.
		Ok(())
	}

	// No GPU submission by default for the software renderer.
}

// A minimal wgpu-backed renderer that returns a GPU job which records a
// simple render pass into the provided `WgpuRenderContext`. This renderer
// is available only when the native winit backend is enabled since it
// requires the `WgpuRenderContext` type.
#[cfg(feature = "native-winit")]
pub struct WgpuRenderer {
	// Simple color to clear/draw with; in a real renderer this would be
	// replaced by pipeline/shader state and resources.
	pub color: wgpu::Color,
}

#[cfg(feature = "native-winit")]
impl WgpuRenderer {
	pub fn new(color: wgpu::Color) -> Self {
		Self { color }
	}
}

#[cfg(feature = "native-winit")]
impl Renderer for WgpuRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		// Fallback synchronous path is a no-op for this minimal renderer.
		Ok(())
	}

	fn try_gpu_render(
		&mut self,
		_canvas: &Canvas,
		_metrics: &DeviceMetrics,
		_unit: Unit,
	) -> Option<Box<dyn crate::oal::backend::RenderCallback>> {
		// The minimal WgpuRenderer will not provide a GPU job that draws
		// recorded `Canvas` primitives here because we need to capture the
		// canvas command list at the time the renderer is asked. Instead,
		// we return a closure that expects the backend to invoke it during
		// Present with a `WgpuRenderContext` available. The closure will
		// create a simple instanced quad pipeline and draw rectangle-like
		// primitives; circles/rounded rects will be rendered as SDF in the
		// fragment shader when possible.
		let prims = _canvas.drain_commands();

		// Define a typed instance payload for GPU uploads. Using a
		// #[repr(C)] struct with bytemuck makes the upload safe and clear
		// without unsafe code.
		use bytemuck::{Pod, Zeroable};

		#[repr(C)]
		#[derive(Copy, Clone, Pod, Zeroable)]
		struct Instance {
			center: [f32; 2],
			size: [f32; 2],
			color: [f32; 4],
			shape: f32,
			radii: [f32; 2],
			stroke: f32,
			// padding to 12 floats isn't required because fields total 12 f32s
		}

		Some(Box::new(
			move |backend_ctx: Box<dyn std::any::Any + Send>| -> Result<Box<dyn std::any::Any + Send>> {
				use crate::errors::OalError;
				use crate::oal::canvas::Primitive;
				use crate::oal::winit_adapter::WgpuRenderContextOwned;
				use wgpu::util::DeviceExt;

				// We expect the backend to supply a WgpuRenderContextOwned with
				// a valid encoder and an acquired view during Present.
				let mut owned = match backend_ctx.downcast::<WgpuRenderContextOwned>() {
					Ok(b) => *b,
					Err(_) => return Err(OalError::Renderer("unexpected backend context type".into())),
				};

				if let (Some(ref view), Some(ref mut encoder)) = (owned.view.as_ref(), owned.encoder.as_mut()) {
					let device = &*owned.device;
					let queue = &*owned.queue;
					let config = &owned.config;
					let surface_w = config.width as f32;
					let surface_h = config.height as f32;

						// For this minimal pipeline we create a small unit-quad
						// vertex buffer and an index buffer.
						let vertex_data: [f32; 8] = [-0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5];
						let vertex_buf =
							device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
								label: Some("unit-quad-verts"),
								contents: bytemuck::cast_slice(&vertex_data),
								usage: wgpu::BufferUsages::VERTEX,
							});

						let index_data: [u16; 6] = [0, 1, 2, 2, 1, 3];
						let index_buf =
							device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
								label: Some("unit-quad-indices"),
								contents: bytemuck::cast_slice(&index_data),
								usage: wgpu::BufferUsages::INDEX,
							});

						// Gather primitives from the canvas snapshot captured earlier
						// by the renderer call site. We expect the renderer to have
						// drained commands into the job's closure capture; if none
						// exist, we simply perform a clear and return.
						// NOTE: The closure is invoked during Present; callers should
						// arrange to capture commands when creating the job.

						// Build a WGSL shader that renders rectangles, circles and
						// rounded rects using SDFs with AA. Instances now include:
						// center.xy, size.xy, color.rgba, shape_flag, radii.xy, stroke_width.
						let shader_src = r#"
							struct VertexOut {
								@builtin(position) position: vec4<f32>;
								@location(0) color: vec4<f32>;
								@location(1) local: vec2<f32>;
								@location(2) half_size: vec2<f32>;
								@location(3) shape: f32;
								@location(4) radii: vec2<f32>;
								@location(5) stroke: f32;
							};

							@vertex
							fn vs(
								@location(0) pos: vec2<f32>,
								@location(1) offset: vec2<f32>,
								@location(2) half_size: vec2<f32>,
								@location(3) color: vec4<f32>,
								@location(4) shape: f32,
								@location(5) radii: vec2<f32>,
								@location(6) stroke: f32
							) -> VertexOut {
								var out: VertexOut;
								let local = pos * half_size;
								let p = offset + local;
								out.position = vec4<f32>(p.x, p.y, 0.0, 1.0);
								out.color = color;
								out.local = local;
								out.half_size = half_size;
								out.shape = shape;
								out.radii = radii;
								out.stroke = stroke;
								return out;
							}

							fn rect_sdf(p: vec2<f32>, half_ext: vec2<f32>) -> f32 {
								let q = abs(p) - half_ext;
								return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0);
							}

							fn rounded_rect_sdf(p: vec2<f32>, half_ext: vec2<f32>, rad: vec2<f32>) -> f32 {
								// Conservative approach: use min radius for SDF offset
								let r = vec2<f32>(min(rad.x, rad.y), min(rad.x, rad.y));
								let q = abs(p) - (half_ext - r);
								return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r.x;
							}

							@fragment
							fn fs(in: VertexOut) -> @location(0) vec4<f32> {
								let half_ext = in.half_size * 0.5;
								var dist: f32;
								if (in.shape < 0.5) {
									// rectangle
									dist = rect_sdf(in.local, half_ext);
								} else if (in.shape < 1.5) {
									// circle
									let r = min(half_ext.x, half_ext.y);
									dist = length(in.local) - r;
								} else {
									// rounded rect
									dist = rounded_rect_sdf(in.local, half_ext, in.radii);
								}

								let aa = fwidth(dist);
								var alpha: f32;
								if (in.stroke > 0.0) {
									let half_stroke = in.stroke * 0.5;
									let d = abs(dist) - half_stroke;
									alpha = if (aa > 0.0) { smoothstep(0.5 * aa, -0.5 * aa, d) } else { if (d < 0.0) { 1.0 } else { 0.0 } };
								} else {
									alpha = if (aa > 0.0) { smoothstep(0.5 * aa, -0.5 * aa, dist) } else { if (dist < 0.0) { 1.0 } else { 0.0 } };
								}

								return vec4<f32>(in.color.rgb, in.color.a * alpha);
							}
						"#;

						let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
							label: Some("gpu-prims-shader"),
							source: wgpu::ShaderSource::Wgsl(shader_src.into()),
						});

						// Pipeline layout and pipeline
						let pipeline_layout =
							device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
								label: Some("prims-pipeline-layout"),
								bind_group_layouts: &[],
								push_constant_ranges: &[],
							});

						let pipeline =
							device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
								label: Some("prims-pipeline"),
								layout: Some(&pipeline_layout),
								vertex: wgpu::VertexState {
									module: &shader,
									entry_point: Some("vs"),
									compilation_options: wgpu::PipelineCompilationOptions::default(
									),
									buffers: &[
										wgpu::VertexBufferLayout {
											array_stride: std::mem::size_of::<[f32; 2]>()
												as wgpu::BufferAddress,
											step_mode: wgpu::VertexStepMode::Vertex,
											attributes: &[wgpu::VertexAttribute {
												offset: 0,
												shader_location: 0,
												format: wgpu::VertexFormat::Float32x2,
											}],
										},
										// Instance buffer: offset(vec2) + half_size(vec2) + color(vec4) + shape(float)
										wgpu::VertexBufferLayout {
											array_stride: std::mem::size_of::<Instance>()
												as wgpu::BufferAddress,
											step_mode: wgpu::VertexStepMode::Instance,
											attributes: &[
												wgpu::VertexAttribute {
													offset: 0,
													shader_location: 1,
													format: wgpu::VertexFormat::Float32x2,
												},
												wgpu::VertexAttribute {
													offset: 8,
													shader_location: 2,
													format: wgpu::VertexFormat::Float32x2,
												},
												wgpu::VertexAttribute {
													offset: 16,
													shader_location: 3,
													format: wgpu::VertexFormat::Float32x4,
												},
												wgpu::VertexAttribute {
													offset: 32,
													shader_location: 4,
													format: wgpu::VertexFormat::Float32,
												},
												wgpu::VertexAttribute {
													offset: 36,
													shader_location: 5,
													format: wgpu::VertexFormat::Float32x2,
												},
												wgpu::VertexAttribute {
													offset: 44,
													shader_location: 6,
													format: wgpu::VertexFormat::Float32,
												},
											],
										},
									],
								},
								fragment: Some(wgpu::FragmentState {
									module: &shader,
									entry_point: Some("fs"),
									compilation_options: wgpu::PipelineCompilationOptions::default(
									),
									targets: &[Some(wgpu::ColorTargetState {
										format: config.format,
										blend: Some(wgpu::BlendState::ALPHA_BLENDING),
										write_mask: wgpu::ColorWrites::ALL,
									})],
								}),
								primitive: wgpu::PrimitiveState::default(),
								depth_stencil: None,
								multisample: wgpu::MultisampleState::default(),
								multiview: None,
								cache: None,
							});

						// Build instance data from the captured canvas primitives.
						// We use the typed `Instance` struct defined above and
						// push Instance values into a Vec<Instance>, then upload
						// it directly with bytemuck::cast_slice.
						let mut instances: Vec<Instance> = Vec::new();
						// Collect per-frame text vertices/indices so we can
						// upload a single VBO/IBO for all glyphs in the frame
						// rather than allocating per-glyph buffers.
						let mut text_vertices: Vec<f32> = Vec::new();
						let mut text_indices: Vec<u16> = Vec::new();
						let mut next_index: u16 = 0;
						for p in prims.iter() {
							match p {
								Primitive::Rect { rect, .. } => {
									let origin = rect.top_left.clone();
									let size = (rect.width, rect.height);
									let center_x = origin.x + size.0 * 0.5;
									let center_y = origin.y + size.1 * 0.5;
									// Convert to NDC [-1,1]
									let ndc_x = center_x / surface_w * 2.0 - 1.0;
									let ndc_y = 1.0 - (center_y / surface_h * 2.0);
									let size_x = size.0 / surface_w;
									let size_y = size.1 / surface_h;
									// Resolve color from fill_style then line_style (stroke)
									let mut c = engage_ux_core::Color::from_rgb(0, 0, 0);
									let mut stroke_w = 0.0_f32;
									if let Some(fs) = rect.fill_style.as_ref() {
										if let Some(fc) = fs.color.as_ref() {
											c = fc.clone();
										}
									}
									if let Some(ls) = rect.line_style.as_ref() {
										if let Some(lc) = ls.color.as_ref() {
											c = lc.clone();
										}
										stroke_w = ls.stroke_width.unwrap_or(0.0);
									}
									instances.push(Instance {
										center: [ndc_x, ndc_y],
										size: [size_x, size_y],
										color: [
											c.red as f32 / 255.0,
											c.green as f32 / 255.0,
											c.blue as f32 / 255.0,
											c.alpha as f32 / 255.0,
										],
										shape: 0.0,
										radii: [0.0, 0.0],
										stroke: stroke_w / surface_w,
									});
								}
								Primitive::Circle { circle, .. } => {
									let center_x = circle.center.x;
									let center_y = circle.center.y;
									let ndc_x = center_x / surface_w * 2.0 - 1.0;
									let ndc_y = 1.0 - (center_y / surface_h * 2.0);
									let size_x = (circle.radius * 2.0) / surface_w;
									let size_y = (circle.radius * 2.0) / surface_h;
									let mut c = engage_ux_core::Color::from_rgb(0, 0, 0);
									let mut stroke_w = 0.0_f32;
									if let Some(fs) = circle.fill_style.as_ref() {
										if let Some(fc) = fs.color.as_ref() {
											c = fc.clone();
										}
									}
									if let Some(ls) = circle.line_style.as_ref() {
										if let Some(lc) = ls.color.as_ref() {
											c = lc.clone();
										}
										stroke_w = ls.stroke_width.unwrap_or(0.0);
									}
									instances.push(Instance {
										center: [ndc_x, ndc_y],
										size: [size_x, size_y],
										color: [
											c.red as f32 / 255.0,
											c.green as f32 / 255.0,
											c.blue as f32 / 255.0,
											c.alpha as f32 / 255.0,
										],
										shape: 1.0,
										radii: [0.0, 0.0],
										stroke: stroke_w / surface_w,
									});
								}
								Primitive::RoundedRect { rect, .. } => {
									let origin = rect.top_left.clone();
									let size = (rect.width, rect.height);
									let center_x = origin.x + size.0 * 0.5;
									let center_y = origin.y + size.1 * 0.5;
									let ndc_x = center_x / surface_w * 2.0 - 1.0;
									let ndc_y = 1.0 - (center_y / surface_h * 2.0);
									let size_x = size.0 / surface_w;
									let size_y = size.1 / surface_h;
									// approximate radii using top-left/bottom-right averages
									let rad_x = rect
										.radius_top_left
										.min(rect.radius_top_right)
										.min(rect.radius_bottom_right)
										.min(rect.radius_bottom_left);
									let rad_y = rad_x;
									let mut c = engage_ux_core::Color::from_rgb(0, 0, 0);
									let mut stroke_w = 0.0_f32;
									if let Some(fs) = rect.fill_style.as_ref() {
										if let Some(fc) = fs.color.as_ref() {
											c = fc.clone();
										}
									}
									if let Some(ls) = rect.line_style.as_ref() {
										if let Some(lc) = ls.color.as_ref() {
											c = lc.clone();
										}
										stroke_w = ls.stroke_width.unwrap_or(0.0);
									}
									instances.push(Instance {
										center: [ndc_x, ndc_y],
										size: [size_x, size_y],
										color: [
											c.red as f32 / 255.0,
											c.green as f32 / 255.0,
											c.blue as f32 / 255.0,
											c.alpha as f32 / 255.0,
										],
										shape: 2.0,
										radii: [rad_x / surface_w, rad_y / surface_h],
										stroke: if stroke_w > 0.0 {
											stroke_w / surface_w
										} else {
											0.0
										},
									});
								}
								Primitive::Text { text, .. } => {
									// Batch glyph quads into per-frame vertex/index arrays
									// to reduce GPU allocations.
									let atlas_handle = crate::oal::text::global_glyph_atlas();
									let atlas = atlas_handle.lock().unwrap();
									let (aw, ah) = atlas.atlas_size();
									drop(atlas);

									// Resolve text content, position and color from the core Text primitive
									let content = &text.content;
									let pos = (text.position.x, text.position.y);
									let mut color = engage_ux_core::Color::from_rgb(0, 0, 0);
									if let Some(fs) = text.fill_style.as_ref() {
										if let Some(fc) = fs.color.as_ref() {
											color = fc.clone();
										}
									}
									if let Some(ls) = text.line_style.as_ref() {
										if let Some(lc) = ls.color.as_ref() {
											color = lc.clone();
										}
									}

									let mut cursor_x = 0.0_f32;
									for ch in content.chars() {
										if let Some((gx, gy, gw, gh)) =
											atlas_handle.lock().unwrap().get_glyph_placement(ch)
										{
											let u0 = gx as f32 / aw as f32;
											let v0 = gy as f32 / ah as f32;
											let u1 = (gx + gw) as f32 / aw as f32;
											let v1 = (gy + gh) as f32 / ah as f32;

											let px0 = pos.0 + cursor_x;
											let py0 = pos.1;
											let px1 = px0 + gw as f32;
											let py1 = py0 + gh as f32;
											let nx0 = px0 / surface_w * 2.0 - 1.0;
											let ny0 = 1.0 - (py0 / surface_h * 2.0);
											let nx1 = px1 / surface_w * 2.0 - 1.0;
											let ny1 = 1.0 - (py1 / surface_h * 2.0);

											// vertex: pos.xy, uv.xy, color.rgba (8 floats)
											let vv0: [f32; 8] = [
												nx0,
												ny1,
												u0,
												v1,
												color.red as f32 / 255.0,
												color.green as f32 / 255.0,
												color.blue as f32 / 255.0,
												color.alpha as f32 / 255.0,
											];
											let vv1: [f32; 8] = [
												nx1,
												ny1,
												u1,
												v1,
												color.red as f32 / 255.0,
												color.green as f32 / 255.0,
												color.blue as f32 / 255.0,
												color.alpha as f32 / 255.0,
											];
											let vv2: [f32; 8] = [
												nx0,
												ny0,
												u0,
												v0,
												color.red as f32 / 255.0,
												color.green as f32 / 255.0,
												color.blue as f32 / 255.0,
												color.alpha as f32 / 255.0,
											];
											let vv3: [f32; 8] = [
												nx1,
												ny0,
												u1,
												v0,
												color.red as f32 / 255.0,
												color.green as f32 / 255.0,
												color.blue as f32 / 255.0,
												color.alpha as f32 / 255.0,
											];

											text_vertices.extend_from_slice(&vv0);
											text_vertices.extend_from_slice(&vv1);
											text_vertices.extend_from_slice(&vv2);
											text_vertices.extend_from_slice(&vv3);

											text_indices.push(next_index);
											text_indices.push(next_index + 1);
											text_indices.push(next_index + 2);
											text_indices.push(next_index + 2);
											text_indices.push(next_index + 1);
											text_indices.push(next_index + 3);
											next_index = next_index.wrapping_add(4);

											cursor_x += gw as f32;
										}
									}
								}
								_ => {}
							}
						}

						let instance_count = instances.len() as u32;
						let instance_buf = if instance_count > 0 {
							Some(
								device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
									label: Some("prims-instances"),
									contents: bytemuck::cast_slice(&instances),
									usage: wgpu::BufferUsages::VERTEX,
								}),
							)
						} else {
							None
						};

						// Begin render pass
						let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
							label: Some("prims-pass"),
							color_attachments: &[Some(wgpu::RenderPassColorAttachment {
								view,
								resolve_target: None,
								depth_slice: None,
								ops: wgpu::Operations {
									load: wgpu::LoadOp::Load,
									store: wgpu::StoreOp::Store,
								},
							})],
							depth_stencil_attachment: None,
							occlusion_query_set: None,
							timestamp_writes: None,
						});

						rpass.set_pipeline(&pipeline);
						rpass.set_vertex_buffer(0, vertex_buf.slice(..));
						if let Some(buf) = instance_buf.as_ref() {
							rpass.set_vertex_buffer(1, buf.slice(..));
						}
						rpass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);

						// Issue an instanced draw using the built instance buffer
						// when we have instances; otherwise draw a single unit
						// quad to keep the frame valid.
						if instance_count > 0 {
							rpass.draw_indexed(0..6, 0, 0..instance_count);
						} else {
							rpass.draw_indexed(0..6, 0, 0..1);
						}

						// Render text quads collected earlier using a simple textured shader
						if !text_vertices.is_empty() {
							let atlas_bgl =
								device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
									label: Some("glyph-atlas-bgl-temp"),
									entries: &[
										wgpu::BindGroupLayoutEntry {
											binding: 0,
											visibility: wgpu::ShaderStages::FRAGMENT,
											ty: wgpu::BindingType::Texture {
												multisampled: false,
												view_dimension: wgpu::TextureViewDimension::D2,
												sample_type: wgpu::TextureSampleType::Float {
													filterable: true,
												},
											},
											count: None,
										},
										wgpu::BindGroupLayoutEntry {
											binding: 1,
											visibility: wgpu::ShaderStages::FRAGMENT,
											ty: wgpu::BindingType::Sampler(
												wgpu::SamplerBindingType::Filtering,
											),
											count: None,
										},
									],
								});
							let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
								label: Some("glyph_atlas_sampler"),
								address_mode_u: wgpu::AddressMode::ClampToEdge,
								address_mode_v: wgpu::AddressMode::ClampToEdge,
								address_mode_w: wgpu::AddressMode::ClampToEdge,
								mag_filter: wgpu::FilterMode::Linear,
								min_filter: wgpu::FilterMode::Linear,
								mipmap_filter: wgpu::FilterMode::Nearest,
								..Default::default()
							});
							let atlas_handle = crate::oal::text::global_glyph_atlas();
							let atlas_bg = atlas_handle
								.lock()
								.unwrap()
								.create_bind_group_for_atlas(device, &atlas_bgl, &atlas_sampler);
							if let Some(bg) = atlas_bg {
								// Use (and cache) a per-device bind-group-layout and
								// text pipeline. The cache is keyed by the device
								// pointer address to survive device recreations.
								let device_id = device as *const _ as usize;
								let cache_lock =
									GPU_DEVICE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
								let mut cache = cache_lock.lock().unwrap();
								let entry = cache.entry(device_id).or_insert_with(|| {
									CachedDeviceResources {
										text_pipeline: None,
										atlas_bgl: None,
										atlas_bind_group: None,
										atlas_sampler: None,
										text_shader: None,
										vertex_buf: None,
										vertex_capacity: 0,
										index_buf: None,
										index_capacity: 0,
									}
								});

								// Ensure atlas bind-group-layout exists in cache
								if entry.atlas_bgl.is_none() {
									let bgl = device.create_bind_group_layout(
										&wgpu::BindGroupLayoutDescriptor {
											label: Some("glyph-atlas-bgl"),
											entries: &[
												wgpu::BindGroupLayoutEntry {
													binding: 0,
													visibility: wgpu::ShaderStages::FRAGMENT,
													ty: wgpu::BindingType::Texture {
														multisampled: false,
														view_dimension:
															wgpu::TextureViewDimension::D2,
														sample_type:
															wgpu::TextureSampleType::Float {
																filterable: true,
															},
													},
													count: None,
												},
												wgpu::BindGroupLayoutEntry {
													binding: 1,
													visibility: wgpu::ShaderStages::FRAGMENT,
													ty: wgpu::BindingType::Sampler(
														wgpu::SamplerBindingType::Filtering,
													),
													count: None,
												},
											],
										},
									);
									entry.atlas_bgl = Some(bgl);
								}
								let atlas_bgl_ref = entry.atlas_bgl.as_ref().unwrap();

								// Ensure text shader + pipeline exist in cache for this device
								if entry.text_shader.is_none() {
									let text_shader_src = r#"
										@vertex
										fn vs(@location(0) pos: vec2<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>) -> @builtin(position) vec4<f32> {
											return vec4<f32>(pos, 0.0, 1.0);
										}

										@fragment
										fn fs(@location(0) uv: vec2<f32>, @location(1) color: vec4<f32>, @binding(0) @group(0) tex: texture_2d<f32>, @binding(1) @group(0) samp: sampler) -> @location(0) vec4<f32> {
											let s = textureSample(tex, samp, uv);
											return vec4<f32>(color.rgb, color.a * s.a);
										}
									"#;
									let shader =
										device.create_shader_module(wgpu::ShaderModuleDescriptor {
											label: Some("text-shader"),
											source: wgpu::ShaderSource::Wgsl(
												text_shader_src.into(),
											),
										});
									entry.text_shader = Some(shader);
								}
								// Create pipeline if missing
								if entry.text_pipeline.is_none() {
									let shader_ref = entry.text_shader.as_ref().unwrap();
									let pipeline_layout = device.create_pipeline_layout(
										&wgpu::PipelineLayoutDescriptor {
											label: Some("text-pipeline-layout"),
											bind_group_layouts: &[atlas_bgl_ref],
											push_constant_ranges: &[],
										},
									);
									let text_pipeline = device.create_render_pipeline(
										&wgpu::RenderPipelineDescriptor {
											label: Some("text-pipeline"),
											layout: Some(&pipeline_layout),
											vertex: wgpu::VertexState {
												module: shader_ref,
												entry_point: Some("vs"),
												compilation_options:
													wgpu::PipelineCompilationOptions::default(),
												buffers: &[wgpu::VertexBufferLayout {
													array_stride: (std::mem::size_of::<f32>() * 8)
														as wgpu::BufferAddress,
													step_mode: wgpu::VertexStepMode::Vertex,
													attributes: &[
														wgpu::VertexAttribute {
															offset: 0,
															shader_location: 0,
															format: wgpu::VertexFormat::Float32x2,
														},
														wgpu::VertexAttribute {
															offset: 8,
															shader_location: 1,
															format: wgpu::VertexFormat::Float32x2,
														},
														wgpu::VertexAttribute {
															offset: 16,
															shader_location: 2,
															format: wgpu::VertexFormat::Float32x4,
														},
													],
												}],
											},
											fragment: Some(wgpu::FragmentState {
												module: shader_ref,
												entry_point: Some("fs"),
												compilation_options:
													wgpu::PipelineCompilationOptions::default(),
												targets: &[Some(wgpu::ColorTargetState {
													format: config.format,
													blend: Some(wgpu::BlendState::ALPHA_BLENDING),
													write_mask: wgpu::ColorWrites::ALL,
												})],
											}),
											primitive: wgpu::PrimitiveState::default(),
											depth_stencil: None,
											multisample: wgpu::MultisampleState::default(),
											multiview: None,
											cache: None,
										},
									);
									entry.text_pipeline = Some(text_pipeline);
								}
								let text_pipeline = entry.text_pipeline.as_ref().unwrap();

								// Create or reuse a bind-group for the atlas tied to the
								// atlas version token. This avoids recreating bind groups
								// every frame when the atlas hasn't changed.
								let atlas_handle = crate::oal::text::global_glyph_atlas();
								let atlas_version = atlas_handle.lock().unwrap().atlas_version();

								// Try to reuse a cached bind-group for the current atlas version.
								let mut bg_ref: Option<&wgpu::BindGroup> = None;
								if let Some((ver, bg)) = &entry.atlas_bind_group {
									if *ver == atlas_version {
										bg_ref = Some(bg);
									}
								}

								// If we don't have a cached bind-group for this version,
								// create (or reuse) a sampler and request a new bind-group
								// from the atlas.
								if bg_ref.is_none() {
									if entry.atlas_sampler.is_none() {
										entry.atlas_sampler =
											Some(device.create_sampler(&wgpu::SamplerDescriptor {
												label: Some("glyph_atlas_sampler"),
												address_mode_u: wgpu::AddressMode::ClampToEdge,
												address_mode_v: wgpu::AddressMode::ClampToEdge,
												address_mode_w: wgpu::AddressMode::ClampToEdge,
												mag_filter: wgpu::FilterMode::Linear,
												min_filter: wgpu::FilterMode::Linear,
												mipmap_filter: wgpu::FilterMode::Nearest,
												..Default::default()
											}));
									}
									let sampler_ref = entry.atlas_sampler.as_ref().unwrap();
									if let Some(new_bg) =
										atlas_handle.lock().unwrap().create_bind_group_for_atlas(
											device,
											atlas_bgl_ref,
											sampler_ref,
										) {
										entry.atlas_bind_group = Some((atlas_version, new_bg));
										bg_ref = Some(&entry.atlas_bind_group.as_ref().unwrap().1);
									}
								}

								if let Some(bg) = bg_ref {
									// Reuse or (re)create per-device vertex/index buffers
									// to avoid allocating each frame. We use queue.write_buffer
									// to update existing buffers when possible.
									let vertex_bytes = bytemuck::cast_slice(&text_vertices);
									let index_bytes: &[u8] = bytemuck::cast_slice(&text_indices);
									// ensure vertex buffer capacity
									if entry.vertex_buf.is_none()
										|| entry.vertex_capacity < vertex_bytes.len()
									{
										// allocate new buffer with capacity (round up)
										let cap = vertex_bytes.len().next_power_of_two();
										let buf = device.create_buffer(&wgpu::BufferDescriptor {
											label: Some("text-verts-cached"),
											size: cap as wgpu::BufferAddress,
											usage: wgpu::BufferUsages::VERTEX
												| wgpu::BufferUsages::COPY_DST,
											mapped_at_creation: false,
										});
										entry.vertex_buf = Some(buf);
										entry.vertex_capacity = cap;
									}
									// ensure index buffer capacity
									if entry.index_buf.is_none()
										|| entry.index_capacity < index_bytes.len()
									{
										let cap = index_bytes.len().next_power_of_two();
										let buf = device.create_buffer(&wgpu::BufferDescriptor {
											label: Some("text-idx-cached"),
											size: cap as wgpu::BufferAddress,
											usage: wgpu::BufferUsages::INDEX
												| wgpu::BufferUsages::COPY_DST,
											mapped_at_creation: false,
										});
										entry.index_buf = Some(buf);
										entry.index_capacity = cap;
									}

									// write data into buffers
									if let Some(vbuf) = entry.vertex_buf.as_ref() {
										queue.write_buffer(vbuf, 0, vertex_bytes);
									}
									if let Some(ibuf) = entry.index_buf.as_ref() {
										queue.write_buffer(ibuf, 0, index_bytes);
									}

									rpass.set_pipeline(text_pipeline);
									rpass.set_bind_group(0, bg, &[]);
									if let Some(vbuf) = entry.vertex_buf.as_ref() {
										rpass.set_vertex_buffer(0, vbuf.slice(..));
									}
									if let Some(ibuf) = entry.index_buf.as_ref() {
										rpass.set_index_buffer(
											ibuf.slice(..),
											wgpu::IndexFormat::Uint16,
										);
									}
									rpass.draw_indexed(0..(text_indices.len() as u32), 0, 0..1);
								}
							}
						}

					drop(rpass);

						// Return the (possibly modified) owned context so the backend
						// can finish/submit the encoder.
						return Ok(Box::new(owned));
					}

					Err(OalError::Renderer("invalid render context".into()))
				},
			))
	}
}

// Public docs: A Renderer is a minimal drawing backend used by the OAL to
// present the logical `Canvas` contents into a platform surface. The
// `NoopRenderer` is provided for tests and headless operation; real
// renderers should implement `render` and handle the supplied `DeviceMetrics`
// and `Unit` to produce pixels.
