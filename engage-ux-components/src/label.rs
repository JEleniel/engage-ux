//! Label component for displaying static text

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
	Left,
	Center,
	Right,
}

/// Label component for displaying text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
	properties: ComponentProperties,
	text: String,
	color: Color,
	font_size: f32,
	align: TextAlign,
}

impl Label {
	/// Create a new label with the given text
	pub fn new(id: ComponentId, text: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			text: text.into(),
			color: Color::from_hex("#000000").unwrap(),
			font_size: 16.0,
			align: TextAlign::Left,
		}
	}

	/// Get the label text
	pub fn text(&self) -> &str {
		&self.text
	}

	/// Set the label text
	pub fn set_text(&mut self, text: impl Into<String>) {
		self.text = text.into();
	}

	/// Get the text color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set the text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get the font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set the font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size.max(1.0);
	}

	/// Get text alignment
	pub fn align(&self) -> TextAlign {
		self.align
	}

	/// Set text alignment
	pub fn set_align(&mut self, align: TextAlign) {
		self.align = align;
	}
}

impl Component for Label {
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
	fn test_label_creation() {
		let label = Label::new(1, "Hello, World!");
		assert_eq!(label.id(), 1);
		assert_eq!(label.text(), "Hello, World!");
		assert_eq!(label.font_size(), 16.0);
	}

	#[test]
	fn test_label_text_modification() {
		let mut label = Label::new(1, "Initial");
		label.set_text("Modified");
		assert_eq!(label.text(), "Modified");
	}

	#[test]
	fn test_label_color() {
		let mut label = Label::new(1, "Text");
		let color = Color::from_hex("#FF0000").unwrap();
		label.set_color(color.clone());
		assert_eq!(label.color(), &color);
	}

	#[test]
	fn test_label_font_size() {
		let mut label = Label::new(1, "Text");
		label.set_font_size(24.0);
		assert_eq!(label.font_size(), 24.0);
		
		// Test minimum size enforcement
		label.set_font_size(0.5);
		assert_eq!(label.font_size(), 1.0);
	}

	#[test]
	fn test_label_alignment() {
		let mut label = Label::new(1, "Text");
		assert_eq!(label.align(), TextAlign::Left);
		
		label.set_align(TextAlign::Center);
		assert_eq!(label.align(), TextAlign::Center);
	}

	#[test]
	fn test_label_component_trait() {
		let mut label = Label::new(1, "Text");
		assert!(label.is_visible());
		assert!(label.is_enabled());
		
		label.set_visible(false);
		assert!(!label.is_visible());
	}
}
