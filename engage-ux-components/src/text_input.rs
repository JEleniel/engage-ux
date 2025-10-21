//! Text input component

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Input type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
	Text,
	Password,
	Email,
	Number,
	Tel,
	Url,
	Search,
}

/// Text input component
#[derive(Clone, Serialize, Deserialize)]
pub struct TextInput {
	properties: ComponentProperties,
	value: String,
	placeholder: String,
	input_type: InputType,
	max_length: Option<usize>,
	read_only: bool,
	disabled: bool,
	required: bool,
	autocomplete: bool,
	color: Color,
	background_color: Color,
	border_color: Color,
	focus_border_color: Color,
	font_size: f32,
	#[serde(skip)]
	on_change: Option<EventCallback>,
	#[serde(skip)]
	on_focus: Option<EventCallback>,
	#[serde(skip)]
	on_blur: Option<EventCallback>,
}

impl TextInput {
	/// Create a new text input
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			value: String::new(),
			placeholder: String::new(),
			input_type: InputType::Text,
			max_length: None,
			read_only: false,
			disabled: false,
			required: false,
			autocomplete: true,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
			focus_border_color: Color::from_hex("#1976D2").unwrap(),
			font_size: 14.0,
			on_change: None,
			on_focus: None,
			on_blur: None,
		}
	}

	/// Get value
	pub fn value(&self) -> &str {
		&self.value
	}

	/// Set value
	pub fn set_value(&mut self, value: impl Into<String>) {
		let new_value = value.into();
		if let Some(max_len) = self.max_length {
			if new_value.len() <= max_len {
				self.value = new_value;
			}
		} else {
			self.value = new_value;
		}
	}

	/// Get placeholder
	pub fn placeholder(&self) -> &str {
		&self.placeholder
	}

	/// Set placeholder
	pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
		self.placeholder = placeholder.into();
	}

	/// Get input type
	pub fn input_type(&self) -> InputType {
		self.input_type
	}

	/// Set input type
	pub fn set_input_type(&mut self, input_type: InputType) {
		self.input_type = input_type;
	}

	/// Get max length
	pub fn max_length(&self) -> Option<usize> {
		self.max_length
	}

	/// Set max length
	pub fn set_max_length(&mut self, max_length: Option<usize>) {
		self.max_length = max_length;
	}

	/// Check if read-only
	pub fn is_read_only(&self) -> bool {
		self.read_only
	}

	/// Set read-only
	pub fn set_read_only(&mut self, read_only: bool) {
		self.read_only = read_only;
	}

	/// Check if disabled
	pub fn is_disabled(&self) -> bool {
		self.disabled
	}

	/// Set disabled
	pub fn set_disabled(&mut self, disabled: bool) {
		self.disabled = disabled;
	}

	/// Check if required
	pub fn is_required(&self) -> bool {
		self.required
	}

	/// Set required
	pub fn set_required(&mut self, required: bool) {
		self.required = required;
	}

	/// Check if autocomplete is enabled
	pub fn has_autocomplete(&self) -> bool {
		self.autocomplete
	}

	/// Set autocomplete
	pub fn set_autocomplete(&mut self, autocomplete: bool) {
		self.autocomplete = autocomplete;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Color) {
		self.border_color = color;
	}

	/// Set focus border color
	pub fn set_focus_border_color(&mut self, color: Color) {
		self.focus_border_color = color;
	}

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set change callback
	pub fn set_on_change(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_change = Some(std::sync::Arc::new(callback));
	}

	/// Handle change event
	pub fn handle_change(&self, event: &Event) {
		if let Some(ref callback) = self.on_change {
			callback(event);
		}
	}

	/// Set focus callback
	pub fn set_on_focus(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_focus = Some(std::sync::Arc::new(callback));
	}

	/// Handle focus event
	pub fn handle_focus(&self, event: &Event) {
		if let Some(ref callback) = self.on_focus {
			callback(event);
		}
	}

	/// Set blur callback
	pub fn set_on_blur(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_blur = Some(std::sync::Arc::new(callback));
	}

	/// Handle blur event
	pub fn handle_blur(&self, event: &Event) {
		if let Some(ref callback) = self.on_blur {
			callback(event);
		}
	}
}

impl Component for TextInput {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn properties(&self) -> &ComponentProperties {
		&self.properties
	}

	fn properties_mut(&mut self) -> &mut ComponentProperties {
		&mut self.properties
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_text_input_creation() {
		let input = TextInput::new(1);
		assert_eq!(input.id(), 1);
		assert_eq!(input.value(), "");
		assert_eq!(input.input_type(), InputType::Text);
	}

	#[test]
	fn test_text_input_value() {
		let mut input = TextInput::new(1);
		input.set_value("Hello");
		assert_eq!(input.value(), "Hello");
	}

	#[test]
	fn test_text_input_max_length() {
		let mut input = TextInput::new(1);
		input.set_max_length(Some(5));
		input.set_value("Too long text");
		assert_eq!(input.value(), ""); // Value not set due to max length

		input.set_value("Short");
		assert_eq!(input.value(), "Short");
	}

	#[test]
	fn test_text_input_placeholder() {
		let mut input = TextInput::new(1);
		input.set_placeholder("Enter text...");
		assert_eq!(input.placeholder(), "Enter text...");
	}

	#[test]
	fn test_text_input_type() {
		let mut input = TextInput::new(1);
		input.set_input_type(InputType::Password);
		assert_eq!(input.input_type(), InputType::Password);
	}

	#[test]
	fn test_text_input_read_only() {
		let mut input = TextInput::new(1);
		assert!(!input.is_read_only());
		input.set_read_only(true);
		assert!(input.is_read_only());
	}

	#[test]
	fn test_text_input_disabled() {
		let mut input = TextInput::new(1);
		assert!(!input.is_disabled());
		input.set_disabled(true);
		assert!(input.is_disabled());
	}

	#[test]
	fn test_text_input_required() {
		let mut input = TextInput::new(1);
		assert!(!input.is_required());
		input.set_required(true);
		assert!(input.is_required());
	}
}
