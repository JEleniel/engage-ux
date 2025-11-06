use serde::{Deserialize, Serialize};

/// Represents a focus event within the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FocusEvent {
	/// Event indicating that a component has gained focus.
	Gained {
		/// Component id that gained focus
		component_id: u128,
	},
	/// Event indicating that a component has lost focus.
	Lost {
		/// Component id that lost focus
		component_id: u128,
	},
}
