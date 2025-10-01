//! Container component for layout

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutDirection {
	Row,
	Column,
}

/// Container component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
	properties: ComponentProperties,
	children: Vec<ComponentId>,
	direction: LayoutDirection,
	padding: f32,
	gap: f32,
}

impl Container {
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			children: Vec::new(),
			direction: LayoutDirection::Column,
			padding: 0.0,
			gap: 0.0,
		}
	}

	pub fn add_child(&mut self, child_id: ComponentId) {
		self.children.push(child_id);
	}

	pub fn children(&self) -> &[ComponentId] {
		&self.children
	}
}

impl Component for Container {
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
