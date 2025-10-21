//! Line numbers component for code editors

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Line numbers component for displaying line numbers in code editors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineNumbers {
	properties: ComponentProperties,
	start_line: usize,
	end_line: usize,
	current_line: Option<usize>,
	color: Color,
	background_color: Color,
	current_line_color: Color,
	current_line_background: Color,
	font_size: f32,
	padding: f32,
	width: f32,
}

impl LineNumbers {
	/// Create a new line numbers component
	pub fn new(id: ComponentId, total_lines: usize) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			start_line: 1,
			end_line: total_lines,
			current_line: None,
			color: Color::from_hex("#858585").unwrap(),
			background_color: Color::from_hex("#F5F5F5").unwrap(),
			current_line_color: Color::from_hex("#000000").unwrap(),
			current_line_background: Color::from_hex("#E0E0E0").unwrap(),
			font_size: 12.0,
			padding: 8.0,
			width: 50.0,
		}
	}

	/// Get start line
	pub fn start_line(&self) -> usize {
		self.start_line
	}

	/// Set start line
	pub fn set_start_line(&mut self, start: usize) {
		self.start_line = start.max(1);
	}

	/// Get end line
	pub fn end_line(&self) -> usize {
		self.end_line
	}

	/// Set end line (total lines)
	pub fn set_end_line(&mut self, end: usize) {
		self.end_line = end.max(1);
	}

	/// Get total number of lines
	pub fn total_lines(&self) -> usize {
		self.end_line - self.start_line + 1
	}

	/// Get current line (highlighted)
	pub fn current_line(&self) -> Option<usize> {
		self.current_line
	}

	/// Set current line (highlighted)
	pub fn set_current_line(&mut self, line: Option<usize>) {
		self.current_line = line;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set current line color
	pub fn set_current_line_color(&mut self, color: Color) {
		self.current_line_color = color;
	}

	/// Set current line background
	pub fn set_current_line_background(&mut self, color: Color) {
		self.current_line_background = color;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size.max(8.0);
	}

	/// Get padding
	pub fn padding(&self) -> f32 {
		self.padding
	}

	/// Set padding
	pub fn set_padding(&mut self, padding: f32) {
		self.padding = padding.max(0.0);
	}

	/// Get width
	pub fn width(&self) -> f32 {
		self.width
	}

	/// Set width
	pub fn set_width(&mut self, width: f32) {
		self.width = width.max(20.0);
	}
}

impl Component for LineNumbers {
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
	fn test_line_numbers_creation() {
		let line_numbers = LineNumbers::new(1, 100);
		assert_eq!(line_numbers.id(), 1);
		assert_eq!(line_numbers.start_line(), 1);
		assert_eq!(line_numbers.end_line(), 100);
		assert_eq!(line_numbers.total_lines(), 100);
	}

	#[test]
	fn test_line_numbers_range() {
		let mut line_numbers = LineNumbers::new(1, 50);
		line_numbers.set_start_line(10);
		line_numbers.set_end_line(20);
		assert_eq!(line_numbers.total_lines(), 11);
	}

	#[test]
	fn test_line_numbers_current_line() {
		let mut line_numbers = LineNumbers::new(1, 100);
		assert_eq!(line_numbers.current_line(), None);

		line_numbers.set_current_line(Some(42));
		assert_eq!(line_numbers.current_line(), Some(42));
	}

	#[test]
	fn test_line_numbers_sizing() {
		let mut line_numbers = LineNumbers::new(1, 100);
		line_numbers.set_width(60.0);
		line_numbers.set_padding(10.0);
		line_numbers.set_font_size(14.0);

		assert_eq!(line_numbers.width(), 60.0);
		assert_eq!(line_numbers.padding(), 10.0);
		assert_eq!(line_numbers.font_size(), 14.0);
	}
}
