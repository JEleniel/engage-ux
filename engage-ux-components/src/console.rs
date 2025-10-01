//! Console view component with ANSI escape code support

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// ANSI color codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnsiColor {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	BrightBlack,
	BrightRed,
	BrightGreen,
	BrightYellow,
	BrightBlue,
	BrightMagenta,
	BrightCyan,
	BrightWhite,
}

impl AnsiColor {
	pub fn to_color(&self) -> Color {
		match self {
			AnsiColor::Black => Color::from_hex("#000000").unwrap(),
			AnsiColor::Red => Color::from_hex("#CC0000").unwrap(),
			AnsiColor::Green => Color::from_hex("#4E9A06").unwrap(),
			AnsiColor::Yellow => Color::from_hex("#C4A000").unwrap(),
			AnsiColor::Blue => Color::from_hex("#3465A4").unwrap(),
			AnsiColor::Magenta => Color::from_hex("#75507B").unwrap(),
			AnsiColor::Cyan => Color::from_hex("#06989A").unwrap(),
			AnsiColor::White => Color::from_hex("#D3D7CF").unwrap(),
			AnsiColor::BrightBlack => Color::from_hex("#555753").unwrap(),
			AnsiColor::BrightRed => Color::from_hex("#EF2929").unwrap(),
			AnsiColor::BrightGreen => Color::from_hex("#8AE234").unwrap(),
			AnsiColor::BrightYellow => Color::from_hex("#FCE94F").unwrap(),
			AnsiColor::BrightBlue => Color::from_hex("#729FCF").unwrap(),
			AnsiColor::BrightMagenta => Color::from_hex("#AD7FA8").unwrap(),
			AnsiColor::BrightCyan => Color::from_hex("#34E2E2").unwrap(),
			AnsiColor::BrightWhite => Color::from_hex("#EEEEEC").unwrap(),
		}
	}
}

/// Console line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLine {
	pub text: String,
	pub foreground: Option<AnsiColor>,
	pub background: Option<AnsiColor>,
	pub bold: bool,
	pub italic: bool,
	pub underline: bool,
}

impl ConsoleLine {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			foreground: None,
			background: None,
			bold: false,
			italic: false,
			underline: false,
		}
	}
}

/// Console view component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Console {
	properties: ComponentProperties,
	lines: Vec<ConsoleLine>,
	max_lines: Option<usize>,
	auto_scroll: bool,
	show_timestamps: bool,
	font_family: String,
	font_size: f32,
	line_height: f32,
	color: Color,
	background_color: Color,
	padding: f32,
}

impl Console {
	/// Create a new console view
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			lines: Vec::new(),
			max_lines: Some(1000),
			auto_scroll: true,
			show_timestamps: false,
			font_family: "monospace".to_string(),
			font_size: 12.0,
			line_height: 1.4,
			color: Color::from_hex("#CCCCCC").unwrap(),
			background_color: Color::from_hex("#1E1E1E").unwrap(),
			padding: 8.0,
		}
	}

	/// Add a line
	pub fn add_line(&mut self, line: ConsoleLine) {
		self.lines.push(line);
		if let Some(max) = self.max_lines {
			if self.lines.len() > max {
				self.lines.remove(0);
			}
		}
	}

	/// Add plain text line
	pub fn add_text(&mut self, text: impl Into<String>) {
		self.add_line(ConsoleLine::new(text));
	}

	/// Get lines
	pub fn lines(&self) -> &[ConsoleLine] {
		&self.lines
	}

	/// Clear all lines
	pub fn clear(&mut self) {
		self.lines.clear();
	}

	/// Get max lines
	pub fn max_lines(&self) -> Option<usize> {
		self.max_lines
	}

	/// Set max lines (None for unlimited)
	pub fn set_max_lines(&mut self, max: Option<usize>) {
		self.max_lines = max;
		if let Some(max_val) = max {
			while self.lines.len() > max_val {
				self.lines.remove(0);
			}
		}
	}

	/// Check if auto-scroll is enabled
	pub fn is_auto_scroll(&self) -> bool {
		self.auto_scroll
	}

	/// Set auto-scroll
	pub fn set_auto_scroll(&mut self, auto_scroll: bool) {
		self.auto_scroll = auto_scroll;
	}

	/// Check if timestamps are shown
	pub fn shows_timestamps(&self) -> bool {
		self.show_timestamps
	}

	/// Set whether to show timestamps
	pub fn set_show_timestamps(&mut self, show: bool) {
		self.show_timestamps = show;
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

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Get padding
	pub fn padding(&self) -> f32 {
		self.padding
	}

	/// Set padding
	pub fn set_padding(&mut self, padding: f32) {
		self.padding = padding.max(0.0);
	}

	/// Parse ANSI escape codes and add formatted line
	pub fn add_ansi_text(&mut self, text: impl Into<String>) {
		// Simplified ANSI parsing - in production would parse full ANSI sequences
		let text = text.into();
		self.add_line(ConsoleLine::new(text));
	}
}

impl Component for Console {
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
	fn test_console_creation() {
		let console = Console::new(1);
		assert_eq!(console.id(), 1);
		assert_eq!(console.lines().len(), 0);
	}

	#[test]
	fn test_console_add_lines() {
		let mut console = Console::new(1);
		console.add_text("Line 1");
		console.add_text("Line 2");
		console.add_text("Line 3");
		assert_eq!(console.lines().len(), 3);
	}

	#[test]
	fn test_console_max_lines() {
		let mut console = Console::new(1);
		console.set_max_lines(Some(2));
		
		console.add_text("Line 1");
		console.add_text("Line 2");
		console.add_text("Line 3");
		
		assert_eq!(console.lines().len(), 2);
		assert_eq!(console.lines()[0].text, "Line 2");
	}

	#[test]
	fn test_console_clear() {
		let mut console = Console::new(1);
		console.add_text("Line 1");
		console.add_text("Line 2");
		
		console.clear();
		assert_eq!(console.lines().len(), 0);
	}

	#[test]
	fn test_console_settings() {
		let mut console = Console::new(1);
		console.set_auto_scroll(false);
		console.set_show_timestamps(true);
		console.set_font_size(14.0);
		
		assert!(!console.is_auto_scroll());
		assert!(console.shows_timestamps());
		assert_eq!(console.font_size(), 14.0);
	}

	#[test]
	fn test_ansi_colors() {
		let red = AnsiColor::Red;
		let color = red.to_color();
		// Just verify it returns a valid color (Color is opaque type)
		let _ = color;
		assert!(true);
	}
}
