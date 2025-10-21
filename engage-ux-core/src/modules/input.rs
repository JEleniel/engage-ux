//! Input handling system for Engage UX
//!
//! Provides comprehensive input handling for keyboard, mouse, and touch events
//! with full support for accessibility and multi-modal interaction.

pub mod keyboard;
pub mod mouse;
pub mod touch;

use keyboard_types::KeyboardEvent;
pub use mouse::{MouseButton, MouseEvent, MouseState};
pub use touch::{Touch, TouchEvent, TouchPhase, TouchState};

/// Unified input event that can represent any input type
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
	/// Keyboard input event
	Keyboard(KeyboardEvent),
	/// Mouse input event
	Mouse(MouseEvent),
	/// Touch input event
	Touch(TouchEvent),
	/// Custom input event from other devices
	Custom(CustomInputEvent),
}

/// Custom input event for supporting additional input devices
/// (gamepad, stylus, motion sensors, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct CustomInputEvent {
	/// Device type identifier
	pub device_type: String,
	/// Event type identifier
	pub event_type: String,
	/// Event data as key-value pairs
	pub data: std::collections::HashMap<String, CustomInputValue>,
}

/// Value type for custom input data
#[derive(Debug, Clone, PartialEq)]
pub enum CustomInputValue {
	/// Boolean value
	Bool(bool),
	/// Integer value
	Int(i64),
	/// Float value
	Float(f64),
	/// String value
	String(String),
	/// Array of values
	Array(Vec<CustomInputValue>),
}

impl CustomInputEvent {
	/// Create a new custom input event
	pub fn new(device_type: impl Into<String>, event_type: impl Into<String>) -> Self {
		Self {
			device_type: device_type.into(),
			event_type: event_type.into(),
			data: std::collections::HashMap::new(),
		}
	}

	/// Add boolean data
	pub fn with_bool(mut self, key: impl Into<String>, value: bool) -> Self {
		self.data.insert(key.into(), CustomInputValue::Bool(value));
		self
	}

	/// Add integer data
	pub fn with_int(mut self, key: impl Into<String>, value: i64) -> Self {
		self.data.insert(key.into(), CustomInputValue::Int(value));
		self
	}

	/// Add float data
	pub fn with_float(mut self, key: impl Into<String>, value: f64) -> Self {
		self.data.insert(key.into(), CustomInputValue::Float(value));
		self
	}

	/// Add string data
	pub fn with_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
		self.data
			.insert(key.into(), CustomInputValue::String(value.into()));
		self
	}

	/// Get boolean value
	pub fn get_bool(&self, key: &str) -> Option<bool> {
		if let Some(CustomInputValue::Bool(v)) = self.data.get(key) {
			Some(*v)
		} else {
			None
		}
	}

	/// Get integer value
	pub fn get_int(&self, key: &str) -> Option<i64> {
		if let Some(CustomInputValue::Int(v)) = self.data.get(key) {
			Some(*v)
		} else {
			None
		}
	}

	/// Get float value
	pub fn get_float(&self, key: &str) -> Option<f64> {
		if let Some(CustomInputValue::Float(v)) = self.data.get(key) {
			Some(*v)
		} else {
			None
		}
	}

	/// Get string value
	pub fn get_string(&self, key: &str) -> Option<&str> {
		if let Some(CustomInputValue::String(v)) = self.data.get(key) {
			Some(v.as_str())
		} else {
			None
		}
	}
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

	/// Handle a custom input event
	fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
		let _ = event;
		false
	}

	/// Handle any input event
	fn handle_input(&mut self, event: &InputEvent) -> bool {
		match event {
			InputEvent::Keyboard(e) => self.handle_keyboard(e),
			InputEvent::Mouse(e) => self.handle_mouse(e),
			InputEvent::Touch(e) => self.handle_touch(e),
			InputEvent::Custom(e) => self.handle_custom(e),
		}
	}
}

#[cfg(test)]
mod tests {}
