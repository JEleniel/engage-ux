use crate::{event::keyboard_event::KeyboardModifierKeys, geometry::Offset};
use serde::{Deserialize, Serialize};

/// Represents a mouse wheel event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelEvent {
	/// The amount of scroll in logical pixels.
	pub delta: Offset,
	/// The keyboard modifier keys that were active during the event.
	pub modifiers: KeyboardModifierKeys,
}
