use serde::{Deserialize, Serialize};

use crate::geometry::Point;

/// Represents a touch event with relevant data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchEvent {
	/// The action associated with the touch event.
	pub action: TouchAction,
	/// The position of the touch event.
	pub position: Point,
	/// The pressure applied during the touch event.
	pub pressure: f32,
}

/// Enum representing the type of touch action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TouchAction {
	/// The touch action has started.
	Start,
	/// The touch action is moving.
	Move,
	/// The touch action has ended.
	End,
	/// The touch action has been canceled.
	Cancel,
}
