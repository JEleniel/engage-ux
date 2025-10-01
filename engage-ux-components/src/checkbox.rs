//! Checkbox component

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Checkbox state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckboxState {
	Unchecked,
	Checked,
	Indeterminate,
}

/// Checkbox component
#[derive(Clone, Serialize, Deserialize)]
pub struct Checkbox {
	properties: ComponentProperties,
	state: CheckboxState,
	label: String,
	disabled: bool,
	color: Color,
	check_color: Color,
	label_color: Color,
	size: f32,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl Checkbox {
	/// Create a new checkbox
	pub fn new(id: ComponentId, label: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			state: CheckboxState::Unchecked,
			label: label.into(),
			disabled: false,
			color: Color::from_hex("#1976D2").unwrap(),
			check_color: Color::from_hex("#FFFFFF").unwrap(),
			label_color: Color::from_hex("#000000").unwrap(),
			size: 20.0,
			on_change: None,
		}
	}

	/// Get state
	pub fn state(&self) -> CheckboxState {
		self.state
	}

	/// Set state
	pub fn set_state(&mut self, state: CheckboxState) {
		self.state = state;
	}

	/// Check if checked
	pub fn is_checked(&self) -> bool {
		self.state == CheckboxState::Checked
	}

	/// Set checked
	pub fn set_checked(&mut self, checked: bool) {
		self.state = if checked {
			CheckboxState::Checked
		} else {
			CheckboxState::Unchecked
		};
	}

	/// Check if indeterminate
	pub fn is_indeterminate(&self) -> bool {
		self.state == CheckboxState::Indeterminate
	}

	/// Set indeterminate
	pub fn set_indeterminate(&mut self, indeterminate: bool) {
		if indeterminate {
			self.state = CheckboxState::Indeterminate;
		}
	}

	/// Toggle checked state
	pub fn toggle(&mut self) {
		self.state = match self.state {
			CheckboxState::Unchecked => CheckboxState::Checked,
			CheckboxState::Checked => CheckboxState::Unchecked,
			CheckboxState::Indeterminate => CheckboxState::Checked,
		};
	}

	/// Get label
	pub fn label(&self) -> &str {
		&self.label
	}

	/// Set label
	pub fn set_label(&mut self, label: impl Into<String>) {
		self.label = label.into();
	}

	/// Check if disabled
	pub fn is_disabled(&self) -> bool {
		self.disabled
	}

	/// Set disabled
	pub fn set_disabled(&mut self, disabled: bool) {
		self.disabled = disabled;
	}

	/// Set checkbox color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set check mark color
	pub fn set_check_color(&mut self, color: Color) {
		self.check_color = color;
	}

	/// Set label color
	pub fn set_label_color(&mut self, color: Color) {
		self.label_color = color;
	}

	/// Get size
	pub fn size(&self) -> f32 {
		self.size
	}

	/// Set size
	pub fn set_size(&mut self, size: f32) {
		self.size = size.max(10.0);
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

impl Component for Checkbox {
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
	fn test_checkbox_creation() {
		let checkbox = Checkbox::new(1, "Accept terms");
		assert_eq!(checkbox.id(), 1);
		assert_eq!(checkbox.label(), "Accept terms");
		assert!(!checkbox.is_checked());
	}

	#[test]
	fn test_checkbox_checked() {
		let mut checkbox = Checkbox::new(1, "Test");
		checkbox.set_checked(true);
		assert!(checkbox.is_checked());
		assert_eq!(checkbox.state(), CheckboxState::Checked);
	}

	#[test]
	fn test_checkbox_indeterminate() {
		let mut checkbox = Checkbox::new(1, "Test");
		checkbox.set_indeterminate(true);
		assert!(checkbox.is_indeterminate());
		assert_eq!(checkbox.state(), CheckboxState::Indeterminate);
	}

	#[test]
	fn test_checkbox_toggle() {
		let mut checkbox = Checkbox::new(1, "Test");
		checkbox.toggle();
		assert!(checkbox.is_checked());
		checkbox.toggle();
		assert!(!checkbox.is_checked());
	}

	#[test]
	fn test_checkbox_disabled() {
		let mut checkbox = Checkbox::new(1, "Test");
		assert!(!checkbox.is_disabled());
		checkbox.set_disabled(true);
		assert!(checkbox.is_disabled());
	}

	#[test]
	fn test_checkbox_size() {
		let mut checkbox = Checkbox::new(1, "Test");
		checkbox.set_size(24.0);
		assert_eq!(checkbox.size(), 24.0);
	}
}
