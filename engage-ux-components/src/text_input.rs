//! Text input component

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Text input component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInput {
	properties: ComponentProperties,
	value: String,
	placeholder: String,
}

impl TextInput {
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			value: String::new(),
			placeholder: String::new(),
		}
	}

	pub fn value(&self) -> &str {
		&self.value
	}

	pub fn set_value(&mut self, value: impl Into<String>) {
		self.value = value.into();
	}
}

impl Component for TextInput {
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
