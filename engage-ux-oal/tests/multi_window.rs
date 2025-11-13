//! Test multi-window create/destroy flows for the OAL.

use std::sync::Arc;

use engage_ux_oal::{DeviceMetrics, Oal, WindowDesc};

#[test]
fn create_and_destroy_multiple_windows() {
	let bus = Arc::new(engage_ux_core::event::EventBus::new());
	let metrics = DeviceMetrics::with_pixels(10);

	let oal = Oal::new(bus.clone(), metrics).expect("Oal::new");

	let desc1 = WindowDesc::new("w1", (50.0, 50.0));
	let desc2 = WindowDesc::new("w2", (60.0, 60.0));
	let desc3 = WindowDesc::new("w3", (70.0, 70.0));

	let w1 = oal.create_window(desc1).expect("create w1");
	let w2 = oal.create_window(desc2).expect("create w2");
	let w3 = oal.create_window(desc3).expect("create w3");

	// Destroy in different order
	assert!(oal.destroy_window(w2.id).is_ok());
	assert!(oal.destroy_window(w1.id).is_ok());
	assert!(oal.destroy_window(w3.id).is_ok());

	// Further destroys should return error
	assert!(oal.destroy_window(w1.id).is_err());
}
