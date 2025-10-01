//! Toggle switch component

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Toggle size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToggleSize {
	Small,
	Medium,
	Large,
}

impl ToggleSize {
	pub fn width(&self) -> f32 {
		match self {
			ToggleSize::Small => 32.0,
			ToggleSize::Medium => 44.0,
			ToggleSize::Large => 56.0,
		}
	}

	pub fn height(&self) -> f32 {
		match self {
			ToggleSize::Small => 18.0,
			ToggleSize::Medium => 24.0,
			ToggleSize::Large => 30.0,
		}
	}
}

/// Toggle component
#[derive(Clone, Serialize, Deserialize)]
pub struct Toggle {
	properties: ComponentProperties,
	active: bool,
	disabled: bool,
	label: String,
	size: ToggleSize,
	active_color: Color,
	inactive_color: Color,
	thumb_color: Color,
	label_color: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl Toggle {
	/// Create a new toggle
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			active: false,
			disabled: false,
			label: String::new(),
			size: ToggleSize::Medium,
			active_color: Color::from_hex("#1976D2").unwrap(),
			inactive_color: Color::from_hex("#BDBDBD").unwrap(),
			thumb_color: Color::from_hex("#FFFFFF").unwrap(),
			label_color: Color::from_hex("#000000").unwrap(),
			on_change: None,
		}
	}

	/// Create a toggle with label
	pub fn with_label(id: ComponentId, label: impl Into<String>) -> Self {
		let mut toggle = Self::new(id);
		toggle.label = label.into();
		toggle
	}

	/// Check if active
	pub fn is_active(&self) -> bool {
		self.active
	}

	/// Set active
	pub fn set_active(&mut self, active: bool) {
		self.active = active;
	}

	/// Toggle state
	pub fn toggle(&mut self) {
		self.active = !self.active;
	}

	/// Check if disabled
	pub fn is_disabled(&self) -> bool {
		self.disabled
	}

	/// Set disabled
	pub fn set_disabled(&mut self, disabled: bool) {
		self.disabled = disabled;
	}

	/// Get label
	pub fn label(&self) -> &str {
		&self.label
	}

	/// Set label
	pub fn set_label(&mut self, label: impl Into<String>) {
		self.label = label.into();
	}

	/// Get size
	pub fn size(&self) -> ToggleSize {
		self.size
	}

	/// Set size
	pub fn set_size(&mut self, size: ToggleSize) {
		self.size = size;
	}

	/// Set active color
	pub fn set_active_color(&mut self, color: Color) {
		self.active_color = color;
	}

	/// Set inactive color
	pub fn set_inactive_color(&mut self, color: Color) {
		self.inactive_color = color;
	}

	/// Set thumb color
	pub fn set_thumb_color(&mut self, color: Color) {
		self.thumb_color = color;
	}

	/// Set label color
	pub fn set_label_color(&mut self, color: Color) {
		self.label_color = color;
	}

	/// Set change callback
	pub fn set_on_change(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_change = Some(std::sync::Arc::new(callback));
	}

	/// Handle change event
	pub fn handle_change(&self, event: &Event) {
		if let Some(ref callback) = self.on_change {
			callback(event);
		}
	}
}

impl Component for Toggle {
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
	fn test_toggle_creation() {
		let toggle = Toggle::new(1);
		assert_eq!(toggle.id(), 1);
		assert!(!toggle.is_active());
	}

	#[test]
	fn test_toggle_with_label() {
		let toggle = Toggle::with_label(1, "Enable feature");
		assert_eq!(toggle.label(), "Enable feature");
	}

	#[test]
	fn test_toggle_state() {
		let mut toggle = Toggle::new(1);
		toggle.set_active(true);
		assert!(toggle.is_active());
		
		toggle.toggle();
		assert!(!toggle.is_active());
		
		toggle.toggle();
		assert!(toggle.is_active());
	}

	#[test]
	fn test_toggle_disabled() {
		let mut toggle = Toggle::new(1);
		assert!(!toggle.is_disabled());
		toggle.set_disabled(true);
		assert!(toggle.is_disabled());
	}

	#[test]
	fn test_toggle_size() {
		let mut toggle = Toggle::new(1);
		assert_eq!(toggle.size(), ToggleSize::Medium);
		
		toggle.set_size(ToggleSize::Large);
		assert_eq!(toggle.size(), ToggleSize::Large);
		assert_eq!(toggle.size().width(), 56.0);
	}
}
