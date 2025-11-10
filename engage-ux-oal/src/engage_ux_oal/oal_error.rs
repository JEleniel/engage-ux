use thiserror::Error;

#[derive(Debug, Error)]
pub enum OalError {
	#[error("Platform not supported")]
	PlatformNotSupported,

	#[error("Operation must be called from the main thread")]
	WrongThread,

	#[error("Resource unavailable: {0}")]
	ResourceUnavailable(String),

	#[error("Invalid handle")]
	InvalidHandle,

	#[error("Unsupported operation")]
	UnsupportedOperation,

	#[error("I/O error: {0:?}")]
	Io(#[from] std::io::Error),

	#[error("Wayland error: {0}")]
	Wayland(String),

	#[error("Other error: {0}")]
	Other(String),
}

impl From<&str> for OalError {
	fn from(s: &str) -> Self {
		OalError::Other(s.to_string())
	}
}
