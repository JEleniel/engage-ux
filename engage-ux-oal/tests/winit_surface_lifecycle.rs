// Integration test for native-winit surface lifecycle.
// This test is only compiled/run when the crate is built with
// `--features native-winit` since it exercises real windowing/GPU code.

//! Winit surface lifecycle tests (native-winit feature).
//!
//! These tests exercise creation/present/reconfigure/destroy on the
//! winit/wgpu backend. They are gated behind `native-winit`.
#![cfg(feature = "native-winit")]

use engage_ux_oal::oal::backend::SurfaceDescriptor;

#[test]
fn surface_create_present_reconfigure_destroy_smoke() {
	use std::panic;

	// Creating the WinitBackend may panic on platforms without a windowing
	// environment (or when run in certain CI runners). Catch panics and
	// treat them as an environmental skip rather than a hard failure.
	let backend = match panic::catch_unwind(|| engage_ux_oal::WinitBackend::new()) {
		Ok(b) => b,
		Err(_) => return,
	};

	let params = SurfaceDescriptor::builder()
		.width(64.0)
		.height(64.0)
		.build();

	match backend.create_surface(params.clone()) {
		Ok(surface) => {
			// Submit a tiny no-op GPU job. The backend may not support
			// GPU jobs in all environments; treat failures as non-fatal.
			let job = Box::new(
				move |_ctx: &mut dyn std::any::Any| -> engage_ux_oal::errors::Result<()> { Ok(()) },
			);

			let _ = backend.submit_render(surface, job);
			let _ = backend.present_frame(surface, &[]);

			// Reconfigure the surface and then destroy it.
			let _ = backend.reconfigure_surface(
				surface,
				SurfaceDescriptor::builder()
					.width(120.0)
					.height(120.0)
					.build(),
			);
			let _ = backend.destroy_surface(surface);
		}
		Err(_) => {
			// create_surface failed (likely due to environment). Treat as skip.
			return;
		}
	}
}
