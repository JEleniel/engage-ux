use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// A point in 2D space
pub struct Point {
	/// The x (horizontal) coordinate
	pub x: i32,
	/// The y (vertical) coordinate
	pub y: i32,
}
