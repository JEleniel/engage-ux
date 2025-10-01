//! Image component for displaying images

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Image component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
	properties: ComponentProperties,
	source: String,
	alt_text: String,
}

impl Image {
	pub fn new(id: ComponentId, source: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			source: source.into(),
			alt_text: String::new(),
		}
	}
}

impl Component for Image {
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
