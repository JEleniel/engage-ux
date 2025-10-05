//! Text component for displaying formatted text

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Font weight options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
	Thin,
	Light,
	Regular,
	Medium,
	Bold,
	Black,
}

/// Text component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
	properties: ComponentProperties,
	content: String,
	color: Color,
	font_size: f32,
	font_weight: FontWeight,
	italic: bool,
	underline: bool,
}

impl Text {
	/// Create a new text component
	pub fn new(id: ComponentId, content: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			content: content.into(),
			color: Color::from_hex("#000000").unwrap(),
			font_size: 16.0,
			font_weight: FontWeight::Regular,
			italic: false,
			underline: false,
		}
	}

	/// Get text content
	pub fn content(&self) -> &str {
		&self.content
	}

	/// Set text content
	pub fn set_content(&mut self, content: impl Into<String>) {
		self.content = content.into();
	}

	/// Get text color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size.max(1.0);
	}

	/// Get font weight
	pub fn font_weight(&self) -> FontWeight {
		self.font_weight
	}

	/// Set font weight
	pub fn set_font_weight(&mut self, weight: FontWeight) {
		self.font_weight = weight;
	}

	/// Check if text is italic
	pub fn is_italic(&self) -> bool {
		self.italic
	}

	/// Set italic
	pub fn set_italic(&mut self, italic: bool) {
		self.italic = italic;
	}

	/// Check if text is underlined
	pub fn is_underline(&self) -> bool {
		self.underline
	}

	/// Set underline
	pub fn set_underline(&mut self, underline: bool) {
		self.underline = underline;
	}
}

impl Component for Text {
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
	fn test_text_creation() {
		let text = Text::new(1, "Hello");
		assert_eq!(text.id(), 1);
		assert_eq!(text.content(), "Hello");
		assert_eq!(text.font_weight(), FontWeight::Regular);
	}

	#[test]
	fn test_text_content() {
		let mut text = Text::new(1, "Initial");
		text.set_content("Modified");
		assert_eq!(text.content(), "Modified");
	}

	#[test]
	fn test_text_styling() {
		let mut text = Text::new(1, "Styled");
		text.set_font_weight(FontWeight::Bold);
		text.set_italic(true);
		text.set_underline(true);

		assert_eq!(text.font_weight(), FontWeight::Bold);
		assert!(text.is_italic());
		assert!(text.is_underline());
	}
}
