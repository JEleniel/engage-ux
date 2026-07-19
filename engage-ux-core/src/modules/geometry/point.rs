//! Geometric point defined by X and Y coordinates in units
use crate::LineStyle;
use serde::{Deserialize, Serialize};

/// Geometric point defined by X and Y coordinates
/// X: Horizontal, Y: Vertical
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
	/// The X coordinate
	pub x: f32,
	/// The Y coordinate
	pub y: f32,
	/// Optional stroke style attached to a point (some adapters use this)
	pub style: Option<LineStyle>,
}

impl Point {
	/// Create a new `Point` at the given coordinates.
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y, style: None }
	}
}
