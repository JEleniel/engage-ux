use serde::{Deserialize, Serialize};

/// A structure representing the border widths on all four sides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
	/// The width of the top border.
	pub top: f32,
	/// The width of the right border.
	pub right: f32,
	/// The width of the bottom border.
	pub bottom: f32,
	/// The width of the left border.
	pub left: f32,
}
