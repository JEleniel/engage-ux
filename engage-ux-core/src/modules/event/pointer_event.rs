use crate::{
	event::keyboard_event::KeyboardModifierKeys,
	geometry::{Move, Point},
};
use serde::{Deserialize, Serialize};

/// A pointer event, such as from a mouse, touch, or pen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerEvent {
	/// The kind of pointer device.
	pub kind: PointerKind,
	/// The position of the pointer event in local coordinates.
	pub position: Point,
	/// The change in position since the last event.
	/// Replaced the old `Offset` type with `Move`.
	pub delta: Move,
	/// Bitmask of the buttons that are currently pressed.
	pub buttons: u32,
	/// The modifier keys that are currently pressed.
	pub modifiers: KeyboardModifierKeys,
	/// The pressure of the pointer event 0.0-1.0 for pen/touch
	pub pressure: Option<f32>,
	/// The tilt of the pointer event in degrees (x, y) for pen
	pub tilt: Option<(f32, f32)>,
}

/// A trait for types that can produce pointer events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointerKind {
	/// A pointer device that is a mouse.
	Mouse,
	/// A pointer device that is a touch screen.
	Touch,
	/// A pointer device that is a pen.
	Pen,
	/// A pointer device of an unknown type.
	Other,
}
