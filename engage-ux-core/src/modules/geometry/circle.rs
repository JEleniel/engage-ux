//! Geometric circle defined by center point and radius in units
use crate::geometry::Point;
use crate::{FillStyle, LineStyle};
use serde::{Deserialize, Serialize};

/// Geometric circle defined by a center point and radius
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
	/// The center point of the circle
	pub center: Point,
	/// The radius of the circle
	pub radius: f32,
	/// Optional drawing style for the circle
	pub line_style: Option<LineStyle>,
	/// Optional fill style for the circle
	pub fill_style: Option<FillStyle>,
}

impl Circle {
	/// Create a new circle with the specified center and radius.
	pub fn new(center: Point, radius: f32) -> Self {
		Self {
			center,
			radius,
			line_style: None,
			fill_style: None,
		}
	}
	/// Area of the circle
	pub fn area(&self) -> f32 {
		std::f32::consts::PI * self.radius * self.radius
	}

	/// Circumference of the circle
	pub fn circumference(&self) -> f32 {
		2.0 * std::f32::consts::PI * self.radius
	}

	/// Check if a point is inside or on the circle
	pub fn contains_point(&self, point: &Point) -> bool {
		let dx = point.x - self.center.x;
		let dy = point.y - self.center.y;
		dx * dx + dy * dy <= self.radius * self.radius
	}
}
