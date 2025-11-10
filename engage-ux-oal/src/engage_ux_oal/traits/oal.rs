//! Top-level OAL trait.

use crate::OalError;

/// Top-level platform handle trait. Platform crates should provide a concrete type and constructors.
pub trait PlatformTrait: Send + Sync {
	/// Queue a task to be run on the main thread. The provided closure returns a Result
	/// which will be delivered to the caller if they chose to wait via the handle variant.
	fn queue_main_thread(
		&self,
		task: Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
	) -> Result<(), OalError>;

	/// Variant that returns a tokio oneshot receiver the caller can await. This method
	/// requires `tokio` to be available; the crate uses `tokio::sync::oneshot` for this.
	fn queue_main_thread_with_handle(
		&self,
		task: Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
	) -> Result<tokio::sync::oneshot::Receiver<Result<(), OalError>>, OalError>;
}
