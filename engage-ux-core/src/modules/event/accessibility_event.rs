use serde::{Deserialize, Serialize};

/// Specialized events for accessibility features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityEvent {
	/// Event indicating a change in the state of an accessible element.
	StateChanged {
		/// Name of the element whose state changed
		name: String,
	},
	/// Event indicating a new announcement for accessibility.
	Announcement {
		/// Announcement message text
		message: String,
	},
}
