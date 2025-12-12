//! Geometric ellipse (ellipse/oval) primitive
use crate::geometry::Point;
use crate::geometry::rectangle::Rectangle;
use crate::{FillStyle, LineStyle};
use serde::{Deserialize, Serialize};

/// Geometric ellipse defined by a center point and radii along the x/y axes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ellipse {
	/// Center point of the ellipse
	pub center: Point,
	/// Radius along the X axis
	pub rx: f32,
	/// Radius along the Y axis
	pub ry: f32,
	/// Optional drawing style for the circle
	pub line_style: Option<LineStyle>,
	/// Optional fill style for the circle
	pub fill_style: Option<FillStyle>,
}

impl Ellipse {
	/// Create a new ellipse with center and radii.
	pub fn new(center: Point, rx: f32, ry: f32) -> Self {
		Self {
			center,
			rx,
			ry,
			line_style: None,
			fill_style: None,
		}
	}
	/// Area of the ellipse: pi * rx * ry
	pub fn area(&self) -> f32 {
		std::f32::consts::PI * self.rx * self.ry
	}

	/// Check if a point is inside or on the ellipse boundary.
	pub fn contains_point(&self, p: &Point) -> bool {
		if self.rx == 0.0 || self.ry == 0.0 {
			return false;
		}
		let nx = (p.x - self.center.x) / self.rx;
		let ny = (p.y - self.center.y) / self.ry;
		nx * nx + ny * ny <= 1.0
	}

	/// Get axis-aligned bounding box of the ellipse as a `Rectangle`.
	pub fn bounding_box(&self) -> Rectangle {
		let top_left = Point {
			x: self.center.x - self.rx,
			y: self.center.y - self.ry,
			style: None,
		};
		let bottom_right = Point {
			x: self.center.x + self.rx,
			y: self.center.y + self.ry,
			style: None,
		};
		Rectangle::from_points(top_left, bottom_right)
	}
}
