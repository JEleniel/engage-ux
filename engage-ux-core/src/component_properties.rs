use serde::{Deserialize, Serialize};

/// Properties common to all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties {
	pub id: u128,
	pub visible: bool,
	pub enabled: bool,
}

impl ComponentProperties {
	pub fn new(id: u128) -> Self {
		Self {
			id,
			visible: true,
			enabled: true,
		}
	}
}
