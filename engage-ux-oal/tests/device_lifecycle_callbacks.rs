//! Device lifecycle callback tests.
//!
//! Ensure device recreated/reupload callbacks are invoked and handled.

use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use engage_ux_oal::oal::device_lifecycle;

#[test]
fn callback_runs_on_device_recreated() {
	let flag = Arc::new(AtomicBool::new(false));
	let f = flag.clone();
	let cb = Arc::new(move |_old: usize, _new: usize| {
		f.store(true, Ordering::SeqCst);
	});

	let handle = device_lifecycle::register_device_recreated_callback(cb);

	// simulate device recreation
	device_lifecycle::device_recreated(10_usize, 11_usize);

	assert!(flag.load(Ordering::SeqCst));

	device_lifecycle::unregister_device_recreated_callback(handle);
}
