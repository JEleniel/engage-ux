//! Component property helpers
//!
//! Types used by UI components to hold small, portable sets of common
//! properties (identifier, visibility and enabled state). These types are
//! intentionally lightweight and serializable for use across backends and
//! tests.

use crate::component::ComponentId;
use serde::{Deserialize, Serialize};

/// Properties common to all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties {
	/// Stable unique identifier for the component
	pub id: ComponentId,
	/// Whether the component is visible
	pub visible: bool,
	/// Whether the component is enabled (accepts input)
	pub enabled: bool,
}

impl ComponentProperties {
	/// Create a new `ComponentProperties` with sensible defaults.
	///
	/// Defaults: `visible = true`, `enabled = true`.
	pub fn new(id: ComponentId) -> Self {
		Self {
			id,
			visible: true,
			enabled: true,
		}
	}
}
