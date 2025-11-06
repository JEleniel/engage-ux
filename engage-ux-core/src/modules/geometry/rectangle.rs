//! Geometric rectangle defined by top, left, width, and height in units
use crate::geometry::{Border, Point};
use serde::{Deserialize, Serialize};

/// Geometric rectangle defined by top, left, and size
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
	/// The top-left point of the rectangle
	pub top_left: Point,
	/// The width of the rectangle
	pub width: f32,
	/// The height of the rectangle
	pub height: f32,
}

impl Rectangle {
	/// Convenience constructor from top-left and bottom-right points
	pub fn from_points(top_left: Point, bottom_right: Point) -> Self {
		Self {
			top_left: top_left.clone(),
			width: bottom_right.x - top_left.x,
			height: bottom_right.y - top_left.y,
		}
	}

	/// Check if a point is inside the rectangle
	pub fn contains_point(&self, point: &Point) -> bool {
		point.x >= self.top_left.x
			&& point.x <= self.top_left.x + self.width
			&& point.y >= self.top_left.y
			&& point.y <= self.top_left.y + self.height
	}

	/// Get the bottom-right point of the rectangle
	pub fn bottom_right(&self) -> Point {
		Point {
			x: self.top_left.x + self.width,
			y: self.top_left.y + self.height,
		}
	}

	/// Shrink the rectangle by the specified border amounts
	pub fn shrink(&self, border: &Border) -> Self {
		Self {
			top_left: Point {
				x: self.top_left.x + border.left,
				y: self.top_left.y + border.top,
			},
			width: self.width - border.left - border.right,
			height: self.height - border.top - border.bottom,
		}
	}
}
