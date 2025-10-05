//! Tiny-skia-based renderer implementation for Linux
//!
//! This provides a high-quality 2D graphics renderer using tiny-skia,
//! which is particularly suitable for Linux systems requiring Cairo-like capabilities.

use super::renderer::{Color, Rect, RenderBackend, RenderCommand, RenderContext};

/// Tiny-skia-based renderer optimized for Linux
#[derive(Debug)]
pub struct TinySkiaRenderer;

impl TinySkiaRenderer {
	/// Create a new tiny-skia renderer
	pub fn new() -> Self {
		Self
	}
}

impl Default for TinySkiaRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl RenderBackend for TinySkiaRenderer {
	fn create_context(&mut self, width: u32, height: u32) -> Box<dyn RenderContext> {
		Box::new(TinySkiaRenderContext::new(width, height))
	}

	fn name(&self) -> &str {
		"Tiny-Skia Renderer"
	}

	fn is_hardware_accelerated(&self) -> bool {
		false // CPU-based but optimized
	}
}

/// Tiny-skia render context
struct TinySkiaRenderContext {
	width: u32,
	height: u32,
	pixmap: Option<tiny_skia::Pixmap>,
	clip_stack: Vec<Rect>,
}

impl TinySkiaRenderContext {
	fn new(width: u32, height: u32) -> Self {
		let pixmap = tiny_skia::Pixmap::new(width, height);
		Self {
			width,
			height,
			pixmap,
			clip_stack: Vec::new(),
		}
	}

	/// Convert our Color type to tiny-skia Color
	fn to_skia_color(color: Color) -> tiny_skia::Color {
		tiny_skia::Color::from_rgba(
			color.r.clamp(0.0, 1.0),
			color.g.clamp(0.0, 1.0),
			color.b.clamp(0.0, 1.0),
			color.a.clamp(0.0, 1.0),
		)
		.unwrap_or(tiny_skia::Color::BLACK)
	}

	/// Convert our Rect to tiny-skia Rect
	fn to_skia_rect(rect: Rect) -> Option<tiny_skia::Rect> {
		tiny_skia::Rect::from_xywh(rect.x, rect.y, rect.width, rect.height)
	}

	/// Get the current clip mask
	fn get_clip_mask(&self) -> Option<tiny_skia::Mask> {
		if let Some(clip) = self.clip_stack.last() {
			if let Some(rect) = Self::to_skia_rect(*clip) {
				let mut mask = tiny_skia::Mask::new(self.width, self.height)?;
				mask.fill_path(
					&tiny_skia::PathBuilder::from_rect(rect),
					tiny_skia::FillRule::Winding,
					false,
					tiny_skia::Transform::identity(),
				);
				Some(mask)
			} else {
				None
			}
		} else {
			None
		}
	}

	/// Execute a clear command
	fn execute_clear(&mut self, color: Color) {
		if let Some(ref mut pixmap) = self.pixmap {
			pixmap.fill(Self::to_skia_color(color));
		}
	}

	/// Execute a filled rectangle command
	fn execute_fill_rect(&mut self, rect: Rect, color: Color) {
		if let Some(skia_rect) = Self::to_skia_rect(rect) {
			if let Some(pixmap) = self.pixmap.as_mut() {
				let mut paint = tiny_skia::Paint::default();
				paint.set_color(Self::to_skia_color(color));
				paint.anti_alias = true;

				let path = tiny_skia::PathBuilder::from_rect(skia_rect);

				pixmap.fill_path(
					&path,
					&paint,
					tiny_skia::FillRule::Winding,
					tiny_skia::Transform::identity(),
					None,
				);
			}
		}
	}

	/// Execute a stroked rectangle command
	fn execute_stroke_rect(&mut self, rect: Rect, color: Color, width: f32) {
		if let Some(skia_rect) = Self::to_skia_rect(rect) {
			if let Some(pixmap) = self.pixmap.as_mut() {
				let mut paint = tiny_skia::Paint::default();
				paint.set_color(Self::to_skia_color(color));
				paint.anti_alias = true;

				let mut stroke = tiny_skia::Stroke::default();
				stroke.width = width;

				let path = tiny_skia::PathBuilder::from_rect(skia_rect);

				pixmap.stroke_path(
					&path,
					&paint,
					&stroke,
					tiny_skia::Transform::identity(),
					None,
				);
			}
		}
	}

	/// Execute a circle command
	fn execute_circle(&mut self, x: f32, y: f32, radius: f32, color: Color, filled: bool) {
		if let Some(ref mut pixmap) = self.pixmap {
			let mut paint = tiny_skia::Paint::default();
			paint.set_color(Self::to_skia_color(color));
			paint.anti_alias = true;

			if let Some(path) = tiny_skia::PathBuilder::from_circle(x, y, radius) {
				if filled {
					pixmap.fill_path(
						&path,
						&paint,
						tiny_skia::FillRule::Winding,
						tiny_skia::Transform::identity(),
						None,
					);
				} else {
					let mut stroke = tiny_skia::Stroke::default();
					stroke.width = 1.0;
					pixmap.stroke_path(
						&path,
						&paint,
						&stroke,
						tiny_skia::Transform::identity(),
						None,
					);
				}
			}
		}
	}

	/// Execute a line command
	fn execute_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, width: f32) {
		if let Some(ref mut pixmap) = self.pixmap {
			let mut paint = tiny_skia::Paint::default();
			paint.set_color(Self::to_skia_color(color));
			paint.anti_alias = true;

			let mut stroke = tiny_skia::Stroke::default();
			stroke.width = width;

			let mut pb = tiny_skia::PathBuilder::new();
			pb.move_to(x1, y1);
			pb.line_to(x2, y2);

			if let Some(path) = pb.finish() {
				pixmap.stroke_path(
					&path,
					&paint,
					&stroke,
					tiny_skia::Transform::identity(),
					None,
				);
			}
		}
	}
}

impl RenderContext for TinySkiaRenderContext {
	fn execute(&mut self, command: RenderCommand) {
		match command {
			RenderCommand::Clear(color) => {
				self.execute_clear(color);
			}
			RenderCommand::FillRect { rect, color } => {
				self.execute_fill_rect(rect, color);
			}
			RenderCommand::StrokeRect { rect, color, width } => {
				self.execute_stroke_rect(rect, color, width);
			}
			RenderCommand::Circle {
				x,
				y,
				radius,
				color,
				filled,
			} => {
				self.execute_circle(x, y, radius, color, filled);
			}
			RenderCommand::Line {
				x1,
				y1,
				x2,
				y2,
				color,
				width,
			} => {
				self.execute_line(x1, y1, x2, y2, color, width);
			}
			RenderCommand::Text { .. } => {
				// Text rendering would require font rasterization
				// This is a placeholder for future implementation
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
		// Ensure we have a valid pixmap
		if self.pixmap.is_none() {
			self.pixmap = tiny_skia::Pixmap::new(self.width, self.height);
		}
	}

	fn end_frame(&mut self) {
		// In a real implementation, this would present the frame
		// For now, we just keep the pixmap ready
	}

	fn size(&self) -> (u32, u32) {
		(self.width, self.height)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tiny_skia_renderer_creation() {
		let mut renderer = TinySkiaRenderer::new();
		assert_eq!(renderer.name(), "Tiny-Skia Renderer");
		assert!(!renderer.is_hardware_accelerated());
	}

	#[test]
	fn test_tiny_skia_context_creation() {
		let mut renderer = TinySkiaRenderer::new();
		let context = renderer.create_context(800, 600);
		assert_eq!(context.size(), (800, 600));
	}

	#[test]
	fn test_tiny_skia_basic_rendering() {
		let mut renderer = TinySkiaRenderer::new();
		let mut context = renderer.create_context(400, 400);

		context.begin_frame();
		context.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)));
		context.execute(RenderCommand::FillRect {
			rect: Rect::new(10.0, 10.0, 100.0, 100.0),
			color: Color::rgb(1.0, 0.0, 0.0),
		});
		context.end_frame();

		// Test passes if no panic
	}

	#[test]
	fn test_tiny_skia_shapes() {
		let mut renderer = TinySkiaRenderer::new();
		let mut context = renderer.create_context(400, 400);

		context.begin_frame();

		// Test all shape types
		context.execute(RenderCommand::FillRect {
			rect: Rect::new(10.0, 10.0, 50.0, 50.0),
			color: Color::rgb(1.0, 0.0, 0.0),
		});

		context.execute(RenderCommand::StrokeRect {
			rect: Rect::new(100.0, 10.0, 50.0, 50.0),
			color: Color::rgb(0.0, 1.0, 0.0),
			width: 2.0,
		});

		context.execute(RenderCommand::Circle {
			x: 200.0,
			y: 35.0,
			radius: 25.0,
			color: Color::rgb(0.0, 0.0, 1.0),
			filled: true,
		});

		context.execute(RenderCommand::Line {
			x1: 10.0,
			y1: 100.0,
			x2: 200.0,
			y2: 100.0,
			color: Color::rgb(1.0, 1.0, 0.0),
			width: 2.0,
		});

		context.end_frame();
	}

	#[test]
	fn test_tiny_skia_clipping() {
		let mut renderer = TinySkiaRenderer::new();
		let mut context = renderer.create_context(400, 400);

		context.begin_frame();
		context.execute(RenderCommand::SetClip(Rect::new(50.0, 50.0, 100.0, 100.0)));
		context.execute(RenderCommand::FillRect {
			rect: Rect::new(0.0, 0.0, 200.0, 200.0),
			color: Color::rgb(1.0, 0.0, 0.0),
		});
		context.execute(RenderCommand::RestoreClip);
		context.end_frame();
	}
}
