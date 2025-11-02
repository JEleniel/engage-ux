use crate::geometry::{Point, Rectangle, Unit};
use serde::{Deserialize, Serialize};

/// Properties common to all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties {
	pub id: crate::types::ComponentId,
	pub visible: bool,
	pub enabled: bool,
	pub bounds: Rectangle,
}

impl ComponentProperties {
	pub fn new(id: crate::types::ComponentId) -> Self {
		Self {
			id,
			visible: true,
			enabled: true,
			bounds: Rectangle {
				top_left: Point {
					x: Unit::from(0.0),
					y: Unit::from(0.0),
				},
				width: Unit::from(100.0),
				height: Unit::from(100.0),
			},
		}
	}
}
