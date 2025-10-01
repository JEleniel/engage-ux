//! Toggle switch component

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Toggle component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toggle {
	properties: ComponentProperties,
	active: bool,
}

impl Toggle {
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			active: false,
		}
	}

	pub fn is_active(&self) -> bool {
		self.active
	}

	pub fn set_active(&mut self, active: bool) {
		self.active = active;
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
