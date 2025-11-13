//! Tests for OAL event emission and basic window lifecycle.

use std::{sync::Arc, thread, time::Duration};

use engage_ux_oal::{DeviceMetrics, Oal, WindowDesc};

#[test]
fn create_window_and_request_frame_emit_events() {
	// Build an EventBus and subscribe
	let bus = Arc::new(engage_ux_core::event::EventBus::new());
	let receiver = bus.subscribe();

	// Create OAL with simple device metrics
	let metrics = DeviceMetrics::with_pixels(10);
	let oal = Oal::new(bus.clone(), metrics).expect("Oal::new");

	// Create a window (this should emit an initial Window::MovedOrResized event)
	let desc = WindowDesc {
		title: "test-window".to_string(),
		size_in_units: (100.0, 80.0),
	};

	let win = oal.create_window(desc).expect("create_window");

	// Try to receive the initial Window event
	{
		let mut rx = receiver.lock().unwrap();
		// wait a little to allow the event to flow
		for _ in 0..10 {
			if let Ok(ev) = rx.try_recv() {
				match ev {
					engage_ux_core::event::Event::Window { payload, .. } => match payload {
						engage_ux_core::event::WindowEvent::MovedOrResized { .. } => {
							// ok
							break;
						}
						other => panic!("unexpected window payload: {:?}", other),
					},
					other => panic!("expected Window event, got: {:?}", other),
				}
			}
			thread::sleep(Duration::from_millis(5));
		}
	}

	// Request a frame; this should emit a Custom FrameRequested event
	win.request_frame();

	let mut got_frame = false;
	{
		let mut rx = receiver.lock().unwrap();
		for _ in 0..20 {
			if let Ok(ev) = rx.try_recv() {
				if let engage_ux_core::event::Event::Custom { payload, .. } = ev {
					assert_eq!(payload, "FrameRequested");
					got_frame = true;
					break;
				}
			}
			thread::sleep(Duration::from_millis(5));
		}
	}

	assert!(got_frame, "did not receive FrameRequested custom event");
}
