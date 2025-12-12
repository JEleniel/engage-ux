//! Drawing/style properties attached to geometry primitives
use crate::Color;
use serde::{Deserialize, Serialize};

/// Styling options for stroked lines used by geometry primitives.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LineStyle {
	/// Optional stroke color. `None` uses the default/none.
	pub color: Option<Color>,
	/// Stroke width in logical units. `None` indicates hairline/unspecified.
	pub stroke_width: Option<f32>,
	/// Dash pattern specifying on/off lengths in logical units. `None` means solid line.
	pub dash_pattern: Option<Vec<f32>>,
}
