//! Geometric rectangle defined by top, left, width, and height in units
use crate::geometry::{Border, Point};
use crate::{FillStyle, LineStyle};
use serde::{Deserialize, Serialize};

/// Geometric rectangle defined by top, left, size, and optional corner radii
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rectangle {
	/// The top-left point of the rectangle
	pub top_left: Point,
	/// The width of the rectangle
	pub width: f32,
	/// The height of the rectangle
	pub height: f32,
	/// Top-left corner radius
	pub radius_top_left: f32,
	/// Top-right corner radius
	pub radius_top_right: f32,
	/// Bottom-right corner radius
	pub radius_bottom_right: f32,
	/// Bottom-left corner radius
	pub radius_bottom_left: f32,
	/// Optional drawing style for the rectangle (stroke)
	pub line_style: Option<LineStyle>,
	/// Optional fill style for the rectangle
	pub fill_style: Option<FillStyle>,
}

impl Rectangle {
	/// Create a rectangle from top-left point, width and height. Corner radii default to 0.
	pub fn new(top_left: Point, width: f32, height: f32) -> Self {
		Self {
			top_left,
			width,
			height,
			radius_bottom_left: 0.0,
			radius_top_right: 0.0,
			radius_bottom_right: 0.0,
			radius_top_left: 0.0,
			line_style: None,
			fill_style: None,
		}
	}
	/// Convenience constructor from top-left and bottom-right points
	pub fn from_points(top_left: Point, bottom_right: Point) -> Self {
		Self {
			top_left: top_left.clone(),
			width: bottom_right.x - top_left.x,
			height: bottom_right.y - top_left.y,
			radius_top_left: 0.0,
			radius_top_right: 0.0,
			radius_bottom_right: 0.0,
			radius_bottom_left: 0.0,
			line_style: None,
			fill_style: None,
		}
	}

	// Prefer explicit named fields for corner radii. Construct rectangles using
	// struct literal syntax to set `r_tl`, `r_tr`, `r_br`, and `r_bl` individually.

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
			style: None,
		}
	}

	/// Shrink the rectangle by the specified border amounts
	pub fn shrink(&self, border: &Border) -> Self {
		Self {
			top_left: Point {
				x: self.top_left.x + border.left,
				y: self.top_left.y + border.top,
				style: None,
			},
			width: self.width - border.left - border.right,
			height: self.height - border.top - border.bottom,
			radius_top_left: self.radius_top_left,
			radius_top_right: self.radius_top_right,
			radius_bottom_right: self.radius_bottom_right,
			radius_bottom_left: self.radius_bottom_left,
			line_style: self.line_style.clone(),
			fill_style: self.fill_style.clone(),
		}
	}
}
