// Native winit backend (feature-gated). Build with --features native-winit
#[cfg(feature = "native-winit")]
mod native {
	use std::sync::Arc;
	use std::sync::atomic::Ordering;

	use winit::dpi::PhysicalSize;
	use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
	use winit::event_loop::{ControlFlow, EventLoop};
	use winit::window::WindowBuilder;

	use crate::oal::Oal;
	use engage_ux_core::event::{
		Event as CoreEvent, KeyboardModifierKeys, PointerEvent as CorePointerEvent, PointerKind,
		WindowEvent as CoreWindowEvent,
	};
	use engage_ux_core::geometry::{Move, Point, Rectangle};

	/// Run the native winit event loop on the current thread. This will create
	/// platform windows for each OAL window, map winit events to `engage-ux-core`
	/// `Event` variants and emit them on the provided `EventBus`.
	///
	/// This function is fallible for setup errors (for example, failing to
	/// build platform windows); once the event loop is entered it is
	/// intentionally divergent and will not return under normal operation.
	use crate::errors::{OalError, Result};

	pub fn run_event_loop(oal: Arc<Oal>) -> Result<()> {
		// EventLoop::new() may be fallible on some platforms in newer winit
		// releases. Convert any error into an `OalError` so callers receive a
		// typed error instead of panicking.
		let event_loop: EventLoop<()> = EventLoop::new()
			.map_err(|e| OalError::Initialization(format!("event loop creation failed: {}", e)))?;

		// create windows
		let mut winit_map = std::collections::HashMap::new();
		let windows_guard = match oal.windows.lock() {
			Ok(g) => g,
			Err(_poison) => {
				return Err(crate::errors::OalError::PoisonedLock(
					"windows mutex poisoned".into(),
				));
			}
		};

		for (_id, win) in windows_guard.iter() {
			let size = win.canvas.size_in_units;
			// `unit_scale` may be fallible (depends on device metrics retrieval);
			// propagate errors to the caller instead of calling methods on a
			// potential `Result`.
			let unit = oal.unit_scale()?;
			let w_px = unit.to_px(size.0, &oal.metrics).max(1.0) as u32;
			let h_px = unit.to_px(size.1, &oal.metrics).max(1.0) as u32;

			let wb = WindowBuilder::new()
				.with_title(win.title.clone())
				.with_inner_size(PhysicalSize::new(w_px, h_px));

			// The window build can fail; with the `native-winit` feature we
			// provide a typed conversion into `OalError` so callers receive a
			// concrete error type. The `?` operator will convert automatically
			// via the `From` impl produced by `thiserror` when the feature is
			// enabled.
			let window = wb.build(&event_loop)?;

			winit_map.insert(window.id(), win.id);
		}

		// track last cursor pos per window
		let mut last_pos: std::collections::HashMap<u128, (f32, f32)> = Default::default();

		event_loop.run(move |event, _, control_flow| {
			// Allow external callers to request a graceful shutdown of the
			// event loop by clearing the run_loop_flag on the Oal instance.
			if !oal.run_loop_flag.load(Ordering::SeqCst) {
				*control_flow = ControlFlow::Exit;
				return;
			}
			*control_flow = ControlFlow::Wait;

			match event {
				WinitEvent::WindowEvent { event, window_id } => {
					if let Some(&our_id) = winit_map.get(&window_id) {
						match event {
							WinitWindowEvent::CloseRequested => {
								oal.event_bus.emit(CoreEvent::Window {
									source_component_id: our_id,
									timestamp: chrono::Utc::now(),
									payload: CoreWindowEvent::CloseRequested,
								});
							}
							WinitWindowEvent::Resized(size) => {
								let unit = oal.unit_scale()?;
								let w_units = unit.px_to_units(size.width as f32, &oal.metrics);
								let h_units = unit.px_to_units(size.height as f32, &oal.metrics);

								let rect = Rectangle {
									top_left: Point {
										x: 0.0,
										y: 0.0,
										style: None,
									},
									width: w_units,
									height: h_units,
								};

								oal.event_bus.emit(CoreEvent::Window {
									source_component_id: our_id,
									timestamp: chrono::Utc::now(),
									payload: CoreWindowEvent::MovedOrResized { rectangle: rect },
								});
							}
							WinitWindowEvent::CursorMoved { position, .. } => {
								let unit = oal.unit_scale()?;
								let x = unit.px_to_units(position.x as f32, &oal.metrics);
								let y = unit.px_to_units(position.y as f32, &oal.metrics);

								let last = last_pos.get(&our_id).cloned().unwrap_or((x, y));
								let dx = x - last.0;
								let dy = y - last.1;
								last_pos.insert(our_id, (x, y));

								let pe = CorePointerEvent {
									kind: PointerKind::Mouse,
									position: Point { x, y, style: None },
									delta: Move { x: dx, y: dy },
									buttons: 0,
									modifiers: KeyboardModifierKeys::default(),
									pressure: None,
									tilt: None,
								};

								oal.event_bus.emit(CoreEvent::Pointer {
									source_component_id: our_id,
									timestamp: chrono::Utc::now(),
									payload: pe,
								});
							}
							WinitWindowEvent::KeyboardInput { .. } => {
								// Emit a custom keyboard payload with whatever info we can get.
								let payload = serde_json::json!({
									"type": "keyboard",
								});

								oal.event_bus.emit(CoreEvent::Custom {
									source_component_id: our_id,
									timestamp: chrono::Utc::now(),
									payload: payload.to_string(),
								});
							}
							WinitWindowEvent::MouseWheel { delta, .. } => {
								let payload = match delta {
									winit::event::MouseScrollDelta::LineDelta(_x, y) => {
										serde_json::json!({"type":"wheel","delta":y})
									}
									winit::event::MouseScrollDelta::PixelDelta(pos) => {
										serde_json::json!({"type":"wheel","delta":pos.y})
									}
								};

								oal.event_bus.emit(CoreEvent::Custom {
									source_component_id: our_id,
									timestamp: chrono::Utc::now(),
									payload: payload.to_string(),
								});
							}
							_ => {}
						}
					}
				}
				WinitEvent::MainEventsCleared => {
					// request frame for each window
					for (&_wid, &our_id) in winit_map.iter() {
						oal.event_bus.emit(CoreEvent::Custom {
							source_component_id: our_id,
							timestamp: chrono::Utc::now(),
							payload: "FrameRequested".to_string(),
						});
					}
				}
				_ => {}
			}
		});
	}
}

// When native feature is enabled, expose the native run_event_loop at module root
#[cfg(feature = "native-winit")]
pub use self::native::run_event_loop;

// Fallback headless emitter when native-winit feature is not enabled.
#[cfg(not(feature = "native-winit"))]
use std::time::Duration;

#[cfg(not(feature = "native-winit"))]
use std::sync::Arc;

#[cfg(not(feature = "native-winit"))]
use std::sync::atomic::Ordering;

#[cfg(not(feature = "native-winit"))]
use crate::oal::Oal;

#[cfg(not(feature = "native-winit"))]
use crate::errors::Result;

#[cfg(not(feature = "native-winit"))]
/// Headless event loop that emits FrameRequested Custom events periodically.
///
/// The function signature is fallible to match the native backend; the
/// implementation loops until `Oal::stop_event_loop` is called and then
/// returns.
pub fn run_event_loop(oal: Arc<Oal>) -> Result<()> {
	loop {
		// Allow external callers to stop the headless loop.
		if !oal.run_loop_flag.load(Ordering::SeqCst) {
			return Ok(());
		}

		// If the windows mutex is poisoned we return a typed error instead
		// of continuing with potentially inconsistent state.
		let windows_guard = match oal.windows.lock() {
			Ok(g) => g.clone(),
			Err(_poison) => {
				return Err(crate::errors::OalError::PoisonedLock(
					"windows mutex poisoned".into(),
				));
			}
		};
		for (_id, w) in windows_guard.iter() {
			oal.event_bus.emit(engage_ux_core::event::Event::Custom {
				source_component_id: w.id,
				timestamp: chrono::Utc::now(),
				payload: "FrameRequested".to_string(),
			});
		}

		std::thread::sleep(Duration::from_millis(16));
	}
}
