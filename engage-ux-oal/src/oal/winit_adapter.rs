#![cfg(feature = "native-winit")]

use std::collections::HashMap;
use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, Ordering},
	mpsc,
};
use std::thread;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::errors::{OalError, Result};
use crate::oal::backend::{
	Backend, PlatformEvent, SurfaceDescriptor, SurfaceHandle, SurfaceParams,
};
use engage_ux_core::geometry::Rectangle;

use wgpu::util::DeviceExt;

type SurfaceId = u128;

struct GpuSurface {
	window: Window,
	surface: wgpu::Surface,
	config: wgpu::SurfaceConfiguration,
}

/// GPU context provided to `RenderCallback` implementations when running on
/// the winit/wgpu backend. Render callbacks should downcast the `&mut dyn Any`
/// they receive to `&mut WgpuRenderContext` to access the underlying wgpu
/// objects and perform GPU rendering directly.
pub struct WgpuRenderContext<'a> {
	pub device: &'a wgpu::Device,
	pub queue: &'a wgpu::Queue,
	pub surface: &'a wgpu::Surface,
	pub config: &'a wgpu::SurfaceConfiguration,
	/// The acquired frame's texture view (available during Present).
	pub view: Option<&'a wgpu::TextureView>,
	/// A mutable reference to a command encoder that jobs can record into.
	pub encoder: Option<&'a mut wgpu::CommandEncoder>,
}

/// Create a wgpu surface from a winit `Window`.
///
/// Safety: `wgpu::Instance::create_surface` is marked `unsafe` because it relies on
/// platform-specific invariants that the caller must uphold:
///
/// - The underlying raw window handle produced by the `Window` must remain valid for
///   the lifetime of the returned `wgpu::Surface`.
/// - The surface must be used on the same thread that created the window (the event-loop
///   thread in this adapter). Creating or using the surface from other threads may
///   lead to undefined behavior depending on the platform and GPU backend.
/// - The `Window` must not be dropped while the surface is still in use.
///
/// This helper centralizes the single `unsafe` call so the safety justification is easy
/// to review and audit. Callers must ensure they call this on the event-loop thread
/// immediately after the window is built and store the resulting `Surface` together
/// with the `Window` (as `GpuSurface` does) so the lifetimes are tied together.
fn create_surface_from_window(instance: &wgpu::Instance, window: &Window) -> wgpu::Surface {
	// Safety: see function documentation above. We call this on the event-loop thread
	// immediately after `WindowBuilder::build()` and keep the `Window` and `Surface`
	// together in the `GpuSurface` struct to ensure the window outlives the surface.
	unsafe { instance.create_surface(window) }.expect("create surface")
}

/// Commands sent to the event-loop thread.
enum Command {
	Create {
		params: SurfaceDescriptor,
		resp: mpsc::Sender<Result<SurfaceId>>,
	},
	Destroy {
		id: SurfaceId,
		resp: Option<mpsc::Sender<Result<()>>>,
	},
	Present {
		id: SurfaceId,
		dirty: Vec<Rectangle>,
		resp: Option<mpsc::Sender<Result<()>>>,
	},
	Reconfigure {
		id: SurfaceId,
		params: SurfaceDescriptor,
		resp: Option<mpsc::Sender<Result<()>>>,
	},
	SubmitRender {
		id: SurfaceId,
		job: Box<dyn crate::oal::backend::RenderCallback>,
		resp: Option<mpsc::Sender<Result<()>>>,
	},
	SetTitle {
		id: SurfaceId,
		title: String,
		resp: Option<mpsc::Sender<Result<()>>>,
	},
	Stop,
}

/// Winit + wgpu backend adapter. This runs an event loop on a background
/// thread that owns the `EventLoop` and performs window & GPU surface
/// creation. Commands are sent via a channel and PlatformEvents are emitted
/// back via an mpsc channel.
#[derive(Clone)]
pub struct WinitBackend {
	cmd_tx: Arc<Mutex<mpsc::Sender<Command>>>,
	event_rx: Arc<Mutex<mpsc::Receiver<PlatformEvent>>>,
	run_flag: Arc<AtomicBool>,
}

impl WinitBackend {
	pub fn new() -> Self {
		let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
		let (event_tx, event_rx) = mpsc::channel::<PlatformEvent>();

		let cmd_tx = Arc::new(Mutex::new(cmd_tx));
		let event_rx = Arc::new(Mutex::new(event_rx));
		let run_flag = Arc::new(AtomicBool::new(true));

		// Spawn thread with event loop
		let run_flag_thread = run_flag.clone();
		thread::spawn(move || {
			// Create the event loop on this thread.
			let event_loop = match EventLoop::new() {
				Ok(el) => el,
				Err(e) => {
					eprintln!("failed to create winit event loop: {}", e);
					return;
				}
			};

			// Initialize wgpu instance/adapter/device on this thread.
			let instance = wgpu::Instance::default();
			let adapter_res =
				pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
					power_preference: wgpu::PowerPreference::HighPerformance,
					compatible_surface: None,
					force_fallback_adapter: false,
				}));

			let adapter = match adapter_res {
				Ok(a) => a,
				Err(_) => {
					// If we couldn't find an adapter, exit the thread.
					return;
				}
			};

			let device_queue_res = pollster::block_on(adapter.request_device(
				&wgpu::DeviceDescriptor {
					required_features: wgpu::Features::empty(),
					required_limits: wgpu::Limits::default(),
					..Default::default()
				},
				None,
			));

			let (device, queue) = match device_queue_res {
				Ok((d, q)) => (d, q),
				Err(_) => return,
			};

			// Track surfaces created on this thread.
			let mut surfaces: HashMap<SurfaceId, GpuSurface> = HashMap::new();
			let mut next_id: SurfaceId = 1;

			// Pending GPU jobs keyed by surface id. Jobs are executed during the
			// Present pass while the backend holds a mutable encoder.
			let mut pending_jobs: HashMap<SurfaceId, Box<dyn crate::oal::backend::RenderCallback>> =
				HashMap::new();

			// Run the event loop; integrate commands by polling the receiver
			// during MainEventsCleared.
			event_loop.run(move |event, _, control_flow| {
				*control_flow = ControlFlow::Poll;

				// Allow external request to stop the loop.
				if !run_flag_thread.load(Ordering::SeqCst) {
					*control_flow = ControlFlow::Exit;
					return;
				}

				use winit::event::Event;
				use winit::event::StartCause;

				match event {
					Event::MainEventsCleared => {
						// Process pending commands
						loop {
							match cmd_rx.try_recv() {
								Ok(cmd) => match cmd {
									Command::Create { params, resp } => {
										// Create a native window and wgpu surface
										let wb = winit::window::WindowBuilder::new()
											.with_title("engage-ux window")
											.with_inner_size(winit::dpi::PhysicalSize::new(
												params.width.max(1.0) as u32,
												params.height.max(1.0) as u32,
											));

										match wb.build(&event_loop) {
											Ok(window) => {
												// Create the surface using the documented helper that
												// encapsulates the `unsafe` call and explains the
												// safety invariants. See `engage-ux-oal/README.md`
												// for additional guidelines on the lifetime and
												// threading requirements.
												let surface =
													create_surface_from_window(&instance, &window);

												let size = window.inner_size();
												let caps = surface.get_capabilities(&adapter);
												let format = caps
													.formats
													.iter()
													.copied()
													.find(|f| f.describe().srgb)
													.copied()
													.unwrap_or(caps.formats[0]);

												let config = wgpu::SurfaceConfiguration {
													usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
													format,
													width: size.width,
													height: size.height,
													present_mode: wgpu::PresentMode::Fifo,
													alpha_mode: caps.alpha_modes[0],
													view_formats: vec![],
												};

												surface.configure(&device, &config);

												let id = next_id;
												next_id = next_id.saturating_add(1);

												surfaces.insert(
													id,
													GpuSurface {
														window,
														surface,
														config,
													},
												);
												let _ = resp.send(Ok(id));
											}
											Err(e) => {
												let _ = resp.send(Err(OalError::Initialization(
													format!("window build failed: {}", e),
												)));
											}
										}
									}
									Command::Destroy { id, resp } => {
										if let Some(s) = surfaces.remove(&id) {
											let _ = resp.map(|r| r.send(Ok(())));
											drop(s);
										} else {
											let _ = resp.map(|r| {
												r.send(Err(OalError::Window(format!(
													"surface {} not found",
													id
												))))
											});
										}
									}
									Command::SetTitle { id, title, resp } => {
										if let Some(s) = surfaces.get_mut(&id) {
											s.window.set_title(&title);
											let _ = resp.map(|r| r.send(Ok(())));
										} else {
											let _ = resp.map(|r| {
												r.send(Err(OalError::Window(format!(
													"surface {} not found",
													id
												))))
											});
										}
									}
									Command::Reconfigure { id, params, resp } => {
										if let Some(s) = surfaces.get_mut(&id) {
											s.config.width = params.width as u32;
											s.config.height = params.height as u32;
											s.surface.configure(&device, &s.config);
											let _ = resp.map(|r| r.send(Ok(())));
										} else {
											let _ = resp.map(|r| {
												r.send(Err(OalError::Window(format!(
													"surface {} not found",
													id
												))))
											});
										}
									}
									Command::SubmitRender { id, job, resp } => {
										// Store the job to be executed during the next Present call for this surface.
										if surfaces.contains_key(&id) {
											pending_jobs.insert(id, job);
											let _ = resp.map(|r| r.send(Ok(())));
										} else {
											let _ = resp.map(|r| {
												r.send(Err(OalError::Window(format!(
													"surface {} not found",
													id
												))))
											});
										}
									}

									Command::Present {
										id,
										dirty: _dirty,
										resp,
									} => {
										if let Some(s) = surfaces.get_mut(&id) {
											// Acquire current texture
											match s.surface.get_current_texture() {
												Ok(frame) => {
													let view = frame.texture.create_view(
														&wgpu::TextureViewDescriptor::default(),
													);
													let mut encoder = device
														.create_command_encoder(
															&wgpu::CommandEncoderDescriptor {
																label: Some("engage-ux-encoder"),
															},
														);

													// Simple clear pass (baseline content).
													{
														let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
															label: Some("engage-ux-clear"),
															color_attachments: &[Some(wgpu::RenderPassColorAttachment {
																view: &view,
																resolve_target: None,
																ops: wgpu::Operations {
																	load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
																	store: true,
																},
															})],
															depth_stencil_attachment: None,
														});
													}

													// If a pending GPU job exists for this surface, execute
													// it now while we still hold the mutable encoder. The
													// job may record render passes into the encoder using
													// the provided view.
													if let Some(mut job) = pending_jobs.remove(&id)
													{
														// Provide a context exposing wgpu objects to the job.
														let mut ctx = WgpuRenderContext {
															device: &device,
															queue: &queue,
															surface: &s.surface,
															config: &s.config,
															view: Some(&view),
															encoder: Some(&mut encoder),
														};
														let res = job.call_box(
															&mut ctx as &mut dyn std::any::Any,
														);
														let _ = resp.as_ref().map(|r| r.send(res));
													}

													queue.submit(Some(encoder.finish()));
													frame.present();

													let _ = resp.map(|r| r.send(Ok(())));
												}
												Err(e) => {
													let _ = resp.map(|r| {
														r.send(Err(OalError::Renderer(format!(
															"acquire failed: {:?}",
															e
														))))
													});
												}
											}
										} else {
											let _ = resp.map(|r| {
												r.send(Err(OalError::Window(format!(
													"surface {} not found",
													id
												))))
											});
										}
									}
									Command::Stop => {
										*control_flow = ControlFlow::Exit;
										return;
									}
								},
								Err(std::sync::mpsc::TryRecvError::Empty) => break,
								Err(std::sync::mpsc::TryRecvError::Disconnected) => return,
							}
						}

						// Emit FrameRequested events for all surfaces
						for (&id, _) in surfaces.iter() {
							let _ = event_tx.send(PlatformEvent::FrameRequested { surface: id });
						}
					}
					Event::WindowEvent {
						event: winit::event::WindowEvent::Resized(size),
						window_id,
					} => {
						// Find the surface by window id and reconfigure
						if let Some((_, s)) = surfaces
							.iter_mut()
							.find(|(_, s)| s.window.id() == window_id)
						{
							s.config.width = size.width;
							s.config.height = size.height;
							s.surface.configure(&device, &s.config);
						}
					}
					_ => {}
				}
			});
		});

		Self {
			cmd_tx,
			event_rx,
			run_flag,
		}
	}

	fn send_cmd(&self, cmd: Command) -> Result<()> {
		let tx = self
			.cmd_tx
			.lock()
			.map_err(|_| OalError::PoisonedLock("winit cmd sender poisoned".into()))?;
		tx.send(cmd)
			.map_err(|e| OalError::Initialization(format!("cmd send failed: {}", e)))
	}
}

impl Drop for WinitBackend {
	fn drop(&mut self) {
		let _ = self.send_cmd(Command::Stop);
		self.run_flag.store(false, Ordering::SeqCst);
	}
}

impl Backend for WinitBackend {
	fn create_surface(&self, params: SurfaceDescriptor) -> Result<SurfaceHandle> {
		let (resp_tx, resp_rx) = mpsc::channel();
		let cmd = Command::Create {
			params,
			resp: resp_tx,
		};
		self.send_cmd(cmd)?;
		let res = resp_rx
			.recv()
			.map_err(|e| OalError::Initialization(format!("create response recv failed: {}", e)))?;
		res
	}

	fn poll_events(&self) -> Vec<PlatformEvent> {
		let mut out = Vec::new();
		if let Ok(rx) = self.event_rx.lock() {
			for ev in rx.try_iter() {
				out.push(ev);
			}
		}
		out
	}

	fn present_frame(&self, surface: SurfaceHandle, dirty: &[Rectangle]) -> Result<()> {
		let (resp_tx, resp_rx) = mpsc::channel();
		let dirty_vec = dirty.to_vec();
		let cmd = Command::Present {
			id: surface,
			dirty: dirty_vec,
			resp: Some(resp_tx),
		};
		self.send_cmd(cmd)?;
		resp_rx
			.recv()
			.map_err(|e| OalError::Renderer(format!("present response recv failed: {}", e)))??;
		Ok(())
	}

	fn invalidate_region(&self, _surface: SurfaceHandle, _rects: &[Rectangle]) -> Result<()> {
		Ok(())
	}

	fn destroy_surface(&self, surface: SurfaceHandle) -> Result<()> {
		let (resp_tx, resp_rx) = mpsc::channel();
		let cmd = Command::Destroy {
			id: surface,
			resp: Some(resp_tx),
		};
		self.send_cmd(cmd)?;
		resp_rx.recv().map_err(|e| {
			OalError::Initialization(format!("destroy response recv failed: {}", e))
		})??;
		Ok(())
	}

	fn reconfigure_surface(&self, surface: SurfaceHandle, params: SurfaceParams) -> Result<()> {
		let (resp_tx, resp_rx) = mpsc::channel();
		let cmd = Command::Reconfigure {
			id: surface,
			params,
			resp: Some(resp_tx),
		};
		self.send_cmd(cmd)?;
		resp_rx.recv().map_err(|e| {
			OalError::Initialization(format!("reconfigure response recv failed: {}", e))
		})??;
		Ok(())
	}

	fn set_surface_title(&self, surface: SurfaceHandle, title: &str) -> Result<()> {
		let (resp_tx, resp_rx) = mpsc::channel();
		let cmd = Command::SetTitle {
			id: surface,
			title: title.to_string(),
			resp: Some(resp_tx),
		};
		self.send_cmd(cmd)?;
		resp_rx.recv().map_err(|e| {
			OalError::Initialization(format!("set title response recv failed: {}", e))
		})??;
		Ok(())
	}

	fn submit_render(
		&self,
		surface: SurfaceHandle,
		job: Box<dyn crate::oal::backend::RenderCallback>,
	) -> Result<()> {
		let (resp_tx, resp_rx) = mpsc::channel();
		let cmd = Command::SubmitRender {
			id: surface,
			job,
			resp: Some(resp_tx),
		};
		self.send_cmd(cmd)?;
		resp_rx.recv().map_err(|e| {
			OalError::Renderer(format!("submit_render response recv failed: {}", e))
		})??;
		Ok(())
	}
}
