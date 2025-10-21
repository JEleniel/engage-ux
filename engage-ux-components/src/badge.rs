//! Badge component for notifications and status indicators

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Badge variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeVariant {
	Default,
	Primary,
	Success,
	Warning,
	Error,
	Info,
}

/// Badge shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeShape {
	Rounded,
	Pill,
	Square,
	Circle,
}

/// Badge component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
	properties: ComponentProperties,
	content: String,
	variant: BadgeVariant,
	shape: BadgeShape,
	dot: bool,
	max: Option<u32>,
	color: Color,
	background_color: Color,
	font_size: f32,
}

impl Badge {
	/// Create a new badge
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			content: String::new(),
			variant: BadgeVariant::Default,
			shape: BadgeShape::Rounded,
			dot: false,
			max: None,
			color: Color::from_hex("#FFFFFF").unwrap(),
			background_color: Color::from_hex("#757575").unwrap(),
			font_size: 12.0,
		}
	}

	/// Create a badge with content
	pub fn with_content(id: ComponentId, content: impl Into<String>) -> Self {
		let mut badge = Self::new(id);
		badge.content = content.into();
		badge
	}

	/// Create a badge with numeric count
	pub fn with_count(id: ComponentId, count: u32) -> Self {
		let mut badge = Self::new(id);
		badge.set_count(count);
		badge
	}

	/// Get content
	pub fn content(&self) -> &str {
		&self.content
	}

	/// Set content
	pub fn set_content(&mut self, content: impl Into<String>) {
		self.content = content.into();
	}

	/// Set numeric count
	pub fn set_count(&mut self, count: u32) {
		if let Some(max) = self.max {
			if count > max {
				self.content = format!("{}+", max);
			} else {
				self.content = count.to_string();
			}
		} else {
			self.content = count.to_string();
		}
	}

	/// Get variant
	pub fn variant(&self) -> BadgeVariant {
		self.variant
	}

	/// Set variant
	pub fn set_variant(&mut self, variant: BadgeVariant) {
		self.variant = variant;
		// Update colors based on variant
		match variant {
			BadgeVariant::Primary => {
				self.background_color = Color::from_hex("#1976D2").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
			}
			BadgeVariant::Success => {
				self.background_color = Color::from_hex("#4CAF50").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
			}
			BadgeVariant::Warning => {
				self.background_color = Color::from_hex("#FF9800").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
			}
			BadgeVariant::Error => {
				self.background_color = Color::from_hex("#F44336").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
			}
			BadgeVariant::Info => {
				self.background_color = Color::from_hex("#2196F3").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
			}
			BadgeVariant::Default => {
				self.background_color = Color::from_hex("#757575").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
			}
		}
	}

	/// Get shape
	pub fn shape(&self) -> BadgeShape {
		self.shape
	}

	/// Set shape
	pub fn set_shape(&mut self, shape: BadgeShape) {
		self.shape = shape;
	}

	/// Check if dot badge
	pub fn is_dot(&self) -> bool {
		self.dot
	}

	/// Set dot badge (small indicator without text)
	pub fn set_dot(&mut self, dot: bool) {
		self.dot = dot;
	}

	/// Get max count
	pub fn max(&self) -> Option<u32> {
		self.max
	}

	/// Set max count (shows "max+" when exceeded)
	pub fn set_max(&mut self, max: Option<u32>) {
		self.max = max;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}
}

impl Component for Badge {
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
	fn test_badge_creation() {
		let badge = Badge::new(1);
		assert_eq!(badge.id(), 1);
		assert_eq!(badge.content(), "");
	}

	#[test]
	fn test_badge_with_content() {
		let badge = Badge::with_content(1, "New");
		assert_eq!(badge.content(), "New");
	}

	#[test]
	fn test_badge_with_count() {
		let badge = Badge::with_count(1, 5);
		assert_eq!(badge.content(), "5");
	}

	#[test]
	fn test_badge_max_count() {
		let mut badge = Badge::new(1);
		badge.set_max(Some(99));
		badge.set_count(150);
		assert_eq!(badge.content(), "99+");
	}

	#[test]
	fn test_badge_variant() {
		let mut badge = Badge::new(1);
		badge.set_variant(BadgeVariant::Success);
		assert_eq!(badge.variant(), BadgeVariant::Success);
	}

	#[test]
	fn test_badge_shape() {
		let mut badge = Badge::new(1);
		badge.set_shape(BadgeShape::Circle);
		assert_eq!(badge.shape(), BadgeShape::Circle);
	}

	#[test]
	fn test_badge_dot() {
		let mut badge = Badge::new(1);
		assert!(!badge.is_dot());
		badge.set_dot(true);
		assert!(badge.is_dot());
	}
}
