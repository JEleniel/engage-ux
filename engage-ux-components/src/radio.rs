//! Radio button component

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Radio button component
#[derive(Clone, Serialize, Deserialize)]
pub struct RadioButton {
	properties: ComponentProperties,
	selected: bool,
	label: String,
	value: String,
	group: String,
	disabled: bool,
	color: Color,
	selected_color: Color,
	label_color: Color,
	size: f32,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl RadioButton {
	/// Create a new radio button
	pub fn new(
		id: ComponentId,
		label: impl Into<String>,
		value: impl Into<String>,
		group: impl Into<String>,
	) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			selected: false,
			label: label.into(),
			value: value.into(),
			group: group.into(),
			disabled: false,
			color: Color::from_hex("#BDBDBD").unwrap(),
			selected_color: Color::from_hex("#1976D2").unwrap(),
			label_color: Color::from_hex("#000000").unwrap(),
			size: 20.0,
			on_change: None,
		}
	}

	/// Check if selected
	pub fn is_selected(&self) -> bool {
		self.selected
	}

	/// Set selected
	pub fn set_selected(&mut self, selected: bool) {
		self.selected = selected;
	}

	/// Get label
	pub fn label(&self) -> &str {
		&self.label
	}

	/// Set label
	pub fn set_label(&mut self, label: impl Into<String>) {
		self.label = label.into();
	}

	/// Get value
	pub fn value(&self) -> &str {
		&self.value
	}

	/// Set value
	pub fn set_value(&mut self, value: impl Into<String>) {
		self.value = value.into();
	}

	/// Get group name
	pub fn group(&self) -> &str {
		&self.group
	}

	/// Set group name
	pub fn set_group(&mut self, group: impl Into<String>) {
		self.group = group.into();
	}

	/// Check if disabled
	pub fn is_disabled(&self) -> bool {
		self.disabled
	}

	/// Set disabled
	pub fn set_disabled(&mut self, disabled: bool) {
		self.disabled = disabled;
	}

	/// Set unselected color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set selected color
	pub fn set_selected_color(&mut self, color: Color) {
		self.selected_color = color;
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

impl Component for RadioButton {
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

/// Radio group for managing multiple radio buttons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioGroup {
	properties: ComponentProperties,
	name: String,
	selected_value: Option<String>,
	buttons: Vec<String>, // Button IDs
}

impl RadioGroup {
	/// Create a new radio group
	pub fn new(id: ComponentId, name: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			name: name.into(),
			selected_value: None,
			buttons: Vec::new(),
		}
	}

	/// Get group name
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Add button to group
	pub fn add_button(&mut self, button_id: impl Into<String>) {
		self.buttons.push(button_id.into());
	}

	/// Get selected value
	pub fn selected_value(&self) -> Option<&str> {
		self.selected_value.as_deref()
	}

	/// Set selected value
	pub fn set_selected_value(&mut self, value: Option<String>) {
		self.selected_value = value;
	}

	/// Get button IDs
	pub fn buttons(&self) -> &[String] {
		&self.buttons
	}
}

impl Component for RadioGroup {
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
	fn test_radio_button_creation() {
		let radio = RadioButton::new(1, "Option 1", "opt1", "choices");
		assert_eq!(radio.id(), 1);
		assert_eq!(radio.label(), "Option 1");
		assert_eq!(radio.value(), "opt1");
		assert_eq!(radio.group(), "choices");
		assert!(!radio.is_selected());
	}

	#[test]
	fn test_radio_button_selection() {
		let mut radio = RadioButton::new(1, "Option 1", "opt1", "choices");
		radio.set_selected(true);
		assert!(radio.is_selected());
	}

	#[test]
	fn test_radio_button_disabled() {
		let mut radio = RadioButton::new(1, "Option 1", "opt1", "choices");
		assert!(!radio.is_disabled());
		radio.set_disabled(true);
		assert!(radio.is_disabled());
	}

	#[test]
	fn test_radio_button_size() {
		let mut radio = RadioButton::new(1, "Option 1", "opt1", "choices");
		radio.set_size(24.0);
		assert_eq!(radio.size(), 24.0);
	}

	#[test]
	fn test_radio_group_creation() {
		let group = RadioGroup::new(1, "choices");
		assert_eq!(group.id(), 1);
		assert_eq!(group.name(), "choices");
		assert_eq!(group.selected_value(), None);
	}

	#[test]
	fn test_radio_group_buttons() {
		let mut group = RadioGroup::new(1, "choices");
		group.add_button("btn1");
		group.add_button("btn2");
		assert_eq!(group.buttons().len(), 2);
	}

	#[test]
	fn test_radio_group_selection() {
		let mut group = RadioGroup::new(1, "choices");
		group.set_selected_value(Some("opt1".to_string()));
		assert_eq!(group.selected_value(), Some("opt1"));
	}
}
