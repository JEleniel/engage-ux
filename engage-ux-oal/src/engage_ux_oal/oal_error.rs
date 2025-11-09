use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum OalError {
	#[cfg(feature = "wayland")]
	#[error("A Wayland error occurred: {0:?}")]
	WaylandError(#[from] wayland_client::Error),
	#[error("Platform not supported")]
	PlatformNotSupported,
	#[error("Other error: {0}")]
	Other(String),
}

impl From<&str> for OalError {
	fn from(s: &str) -> Self {
		OalError::Other(s.to_string())
	}
}
