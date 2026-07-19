use crate::geometry::{Border, Rectangle};

/// Represents the bounds of a UI element, including its outer rectangle,
/// margin, and padding.
#[derive(Debug, Clone)]
pub struct Bounds {
	/// The outer rectangle defining the bounds.
	pub bounds: Rectangle,
	/// The margin (white space) around the element.
	pub margin: Border,
	/// The padding (filled space) inside the element.
	pub padding: Border,
}

impl Bounds {
	/// Get the rectangle representing the margin area.
	pub fn margin_rectangle(&self) -> Rectangle {
		self.bounds.shrink(&self.margin)
	}

	/// Get the rectangle representing the padding area.
	pub fn padding_rectangle(&self) -> Rectangle {
		self.margin_rectangle().shrink(&self.padding)
	}
}
