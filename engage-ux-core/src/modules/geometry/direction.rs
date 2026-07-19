use serde::{Deserialize, Serialize};

/// Directions relative to the display geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
	/// Top
	Top,
	/// Top Left
	TopLeft,
	/// Top Right
	TopRight,
	/// Bottom
	Bottom,
	/// Bottom Left
	BottomLeft,
	/// Bottom Right
	BottomRight,
	/// Left
	Left,
	/// Right
	Right,
}
