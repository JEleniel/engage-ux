#![cfg(feature = "wayland")]

use std::thread;
use std::time::Duration;

use engage_ux_oal::platform::wayland::WaylandRuntime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create runtime on main thread
	let (mut runtime, _handle) = WaylandRuntime::new()?;

	// Create a simple top-level window (must be on main thread)
	let window = runtime.create_window("example", 320, 240)?;

	// Present a single solid-color frame: RGBA premultiplied
	// Build a small red frame
	let width = 320u32;
	let height = 240u32;
	let mut pixels = vec![0u8; (width * height * 4) as usize];
	for i in 0..(width * height) as usize {
		pixels[i * 4] = 0xFF; // R
		pixels[i * 4 + 1] = 0x00; // G
		pixels[i * 4 + 2] = 0x00; // B
		pixels[i * 4 + 3] = 0xFF; // A
	}

	// Build Frame
	let frame = engage_ux_oal::traits::Frame {
		size: engage_ux_oal::types::DeviceSize::new(width, height),
		rgba_pixels: Some(pixels),
	};

	// Present and process events for a short time
	window.present(frame, &[])?;

	// Drive the runtime for 50ms to allow Wayland events to be dispatched
	runtime.process_main_thread_tasks(Duration::from_millis(50))?;

	// Keep process alive briefly so compositor sees the buffer
	thread::sleep(Duration::from_millis(200));

	Ok(())
}
