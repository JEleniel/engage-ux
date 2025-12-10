use serde::{Deserialize, Serialize};

use crate::Color;

/// Fill style applied to closed geometry shapes. When `color` is `None`
/// the shape is considered unfilled (transparent).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FillStyle {
	/// Optional fill color for the shape.
	pub color: Option<Color>,
}
