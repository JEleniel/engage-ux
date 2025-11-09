#[cfg(feature = "wayland")]
use std::fmt;
#[cfg(feature = "wayland")]
use thiserror::Error;

/// The unified OAL error type.
#[cfg(feature = "wayland")]
#[derive(Debug, Error)]
pub enum OalError {
	#[error("platform not supported")]
	PlatformNotSupported,
	#[error("Operation must be called frtom the main thread")]
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
	Wayland(#[from] wayland_client::Error),

	#[error("other: {0}")]
	Other(String),
}

impl From<&str> for OalError {
	fn from(s: &str) -> Self {
		OalError::Other(s.to_string())
	}
}

impl From<String> for OalError {
	fn from(s: String) -> Self {
		OalError::Other(s)
	}
}

// Platform-specific conversions. Keep these behind feature gates so the crate
// remains lightweight when platform backends are not enabled.
#[cfg(feature = "wayland")]
impl From<wayland_client::ConnectError> for OalError {
	fn from(e: wayland_client::ConnectError) -> Self {
		OalError::Platform(PlatformError(format!("wayland connect failed: {:?}", e)))
	}
}
