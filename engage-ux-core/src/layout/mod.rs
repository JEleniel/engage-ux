//! Layout system with relative units and positioning
//!
//! Provides flexible layout capabilities including:
//! - Relative units (rb, rp, %)
//! - Absolute and relative positioning
//! - Size constraints (min/max)
//! - Fill sizing mode

pub mod constraints;
pub mod position;
pub mod size;
pub mod units;

pub use constraints::Constraints;
pub use position::{Position, PositionMode};
pub use size::{Size, SizeMode};
pub use units::{RelativeUnit, Unit};

use serde::{Deserialize, Serialize};

/// Complete layout specification for a component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Layout {
	/// Horizontal position
	pub left: Option<Unit>,
	/// Vertical position
	pub top: Option<Unit>,
	/// Right edge position (alternative to left + width)
	pub right: Option<Unit>,
	/// Bottom edge position (alternative to top + height)
	pub bottom: Option<Unit>,
	/// Width specification
	pub width: Option<Size>,
	/// Height specification
	pub height: Option<Size>,
	/// Minimum width constraint
	pub min_width: Option<Unit>,
	/// Maximum width constraint
	pub max_width: Option<Unit>,
	/// Minimum height constraint
	pub min_height: Option<Unit>,
	/// Maximum height constraint
	pub max_height: Option<Unit>,
	/// Position mode (absolute or relative)
	pub position_mode: PositionMode,
}

impl Layout {
	/// Create a new layout with default values
	pub fn new() -> Self {
		Self {
			left: None,
			top: None,
			right: None,
			bottom: None,
			width: None,
			height: None,
			min_width: None,
			max_width: None,
			min_height: None,
			max_height: None,
			position_mode: PositionMode::Relative,
		}
	}

	/// Set left position
	pub fn with_left(mut self, left: Unit) -> Self {
		self.left = Some(left);
		self
	}

	/// Set top position
	pub fn with_top(mut self, top: Unit) -> Self {
		self.top = Some(top);
		self
	}

	/// Set right position
	pub fn with_right(mut self, right: Unit) -> Self {
		self.right = Some(right);
		self
	}

	/// Set bottom position
	pub fn with_bottom(mut self, bottom: Unit) -> Self {
		self.bottom = Some(bottom);
		self
	}

	/// Set width
	pub fn with_width(mut self, width: Size) -> Self {
		self.width = Some(width);
		self
	}

	/// Set height
	pub fn with_height(mut self, height: Size) -> Self {
		self.height = Some(height);
		self
	}

	/// Set minimum width
	pub fn with_min_width(mut self, min_width: Unit) -> Self {
		self.min_width = Some(min_width);
		self
	}

	/// Set maximum width
	pub fn with_max_width(mut self, max_width: Unit) -> Self {
		self.max_width = Some(max_width);
		self
	}

	/// Set minimum height
	pub fn with_min_height(mut self, min_height: Unit) -> Self {
		self.min_height = Some(min_height);
		self
	}

	/// Set maximum height
	pub fn with_max_height(mut self, max_height: Unit) -> Self {
		self.max_height = Some(max_height);
		self
	}

	/// Set position mode
	pub fn with_position_mode(mut self, mode: PositionMode) -> Self {
		self.position_mode = mode;
		self
	}

	/// Calculate actual pixel values from layout specification
	pub fn calculate_bounds(
		&self,
		parent_width: f32,
		parent_height: f32,
		base_size: f32,
		inherited_size: f32,
	) -> CalculatedBounds {
		let mut bounds = CalculatedBounds::default();

		// Calculate width
		if let Some(width) = &self.width {
			bounds.width = width.calculate(parent_width, parent_height, base_size, inherited_size);
		} else if let (Some(left), Some(right)) = (&self.left, &self.right) {
			// Width from left and right
			let left_px = left.to_pixels(parent_width, base_size, inherited_size);
			let right_px = right.to_pixels(parent_width, base_size, inherited_size);
			bounds.width = parent_width - left_px - right_px;
		} else {
			bounds.width = 0.0;
		}

		// Calculate height
		if let Some(height) = &self.height {
			bounds.height =
				height.calculate(parent_width, parent_height, base_size, inherited_size);
		} else if let (Some(top), Some(bottom)) = (&self.top, &self.bottom) {
			// Height from top and bottom
			let top_px = top.to_pixels(parent_height, base_size, inherited_size);
			let bottom_px = bottom.to_pixels(parent_height, base_size, inherited_size);
			bounds.height = parent_height - top_px - bottom_px;
		} else {
			bounds.height = 0.0;
		}

		// Apply constraints
		if let Some(min_width) = &self.min_width {
			let min_px = min_width.to_pixels(parent_width, base_size, inherited_size);
			bounds.width = bounds.width.max(min_px);
		}
		if let Some(max_width) = &self.max_width {
			let max_px = max_width.to_pixels(parent_width, base_size, inherited_size);
			bounds.width = bounds.width.min(max_px);
		}
		if let Some(min_height) = &self.min_height {
			let min_px = min_height.to_pixels(parent_height, base_size, inherited_size);
			bounds.height = bounds.height.max(min_px);
		}
		if let Some(max_height) = &self.max_height {
			let max_px = max_height.to_pixels(parent_height, base_size, inherited_size);
			bounds.height = bounds.height.min(max_px);
		}

		// Calculate position
		bounds.x = self
			.left
			.as_ref()
			.map(|u| u.to_pixels(parent_width, base_size, inherited_size))
			.unwrap_or(0.0);

		bounds.y = self
			.top
			.as_ref()
			.map(|u| u.to_pixels(parent_height, base_size, inherited_size))
			.unwrap_or(0.0);

		bounds
	}
}

impl Default for Layout {
	fn default() -> Self {
		Self::new()
	}
}

/// Calculated bounds in absolute pixels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalculatedBounds {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl CalculatedBounds {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}
}

impl Default for CalculatedBounds {
	fn default() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			width: 0.0,
			height: 0.0,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_layout_creation() {
		let layout = Layout::new();
		assert_eq!(layout.position_mode, PositionMode::Relative);
		assert!(layout.left.is_none());
		assert!(layout.width.is_none());
	}

	#[test]
	fn test_layout_builder() {
		let layout = Layout::new()
			.with_left(Unit::Pixels(10.0))
			.with_top(Unit::Pixels(20.0))
			.with_width(Size::Fixed(Unit::Pixels(100.0)))
			.with_height(Size::Fixed(Unit::Pixels(50.0)));

		assert_eq!(layout.left, Some(Unit::Pixels(10.0)));
		assert_eq!(layout.top, Some(Unit::Pixels(20.0)));
		assert_eq!(layout.width, Some(Size::Fixed(Unit::Pixels(100.0))));
		assert_eq!(layout.height, Some(Size::Fixed(Unit::Pixels(50.0))));
	}

	#[test]
	fn test_calculate_bounds_absolute() {
		let layout = Layout::new()
			.with_left(Unit::Pixels(10.0))
			.with_top(Unit::Pixels(20.0))
			.with_width(Size::Fixed(Unit::Pixels(100.0)))
			.with_height(Size::Fixed(Unit::Pixels(50.0)));

		let bounds = layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);

		assert_eq!(bounds.x, 10.0);
		assert_eq!(bounds.y, 20.0);
		assert_eq!(bounds.width, 100.0);
		assert_eq!(bounds.height, 50.0);
	}

	#[test]
	fn test_calculate_bounds_with_constraints() {
		let layout = Layout::new()
			.with_width(Size::Fixed(Unit::Pixels(100.0)))
			.with_height(Size::Fixed(Unit::Pixels(50.0)))
			.with_min_width(Unit::Pixels(150.0));

		let bounds = layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);

		// Width should be constrained to minimum
		assert_eq!(bounds.width, 150.0);
		assert_eq!(bounds.height, 50.0);
	}

	#[test]
	fn test_calculate_bounds_percentage() {
		let layout = Layout::new()
			.with_width(Size::Fixed(Unit::percent(50.0)))
			.with_height(Size::Fixed(Unit::percent(25.0)));

		let bounds = layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);

		// 50% of max(800, 600) = 400, 25% = 200
		assert_eq!(bounds.width, 400.0);
		assert_eq!(bounds.height, 200.0);
	}

	#[test]
	fn test_calculate_bounds_relative_base() {
		let layout = Layout::new()
			.with_width(Size::Fixed(Unit::rb(2.0)))
			.with_height(Size::Fixed(Unit::rb(1.5)));

		let bounds = layout.calculate_bounds(800.0, 600.0, 16.0, 20.0);

		// 2 * 16 = 32, 1.5 * 16 = 24
		assert_eq!(bounds.width, 32.0);
		assert_eq!(bounds.height, 24.0);
	}

	#[test]
	fn test_calculate_bounds_from_edges() {
		let layout = Layout::new()
			.with_left(Unit::Pixels(10.0))
			.with_right(Unit::Pixels(10.0))
			.with_top(Unit::Pixels(20.0))
			.with_bottom(Unit::Pixels(20.0));

		let bounds = layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);

		// Width = 800 - 10 - 10 = 780
		// Height = 600 - 20 - 20 = 560
		assert_eq!(bounds.width, 780.0);
		assert_eq!(bounds.height, 560.0);
		assert_eq!(bounds.x, 10.0);
		assert_eq!(bounds.y, 20.0);
	}
}
