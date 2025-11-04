use crate::modules::event::category::EventCategory;
use crate::{geometry::Point, input::mouse::MouseButton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
	MouseDown {
		location: Point,
		button: MouseButton,
	},
	MouseUp {
		location: Point,
		button: MouseButton,
	},
	MouseMove {
		location: Point,
	},
	MouseWheel {
		delta_x: f32,
		delta_y: f32,
	},
	KeyDown {
		key_event: keyboard_types::KeyboardEvent,
	},
	KeyUp {
		key_event: keyboard_types::KeyboardEvent,
	},
	TextInput {
		text: String,
	},
	FocusGained,
	FocusLost,
	Click,
	ValueChanged,
	Resize {
		width: f32,
		height: f32,
	},
	Custom {
		name: String,
		data: String,
	},
}

impl EventType {
	/// Return the high-level category for this event type.
	pub fn category(&self) -> EventCategory {
		match self {
			EventType::MouseDown { .. }
			| EventType::MouseUp { .. }
			| EventType::MouseMove { .. }
			| EventType::MouseWheel { .. }
			| EventType::Click => EventCategory::Mouse,

			EventType::KeyDown { .. } | EventType::KeyUp { .. } => EventCategory::Keyboard,

			EventType::TextInput { .. } => EventCategory::Text,

			EventType::FocusGained | EventType::FocusLost => EventCategory::Focus,

			EventType::ValueChanged => EventCategory::Interaction,

			EventType::Resize { .. } => EventCategory::Window,

			EventType::Custom { .. } => EventCategory::Custom,
		}
	}
}
