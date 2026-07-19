//! Test that the headless run_event_loop observes `Oal::stop_event_loop` and exits.

use std::sync::Arc;

use engage_ux_oal::{DeviceMetrics, Oal, run_event_loop};

#[test]
fn headless_run_loop_stops_on_stop_event_loop() {
	let bus = Arc::new(engage_ux_core::event::EventBus::new());
	let metrics = DeviceMetrics::with_pixels(10);

	// Create the OAL inside an Arc so we can share with the spawned thread.
	let oal = Arc::new(Oal::new(bus.clone(), metrics).expect("Oal::new"));

	let oal_for_thread = oal.clone();

	// Spawn the headless run loop. It should exit when we call stop_event_loop.
	let handle = std::thread::spawn(move || {
		// `run_event_loop` returns a Result; bubble up failures as panics so the test fails.
		run_event_loop(oal_for_thread).expect("run_event_loop failed");
	});

	// Give the loop a small moment to start and do a couple iterations.
	std::thread::sleep(std::time::Duration::from_millis(50));

	// Signal the event loop to stop and wait for the thread to exit.
	oal.stop_event_loop().expect("stop_event_loop");

	// Join and ensure the thread finished normally.
	let join_res = handle.join();
	assert!(join_res.is_ok(), "run_event_loop thread panicked");
}
