//! Radio button component

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Radio button component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioButton {
	properties: ComponentProperties,
	selected: bool,
	label: String,
	group: String,
}

impl RadioButton {
	pub fn new(id: ComponentId, label: impl Into<String>, group: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			selected: false,
			label: label.into(),
			group: group.into(),
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
