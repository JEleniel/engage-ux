//! Card component for grouping content

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Card component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
	properties: ComponentProperties,
	title: Option<String>,
	background_color: Color,
	border_radius: f32,
	elevation: u8,
}

impl Card {
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: None,
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_radius: 4.0,
			elevation: 1,
		}
	}

	pub fn with_title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}

impl Component for Card {
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
