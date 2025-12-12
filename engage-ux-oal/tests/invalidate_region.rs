//! Invalidate region tests for the OAL backends.
//!
//! Exercises `invalidate_region` and related surface lifecycle behavior.

use engage_ux_oal::oal::backend::{Backend, HeadlessBackend, SurfaceDescriptor};

#[test]
fn exercise_invalidate_region() {
	let backend = HeadlessBackend::new();
	let desc = SurfaceDescriptor::builder()
		.width(100.0)
		.height(100.0)
		.build();
	let surface = backend.create_surface(desc).expect("create surface");
	// Call invalidate_region with an empty rect list to ensure the method is exercised
	backend
		.invalidate_region(surface, &[])
		.expect("invalidate region");
}
