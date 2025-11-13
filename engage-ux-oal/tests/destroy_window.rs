//! Test for `Oal::destroy_window` behavior.

use std::sync::Arc;

use engage_ux_oal::{DeviceMetrics, Oal, WindowDesc};

#[test]
fn destroy_window_removes_window() {
	let bus = Arc::new(engage_ux_core::event::EventBus::new());
	let metrics = DeviceMetrics::with_pixels(10);
	let oal = Oal::new(bus.clone(), metrics).expect("Oal::new");

	let desc = WindowDesc::new("t", (10.0, 10.0));
	let win = oal.create_window(desc).expect("create_window");
	let id = win.id;

	// First destroy should succeed
	assert!(oal.destroy_window(id).is_ok());

	// Second destroy should return an error (not found)
	assert!(oal.destroy_window(id).is_err());
}
