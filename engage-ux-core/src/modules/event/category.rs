use serde::{Deserialize, Serialize};

/// High-level categories for events. These are used to allow filtered
/// subscriptions to the event bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
	Mouse,
	Keyboard,
	Text,
	Focus,
	Interaction,
	Window,
	Custom,
}

impl std::fmt::Display for EventCategory {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			EventCategory::Mouse => "mouse",
			EventCategory::Keyboard => "keyboard",
			EventCategory::Text => "text",
			EventCategory::Focus => "focus",
			EventCategory::Interaction => "interaction",
			EventCategory::Window => "window",
			EventCategory::Custom => "custom",
		};
		write!(f, "{}", s)
	}
}
