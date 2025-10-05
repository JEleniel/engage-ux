//! Size specification with support for fill mode

use super::units::Unit;
use serde::{Deserialize, Serialize};

/// Size specification for width or height
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Size {
	/// Fixed size in specified units
	Fixed(Unit),
	/// Fill available space in parent
	Fill,
	/// Fit content (minimum size needed)
	FitContent,
}

impl Size {
	/// Calculate the actual pixel size
	pub fn calculate(
		&self,
		parent_width: f32,
		parent_height: f32,
		base_size: f32,
		inherited_size: f32,
	) -> f32 {
		match self {
			Size::Fixed(unit) => {
				// For fixed sizes, we use the larger parent dimension
				// to handle both width and height calculations
				let parent_dim = parent_width.max(parent_height);
				unit.to_pixels(parent_dim, base_size, inherited_size)
			}
			Size::Fill => parent_width.max(parent_height),
			Size::FitContent => 0.0, // Would be calculated based on content
		}
	}
}

/// Size mode enum for easier API usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeMode {
	/// Fixed size
	Fixed,
	/// Fill parent
	Fill,
	/// Fit content
	FitContent,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fixed_size() {
		let size = Size::Fixed(Unit::pixels(100.0));
		assert_eq!(size.calculate(800.0, 600.0, 16.0, 16.0), 100.0);
	}

	#[test]
	fn test_fill_size() {
		let size = Size::Fill;
		// Fill returns the larger of width/height
		assert_eq!(size.calculate(800.0, 600.0, 16.0, 16.0), 800.0);
		assert_eq!(size.calculate(400.0, 900.0, 16.0, 16.0), 900.0);
	}

	#[test]
	fn test_fit_content_size() {
		let size = Size::FitContent;
		// FitContent returns 0 as placeholder (would be calculated from content)
		assert_eq!(size.calculate(800.0, 600.0, 16.0, 16.0), 0.0);
	}

	#[test]
	fn test_fixed_with_relative_units() {
		let size = Size::Fixed(Unit::percent(50.0));
		// 50% of max(800, 600) = 400
		assert_eq!(size.calculate(800.0, 600.0, 16.0, 16.0), 400.0);
	}
}
