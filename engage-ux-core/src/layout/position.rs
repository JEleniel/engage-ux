//! Position types and modes

use serde::{Deserialize, Serialize};

/// Position mode for components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionMode {
	/// Position relative to parent's content box
	Relative,
	/// Position absolute within parent
	Absolute,
}

/// Position specification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub mode: PositionMode,
}

impl Position {
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			x,
			y,
			mode: PositionMode::Relative,
		}
	}

	pub fn absolute(x: f32, y: f32) -> Self {
		Self {
			x,
			y,
			mode: PositionMode::Absolute,
		}
	}

	pub fn relative(x: f32, y: f32) -> Self {
		Self {
			x,
			y,
			mode: PositionMode::Relative,
		}
	}
}

impl Default for Position {
	fn default() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			mode: PositionMode::Relative,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_position_creation() {
		let pos = Position::new(10.0, 20.0);
		assert_eq!(pos.x, 10.0);
		assert_eq!(pos.y, 20.0);
		assert_eq!(pos.mode, PositionMode::Relative);
	}

	#[test]
	fn test_position_absolute() {
		let pos = Position::absolute(100.0, 200.0);
		assert_eq!(pos.mode, PositionMode::Absolute);
	}

	#[test]
	fn test_position_relative() {
		let pos = Position::relative(50.0, 75.0);
		assert_eq!(pos.mode, PositionMode::Relative);
	}
}
