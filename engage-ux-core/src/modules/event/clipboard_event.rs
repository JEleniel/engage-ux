use serde::{Deserialize, Serialize};

/// Represents clipboard-related events such as copy, cut, and paste.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardEvent {
	/// Event indicating a copy action.
	Copy {
		/// Bytes copied to the clipboard
		data: Vec<u8>,
	},
	/// Event indicating a cut action.
	Cut {
		/// Bytes cut to the clipboard
		data: Vec<u8>,
	},
	/// Event indicating a paste action.
	Paste {
		/// Bytes pasted from the clipboard
		data: Vec<u8>,
	},
}
