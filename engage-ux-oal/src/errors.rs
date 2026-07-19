use thiserror::Error;

/// Errors returned by the OAL crate.
///
/// This enum represents the small set of failure modes the OAL currently
/// exposes to consumers. Variant payloads provide a short diagnostic string.
#[derive(Debug, Error)]
pub enum OalError {
	/// Initialization failed (e.g., creating platform resources).
	#[error("Initialization failed: {0}")]
	Initialization(String),

	/// Window related error (creation, teardown, or invalid operation).
	#[error("Window error: {0}")]
	Window(String),

	/// Renderer error (failure during rendering or presentation).
	#[error("Renderer error: {0}")]
	Renderer(String),

	/// Errors originating from the `winit` backend. This variant is only
	/// available when the `native-winit` feature is enabled and wraps the
	/// concrete `winit` error type.
	#[cfg(feature = "native-winit")]
	#[cfg_attr(feature = "native-winit", error("Winit error: {0}"))]
	#[cfg(feature = "native-winit")]
	Winit(#[from] winit::error::OsError),

	/// A mutex was poisoned. This indicates a previous panic while holding
	/// the lock. The inner message provides context about which lock was
	/// poisoned.
	#[error("Mutex poisoned: {0}")]
	PoisonedLock(String),
}

/// Result type alias specialized to `OalError` for convenience.
pub type Result<T> = std::result::Result<T, OalError>;
