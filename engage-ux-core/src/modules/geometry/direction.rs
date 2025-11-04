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

/// Directions as measured by a compass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompassDirection {
	/// North
	North,
	/// North East
	NorthEast,
	/// East
	East,
	/// South East
	SouthEast,
	/// South
	South,
	/// South West
	SouthWest,
	/// West
	West,
	/// North West
	NorthWest,
}
