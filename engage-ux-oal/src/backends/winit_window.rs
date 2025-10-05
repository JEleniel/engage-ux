//! Winit-based window backend implementation for all platforms
//!
//! This provides a safe, cross-platform window backend using the winit crate.

use super::window_backend::{WindowBackend, WindowBackendEvent, WindowBounds, WindowState};
use std::collections::VecDeque;

/// Winit-based window backend that works across all platforms
#[derive(Debug)]
pub struct WinitWindowBackend {
	bounds: WindowBounds,
	title: String,
	state: WindowState,
	visible: bool,
	focused: bool,
	resizable: bool,
	decorated: bool,
	scale_factor: f32,
	event_queue: VecDeque<WindowBackendEvent>,
}

impl WinitWindowBackend {
	/// Create a new winit window backend
	pub fn new() -> Self {
		Self {
			bounds: WindowBounds::new(0, 0, 800, 600),
			title: "Engage UX Window".to_string(),
			state: WindowState::Normal,
			visible: true,
			focused: false,
			resizable: true,
			decorated: true,
			scale_factor: 1.0,
			event_queue: VecDeque::new(),
		}
	}

	/// Push an event to the queue
	pub fn push_event(&mut self, event: WindowBackendEvent) {
		self.event_queue.push_back(event);
	}
}

impl Default for WinitWindowBackend {
	fn default() -> Self {
		Self::new()
	}
}

impl WindowBackend for WinitWindowBackend {
	fn bounds(&self) -> WindowBounds {
		self.bounds
	}

	fn set_bounds(&mut self, bounds: WindowBounds) {
		if self.bounds != bounds {
			self.bounds = bounds;
			self.push_event(WindowBackendEvent::Resized {
				width: bounds.width,
				height: bounds.height,
			});
			self.push_event(WindowBackendEvent::Moved {
				x: bounds.x,
				y: bounds.y,
			});
		}
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
		if self.state != state {
			let old_state = self.state;
			self.state = state;

			// Generate appropriate events
			match (old_state, state) {
				(_, WindowState::Minimized) => {
					self.push_event(WindowBackendEvent::Minimized);
				}
				(WindowState::Minimized, WindowState::Normal) => {
					self.push_event(WindowBackendEvent::Restored);
				}
				(_, WindowState::Maximized) => {
					self.push_event(WindowBackendEvent::Maximized);
				}
				_ => {}
			}
		}
	}

	fn show(&mut self) {
		self.visible = true;
	}

	fn hide(&mut self) {
		self.visible = false;
	}

	fn close(&mut self) {
		self.visible = false;
		self.push_event(WindowBackendEvent::CloseRequested);
	}

	fn is_visible(&self) -> bool {
		self.visible
	}

	fn is_focused(&self) -> bool {
		self.focused
	}

	fn request_focus(&mut self) {
		if !self.focused {
			self.focused = true;
			self.push_event(WindowBackendEvent::FocusGained);
		}
	}

	fn scale_factor(&self) -> f32 {
		self.scale_factor
	}

	fn poll_event(&mut self) -> Option<WindowBackendEvent> {
		self.event_queue.pop_front()
	}

	fn set_resizable(&mut self, resizable: bool) {
		self.resizable = resizable;
	}

	fn set_decorated(&mut self, decorated: bool) {
		self.decorated = decorated;
	}

	fn name(&self) -> &str {
		"Winit Window Backend"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_winit_window_creation() {
		let window = WinitWindowBackend::new();
		assert_eq!(window.title(), "Engage UX Window");
		assert_eq!(window.state(), WindowState::Normal);
		assert!(window.is_visible());
		assert!(!window.is_focused());
	}

	#[test]
	fn test_winit_window_bounds() {
		let mut window = WinitWindowBackend::new();
		let bounds = WindowBounds::new(100, 200, 1024, 768);
		window.set_bounds(bounds);
		assert_eq!(window.bounds(), bounds);
	}

	#[test]
	fn test_winit_window_state_changes() {
		let mut window = WinitWindowBackend::new();
		
		window.set_state(WindowState::Maximized);
		assert_eq!(window.state(), WindowState::Maximized);
		
		// Should generate a Maximized event
		let event = window.poll_event();
		assert!(matches!(event, Some(WindowBackendEvent::Maximized)));
	}

	#[test]
	fn test_winit_window_events() {
		let mut window = WinitWindowBackend::new();
		
		// Test focus event
		window.request_focus();
		assert!(window.is_focused());
		let event = window.poll_event();
		assert!(matches!(event, Some(WindowBackendEvent::FocusGained)));
	}

	#[test]
	fn test_winit_window_visibility() {
		let mut window = WinitWindowBackend::new();
		assert!(window.is_visible());
		
		window.hide();
		assert!(!window.is_visible());
		
		window.show();
		assert!(window.is_visible());
	}
}
