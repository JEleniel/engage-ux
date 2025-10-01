//! Component trait and base structures
//!
//! Defines the base trait that all UI components must implement.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Unique identifier for components
pub type ComponentId = u64;

/// Rectangle representing position and size
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self { x, y, width, height }
	}

	pub fn contains_point(&self, x: f32, y: f32) -> bool {
		x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
	}
}

/// Properties common to all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties {
	pub id: ComponentId,
	pub visible: bool,
	pub enabled: bool,
	pub bounds: Rect,
}

impl ComponentProperties {
	pub fn new(id: ComponentId) -> Self {
		Self {
			id,
			visible: true,
			enabled: true,
			bounds: Rect::new(0.0, 0.0, 100.0, 100.0),
		}
	}
}

/// Base trait for all UI components
pub trait Component: Send + Sync {
	/// Get the component's unique identifier
	fn id(&self) -> ComponentId;

	/// Get component properties
	fn properties(&self) -> &ComponentProperties;

	/// Get mutable component properties
	fn properties_mut(&mut self) -> &mut ComponentProperties;

	/// Check if component is visible
	fn is_visible(&self) -> bool {
		self.properties().visible
	}

	/// Set visibility
	fn set_visible(&mut self, visible: bool) {
		self.properties_mut().visible = visible;
	}

	/// Check if component is enabled
	fn is_enabled(&self) -> bool {
		self.properties().enabled
	}

	/// Set enabled state
	fn set_enabled(&mut self, enabled: bool) {
		self.properties_mut().enabled = enabled;
	}

	/// Get component bounds
	fn bounds(&self) -> Rect {
		self.properties().bounds
	}

	/// Set component bounds
	fn set_bounds(&mut self, bounds: Rect) {
		self.properties_mut().bounds = bounds;
	}
}

/// Thread-safe wrapper for components
pub type ComponentRef = Arc<RwLock<dyn Component>>;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rect_creation() {
		let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
		assert_eq!(rect.x, 10.0);
		assert_eq!(rect.y, 20.0);
		assert_eq!(rect.width, 100.0);
		assert_eq!(rect.height, 50.0);
	}

	#[test]
	fn test_rect_contains_point() {
		let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
		assert!(rect.contains_point(50.0, 50.0));
		assert!(rect.contains_point(0.0, 0.0));
		assert!(rect.contains_point(100.0, 100.0));
		assert!(!rect.contains_point(101.0, 50.0));
		assert!(!rect.contains_point(50.0, 101.0));
	}

	#[test]
	fn test_component_properties() {
		let props = ComponentProperties::new(1);
		assert_eq!(props.id, 1);
		assert!(props.visible);
		assert!(props.enabled);
	}
}
