//! Window backend abstraction for platform-specific window management

use std::fmt;

/// Window backend event
#[derive(Debug, Clone, PartialEq)]
pub enum WindowBackendEvent {
	/// Window was resized
	Resized { width: u32, height: u32 },
	/// Window was moved
	Moved { x: i32, y: i32 },
	/// Window close requested
	CloseRequested,
	/// Window gained focus
	FocusGained,
	/// Window lost focus
	FocusLost,
	/// Window was minimized
	Minimized,
	/// Window was restored
	Restored,
	/// Window was maximized
	Maximized,
	/// DPI changed
	DpiChanged { scale: f32 },
}

/// Window position and size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowBounds {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

impl WindowBounds {
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
	Normal,
	Minimized,
	Maximized,
	Fullscreen,
}

/// Platform-specific window backend
pub trait WindowBackend: fmt::Debug {
	/// Get the window bounds
	fn bounds(&self) -> WindowBounds;

	/// Set the window bounds
	fn set_bounds(&mut self, bounds: WindowBounds);

	/// Get the window title
	fn title(&self) -> &str;

	/// Set the window title
	fn set_title(&mut self, title: &str);

	/// Get the window state
	fn state(&self) -> WindowState;

	/// Set the window state
	fn set_state(&mut self, state: WindowState);

	/// Show the window
	fn show(&mut self);

	/// Hide the window
	fn hide(&mut self);

	/// Close the window
	fn close(&mut self);

	/// Check if window is visible
	fn is_visible(&self) -> bool;

	/// Check if window is focused
	fn is_focused(&self) -> bool;

	/// Request focus
	fn request_focus(&mut self);

	/// Get DPI scale factor
	fn scale_factor(&self) -> f32 {
		1.0
	}

	/// Poll for events (returns None when no events available)
	fn poll_event(&mut self) -> Option<WindowBackendEvent>;

	/// Set window resizable
	fn set_resizable(&mut self, resizable: bool);

	/// Set window decorations (title bar, borders)
	fn set_decorated(&mut self, decorated: bool);

	/// Get backend name
	fn name(&self) -> &str;
}

/// Stub window backend for testing and unsupported platforms
#[derive(Debug)]
pub struct StubWindowBackend {
	bounds: WindowBounds,
	title: String,
	state: WindowState,
	visible: bool,
	focused: bool,
	resizable: bool,
	decorated: bool,
}

impl Default for StubWindowBackend {
	fn default() -> Self {
		Self {
			bounds: WindowBounds::new(0, 0, 800, 600),
			title: "Engage UX Window".to_string(),
			state: WindowState::Normal,
			visible: true,
			focused: false,
			resizable: true,
			decorated: true,
		}
	}
}

impl WindowBackend for StubWindowBackend {
	fn bounds(&self) -> WindowBounds {
		self.bounds
	}

	fn set_bounds(&mut self, bounds: WindowBounds) {
		self.bounds = bounds;
	}

	fn title(&self) -> &str {
		&self.title
	}

	fn set_title(&mut self, title: &str) {
		self.title = title.to_string();
	}

	fn state(&self) -> WindowState {
		self.state
	}

	fn set_state(&mut self, state: WindowState) {
		self.state = state;
	}

	fn show(&mut self) {
		self.visible = true;
	}

	fn hide(&mut self) {
		self.visible = false;
	}

	fn close(&mut self) {
		self.visible = false;
	}

	fn is_visible(&self) -> bool {
		self.visible
	}

	fn is_focused(&self) -> bool {
		self.focused
	}

	fn request_focus(&mut self) {
		self.focused = true;
	}

	fn poll_event(&mut self) -> Option<WindowBackendEvent> {
		None
	}

	fn set_resizable(&mut self, resizable: bool) {
		self.resizable = resizable;
	}

	fn set_decorated(&mut self, decorated: bool) {
		self.decorated = decorated;
	}

	fn name(&self) -> &str {
		"Stub Window Backend"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_window_bounds() {
		let bounds = WindowBounds::new(100, 100, 800, 600);
		assert_eq!(bounds.x, 100);
		assert_eq!(bounds.y, 100);
		assert_eq!(bounds.width, 800);
		assert_eq!(bounds.height, 600);
	}

	#[test]
	fn test_stub_window_backend() {
		let mut backend = StubWindowBackend::default();
		assert_eq!(backend.name(), "Stub Window Backend");
		assert!(backend.is_visible());
		assert!(!backend.is_focused());

		backend.set_title("Test Window");
		assert_eq!(backend.title(), "Test Window");

		backend.hide();
		assert!(!backend.is_visible());

		backend.show();
		assert!(backend.is_visible());
	}

	#[test]
	fn test_window_state() {
		let mut backend = StubWindowBackend::default();
		assert_eq!(backend.state(), WindowState::Normal);

		backend.set_state(WindowState::Maximized);
		assert_eq!(backend.state(), WindowState::Maximized);
	}

	#[test]
	fn test_window_bounds_update() {
		let mut backend = StubWindowBackend::default();
		let new_bounds = WindowBounds::new(200, 200, 1024, 768);
		backend.set_bounds(new_bounds);
		assert_eq!(backend.bounds(), new_bounds);
	}
}
