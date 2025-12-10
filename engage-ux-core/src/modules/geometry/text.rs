//! Text primitive for simple layout and placement
use crate::geometry::Point;
use crate::{FillStyle, LineStyle};
use serde::{Deserialize, Serialize};

/// Simple text primitive representing a string at a position with a font size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
	/// The anchor position of the text (top-left by convention)
	pub position: Point,
	/// The textual content
	pub content: String,
	/// Font size in units
	pub font_size: f32,
	/// Optional text anchor (start/middle/end) matching SVG `text-anchor`
	pub anchor: Option<String>,
	/// Optional stroke style for the text (rare)
	pub line_style: Option<LineStyle>,
	/// Optional fill style (text color)
	pub fill_style: Option<FillStyle>,
}

impl Text {
	/// Create a new text primitive
	pub fn new(position: Point, content: String, font_size: f32) -> Self {
		Self {
			position,
			content,
			font_size,
			anchor: None,
			line_style: None,
			fill_style: None,
		}
	}

	/// Estimate a simple bounding box for the text. This is a heuristic and
	/// not intended to replace a font metrics engine. The width is estimated
	/// as 0.6 * font_size * number_of_chars.
	pub fn estimated_bbox(&self) -> (Point, f32, f32) {
		let width = 0.6 * self.font_size * (self.content.len() as f32);
		let height = self.font_size;
		(self.position.clone(), width, height)
	}
}
