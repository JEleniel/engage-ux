//! Slider/range selector component

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Slider component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slider {
	properties: ComponentProperties,
	value: f32,
	min: f32,
	max: f32,
	step: f32,
}

impl Slider {
	pub fn new(id: ComponentId, min: f32, max: f32) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			value: min,
			min,
			max,
			step: 1.0,
		}
	}

	pub fn value(&self) -> f32 {
		self.value
	}

	pub fn set_value(&mut self, value: f32) {
		self.value = value.clamp(self.min, self.max);
	}
}

impl Component for Slider {
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
