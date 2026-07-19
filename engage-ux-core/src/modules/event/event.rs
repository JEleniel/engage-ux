use serde::{Deserialize, Serialize};

use crate::event::{
	AccessibilityEvent, ClipboardEvent, DragDropEvent, FocusEvent, GestureEvent, KeyboardEvent,
	MediaEvent, PointerEvent, SystemEvent, TextInputEvent, TouchEvent, ValueChangedEvent,
	WheelEvent, WindowEvent,
};

use chrono::{DateTime, Utc};

/// Timestamp alias used for events.
pub type Timestamp = DateTime<Utc>;

/// The different types of events that can occur in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
	/// Pointer events such as mouse, touch, or pen input.
	Pointer {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Pointer event payload
		payload: PointerEvent,
	},
	/// Keyboard events such as key presses and releases.
	Keyboard {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Keyboard event payload
		payload: KeyboardEvent,
	},
	/// Wheel events such as mouse wheel scrolling.
	Wheel {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Wheel event payload
		payload: WheelEvent,
	},
	/// Touch events such as touch screen interactions.
	Touch {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Touch event payload
		payload: TouchEvent,
	},
	/// Gesture events such as pinch, swipe, or rotate.
	Gesture {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Gesture event payload
		payload: GestureEvent,
	},
	/// Focus events such as element focus and blur.
	Focus {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Focus event payload
		payload: FocusEvent,
	},
	/// Text input events such as character input and composition.
	TextInput {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Text input event payload
		payload: TextInputEvent,
	},
	/// Value changed events such as input value changes.
	ValueChanged {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Value changed payload
		payload: ValueChangedEvent,
	},
	/// Clipboard events such as copy, cut, and paste.
	Clipboard {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Clipboard event payload
		payload: ClipboardEvent,
	},
	/// Drag and drop events such as drag start, drag end, and drop.
	DragDrop {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Drag/drop event payload
		payload: DragDropEvent,
	},
	/// Window events such as window creation, destruction, and focus changes.
	Window {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Window event payload
		payload: WindowEvent,
	},
	/// Media events such as play, pause, and volume changes.
	Media {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Media event payload
		payload: MediaEvent,
	},
	/// System events such as system notifications and alerts.
	System {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// System event payload
		payload: SystemEvent,
	},
	/// Accessibility events such as screen reader interactions and keyboard navigation.
	Accessibility {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Accessibility event payload
		payload: AccessibilityEvent,
	},
	/// Custom events with arbitrary JSON payloads.
	Custom {
		/// Component id where the event originated
		source_component_id: crate::component::ComponentId,
		/// Event timestamp (UTC)
		timestamp: Timestamp,
		/// Arbitrary JSON string payload
		payload: String,
	},
}
