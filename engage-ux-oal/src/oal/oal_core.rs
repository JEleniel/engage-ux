use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::errors::{OalError, Result};
use crate::oal::backend::{
	HeadlessBackend, PlatformEvent, SurfaceDescriptor, SurfaceHandle, SurfaceParams,
};
use crate::oal::platform::Platform;
use crate::oal::renderer::Renderer;
use crate::oal::{DeviceMetrics, Unit, Window, WindowDesc};
use engage_ux_core::event::EventBus;

/// Main OAL handle
/// The main OAL handle.
///
/// `Oal` owns device metrics, the global unit scale, and a collection of
/// logical windows. It also holds an `EventBus` used to emit platform-agnostic
/// events to the rest of the system.
pub struct Oal {
	pub(crate) event_bus: Arc<EventBus>,
	pub(crate) metrics: DeviceMetrics,
	pub(crate) unit: Mutex<Unit>,
	pub(crate) windows: Mutex<HashMap<u128, Window>>,
	// Flag used to signal the run-event-loop to stop (headless or native when supported)
	pub(crate) run_loop_flag: Arc<AtomicBool>,
	// Platform backend used to manage surfaces/windows
	pub(crate) platform: Platform,
}

impl Oal {
	/// Create a new OAL instance backed by the provided `EventBus` and
	/// `DeviceMetrics`.
	pub fn new(event_bus: Arc<EventBus>, metrics: DeviceMetrics) -> Result<Self> {
		// Default to a headless platform backend unless the caller wires a
		// different platform in later wiring steps.
		let platform = Platform::new(HeadlessBackend::new());

		Ok(Self {
			event_bus,
			metrics,
			unit: Mutex::new(Unit::Metric),
			windows: Mutex::new(HashMap::new()),
			run_loop_flag: Arc::new(AtomicBool::new(true)),
			platform,
		})
	}

	/// Signal the event loop to stop. For headless backend this will cause
	/// `run_event_loop` to return; for native backends this will attempt to
	/// set the control flow to exit (platform permitting).
	pub fn stop_event_loop(&self) -> Result<()> {
		self.run_loop_flag.store(false, Ordering::SeqCst);
		Ok(())
	}

	/// Create a platform surface (backed by the current Platform implementation).
	pub fn create_surface(&self, params: SurfaceParams) -> Result<SurfaceHandle> {
		self.platform.create_surface(params)
	}

	/// Present a frame for a platform surface.
	pub fn present_frame(
		&self,
		surface: SurfaceHandle,
		dirty: &[engage_ux_core::geometry::Rectangle],
	) -> Result<()> {
		self.platform.present_frame(surface, dirty)
	}

	/// Poll platform events from the underlying backend.
	pub fn poll_platform_events(&self) -> Vec<PlatformEvent> {
		self.platform.poll_events()
	}

	/// Destroy a platform surface.
	pub fn destroy_surface(&self, surface: SurfaceHandle) -> Result<()> {
		self.platform.destroy_surface(surface)
	}

	/// Create and register a new logical window. Returns the created `Window`.
	///
	/// This will emit an initial `Window::MovedOrResized` event to indicate the
	/// window is present and ready for layout.
	pub fn create_window(&self, desc: WindowDesc) -> Result<Window> {
		// Create logical window first
		let mut win = Window::new(desc.clone(), self.event_bus.clone(), self.metrics.clone());
		let id = win.id;

		// Create a platform surface for this window. If platform surface
		// creation fails we return the error and do not register the logical
		// window.
		let params = SurfaceDescriptor::builder()
			.width(desc.size_in_units.0)
			.height(desc.size_in_units.1)
			.build();
		let surface = self.platform.create_surface(params)?;

		// Associate surface with the logical window
		win.platform_surface = Some(surface);

		// emit an initial Window moved/resized event to indicate the window is present
		let rectangle = engage_ux_core::geometry::Rectangle {
			top_left: engage_ux_core::geometry::Point { x: 0.0, y: 0.0 },
			width: desc.size_in_units.0,
			height: desc.size_in_units.1,
		};

		self.event_bus.emit(engage_ux_core::event::Event::Window {
			source_component_id: id,
			timestamp: chrono::Utc::now(),
			payload: engage_ux_core::event::WindowEvent::MovedOrResized { rectangle },
		});

		// Acquire the windows lock; return a typed error if the lock is
		// poisoned to avoid continuing with potentially inconsistent state.
		let mut windows_guard: MutexGuard<HashMap<u128, Window>> = match self.windows.lock() {
			Ok(g) => g,
			Err(_poison) => return Err(OalError::PoisonedLock("windows mutex poisoned".into())),
		};

		windows_guard.insert(id, win.clone());

		Ok(win)
	}

	/// Set the global unit scale used by the OAL.
	pub fn set_unit_scale(&self, u: Unit) -> Result<()> {
		match self.unit.lock() {
			Ok(mut guard) => {
				*guard = u;
				Ok(())
			}
			Err(_poison) => Err(OalError::PoisonedLock("unit mutex poisoned".into())),
		}
	}

	/// Get the current unit scale.
	pub fn unit_scale(&self) -> Result<Unit> {
		match self.unit.lock() {
			Ok(guard) => Ok(*guard),
			Err(_poison) => Err(OalError::PoisonedLock("unit mutex poisoned".into())),
		}
	}

	/// Destroy a previously-created window by id. Emits a `CloseRequested`
	/// window event and removes the window from internal tracking. Returns
	/// `OalError::Window` if the id isn't found.
	pub fn destroy_window(&self, id: u128) -> Result<()> {
		// Emit a CloseRequested window event to inform consumers.
		self.event_bus.emit(engage_ux_core::event::Event::Window {
			source_component_id: id,
			timestamp: chrono::Utc::now(),
			payload: engage_ux_core::event::WindowEvent::CloseRequested,
		});

		// Remove the window from our registry, recovering from poisoning if needed.
		let mut windows_guard: MutexGuard<HashMap<u128, Window>> = match self.windows.lock() {
			Ok(g) => g,
			Err(_poison) => return Err(OalError::PoisonedLock("windows mutex poisoned".into())),
		};

		match windows_guard.remove(&id) {
			Some(win) => {
				// If the logical window had an associated platform surface, destroy it.
				if let Some(surface) = win.platform_surface {
					self.platform.destroy_surface(surface)?;
				}
				Ok(())
			}
			None => Err(OalError::Window(format!("window id {} not found", id))),
		}
	}

	/// Resize a logical window and reconfigure its platform surface.
	///
	/// This updates the logical `Window`'s view and, if a platform surface is
	/// associated, destroys and recreates it with the new size so backends that
	/// require explicit reconfiguration (like wgpu) are handled.
	pub fn resize_window(&self, id: u128, width: f32, height: f32) -> Result<()> {
		// Acquire the windows lock; recover from poisoning
		let mut windows_guard: MutexGuard<HashMap<u128, Window>> = match self.windows.lock() {
			Ok(g) => g,
			Err(_poison) => return Err(OalError::PoisonedLock("windows mutex poisoned".into())),
		};

		let win = match windows_guard.get_mut(&id) {
			Some(w) => w,
			None => return Err(OalError::Window(format!("window id {} not found", id))),
		};

		// Update the logical canvas size (canvas == window size in Units)
		win.canvas.resize((width, height));

		// If there's an associated platform surface, reconfigure it with the new size.
		if let Some(surface) = win.platform_surface {
			let params = SurfaceDescriptor::builder()
				.width(width)
				.height(height)
				.build();
			self.platform.reconfigure_surface(surface, params)?;
			// keep the same surface handle
			win.platform_surface = Some(surface);
		}

		// Emit a moved/resized event so consumers can react.
		let rectangle = engage_ux_core::geometry::Rectangle {
			top_left: engage_ux_core::geometry::Point { x: 0.0, y: 0.0 },
			width,
			height,
		};

		self.event_bus.emit(engage_ux_core::event::Event::Window {
			source_component_id: id,
			timestamp: chrono::Utc::now(),
			payload: engage_ux_core::event::WindowEvent::MovedOrResized { rectangle },
		});

		Ok(())
	}

	/// Render and present a logical window using the provided `renderer`.
	///
	/// This is the single entry-point to present window contents. It calls
	/// the renderer to produce the frame and then forwards the present to the
	/// platform surface if one is associated with the window.
	pub fn present_window<R: Renderer>(&self, id: u128, renderer: &mut R) -> Result<()> {
		// Acquire windows lock for reading the window state.
		let windows_guard = match self.windows.lock() {
			Ok(g) => g,
			Err(_poison) => return Err(OalError::PoisonedLock("windows mutex poisoned".into())),
		};

		let win_ref = match windows_guard.get(&id) {
			Some(w) => w,
			None => return Err(OalError::Window(format!("window id {} not found", id))),
		};

		// Snapshot the window state so we don't hold the lock while calling
		// renderer or platform operations.
		let (canvas, metrics, platform_surface) = win_ref.snapshot();

		// Render into the window's canvas using current unit scale.
		let unit = self.unit_scale()?;
		// Prefer GPU-accelerated path when the renderer can provide a
		// backend `RenderCallback`. The renderer's `try_gpu_render` may
		// return `Some(job)` which will be submitted to the platform to run
		// on the backend's GPU/context-owning thread. If no GPU job is
		// provided, fall back to the synchronous software `render`.
		if let Some(job) = renderer.try_gpu_render(&canvas, &metrics, unit) {
			if let Some(surface) = platform_surface {
				// Submit the GPU job and then present the surface.
				self.platform.submit_render(surface, job)?;
				let logical = canvas.logical_size();
				let rect = engage_ux_core::geometry::Rectangle {
					top_left: engage_ux_core::geometry::Point { x: 0.0, y: 0.0 },
					width: logical.0,
					height: logical.1,
				};
				let dirty = vec![rect];
				self.platform.present_frame(surface, &dirty)?;
			} else {
				// No platform surface available; fall back to software rendering.
				renderer.render(&canvas, &metrics, unit)?;
			}
		} else {
			renderer.render(&canvas, &metrics, unit)?;
		}

		// If a platform surface exists, present it. Use the full view rect as dirty.
		if let Some(surface) = platform_surface {
			let logical = canvas.logical_size();
			let rect = engage_ux_core::geometry::Rectangle {
				top_left: engage_ux_core::geometry::Point { x: 0.0, y: 0.0 },
				width: logical.0,
				height: logical.1,
			};
			let dirty = vec![rect];
			self.platform.present_frame(surface, &dirty)?;
		}

		Ok(())
	}
}
