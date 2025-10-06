//! Text area component for multi-line text input

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Text area component
#[derive(Clone, Serialize, Deserialize)]
pub struct TextArea {
	properties: ComponentProperties,
	value: String,
	placeholder: String,
	rows: usize,
	cols: usize,
	max_length: Option<usize>,
	read_only: bool,
	color: Color,
	background_color: Color,
	border_color: Color,
	font_size: f32,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl TextArea {
	/// Create a new text area
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			value: String::new(),
			placeholder: String::new(),
			rows: 4,
			cols: 40,
			max_length: None,
			read_only: false,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
			font_size: 14.0,
			on_change: None,
		}
	}

	/// Get the current value
	pub fn value(&self) -> &str {
		&self.value
	}

	/// Set the value
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

	/// Get placeholder text
	pub fn placeholder(&self) -> &str {
		&self.placeholder
	}

	/// Set placeholder text
	pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
		self.placeholder = placeholder.into();
	}

	/// Get number of rows
	pub fn rows(&self) -> usize {
		self.rows
	}

	/// Set number of rows
	pub fn set_rows(&mut self, rows: usize) {
		self.rows = rows.max(1);
	}

	/// Get number of columns
	pub fn cols(&self) -> usize {
		self.cols
	}

	/// Set number of columns
	pub fn set_cols(&mut self, cols: usize) {
		self.cols = cols.max(1);
	}

	/// Set maximum length
	pub fn set_max_length(&mut self, max_length: Option<usize>) {
		self.max_length = max_length;
	}

	/// Get maximum length
	pub fn max_length(&self) -> Option<usize> {
		self.max_length
	}

	/// Set read-only state
	pub fn set_read_only(&mut self, read_only: bool) {
		self.read_only = read_only;
	}

	/// Check if read-only
	pub fn is_read_only(&self) -> bool {
		self.read_only
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

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set change event handler
	pub fn set_on_change(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_change = Some(std::sync::Arc::new(callback));
	}

	/// Handle change event
	pub fn handle_change(&self, event: &Event) {
		if let Some(ref callback) = self.on_change {
			callback(event);
		}
	}
}

impl Component for TextArea {
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
	fn test_text_area_creation() {
		let textarea = TextArea::new(1);
		assert_eq!(textarea.id(), 1);
		assert_eq!(textarea.value(), "");
		assert_eq!(textarea.rows(), 4);
		assert_eq!(textarea.cols(), 40);
	}

	#[test]
	fn test_text_area_value() {
		let mut textarea = TextArea::new(1);
		textarea.set_value("Hello\nWorld");
		assert_eq!(textarea.value(), "Hello\nWorld");
	}

	#[test]
	fn test_text_area_max_length() {
		let mut textarea = TextArea::new(1);
		textarea.set_max_length(Some(10));
		textarea.set_value("This is too long");
		assert_eq!(textarea.value(), ""); // Value not set due to max length

		textarea.set_value("Short");
		assert_eq!(textarea.value(), "Short");
	}

	#[test]
	fn test_text_area_placeholder() {
		let mut textarea = TextArea::new(1);
		textarea.set_placeholder("Enter text here...");
		assert_eq!(textarea.placeholder(), "Enter text here...");
	}

	#[test]
	fn test_text_area_dimensions() {
		let mut textarea = TextArea::new(1);
		textarea.set_rows(10);
		textarea.set_cols(80);
		assert_eq!(textarea.rows(), 10);
		assert_eq!(textarea.cols(), 80);
	}

	#[test]
	fn test_text_area_read_only() {
		let mut textarea = TextArea::new(1);
		assert!(!textarea.is_read_only());
		textarea.set_read_only(true);
		assert!(textarea.is_read_only());
	}
}
