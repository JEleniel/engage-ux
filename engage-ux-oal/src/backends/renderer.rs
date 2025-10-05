//! Rendering backend abstraction
//!
//! Provides a platform-independent rendering interface that can be implemented
//! for different graphics APIs (Direct2D, Core Graphics, Cairo, etc.)

use std::fmt;

/// Color for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {
	pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self { r, g, b, a }
	}

	pub fn rgb(r: f32, g: f32, b: f32) -> Self {
		Self::rgba(r, g, b, 1.0)
	}
}

/// Rectangle for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
	Left,
	Center,
	Right,
}

/// Rendering commands
#[derive(Debug, Clone)]
pub enum RenderCommand {
	/// Clear the screen with a color
	Clear(Color),
	/// Draw a filled rectangle
	FillRect { rect: Rect, color: Color },
	/// Draw a stroked rectangle
	StrokeRect {
		rect: Rect,
		color: Color,
		width: f32,
	},
	/// Draw text
	Text {
		text: String,
		x: f32,
		y: f32,
		font_size: f32,
		color: Color,
		align: TextAlign,
	},
	/// Draw a line
	Line {
		x1: f32,
		y1: f32,
		x2: f32,
		y2: f32,
		color: Color,
		width: f32,
	},
	/// Draw a circle
	Circle {
		x: f32,
		y: f32,
		radius: f32,
		color: Color,
		filled: bool,
	},
	/// Set clip region
	SetClip(Rect),
	/// Restore previous clip
	RestoreClip,
}

/// Rendering context for a frame
pub trait RenderContext {
	/// Execute a render command
	fn execute(&mut self, command: RenderCommand);

	/// Execute multiple commands
	fn execute_batch(&mut self, commands: &[RenderCommand]) {
		for command in commands {
			self.execute(command.clone());
		}
	}

	/// Begin a new frame
	fn begin_frame(&mut self);

	/// End the current frame and present
	fn end_frame(&mut self);

	/// Get the size of the render target
	fn size(&self) -> (u32, u32);
}

/// Platform-specific rendering backend
pub trait RenderBackend: fmt::Debug {
	/// Create a new render context
	fn create_context(&mut self, width: u32, height: u32) -> Box<dyn RenderContext>;

	/// Get backend name
	fn name(&self) -> &str;

	/// Check if backend is hardware accelerated
	fn is_hardware_accelerated(&self) -> bool {
		false
	}
}

/// Stub renderer for testing and unsupported platforms
#[derive(Debug)]
pub struct StubRenderer;

impl RenderBackend for StubRenderer {
	fn create_context(&mut self, width: u32, height: u32) -> Box<dyn RenderContext> {
		Box::new(StubRenderContext { width, height })
	}

	fn name(&self) -> &str {
		"Stub Renderer"
	}

	fn is_hardware_accelerated(&self) -> bool {
		false
	}
}

/// Stub render context
struct StubRenderContext {
	width: u32,
	height: u32,
}

impl RenderContext for StubRenderContext {
	fn execute(&mut self, _command: RenderCommand) {
		// Stub: no-op
	}

	fn begin_frame(&mut self) {
		// Stub: no-op
	}

	fn end_frame(&mut self) {
		// Stub: no-op
	}

	fn size(&self) -> (u32, u32) {
		(self.width, self.height)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_color_creation() {
		let color = Color::rgba(1.0, 0.5, 0.0, 0.8);
		assert_eq!(color.r, 1.0);
		assert_eq!(color.g, 0.5);
		assert_eq!(color.b, 0.0);
		assert_eq!(color.a, 0.8);
	}

	#[test]
	fn test_rect_creation() {
		let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
		assert_eq!(rect.x, 10.0);
		assert_eq!(rect.y, 20.0);
		assert_eq!(rect.width, 100.0);
		assert_eq!(rect.height, 50.0);
	}

	#[test]
	fn test_stub_renderer() {
		let mut renderer = StubRenderer;
		assert_eq!(renderer.name(), "Stub Renderer");
		assert!(!renderer.is_hardware_accelerated());

		let mut context = renderer.create_context(800, 600);
		assert_eq!(context.size(), (800, 600));

		context.begin_frame();
		context.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)));
		context.end_frame();
	}

	#[test]
	fn test_render_commands() {
		let rect = Rect::new(10.0, 10.0, 100.0, 100.0);
		let color = Color::rgb(1.0, 0.0, 0.0);

		let command = RenderCommand::FillRect { rect, color };
		match command {
			RenderCommand::FillRect { rect: r, color: c } => {
				assert_eq!(r, rect);
				assert_eq!(c, color);
			}
			_ => panic!("Wrong command type"),
		}
	}
}
