//! Mouse input handling

use std::collections::HashSet;

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
	/// Left mouse button
	Left,
	/// Right mouse button
	Right,
	/// Middle mouse button (scroll wheel click)
	Middle,
	/// Additional button (e.g., back button)
	Button4,
	/// Additional button (e.g., forward button)
	Button5,
}

/// Mouse event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
	/// Mouse button pressed
	ButtonDown,
	/// Mouse button released
	ButtonUp,
	/// Mouse moved
	Move,
	/// Mouse wheel scrolled
	Wheel,
	/// Mouse entered component
	Enter,
	/// Mouse left component
	Leave,
}

/// Mouse event with position and button information
#[derive(Debug, Clone, PartialEq)]
pub struct MouseEvent {
	/// Type of mouse event
	pub event_type: MouseEventType,
	/// Button involved (if applicable)
	pub button: Option<MouseButton>,
	/// X coordinate
	pub x: f32,
	/// Y coordinate
	pub y: f32,
	/// Scroll delta X (for wheel events)
	pub delta_x: f32,
	/// Scroll delta Y (for wheel events)
	pub delta_y: f32,
}

impl MouseEvent {
	/// Create a button down event
	pub fn button_down(button: MouseButton, x: f32, y: f32) -> Self {
		Self {
			event_type: MouseEventType::ButtonDown,
			button: Some(button),
			x,
			y,
			delta_x: 0.0,
			delta_y: 0.0,
		}
	}

	/// Create a button up event
	pub fn button_up(button: MouseButton, x: f32, y: f32) -> Self {
		Self {
			event_type: MouseEventType::ButtonUp,
			button: Some(button),
			x,
			y,
			delta_x: 0.0,
			delta_y: 0.0,
		}
	}

	/// Create a mouse move event
	pub fn move_event(x: f32, y: f32) -> Self {
		Self {
			event_type: MouseEventType::Move,
			button: None,
			x,
			y,
			delta_x: 0.0,
			delta_y: 0.0,
		}
	}

	/// Create a mouse wheel event
	pub fn wheel(x: f32, y: f32, delta_x: f32, delta_y: f32) -> Self {
		Self {
			event_type: MouseEventType::Wheel,
			button: None,
			x,
			y,
			delta_x,
			delta_y,
		}
	}

	/// Create a mouse enter event
	pub fn enter(x: f32, y: f32) -> Self {
		Self {
			event_type: MouseEventType::Enter,
			button: None,
			x,
			y,
			delta_x: 0.0,
			delta_y: 0.0,
		}
	}

	/// Create a mouse leave event
	pub fn leave(x: f32, y: f32) -> Self {
		Self {
			event_type: MouseEventType::Leave,
			button: None,
			x,
			y,
			delta_x: 0.0,
			delta_y: 0.0,
		}
	}

	/// Get position as tuple
	pub fn position(&self) -> (f32, f32) {
		(self.x, self.y)
	}
}

/// Mouse state tracker
#[derive(Debug, Default)]
pub struct MouseState {
	/// Currently pressed buttons
	pressed_buttons: HashSet<MouseButton>,
	/// Current X position
	x: f32,
	/// Current Y position
	y: f32,
	/// Previous X position
	prev_x: f32,
	/// Previous Y position
	prev_y: f32,
}

impl MouseState {
	/// Create a new mouse state
	pub fn new() -> Self {
		Self::default()
	}

	/// Update state with an event
	pub fn update(&mut self, event: &MouseEvent) {
		self.prev_x = self.x;
		self.prev_y = self.y;
		self.x = event.x;
		self.y = event.y;

		match event.event_type {
			MouseEventType::ButtonDown => {
				if let Some(button) = event.button {
					self.pressed_buttons.insert(button);
				}
			}
			MouseEventType::ButtonUp => {
				if let Some(button) = event.button {
					self.pressed_buttons.remove(&button);
				}
			}
			_ => {}
		}
	}

	/// Check if a button is currently pressed
	pub fn is_button_pressed(&self, button: MouseButton) -> bool {
		self.pressed_buttons.contains(&button)
	}

	/// Get current position
	pub fn position(&self) -> (f32, f32) {
		(self.x, self.y)
	}

	/// Get movement delta
	pub fn delta(&self) -> (f32, f32) {
		(self.x - self.prev_x, self.y - self.prev_y)
	}

	/// Clear all pressed buttons
	pub fn clear(&mut self) {
		self.pressed_buttons.clear();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mouse_event_creation() {
		let event = MouseEvent::button_down(MouseButton::Left, 100.0, 200.0);
		assert_eq!(event.event_type, MouseEventType::ButtonDown);
		assert_eq!(event.button, Some(MouseButton::Left));
		assert_eq!(event.position(), (100.0, 200.0));
	}

	#[test]
	fn test_mouse_state() {
		let mut state = MouseState::new();

		let down = MouseEvent::button_down(MouseButton::Left, 100.0, 100.0);
		state.update(&down);
		assert!(state.is_button_pressed(MouseButton::Left));
		assert_eq!(state.position(), (100.0, 100.0));

		let up = MouseEvent::button_up(MouseButton::Left, 150.0, 150.0);
		state.update(&up);
		assert!(!state.is_button_pressed(MouseButton::Left));
		assert_eq!(state.position(), (150.0, 150.0));
	}

	#[test]
	fn test_mouse_movement() {
		let mut state = MouseState::new();

		let move1 = MouseEvent::move_event(100.0, 100.0);
		state.update(&move1);

		let move2 = MouseEvent::move_event(150.0, 200.0);
		state.update(&move2);

		assert_eq!(state.delta(), (50.0, 100.0));
	}

	#[test]
	fn test_mouse_wheel_event() {
		let event = MouseEvent::wheel(100.0, 100.0, 10.0, -20.0);
		assert_eq!(event.event_type, MouseEventType::Wheel);
		assert_eq!(event.delta_x, 10.0);
		assert_eq!(event.delta_y, -20.0);
	}
}
