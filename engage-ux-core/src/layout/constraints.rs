//! Size constraints for layout

use super::units::Unit;
use serde::{Deserialize, Serialize};

/// Size constraints (min/max width and height)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraints {
	pub min_width: Option<Unit>,
	pub max_width: Option<Unit>,
	pub min_height: Option<Unit>,
	pub max_height: Option<Unit>,
}

impl Constraints {
	/// Create new constraints with no limits
	pub fn new() -> Self {
		Self {
			min_width: None,
			max_width: None,
			min_height: None,
			max_height: None,
		}
	}

	/// Set minimum width
	pub fn with_min_width(mut self, min: Unit) -> Self {
		self.min_width = Some(min);
		self
	}

	/// Set maximum width
	pub fn with_max_width(mut self, max: Unit) -> Self {
		self.max_width = Some(max);
		self
	}

	/// Set minimum height
	pub fn with_min_height(mut self, min: Unit) -> Self {
		self.min_height = Some(min);
		self
	}

	/// Set maximum height
	pub fn with_max_height(mut self, max: Unit) -> Self {
		self.max_height = Some(max);
		self
	}

	/// Apply constraints to a width value
	pub fn constrain_width(
		&self,
		width: f32,
		parent_width: f32,
		base_size: f32,
		inherited_size: f32,
	) -> f32 {
		let mut constrained = width;

		if let Some(min) = &self.min_width {
			let min_px = min.to_pixels(parent_width, base_size, inherited_size);
			constrained = constrained.max(min_px);
		}

		if let Some(max) = &self.max_width {
			let max_px = max.to_pixels(parent_width, base_size, inherited_size);
			constrained = constrained.min(max_px);
		}

		constrained
	}

	/// Apply constraints to a height value
	pub fn constrain_height(
		&self,
		height: f32,
		parent_height: f32,
		base_size: f32,
		inherited_size: f32,
	) -> f32 {
		let mut constrained = height;

		if let Some(min) = &self.min_height {
			let min_px = min.to_pixels(parent_height, base_size, inherited_size);
			constrained = constrained.max(min_px);
		}

		if let Some(max) = &self.max_height {
			let max_px = max.to_pixels(parent_height, base_size, inherited_size);
			constrained = constrained.min(max_px);
		}

		constrained
	}
}

impl Default for Constraints {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_constraints_creation() {
		let constraints = Constraints::new();
		assert!(constraints.min_width.is_none());
		assert!(constraints.max_width.is_none());
	}

	#[test]
	fn test_constraints_builder() {
		let constraints = Constraints::new()
			.with_min_width(Unit::pixels(100.0))
			.with_max_width(Unit::pixels(500.0));

		assert_eq!(constraints.min_width, Some(Unit::pixels(100.0)));
		assert_eq!(constraints.max_width, Some(Unit::pixels(500.0)));
	}

	#[test]
	fn test_constrain_width_min() {
		let constraints = Constraints::new().with_min_width(Unit::pixels(100.0));

		// Below minimum - should be clamped up
		assert_eq!(constraints.constrain_width(50.0, 800.0, 16.0, 16.0), 100.0);

		// Above minimum - should pass through
		assert_eq!(constraints.constrain_width(150.0, 800.0, 16.0, 16.0), 150.0);
	}

	#[test]
	fn test_constrain_width_max() {
		let constraints = Constraints::new().with_max_width(Unit::pixels(500.0));

		// Below maximum - should pass through
		assert_eq!(constraints.constrain_width(400.0, 800.0, 16.0, 16.0), 400.0);

		// Above maximum - should be clamped down
		assert_eq!(constraints.constrain_width(600.0, 800.0, 16.0, 16.0), 500.0);
	}

	#[test]
	fn test_constrain_width_both() {
		let constraints = Constraints::new()
			.with_min_width(Unit::pixels(100.0))
			.with_max_width(Unit::pixels(500.0));

		// Within range - should pass through
		assert_eq!(constraints.constrain_width(300.0, 800.0, 16.0, 16.0), 300.0);

		// Below minimum
		assert_eq!(constraints.constrain_width(50.0, 800.0, 16.0, 16.0), 100.0);

		// Above maximum
		assert_eq!(constraints.constrain_width(600.0, 800.0, 16.0, 16.0), 500.0);
	}

	#[test]
	fn test_constrain_height() {
		let constraints = Constraints::new()
			.with_min_height(Unit::pixels(50.0))
			.with_max_height(Unit::pixels(300.0));

		assert_eq!(
			constraints.constrain_height(200.0, 600.0, 16.0, 16.0),
			200.0
		);
		assert_eq!(constraints.constrain_height(30.0, 600.0, 16.0, 16.0), 50.0);
		assert_eq!(
			constraints.constrain_height(400.0, 600.0, 16.0, 16.0),
			300.0
		);
	}
}
