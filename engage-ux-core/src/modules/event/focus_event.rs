use serde::{Deserialize, Serialize};

/// Represents a focus event within the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FocusEvent {
	/// Event indicating that a component has gained focus.
	Gained { component_id: u128 },
	/// Event indicating that a component has lost focus.
	Lost { component_id: u128 },
}
