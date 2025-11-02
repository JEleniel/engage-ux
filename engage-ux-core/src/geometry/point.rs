use crate::geometry::Unit;
use serde::{Deserialize, Serialize};

/// Geometric point defined by X and Y coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
	/// The X coordinate
	pub x: Unit,
	/// The Y coordinate
	pub y: Unit,
}

impl Point {
	/// Convenience constructor
	pub fn new(x: Unit, y: Unit) -> Self {
		Self { x, y }
	}
}
