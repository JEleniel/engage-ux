//! Icon component for displaying icons

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Icon component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
	properties: ComponentProperties,
	name: String,
	size: f32,
	color: Color,
	rotation: f32,
	flip_horizontal: bool,
	flip_vertical: bool,
}

impl Icon {
	/// Create a new icon
	pub fn new(id: ComponentId, name: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			name: name.into(),
			size: 24.0,
			color: Color::from_hex("#000000").unwrap(),
			rotation: 0.0,
			flip_horizontal: false,
			flip_vertical: false,
		}
	}

	/// Get icon name
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Set icon name
	pub fn set_name(&mut self, name: impl Into<String>) {
		self.name = name.into();
	}

	/// Get size
	pub fn size(&self) -> f32 {
		self.size
	}

	/// Set size
	pub fn set_size(&mut self, size: f32) {
		self.size = size.max(1.0);
	}

	/// Get color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get rotation in degrees
	pub fn rotation(&self) -> f32 {
		self.rotation
	}

	/// Set rotation in degrees
	pub fn set_rotation(&mut self, degrees: f32) {
		self.rotation = degrees % 360.0;
	}

	/// Check if horizontally flipped
	pub fn is_flipped_horizontal(&self) -> bool {
		self.flip_horizontal
	}

	/// Set horizontal flip
	pub fn set_flip_horizontal(&mut self, flip: bool) {
		self.flip_horizontal = flip;
	}

	/// Check if vertically flipped
	pub fn is_flipped_vertical(&self) -> bool {
		self.flip_vertical
	}

	/// Set vertical flip
	pub fn set_flip_vertical(&mut self, flip: bool) {
		self.flip_vertical = flip;
	}
}

impl Component for Icon {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn properties(&self) -> &ComponentProperties {
		&self.properties
	}

	fn properties_mut(&mut self) -> &mut ComponentProperties {
		&mut self.properties
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_icon_creation() {
		let icon = Icon::new(1, "home");
		assert_eq!(icon.id(), 1);
		assert_eq!(icon.name(), "home");
		assert_eq!(icon.size(), 24.0);
	}

	#[test]
	fn test_icon_size() {
		let mut icon = Icon::new(1, "home");
		icon.set_size(32.0);
		assert_eq!(icon.size(), 32.0);
	}

	#[test]
	fn test_icon_rotation() {
		let mut icon = Icon::new(1, "arrow");
		icon.set_rotation(90.0);
		assert_eq!(icon.rotation(), 90.0);

		icon.set_rotation(450.0); // Should wrap to 90
		assert_eq!(icon.rotation(), 90.0);
	}

	#[test]
	fn test_icon_flip() {
		let mut icon = Icon::new(1, "arrow");
		assert!(!icon.is_flipped_horizontal());
		assert!(!icon.is_flipped_vertical());

		icon.set_flip_horizontal(true);
		icon.set_flip_vertical(true);
		assert!(icon.is_flipped_horizontal());
		assert!(icon.is_flipped_vertical());
	}
}
