//! Checkbox component

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Checkbox component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkbox {
	properties: ComponentProperties,
	checked: bool,
	label: String,
}

impl Checkbox {
	pub fn new(id: ComponentId, label: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			checked: false,
			label: label.into(),
		}
	}

	pub fn is_checked(&self) -> bool {
		self.checked
	}

	pub fn set_checked(&mut self, checked: bool) {
		self.checked = checked;
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
