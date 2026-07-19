use crate::{event::keyboard_event::KeyboardModifierKeys, geometry::Move};
use serde::{Deserialize, Serialize};

/// Represents a mouse wheel event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelEvent {
	/// The amount of scroll in logical pixels.
	/// Represented as a movement delta (Move) in logical units.
	pub delta: Move,
	/// The keyboard modifier keys that were active during the event.
	pub modifiers: KeyboardModifierKeys,
}
