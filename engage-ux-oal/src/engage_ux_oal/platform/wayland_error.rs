use crate::OalError;

// Convert a Wayland connect error into the crate's OalError. We don't introduce
// new platform-specific error types here; map to a textual `Other` variant so
// callers receive a helpful message.
impl From<wayland_client::ConnectError> for OalError {
	fn from(e: wayland_client::ConnectError) -> Self {
		OalError::Wayland(format!("wayland connect failed: {:?}", e))
	}
}

impl From<wayland_client::Error> for OalError {
	fn from(e: wayland_client::Error) -> Self {
		OalError::Wayland(format!("{:?}", e))
	}
}
