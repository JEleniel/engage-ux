use serde::{Deserialize, Serialize};

use crate::geometry::Point;

/// Represents a drag and drop event within the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragDropEvent {
	/// The action associated with the drag and drop event.
	pub action: DragAction,
	/// The position where the event occurred.
	pub position: Point,
	/// The data being dragged, if any.
	pub data: Option<Vec<u8>>,
}

/// The type of drag and drop event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragAction {
	/// Event indicating the start of a drag operation.
	Start,
	/// Event indicating a drag operation is in progress.
	Move,
	/// Event indicating the completion of a drag operation.
	Drop,
	/// Event indicating the cancellation of a drag operation.
	Cancel,
}
