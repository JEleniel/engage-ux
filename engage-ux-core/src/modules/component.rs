//! Component trait
//!
//! This file contains only the `Component` trait and the thread-safe alias
//! `ComponentRef`. Component-related data structures live in separate files so
//! each `.rs` file contains a single primary struct/trait.

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::component_properties::ComponentProperties;

/// Base trait for all UI components
pub trait Component: Send + Sync {
	/// Get the component's unique identifier
	fn id(&self) -> crate::types::ComponentId;

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
	fn bounds(&self) -> crate::types::Rect {
		self.properties().bounds
	}

	/// Set component bounds
	fn set_bounds(&mut self, bounds: crate::types::Rect) {
		self.properties_mut().bounds = bounds;
	}
}

/// Thread-safe wrapper for components
pub type ComponentRef = Arc<RwLock<dyn Component>>;

