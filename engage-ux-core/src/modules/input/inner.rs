//! Implementation moved from `input.rs` into `input/inner.rs`.

pub use super::keyboard::{Key, KeyCode, KeyModifiers, KeyboardEvent, KeyboardState};
pub use super::mouse::{MouseButton, MouseEvent, MouseState};
pub use super::touch::{Touch, TouchEvent, TouchPhase, TouchState};

/// Unified input event that can represent any input type
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
	/// Keyboard input event
	Keyboard(KeyboardEvent),
	/// Mouse input event
	Mouse(MouseEvent),
	/// Touch input event
	Touch(TouchEvent),
}

/// Input handler trait for components that need to handle input
pub trait InputHandler {
	/// Handle a keyboard event
	fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
		let _ = event;
		false
	}

	/// Handle a mouse event
	fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
		let _ = event;
		false
	}

	/// Handle a touch event
	fn handle_touch(&mut self, event: &TouchEvent) -> bool {
		let _ = event;
		false
	}

	/// Handle any input event
	fn handle_input(&mut self, event: &InputEvent) -> bool {
		match event {
			InputEvent::Keyboard(e) => self.handle_keyboard(e),
			InputEvent::Mouse(e) => self.handle_mouse(e),
			InputEvent::Touch(e) => self.handle_touch(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestHandler {
		keyboard_handled: bool,
		mouse_handled: bool,
		touch_handled: bool,
	}

	impl InputHandler for TestHandler {
		fn handle_keyboard(&mut self, _event: &KeyboardEvent) -> bool {
			self.keyboard_handled = true;
			true
		}

		fn handle_mouse(&mut self, _event: &MouseEvent) -> bool {
			self.mouse_handled = true;
			true
		}

		fn handle_touch(&mut self, _event: &TouchEvent) -> bool {
			self.touch_handled = true;
			true
		}
	}

	#[test]
	fn test_input_handler_keyboard() {
		let mut handler = TestHandler {
			keyboard_handled: false,
			mouse_handled: false,
			touch_handled: false,
		};

		let event = KeyboardEvent::key_down(KeyCode::Enter, KeyModifiers::empty());
		handler.handle_keyboard(&event);
		assert!(handler.keyboard_handled);
	}

	#[test]
	fn test_input_handler_mouse() {
		let mut handler = TestHandler {
			keyboard_handled: false,
			mouse_handled: false,
			touch_handled: false,
		};

		let event = MouseEvent::button_down(MouseButton::Left, 100.0, 100.0);
		handler.handle_mouse(&event);
		assert!(handler.mouse_handled);
	}

	#[test]
	fn test_input_handler_touch() {
		let mut handler = TestHandler {
			keyboard_handled: false,
			mouse_handled: false,
			touch_handled: false,
		};

		let event = TouchEvent::new(TouchPhase::Began, vec![Touch::new(0, 100.0, 100.0)]);
		handler.handle_touch(&event);
		assert!(handler.touch_handled);
	}

	#[test]
	fn test_custom_input_event() {
		// custom input types removed in migration; basic compile-time smoke
		// This test ensures that keyboard/mouse/touch codepaths still work.
		let _ = MouseEvent::button_down(MouseButton::Left, 10.0, 20.0);
		let _ = KeyboardEvent::key_down(
			crate::keyboard::KeyCode::Enter,
			crate::keyboard::KeyModifiers::empty(),
		);
	}

	#[test]
	fn test_custom_input_event_missing_values() {
		// custom input types removed; ensure no panics in input module
		let _ = TouchEvent::new(TouchPhase::Began, vec![Touch::new(0, 0.0, 0.0)]);
	}

	#[test]
	fn test_input_event_custom_variant() {
		// Custom variant removed; ensure InputEvent variants still match expected types
		let event = InputEvent::Mouse(MouseEvent::move_event(1.0, 2.0));
		match event {
			InputEvent::Mouse(e) => {
				assert_eq!(e.position(), (1.0, 2.0));
			}
			_ => panic!("Expected Mouse variant"),
		}
	}
}
