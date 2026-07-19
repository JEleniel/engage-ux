//! Geometric line defined by start and end points in units
use crate::LineStyle;
use crate::geometry::Point;
use serde::{Deserialize, Serialize};

/// Geometric line segment defined by start and end points
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
	/// The start point of the line
	pub start: Point,
	/// The end point of the line
	pub end: Point,
	/// Optional drawing style for the line
	pub line_style: Option<LineStyle>,
}

impl Line {
	/// Create a new line segment from `start` to `end`.
	pub fn new(start: Point, end: Point) -> Self {
		Self {
			start,
			end,
			line_style: None,
		}
	}
	/// Length of the line segment
	pub fn length(&self) -> f32 {
		let dx = self.end.x - self.start.x;
		let dy = self.end.y - self.start.y;
		(dx * dx + dy * dy).sqrt()
	}

	/// Midpoint of the line segment
	pub fn midpoint(&self) -> Point {
		Point {
			x: (self.start.x + self.end.x) / 2.0,
			y: (self.start.y + self.end.y) / 2.0,
			style: None,
		}
	}

	/// Direction vector from start to end (not normalized)
	pub fn direction_vector(&self) -> Point {
		Point {
			x: self.end.x - self.start.x,
			y: self.end.y - self.start.y,
			style: None,
		}
	}

	/// Check if a point lies on the line segment (within a small epsilon)
	pub fn contains_point(&self, point: &Point) -> bool {
		// Use cross-product and bounding-box check with epsilon tolerance
		let epsilon = 1e-5_f32;
		let dx1 = self.end.x - self.start.x;
		let dy1 = self.end.y - self.start.y;
		let dx2 = point.x - self.start.x;
		let dy2 = point.y - self.start.y;

		// Cross product should be near zero for collinearity
		let cross = dx1 * dy2 - dy1 * dx2;
		if cross.abs() > epsilon {
			return false;
		}

		// Dot product to ensure point is between start and end
		let dot = dx2 * dx1 + dy2 * dy1;
		if dot < -epsilon {
			return false;
		}

		let len_sq = dx1 * dx1 + dy1 * dy1;
		if dot - len_sq > epsilon {
			return false;
		}

		true
	}
}
