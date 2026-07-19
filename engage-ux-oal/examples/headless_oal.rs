//! Minimal example showing how to create an OAL instance, create a window, and
//! present using the `NoopRenderer` in headless mode.
//!
//! Run with: cargo run --example headless_oal --package engage-ux-oal

use std::sync::Arc;

use engage_ux_oal::{DeviceMetrics, NoopRenderer, Oal, WindowDesc, run_event_loop};

fn main() -> Result<(), engage_ux_oal::OalError> {
	// Build an EventBus and create the OAL
	let bus = Arc::new(engage_ux_core::event::EventBus::new());
	let metrics = DeviceMetrics::with_pixels(10);

	// Wrap the Oal in an Arc so we can share it with the run loop thread.
	let oal = Arc::new(Oal::new(bus.clone(), metrics)?);

	// Create a simple window
	let desc = WindowDesc::new("example", (100.0, 80.0));
	let mut win = oal.create_window(desc)?;

	// Present using the NoopRenderer (headless)
	let mut renderer = NoopRenderer::new();
	win.present(&mut renderer, engage_ux_oal::Unit::Metric)?;

	// Request a frame (emits FrameRequested event)
	win.request_frame();

	// Spawn the headless run loop on a background thread. After a short
	// delay we'll request a graceful shutdown via `stop_event_loop`.
	let oal_for_thread = oal.clone();
	let handle = std::thread::spawn(move || {
		// This will loop until `stop_event_loop` is called on the Oal.
		run_event_loop(oal_for_thread).expect("run_event_loop failed");
	});

	// Let the headless loop run for a short while then stop it.
	std::thread::sleep(std::time::Duration::from_millis(100));
	oal.stop_event_loop()?;

	// Wait for the loop to exit.
	handle.join().expect("run_event_loop thread panicked");

	Ok(())
}
