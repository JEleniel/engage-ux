//! Avatar component for user profiles

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Avatar shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvatarShape {
	Circle,
	Square,
	Rounded,
}

/// Avatar size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvatarSize {
	Small,
	Medium,
	Large,
	Custom(u32),
}

impl AvatarSize {
	/// Get size in pixels
	pub fn pixels(&self) -> u32 {
		match self {
			AvatarSize::Small => 32,
			AvatarSize::Medium => 48,
			AvatarSize::Large => 64,
			AvatarSize::Custom(size) => *size,
		}
	}
}

/// Avatar component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Avatar {
	properties: ComponentProperties,
	image_url: Option<String>,
	initials: Option<String>,
	icon: Option<String>,
	shape: AvatarShape,
	size: AvatarSize,
	alt_text: String,
	color: Color,
	background_color: Color,
	border_color: Option<Color>,
	border_width: f32,
}

impl Avatar {
	/// Create a new avatar
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			image_url: None,
			initials: None,
			icon: None,
			shape: AvatarShape::Circle,
			size: AvatarSize::Medium,
			alt_text: String::new(),
			color: Color::from_hex("#FFFFFF").unwrap(),
			background_color: Color::from_hex("#757575").unwrap(),
			border_color: None,
			border_width: 0.0,
		}
	}

	/// Create an avatar with an image
	pub fn with_image(id: ComponentId, image_url: impl Into<String>) -> Self {
		let mut avatar = Self::new(id);
		avatar.image_url = Some(image_url.into());
		avatar
	}

	/// Create an avatar with initials
	pub fn with_initials(id: ComponentId, initials: impl Into<String>) -> Self {
		let mut avatar = Self::new(id);
		avatar.initials = Some(initials.into());
		avatar
	}

	/// Create an avatar with an icon
	pub fn with_icon(id: ComponentId, icon: impl Into<String>) -> Self {
		let mut avatar = Self::new(id);
		avatar.icon = Some(icon.into());
		avatar
	}

	/// Get image URL
	pub fn image_url(&self) -> Option<&str> {
		self.image_url.as_deref()
	}

	/// Set image URL
	pub fn set_image_url(&mut self, image_url: Option<String>) {
		self.image_url = image_url;
	}

	/// Get initials
	pub fn initials(&self) -> Option<&str> {
		self.initials.as_deref()
	}

	/// Set initials
	pub fn set_initials(&mut self, initials: Option<String>) {
		self.initials = initials;
	}

	/// Get icon
	pub fn icon(&self) -> Option<&str> {
		self.icon.as_deref()
	}

	/// Set icon
	pub fn set_icon(&mut self, icon: Option<String>) {
		self.icon = icon;
	}

	/// Get shape
	pub fn shape(&self) -> AvatarShape {
		self.shape
	}

	/// Set shape
	pub fn set_shape(&mut self, shape: AvatarShape) {
		self.shape = shape;
	}

	/// Get size
	pub fn size(&self) -> AvatarSize {
		self.size
	}

	/// Set size
	pub fn set_size(&mut self, size: AvatarSize) {
		self.size = size;
	}

	/// Get alt text
	pub fn alt_text(&self) -> &str {
		&self.alt_text
	}

	/// Set alt text (for accessibility)
	pub fn set_alt_text(&mut self, alt_text: impl Into<String>) {
		self.alt_text = alt_text.into();
	}

	/// Set text color (for initials/icons)
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Get border color
	pub fn border_color(&self) -> Option<&Color> {
		self.border_color.as_ref()
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Option<Color>) {
		self.border_color = color;
	}

	/// Get border width
	pub fn border_width(&self) -> f32 {
		self.border_width
	}

	/// Set border width
	pub fn set_border_width(&mut self, width: f32) {
		self.border_width = width.max(0.0);
	}
}

impl Component for Avatar {
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
	fn test_avatar_creation() {
		let avatar = Avatar::new(1);
		assert_eq!(avatar.id(), 1);
		assert_eq!(avatar.shape(), AvatarShape::Circle);
		assert_eq!(avatar.size().pixels(), 48); // Medium
	}

	#[test]
	fn test_avatar_with_image() {
		let avatar = Avatar::with_image(1, "https://example.com/avatar.jpg");
		assert_eq!(avatar.image_url(), Some("https://example.com/avatar.jpg"));
	}

	#[test]
	fn test_avatar_with_initials() {
		let avatar = Avatar::with_initials(1, "JD");
		assert_eq!(avatar.initials(), Some("JD"));
	}

	#[test]
	fn test_avatar_with_icon() {
		let avatar = Avatar::with_icon(1, "user");
		assert_eq!(avatar.icon(), Some("user"));
	}

	#[test]
	fn test_avatar_shape() {
		let mut avatar = Avatar::new(1);
		avatar.set_shape(AvatarShape::Square);
		assert_eq!(avatar.shape(), AvatarShape::Square);
	}

	#[test]
	fn test_avatar_size() {
		let mut avatar = Avatar::new(1);
		
		avatar.set_size(AvatarSize::Small);
		assert_eq!(avatar.size().pixels(), 32);
		
		avatar.set_size(AvatarSize::Large);
		assert_eq!(avatar.size().pixels(), 64);
		
		avatar.set_size(AvatarSize::Custom(100));
		assert_eq!(avatar.size().pixels(), 100);
	}

	#[test]
	fn test_avatar_border() {
		let mut avatar = Avatar::new(1);
		assert_eq!(avatar.border_width(), 0.0);
		
		avatar.set_border_width(2.0);
		avatar.set_border_color(Some(Color::from_hex("#000000").unwrap()));
		
		assert_eq!(avatar.border_width(), 2.0);
		assert!(avatar.border_color().is_some());
	}
}
