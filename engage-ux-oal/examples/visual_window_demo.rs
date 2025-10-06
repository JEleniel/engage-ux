//! Visual Window Demonstration
//!
//! This example creates an ACTUAL window with visual rendering using winit and softbuffer.
//! Unlike the stub backends, this will display a real window on your screen.
//!
//! Run with: cargo run --example visual_window_demo -p engage-ux-oal

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct App {
	window: Option<Rc<Window>>,
	surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl App {
	fn new() -> Self {
		Self {
			window: None,
			surface: None,
		}
	}

	fn draw(&mut self) {
		if let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut()) {
			let size = window.inner_size();
			let width = size.width;
			let height = size.height;

			if width == 0 || height == 0 {
				return;
			}

			// Resize the buffer if needed
			surface
				.resize(
					NonZeroU32::new(width).unwrap(),
					NonZeroU32::new(height).unwrap(),
				)
				.unwrap();

			// Get the buffer to draw into
			let mut buffer = surface.buffer_mut().unwrap();

			// Clear to dark background
			let bg_color = 0xFF1A1A24; // ARGB format: dark blue-gray
			for pixel in buffer.iter_mut() {
				*pixel = bg_color;
			}

			// Draw some colorful rectangles
			let rects = [
				(50, 50, 200, 100, 0xFFCC3333), // Red
				(300, 50, 200, 100, 0xFF33CC33), // Green
				(550, 50, 200, 100, 0xFF3333CC), // Blue
			];

			for (x, y, w, h, color) in rects {
				draw_filled_rect(&mut buffer, width, height, x, y, w, h, color);
			}

			// Draw outlined rectangles
			draw_rect_outline(&mut buffer, width, height, 50, 200, 200, 100, 0xFFFFFF00, 3);

			// Draw circles
			draw_filled_circle(&mut buffer, width, height, 150, 400, 50, 0xFFFF8800);
			draw_circle_outline(&mut buffer, width, height, 400, 400, 50, 0xFF8800FF, 2);

			// Draw a line
			draw_line(&mut buffer, width, height, 550, 350, 750, 450, 0xFF00FFFF, 2);

			// Draw text message (simple ASCII)
			let text = "Engage UX - Visual Rendering Demo";
			draw_text(&mut buffer, width, height, 50, 520, text, 0xFFAABBFF);

			// Present the buffer to the window
			buffer.present().unwrap();
		}
	}
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window_attributes = Window::default_attributes()
			.with_title("Engage UX - Visual Window Demo")
			.with_inner_size(winit::dpi::LogicalSize::new(800, 600));

		let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
		let context = softbuffer::Context::new(window.clone()).unwrap();
		let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

		self.window = Some(window);
		self.surface = Some(surface);

		// Initial draw
		self.draw();
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => {
				println!("Window close requested");
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				self.draw();
			}
			WindowEvent::Resized(_) => {
				self.draw();
			}
			_ => {}
		}
	}
}

fn main() {
	println!("=== Engage UX - Visual Window Demo ===\n");
	println!("Creating actual window with visual rendering...");

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = App::new();

	println!("✓ Event loop created");
	println!("✓ Window will appear shortly...");
	println!("\nClose the window to exit.");

	event_loop.run_app(&mut app).unwrap();

	println!("\n=== Demo Complete ===");
	println!("Visual rendering successful!");
}

// Helper drawing functions

fn draw_filled_rect(
	buffer: &mut [u32],
	width: u32,
	height: u32,
	x: i32,
	y: i32,
	w: u32,
	h: u32,
	color: u32,
) {
	for py in y..(y + h as i32) {
		if py < 0 || py >= height as i32 {
			continue;
		}
		for px in x..(x + w as i32) {
			if px < 0 || px >= width as i32 {
				continue;
			}
			let index = (py as u32 * width + px as u32) as usize;
			if index < buffer.len() {
				buffer[index] = color;
			}
		}
	}
}

fn draw_rect_outline(
	buffer: &mut [u32],
	width: u32,
	height: u32,
	x: i32,
	y: i32,
	w: u32,
	h: u32,
	color: u32,
	thickness: u32,
) {
	// Top and bottom
	for t in 0..thickness {
		draw_line(
			buffer,
			width,
			height,
			x,
			y + t as i32,
			x + w as i32,
			y + t as i32,
			color,
			1,
		);
		draw_line(
			buffer,
			width,
			height,
			x,
			y + h as i32 - t as i32 - 1,
			x + w as i32,
			y + h as i32 - t as i32 - 1,
			color,
			1,
		);
	}
	// Left and right
	for t in 0..thickness {
		draw_line(
			buffer,
			width,
			height,
			x + t as i32,
			y,
			x + t as i32,
			y + h as i32,
			color,
			1,
		);
		draw_line(
			buffer,
			width,
			height,
			x + w as i32 - t as i32 - 1,
			y,
			x + w as i32 - t as i32 - 1,
			y + h as i32,
			color,
			1,
		);
	}
}

fn draw_filled_circle(
	buffer: &mut [u32],
	width: u32,
	height: u32,
	cx: i32,
	cy: i32,
	radius: i32,
	color: u32,
) {
	let r_sq = radius * radius;
	for dy in -radius..=radius {
		let y = cy + dy;
		if y < 0 || y >= height as i32 {
			continue;
		}
		for dx in -radius..=radius {
			let x = cx + dx;
			if x < 0 || x >= width as i32 {
				continue;
			}
			if dx * dx + dy * dy <= r_sq {
				let index = (y as u32 * width + x as u32) as usize;
				if index < buffer.len() {
					buffer[index] = color;
				}
			}
		}
	}
}

fn draw_circle_outline(
	buffer: &mut [u32],
	width: u32,
	height: u32,
	cx: i32,
	cy: i32,
	radius: i32,
	color: u32,
	thickness: u32,
) {
	let outer_r_sq = (radius + thickness as i32) * (radius + thickness as i32);
	let inner_r_sq = radius * radius;
	for dy in -(radius + thickness as i32)..=(radius + thickness as i32) {
		let y = cy + dy;
		if y < 0 || y >= height as i32 {
			continue;
		}
		for dx in -(radius + thickness as i32)..=(radius + thickness as i32) {
			let x = cx + dx;
			if x < 0 || x >= width as i32 {
				continue;
			}
			let dist_sq = dx * dx + dy * dy;
			if dist_sq <= outer_r_sq && dist_sq >= inner_r_sq {
				let index = (y as u32 * width + x as u32) as usize;
				if index < buffer.len() {
					buffer[index] = color;
				}
			}
		}
	}
}

fn draw_line(
	buffer: &mut [u32],
	width: u32,
	height: u32,
	x0: i32,
	y0: i32,
	x1: i32,
	y1: i32,
	color: u32,
	thickness: u32,
) {
	// Bresenham's line algorithm
	let dx = (x1 - x0).abs();
	let dy = (y1 - y0).abs();
	let sx = if x0 < x1 { 1 } else { -1 };
	let sy = if y0 < y1 { 1 } else { -1 };
	let mut err = dx - dy;
	let mut x = x0;
	let mut y = y0;

	loop {
		// Draw with thickness
		for ty in -(thickness as i32 / 2)..=(thickness as i32 / 2) {
			for tx in -(thickness as i32 / 2)..=(thickness as i32 / 2) {
				let px = x + tx;
				let py = y + ty;
				if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
					let index = (py as u32 * width + px as u32) as usize;
					if index < buffer.len() {
						buffer[index] = color;
					}
				}
			}
		}

		if x == x1 && y == y1 {
			break;
		}

		let e2 = 2 * err;
		if e2 > -dy {
			err -= dy;
			x += sx;
		}
		if e2 < dx {
			err += dx;
			y += sy;
		}
	}
}

fn draw_text(buffer: &mut [u32], width: u32, height: u32, x: i32, y: i32, text: &str, color: u32) {
	// Simple 5x7 bitmap font for ASCII characters
	let mut cx = x;
	for ch in text.chars() {
		draw_char(buffer, width, height, cx, y, ch, color);
		cx += 6; // 5 pixels wide + 1 pixel spacing
	}
}

fn draw_char(buffer: &mut [u32], width: u32, height: u32, x: i32, y: i32, ch: char, color: u32) {
	// Very basic 5x7 font bitmap (only uppercase letters, digits, space, dash, and some symbols)
	let bitmap = match ch {
		'A' => [0x70, 0x88, 0x88, 0xF8, 0x88, 0x88, 0x88],
		'B' => [0xF0, 0x88, 0x88, 0xF0, 0x88, 0x88, 0xF0],
		'C' => [0x70, 0x88, 0x80, 0x80, 0x80, 0x88, 0x70],
		'D' => [0xF0, 0x88, 0x88, 0x88, 0x88, 0x88, 0xF0],
		'E' => [0xF8, 0x80, 0x80, 0xF0, 0x80, 0x80, 0xF8],
		'F' => [0xF8, 0x80, 0x80, 0xF0, 0x80, 0x80, 0x80],
		'G' => [0x70, 0x88, 0x80, 0xB8, 0x88, 0x88, 0x70],
		'H' => [0x88, 0x88, 0x88, 0xF8, 0x88, 0x88, 0x88],
		'I' => [0x70, 0x20, 0x20, 0x20, 0x20, 0x20, 0x70],
		'J' => [0x38, 0x10, 0x10, 0x10, 0x90, 0x90, 0x60],
		'K' => [0x88, 0x90, 0xA0, 0xC0, 0xA0, 0x90, 0x88],
		'L' => [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xF8],
		'M' => [0x88, 0xD8, 0xA8, 0xA8, 0x88, 0x88, 0x88],
		'N' => [0x88, 0xC8, 0xA8, 0x98, 0x88, 0x88, 0x88],
		'O' => [0x70, 0x88, 0x88, 0x88, 0x88, 0x88, 0x70],
		'P' => [0xF0, 0x88, 0x88, 0xF0, 0x80, 0x80, 0x80],
		'Q' => [0x70, 0x88, 0x88, 0x88, 0xA8, 0x90, 0x68],
		'R' => [0xF0, 0x88, 0x88, 0xF0, 0xA0, 0x90, 0x88],
		'S' => [0x70, 0x88, 0x80, 0x70, 0x08, 0x88, 0x70],
		'T' => [0xF8, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20],
		'U' => [0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x70],
		'V' => [0x88, 0x88, 0x88, 0x88, 0x50, 0x50, 0x20],
		'W' => [0x88, 0x88, 0x88, 0xA8, 0xA8, 0xD8, 0x88],
		'X' => [0x88, 0x88, 0x50, 0x20, 0x50, 0x88, 0x88],
		'Y' => [0x88, 0x88, 0x50, 0x20, 0x20, 0x20, 0x20],
		'Z' => [0xF8, 0x08, 0x10, 0x20, 0x40, 0x80, 0xF8],
		' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
		'-' => [0x00, 0x00, 0x00, 0xF8, 0x00, 0x00, 0x00],
		'=' => [0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x00],
		_ => [0xF8, 0xF8, 0xF8, 0xF8, 0xF8, 0xF8, 0xF8], // Unknown char
	};

	for (row, byte) in bitmap.iter().enumerate() {
		for col in 0..5 {
			if (byte >> (7 - col)) & 1 == 1 {
				let px = x + col;
				let py = y + row as i32;
				if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
					let index = (py as u32 * width + px as u32) as usize;
					if index < buffer.len() {
						buffer[index] = color;
					}
				}
			}
		}
	}
}
