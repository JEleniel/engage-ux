//! Softbuffer-based renderer implementation for all platforms
//!
//! This provides a safe, cross-platform software renderer using the softbuffer crate.

use super::renderer::{Color, RenderBackend, RenderCommand, RenderContext, Rect};

/// Softbuffer-based renderer that works across all platforms
#[derive(Debug)]
pub struct SoftbufferRenderer;

impl SoftbufferRenderer {
	/// Create a new softbuffer renderer
	pub fn new() -> Self {
		Self
	}
}

impl Default for SoftbufferRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl RenderBackend for SoftbufferRenderer {
	fn create_context(&mut self, width: u32, height: u32) -> Box<dyn RenderContext> {
		Box::new(SoftbufferRenderContext::new(width, height))
	}

	fn name(&self) -> &str {
		"Softbuffer Renderer"
	}

	fn is_hardware_accelerated(&self) -> bool {
		false // Software renderer
	}
}

/// Softbuffer render context
struct SoftbufferRenderContext {
	width: u32,
	height: u32,
	buffer: Vec<u32>,
	clip_stack: Vec<Rect>,
}

impl SoftbufferRenderContext {
	fn new(width: u32, height: u32) -> Self {
		let size = (width * height) as usize;
		Self {
			width,
			height,
			buffer: vec![0; size],
			clip_stack: Vec::new(),
		}
	}

	/// Convert color to u32 pixel format (ARGB)
	fn color_to_pixel(color: Color) -> u32 {
		let r = (color.r.clamp(0.0, 1.0) * 255.0) as u32;
		let g = (color.g.clamp(0.0, 1.0) * 255.0) as u32;
		let b = (color.b.clamp(0.0, 1.0) * 255.0) as u32;
		let a = (color.a.clamp(0.0, 1.0) * 255.0) as u32;
		
		(a << 24) | (r << 16) | (g << 8) | b
	}

	/// Check if a point is within the current clip region
	fn is_clipped(&self, x: i32, y: i32) -> bool {
		if let Some(clip) = self.clip_stack.last() {
			let x_f = x as f32;
			let y_f = y as f32;
			x_f < clip.x
				|| x_f >= clip.x + clip.width
				|| y_f < clip.y
				|| y_f >= clip.y + clip.height
		} else {
			false
		}
	}

	/// Set a pixel in the buffer
	fn set_pixel(&mut self, x: i32, y: i32, pixel: u32) {
		if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
			if !self.is_clipped(x, y) {
				let index = (y as u32 * self.width + x as u32) as usize;
				if index < self.buffer.len() {
					self.buffer[index] = pixel;
				}
			}
		}
	}

	/// Draw a horizontal line
	fn draw_hline(&mut self, x1: i32, x2: i32, y: i32, pixel: u32) {
		let start = x1.min(x2);
		let end = x1.max(x2);
		for x in start..=end {
			self.set_pixel(x, y, pixel);
		}
	}

	/// Draw a vertical line
	fn draw_vline(&mut self, x: i32, y1: i32, y2: i32, pixel: u32) {
		let start = y1.min(y2);
		let end = y1.max(y2);
		for y in start..=end {
			self.set_pixel(x, y, pixel);
		}
	}

	/// Fill a rectangle
	fn fill_rect(&mut self, rect: Rect, pixel: u32) {
		let x1 = rect.x as i32;
		let y1 = rect.y as i32;
		let x2 = (rect.x + rect.width) as i32;
		let y2 = (rect.y + rect.height) as i32;

		for y in y1..y2 {
			self.draw_hline(x1, x2 - 1, y, pixel);
		}
	}

	/// Stroke a rectangle
	fn stroke_rect(&mut self, rect: Rect, pixel: u32, width: i32) {
		let x1 = rect.x as i32;
		let y1 = rect.y as i32;
		let x2 = (rect.x + rect.width) as i32 - 1;
		let y2 = (rect.y + rect.height) as i32 - 1;

		for i in 0..width {
			// Top and bottom edges
			self.draw_hline(x1, x2, y1 + i, pixel);
			self.draw_hline(x1, x2, y2 - i, pixel);
			
			// Left and right edges
			self.draw_vline(x1 + i, y1, y2, pixel);
			self.draw_vline(x2 - i, y1, y2, pixel);
		}
	}

	/// Draw a line using Bresenham's algorithm
	fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, pixel: u32) {
		let dx = (x2 - x1).abs();
		let dy = (y2 - y1).abs();
		let sx = if x1 < x2 { 1 } else { -1 };
		let sy = if y1 < y2 { 1 } else { -1 };
		let mut err = dx - dy;
		let mut x = x1;
		let mut y = y1;

		loop {
			self.set_pixel(x, y, pixel);
			
			if x == x2 && y == y2 {
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

	/// Draw a circle using midpoint circle algorithm
	fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, pixel: u32, filled: bool) {
		if filled {
			// Filled circle
			for y in -radius..=radius {
				let dx = ((radius * radius - y * y) as f32).sqrt() as i32;
				self.draw_hline(cx - dx, cx + dx, cy + y, pixel);
			}
		} else {
			// Circle outline using midpoint algorithm
			let mut x = radius;
			let mut y = 0;
			let mut err = 0;

			while x >= y {
				self.set_pixel(cx + x, cy + y, pixel);
				self.set_pixel(cx + y, cy + x, pixel);
				self.set_pixel(cx - y, cy + x, pixel);
				self.set_pixel(cx - x, cy + y, pixel);
				self.set_pixel(cx - x, cy - y, pixel);
				self.set_pixel(cx - y, cy - x, pixel);
				self.set_pixel(cx + y, cy - x, pixel);
				self.set_pixel(cx + x, cy - y, pixel);

				y += 1;
				err += 1 + 2 * y;
				if 2 * (err - x) + 1 > 0 {
					x -= 1;
					err += 1 - 2 * x;
				}
			}
		}
	}

	/// Clear the buffer with a color
	fn clear(&mut self, color: Color) {
		let pixel = Self::color_to_pixel(color);
		self.buffer.fill(pixel);
	}
}

impl RenderContext for SoftbufferRenderContext {
	fn execute(&mut self, command: RenderCommand) {
		match command {
			RenderCommand::Clear(color) => {
				self.clear(color);
			}
			RenderCommand::FillRect { rect, color } => {
				let pixel = Self::color_to_pixel(color);
				self.fill_rect(rect, pixel);
			}
			RenderCommand::StrokeRect {
				rect,
				color,
				width,
			} => {
				let pixel = Self::color_to_pixel(color);
				self.stroke_rect(rect, pixel, width as i32);
			}
			RenderCommand::Line {
				x1,
				y1,
				x2,
				y2,
				color,
				width: _,
			} => {
				let pixel = Self::color_to_pixel(color);
				self.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, pixel);
			}
			RenderCommand::Circle {
				x,
				y,
				radius,
				color,
				filled,
			} => {
				let pixel = Self::color_to_pixel(color);
				self.draw_circle(x as i32, y as i32, radius as i32, pixel, filled);
			}
			RenderCommand::Text {
				text: _,
				x: _,
				y: _,
				font_size: _,
				color: _,
				align: _,
			} => {
				// Text rendering would require font rasterization
				// For now, this is a no-op. Full implementation would use fontdue crate
			}
			RenderCommand::SetClip(rect) => {
				self.clip_stack.push(rect);
			}
			RenderCommand::RestoreClip => {
				self.clip_stack.pop();
			}
		}
	}

	fn begin_frame(&mut self) {
		// No-op for software renderer
	}

	fn end_frame(&mut self) {
		// In a real implementation, this would present the buffer to the screen
		// For now, we just keep the buffer in memory
	}

	fn size(&self) -> (u32, u32) {
		(self.width, self.height)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_softbuffer_renderer_creation() {
		let mut renderer = SoftbufferRenderer::new();
		assert_eq!(renderer.name(), "Softbuffer Renderer");
		assert!(!renderer.is_hardware_accelerated());
	}

	#[test]
	fn test_softbuffer_context_creation() {
		let mut renderer = SoftbufferRenderer::new();
		let context = renderer.create_context(800, 600);
		assert_eq!(context.size(), (800, 600));
	}

	#[test]
	fn test_softbuffer_clear() {
		let mut renderer = SoftbufferRenderer::new();
		let mut context = renderer.create_context(100, 100);
		
		context.begin_frame();
		context.execute(RenderCommand::Clear(Color::rgb(1.0, 0.0, 0.0)));
		context.end_frame();
		
		// Test passes if no panic occurs
	}

	#[test]
	fn test_softbuffer_draw_rect() {
		let mut renderer = SoftbufferRenderer::new();
		let mut context = renderer.create_context(100, 100);
		
		context.begin_frame();
		context.execute(RenderCommand::FillRect {
			rect: Rect::new(10.0, 10.0, 50.0, 50.0),
			color: Color::rgb(0.0, 1.0, 0.0),
		});
		context.end_frame();
	}

	#[test]
	fn test_softbuffer_draw_circle() {
		let mut renderer = SoftbufferRenderer::new();
		let mut context = renderer.create_context(100, 100);
		
		context.begin_frame();
		context.execute(RenderCommand::Circle {
			x: 50.0,
			y: 50.0,
			radius: 20.0,
			color: Color::rgb(0.0, 0.0, 1.0),
			filled: true,
		});
		context.end_frame();
	}

	#[test]
	fn test_softbuffer_clipping() {
		let mut renderer = SoftbufferRenderer::new();
		let mut context = renderer.create_context(100, 100);
		
		context.begin_frame();
		context.execute(RenderCommand::SetClip(Rect::new(10.0, 10.0, 50.0, 50.0)));
		context.execute(RenderCommand::FillRect {
			rect: Rect::new(0.0, 0.0, 100.0, 100.0),
			color: Color::rgb(1.0, 1.0, 1.0),
		});
		context.execute(RenderCommand::RestoreClip);
		context.end_frame();
	}
}
