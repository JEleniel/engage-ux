//! Transform types for geometry operations
//!
//! Provides a Move struct (replacement for the old `Offset`) and a
//! `Transform` enum for common transform operations.

use crate::geometry::Point;
use serde::{Deserialize, Serialize};

/// Simple translation struct used in several modules (historical API).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Move {
	/// X translation
	pub x: f32,
	/// Y translation
	pub y: f32,
}

impl Move {
	/// Create a new translation `Move`.
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl From<Move> for Transform {
	fn from(m: Move) -> Self {
		Transform::Move { x: m.x, y: m.y }
	}
}

/// A general transform that can represent common geometric transforms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transform {
	/// Translation / move by the given delta.
	/// Translation by the specified offsets in logical units.
	Move {
		/// X translation in logical units.
		x: f32,
		/// Y translation in logical units.
		y: f32,
	},

	/// Rotation transform.
	///
	/// The rotation is specified in degrees and is performed around the
	/// provided `origin` point. If `origin` is `None`, the rotation is
	/// performed around the coordinate origin (0, 0).
	Rotate {
		/// Rotation angle in degrees. Positive values rotate counter-clockwise.
		angle_degrees: f32,
		/// Optional origin point to rotate around. `None` means (0, 0).
		origin: Option<Point>,
	},

	/// Scale transform.
	///
	/// `sx` is the horizontal scale factor. `sy` is the vertical scale factor
	/// and if `None` the scaling is uniform (i.e. `sy == sx`).
	Scale {
		/// Horizontal scale factor.
		sx: f32,
		/// Optional vertical scale factor. When `None`, equals `sx`.
		sy: Option<f32>,
	},
}

impl Transform {
	/// Convenience: create a translation transform
	pub fn translate(x: f32, y: f32) -> Self {
		Transform::Move { x, y }
	}

	/// Convenience: create a rotation transform
	pub fn rotate(angle_degrees: f32, origin: Option<Point>) -> Self {
		Transform::Rotate {
			angle_degrees,
			origin,
		}
	}

	/// Convenience: create a uniform or non-uniform scale
	pub fn scale(sx: f32, sy: Option<f32>) -> Self {
		Transform::Scale { sx, sy }
	}
}
