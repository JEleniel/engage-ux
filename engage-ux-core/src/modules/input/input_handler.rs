use crate::modules::input::input_event::InputEvent;
use crate::modules::input::mouse::MouseEvent;
use crate::modules::input::touch::TouchEvent;
use keyboard_types::KeyboardEvent;

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
