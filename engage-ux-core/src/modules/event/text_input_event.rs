use serde::{Deserialize, Serialize};

/// Events related to text input, including IME composition events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextInputEvent {
	/// The composition of text has started.
	CompositionStart,
	/// The composition of text has been updated.
	CompositionUpdate(String),
	/// The composition of text has ended.
	CompositionEnd,
	/// Text has been committed.
	Committed(String),
}
