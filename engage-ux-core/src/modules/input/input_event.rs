use crate::modules::input::mouse::MouseEvent;
use crate::modules::input::touch::TouchEvent;
use keyboard_types::KeyboardEvent;

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
