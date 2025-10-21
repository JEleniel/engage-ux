//! Formatted text editor component with rich text support

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Text format style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TextFormat {
	pub bold: bool,
	pub italic: bool,
	pub underline: bool,
	pub strikethrough: bool,
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
	Left,
	Center,
	Right,
	Justify,
}

/// Formatted text editor component
#[derive(Clone, Serialize, Deserialize)]
pub struct TextEditor {
	properties: ComponentProperties,
	content: String,
	placeholder: String,
	format: TextFormat,
	alignment: TextAlignment,
	font_family: String,
	font_size: f32,
	line_height: f32,
	read_only: bool,
	disabled: bool,
	show_toolbar: bool,
	color: Color,
	background_color: Color,
	selection_color: Color,
	toolbar_background: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
	#[serde(skip)]
	on_format_change: Option<EventCallback>,
}

impl TextEditor {
	/// Create a new text editor
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			content: String::new(),
			placeholder: String::new(),
			format: TextFormat::default(),
			alignment: TextAlignment::Left,
			font_family: "sans-serif".to_string(),
			font_size: 14.0,
			line_height: 1.5,
			read_only: false,
			disabled: false,
			show_toolbar: true,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			selection_color: Color::from_hex("#B3D7FF").unwrap(),
			toolbar_background: Color::from_hex("#F5F5F5").unwrap(),
			on_change: None,
			on_format_change: None,
		}
	}

	/// Get content
	pub fn content(&self) -> &str {
		&self.content
	}

	/// Set content
	pub fn set_content(&mut self, content: impl Into<String>) {
		self.content = content.into();
	}

	/// Get placeholder
	pub fn placeholder(&self) -> &str {
		&self.placeholder
	}

	/// Set placeholder
	pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
		self.placeholder = placeholder.into();
	}

	/// Get current format
	pub fn format(&self) -> TextFormat {
		self.format
	}

	/// Set format
	pub fn set_format(&mut self, format: TextFormat) {
		self.format = format;
	}

	/// Toggle bold
	pub fn toggle_bold(&mut self) {
		self.format.bold = !self.format.bold;
	}

	/// Toggle italic
	pub fn toggle_italic(&mut self) {
		self.format.italic = !self.format.italic;
	}

	/// Toggle underline
	pub fn toggle_underline(&mut self) {
		self.format.underline = !self.format.underline;
	}

	/// Toggle strikethrough
	pub fn toggle_strikethrough(&mut self) {
		self.format.strikethrough = !self.format.strikethrough;
	}

	/// Get alignment
	pub fn alignment(&self) -> TextAlignment {
		self.alignment
	}

	/// Set alignment
	pub fn set_alignment(&mut self, alignment: TextAlignment) {
		self.alignment = alignment;
	}

	/// Get font family
	pub fn font_family(&self) -> &str {
		&self.font_family
	}

	/// Set font family
	pub fn set_font_family(&mut self, family: impl Into<String>) {
		self.font_family = family.into();
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size.max(8.0);
	}

	/// Get line height
	pub fn line_height(&self) -> f32 {
		self.line_height
	}

	/// Set line height
	pub fn set_line_height(&mut self, height: f32) {
		self.line_height = height.max(1.0);
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

	/// Check if toolbar is shown
	pub fn shows_toolbar(&self) -> bool {
		self.show_toolbar
	}

	/// Set whether to show toolbar
	pub fn set_show_toolbar(&mut self, show: bool) {
		self.show_toolbar = show;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set selection color
	pub fn set_selection_color(&mut self, color: Color) {
		self.selection_color = color;
	}

	/// Set toolbar background color
	pub fn set_toolbar_background(&mut self, color: Color) {
		self.toolbar_background = color;
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

	/// Set format change callback
	pub fn set_on_format_change(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_format_change = Some(std::sync::Arc::new(callback));
	}

	/// Handle format change event
	pub fn handle_format_change(&self, event: &Event) {
		if let Some(ref callback) = self.on_format_change {
			callback(event);
		}
	}
}

impl Component for TextEditor {
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
	fn test_text_editor_creation() {
		let editor = TextEditor::new(1);
		assert_eq!(editor.id(), 1);
		assert_eq!(editor.content(), "");
		assert!(editor.shows_toolbar());
	}

	#[test]
	fn test_text_editor_content() {
		let mut editor = TextEditor::new(1);
		editor.set_content("Hello, World!");
		assert_eq!(editor.content(), "Hello, World!");
	}

	#[test]
	fn test_text_editor_format() {
		let mut editor = TextEditor::new(1);
		assert!(!editor.format().bold);

		editor.toggle_bold();
		assert!(editor.format().bold);

		editor.toggle_italic();
		assert!(editor.format().italic);

		editor.toggle_underline();
		assert!(editor.format().underline);

		editor.toggle_strikethrough();
		assert!(editor.format().strikethrough);
	}

	#[test]
	fn test_text_editor_alignment() {
		let mut editor = TextEditor::new(1);
		assert_eq!(editor.alignment(), TextAlignment::Left);

		editor.set_alignment(TextAlignment::Center);
		assert_eq!(editor.alignment(), TextAlignment::Center);
	}

	#[test]
	fn test_text_editor_font() {
		let mut editor = TextEditor::new(1);
		editor.set_font_family("Arial");
		editor.set_font_size(16.0);
		editor.set_line_height(1.8);

		assert_eq!(editor.font_family(), "Arial");
		assert_eq!(editor.font_size(), 16.0);
		assert_eq!(editor.line_height(), 1.8);
	}

	#[test]
	fn test_text_editor_states() {
		let mut editor = TextEditor::new(1);
		assert!(!editor.is_read_only());
		assert!(!editor.is_disabled());

		editor.set_read_only(true);
		editor.set_disabled(true);
		assert!(editor.is_read_only());
		assert!(editor.is_disabled());
	}
}
