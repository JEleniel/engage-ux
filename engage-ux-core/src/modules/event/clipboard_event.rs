use serde::{Deserialize, Serialize};

/// Represents clipboard-related events such as copy, cut, and paste.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardEvent {
	/// Event indicating a copy action.
	Copy { data: Vec<u8> },
	/// Event indicating a cut action.
	Cut { data: Vec<u8> },
	/// Event indicating a paste action.
	Paste { data: Vec<u8> },
}
