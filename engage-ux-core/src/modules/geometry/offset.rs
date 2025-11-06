//! Geometric offset defined by X and Y distances in units
use serde::{Deserialize, Serialize};

/// Geometric offset defined by X and Y distances
/// X: Horizontal, Y: Vertical
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offset {
	/// The X distance
	pub x: f32,
	/// The Y distance
	pub y: f32,
}
