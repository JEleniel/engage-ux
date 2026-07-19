//! Example: automatically pick the best available OAL backend and open a window.
//!
//! - If the `native-winit` feature is enabled, the example will prefer the
//!   native winit backend (Wayland/X11 depending on the host).
//! - Otherwise it falls back to the headless backend so the example still
//!   compiles and runs in CI or environments without a compositor.
//!
//! Run with: cargo run -p engage-ux-oal --example auto_backend_demo --features native-winit

use engage_ux_oal::oal::backend::Backend;
#[cfg(feature = "native-winit")]
use std::{thread, time::Duration};

fn main() {
	// Prefer a native backend when compiled with the feature enabled.
	#[cfg(feature = "native-winit")]
	{
		println!("native-winit feature enabled; attempting to use WinitBackend");
		// WinitBackend is re-exported by the crate when the feature is enabled.
		use engage_ux_oal::WinitBackend;
		use engage_ux_oal::oal::backend::SurfaceDescriptor;

		let backend = match std::panic::catch_unwind(|| WinitBackend::new()) {
			Ok(b) => b,
			Err(_) => {
				eprintln!("WinitBackend::new() panicked; falling back to headless");
				run_headless()
			}
		};

		// Try to create a real window surface. If that fails, fallback.
		let params = SurfaceDescriptor::builder()
			.width(800.0)
			.height(600.0)
			.build();
		match backend.create_surface(params) {
			Ok(surface) => {
				println!("Created native surface: {}", surface);

				// Submit a tiny render job if supported. Use a boxed job that
				// flips a flag — similar to the gpu_submit example.
				let job = Box::new(
					move |_ctx: &mut dyn std::any::Any| -> engage_ux_oal::errors::Result<()> {
						// No-op render in this small demo: we don't need the
						// backend context; simply return success.
						let _ = _ctx;
						Ok(())
					},
				);

				if let Err(e) = backend.submit_render(surface, job) {
					eprintln!("submit_render not supported or failed: {:?}", e);
				} else {
					println!("submit_render sent");
				}

				let _ = backend.present_frame(surface, &[]);
				println!("Presenting frame. Window will close shortly...");
				thread::sleep(Duration::from_secs(2));

				let _ = backend.destroy_surface(surface);
				println!("Native demo finished");
			}
			Err(e) => {
				eprintln!(
					"create_surface failed for WinitBackend: {:?}; falling back to headless",
					e
				);
				run_headless();
			}
		}
		return;
	}

	// No native-winit feature: run headless fallback so the example still
	// compiles and can be used in CI/test environments.
	#[cfg(not(feature = "native-winit"))]
	{
		println!("native-winit feature not enabled; running headless demo");
		run_headless();
	}
}

fn run_headless() -> impl std::fmt::Debug {
	use engage_ux_oal::oal::backend::{HeadlessBackend, SurfaceDescriptor};

	let backend = HeadlessBackend::new();
	let params = SurfaceDescriptor::builder()
		.width(640.0)
		.height(480.0)
		.build();

	match backend.create_surface(params) {
		Ok(surface) => {
			println!("Headless surface created: {}", surface);
			// Headless backend cannot run GPU jobs; present_frame is a no-op.
			let _ = backend.present_frame(surface, &[]);
			println!("Headless present done");
			let _ = backend.destroy_surface(surface);
			println!("Headless demo finished");
			surface
		}
		Err(e) => {
			eprintln!("Headless create_surface failed: {:?}", e);
			0u128
		}
	}
}
