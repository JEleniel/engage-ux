//! Icon component for displaying icons

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Icon component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
	properties: ComponentProperties,
	name: String,
	size: f32,
	color: Color,
}

impl Icon {
	pub fn new(id: ComponentId, name: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			name: name.into(),
			size: 24.0,
			color: Color::from_hex("#000000").unwrap(),
		}
	}
}

impl Component for Icon {
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
