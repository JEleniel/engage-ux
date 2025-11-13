// Example: submit a simple render job to the WinitBackend's GPU thread.
// Run with: cargo run -p engage-ux-oal --example gpu_submit --features native-winit

use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use std::{thread, time::Duration};

fn main() {
	// If the native-winit feature is not enabled, print guidance and exit.
	#[cfg(not(feature = "native-winit"))]
	{
		println!("native-winit feature not enabled; run with --features native-winit");
		return;
	}

	#[cfg(feature = "native-winit")]
	{
		use engage_ux_oal::WinitBackend;
		use engage_ux_oal::oal::backend::SurfaceDescriptor;

		// Create backend and a surface (this will create a real window).
		let backend = WinitBackend::new();
		let params = SurfaceDescriptor::builder()
			.width(640.0)
			.height(480.0)
			.build();

		let surface = match backend.create_surface(params) {
			Ok(id) => id,
			Err(e) => {
				eprintln!("create_surface failed: {:?}", e);
				return;
			}
		};

		// Prepare a shared flag we will toggle from the render job to prove it ran.
		let flag = Arc::new(AtomicBool::new(false));
		let flag_job = flag.clone();

		// Build a boxed render job. The job ignores the backend-provided context
		// for this simple example and just flips the flag.
		let job = Box::new(
			move |_ctx: &mut dyn std::any::Any| -> engage_ux_oal::errors::Result<()> {
				flag_job.store(true, Ordering::SeqCst);
				Ok(())
			},
		);

		// Submit the job to run on the backend GPU thread.
		match backend.submit_render(surface, job) {
			Ok(()) => println!("submit_render sent"),
			Err(e) => eprintln!("submit_render error: {:?}", e),
		}

		// Give the backend a short moment to execute the job.
		thread::sleep(Duration::from_millis(200));

		println!("job executed: {}", flag.load(Ordering::SeqCst));

		// Present a frame so the window shows something (clear pass is used by the backend).
		let _ = backend.present_frame(surface, &[]);

		// Keep window open briefly so you can observe it.
		thread::sleep(Duration::from_secs(1));

		let _ = backend.destroy_surface(surface);
	}
}
