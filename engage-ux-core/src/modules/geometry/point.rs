//! Geometric point defined by X and Y coordinatesin units
use serde::{Deserialize, Serialize};

/// Geometric point defined by X and Y coordinates
/// X: Horizontal, Y: Vertical
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
	/// The X coordinate
	pub x: i64,
	/// The Y coordinate
	pub y: i64,
}
