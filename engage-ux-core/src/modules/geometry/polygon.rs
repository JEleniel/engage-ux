//! Polygon and polyline geometric primitives
use crate::geometry::{Point, Rectangle};
use crate::{FillStyle, LineStyle};
use serde::{Deserialize, Serialize};

/// Closed polygon defined by a list of points (must have at least 3 points)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
	/// Points in order (clockwise or counter-clockwise)
	pub points: Vec<Point>,
	/// Optional drawing style for the polygon
	pub line_style: Option<LineStyle>,
	/// Optional fill style for the polygon
	pub fill_style: Option<FillStyle>,
}

impl Polygon {
	/// Create a polygon from points
	pub fn new(points: Vec<Point>) -> Self {
		Self {
			points,
			line_style: None,
			fill_style: None,
		}
	}

	/// Simple point-in-polygon test using ray casting
	pub fn contains_point(&self, p: &Point) -> bool {
		let mut inside = false;
		let n = self.points.len();
		if n < 3 {
			return false;
		}
		let mut j = n - 1;
		for i in 0..n {
			let pi = &self.points[i];
			let pj = &self.points[j];
			let intersect = ((pi.y > p.y) != (pj.y > p.y))
				&& (p.x < (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y + 0.0) + pi.x);
			if intersect {
				inside = !inside;
			}
			j = i;
		}
		inside
	}

	/// Get the axis-aligned bounding box of the polygon
	pub fn bounding_box(&self) -> Option<Rectangle> {
		if self.points.is_empty() {
			return None;
		}
		let mut min_x = self.points[0].x;
		let mut min_y = self.points[0].y;
		let mut max_x = self.points[0].x;
		let mut max_y = self.points[0].y;
		for p in &self.points[1..] {
			if p.x < min_x {
				min_x = p.x;
			}
			if p.y < min_y {
				min_y = p.y;
			}
			if p.x > max_x {
				max_x = p.x;
			}
			if p.y > max_y {
				max_y = p.y;
			}
		}
		Some(Rectangle::from_points(
			Point {
				x: min_x,
				y: min_y,
				style: None,
			},
			Point {
				x: max_x,
				y: max_y,
				style: None,
			},
		))
	}
}

/// Polyline (open) defined by a list of points
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polyline {
	/// Ordered points along the polyline
	pub points: Vec<Point>,
	/// Optional drawing style for polyline (stroke-only)
	pub style: Option<LineStyle>,
}

impl Polyline {
	/// Create a new polyline
	pub fn new(points: Vec<Point>) -> Self {
		Self {
			points,
			style: None,
		}
	}

	/// Total length of the polyline (sum of segment lengths)
	pub fn length(&self) -> f32 {
		let mut total = 0.0_f32;
		if self.points.len() < 2 {
			return 0.0;
		}
		for i in 0..(self.points.len() - 1) {
			let a = &self.points[i];
			let b = &self.points[i + 1];
			let dx = b.x - a.x;
			let dy = b.y - a.y;
			total += (dx * dx + dy * dy).sqrt();
		}
		total
	}
}
