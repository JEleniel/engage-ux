use serde::{Deserialize, Serialize};

/// Event emitted when a value has changed.
/// The value is a JSON encoded object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueChangedEvent {
	/// Name of the field that changed.
	pub field: String,
	/// New value of the field, encoded as a JSON string.
	pub value: String,
}
