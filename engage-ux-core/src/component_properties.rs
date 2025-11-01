use serde::{Deserialize, Serialize};
use crate::types::Rect;

/// Properties common to all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties {
    pub id: crate::types::ComponentId,
    pub visible: bool,
    pub enabled: bool,
    pub bounds: Rect,
}

impl ComponentProperties {
    pub fn new(id: crate::types::ComponentId) -> Self {
        Self {
            id,
            visible: true,
            enabled: true,
            bounds: Rect::new(0.0, 0.0, 100.0, 100.0),
        }
    }
}
