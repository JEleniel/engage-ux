use thiserror::Error;

/// Top-level error type for Engage UX workspace.
#[derive(Debug, Error)]
pub enum EngageError {
	/// Generic error with a message.
	#[error("{0}")]
	Message(String),

	/// Wrap color parsing errors from the color module.
	#[error(transparent)]
	Color(#[from] crate::ColorError),

	/// Wrap JSON/serde errors.
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),

	/// Wrap integer parsing errors.
	#[error(transparent)]
	ParseInt(#[from] std::num::ParseIntError),

	/// IO errors.
	#[error(transparent)]
	Io(#[from] std::io::Error),
}

/// Convenient Result alias for the workspace.
pub type Result<T> = std::result::Result<T, EngageError>;
