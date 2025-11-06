use serde::{Deserialize, Serialize};

/// Specialized events for accessibility features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityEvent {
	/// Event indicating a change in the state of an accessible element.
	StateChanged { name: String },
	/// Event indicating a new announcement for accessibility.
	Announcement { message: String },
}
