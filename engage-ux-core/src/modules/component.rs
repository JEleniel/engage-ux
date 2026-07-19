//! Component trait and types
//!
//! Defines the base `Component` trait that all UI components implement, along with
//! common type aliases used throughout the framework.

use std::sync::Arc;

pub use crate::component_properties::ComponentProperties;

/// Unique identifier for a component instance
pub type ComponentId = u128;

/// Callback for handling component events
///
/// Event callbacks are optional handlers that components can invoke when certain
/// events occur (e.g., button clicks, text changes). The callback receives a reference
/// to the event that triggered it. Uses Arc to allow sharing and cloning across
/// components that derive Clone.
pub type EventCallback = Arc<dyn Fn(&crate::event::Event) + Send + Sync>;

/// Base trait that all UI components implement
///
/// This trait provides a common interface for managing component properties
/// (visibility, enabled state, and unique identifier) across all component types.
/// Components should implement this trait to participate in the component system.
///
/// Components can optionally implement lifecycle hooks like `on_register` to subscribe
/// to the EventBus when they're added to the application.
pub trait Component: Send + Sync {
	/// Get the unique identifier for this component
	fn id(&self) -> ComponentId;

	/// Get a reference to this component's properties
	fn properties(&self) -> &ComponentProperties;

	/// Get a mutable reference to this component's properties
	fn properties_mut(&mut self) -> &mut ComponentProperties;

	/// Optional: Called when the component is registered with an app
	///
	/// This is a good place to subscribe to the EventBus to listen for events
	/// directed at this component (events where `source_component_id` matches
	/// this component's ID). Override this method to set up subscriptions.
	///
	/// # Example
	///
	/// ```ignore
	/// fn on_register(&mut self, bus: &EventBus) {
	///     self.event_receiver = Some(bus.subscribe());
	/// }
	/// ```
	fn on_register(&mut self, _bus: &crate::event::EventBus) {
		// Default: do nothing
	}

	/// Optional: Called when the component is about to be unregistered from an app
	///
	/// This is a good place to clean up resources, stop listening for events, etc.
	fn on_unregister(&mut self) {
		// Default: do nothing
	}

	/// Check if the component is visible
	#[inline]
	fn is_visible(&self) -> bool {
		self.properties().visible
	}

	/// Check if the component is enabled (accepts input)
	#[inline]
	fn is_enabled(&self) -> bool {
		self.properties().enabled
	}

	/// Set the visibility of the component
	#[inline]
	fn set_visible(&mut self, visible: bool) {
		self.properties_mut().visible = visible;
	}

	/// Set the enabled state of the component
	#[inline]
	fn set_enabled(&mut self, enabled: bool) {
		self.properties_mut().enabled = enabled;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// Mock component for testing
	struct MockComponent {
		properties: ComponentProperties,
	}

	impl Component for MockComponent {
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

	#[test]
	fn test_component_visibility() {
		let mut component = MockComponent {
			properties: ComponentProperties::new(1),
		};

		assert!(component.is_visible());
		component.set_visible(false);
		assert!(!component.is_visible());
	}

	#[test]
	fn test_component_enabled() {
		let mut component = MockComponent {
			properties: ComponentProperties::new(1),
		};

		assert!(component.is_enabled());
		component.set_enabled(false);
		assert!(!component.is_enabled());
	}

	#[test]
	fn test_component_id() {
		let component = MockComponent {
			properties: ComponentProperties::new(42),
		};

		assert_eq!(component.id(), 42);
	}
}
