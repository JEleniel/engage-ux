use serde::{Deserialize, Serialize};

use crate::event::{
	AccessibilityEvent, ClipboardEvent, DragDropEvent, FocusEvent, GestureEvent, KeyboardEvent,
	MediaEvent, PointerEvent, SystemEvent, TextInputEvent, TouchEvent, ValueChangedEvent,
	WheelEvent, WindowEvent,
};

/// The different types of events that can occur in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
	/// Pointer events such as mouse, touch, or pen input.
	Pointer {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: PointerEvent,
	},
	/// Keyboard events such as key presses and releases.
	Keyboard {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: KeyboardEvent,
	},
	/// Wheel events such as mouse wheel scrolling.
	Wheel {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: WheelEvent,
	},
	/// Touch events such as touch screen interactions.
	Touch {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: TouchEvent,
	},
	/// Gesture events such as pinch, swipe, or rotate.
	Gesture {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: GestureEvent,
	},
	/// Focus events such as element focus and blur.
	Focus {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: FocusEvent,
	},
	/// Text input events such as character input and composition.
	TextInput {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: TextInputEvent,
	},
	/// Value changed events such as input value changes.
	ValueChanged {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: ValueChangedEvent,
	},
	/// Clipboard events such as copy, cut, and paste.
	Clipboard {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: ClipboardEvent,
	},
	/// Drag and drop events such as drag start, drag end, and drop.
	DragDrop {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: DragDropEvent,
	},
	/// Window events such as window creation, destruction, and focus changes.
	Window {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: WindowEvent,
	},
	/// Media events such as play, pause, and volume changes.
	Media {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: MediaEvent,
	},
	/// System events such as system notifications and alerts.
	System {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: SystemEvent,
	},
	/// Accessibility events such as screen reader interactions and keyboard navigation.
	Accessibility {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: AccessibilityEvent,
	},
	/// Custom events with arbitrary JSON payloads.
	Custom {
		source_component_id: u128,
		timestamp: chrono::DateTime<chrono::Utc>,
		payload: String,
	},
}
