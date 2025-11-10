//! Concrete OAL wrapper that selects and instantiates a platform runtime.
//!
//! This module provides a concrete `Oal` struct that an embedding application
//! constructs. Internally it detects the available platform (feature-gated)
//! and creates the appropriate runtime/handle. The public API surface is
//! exposed as methods on `Oal` and delegates to the platform implementation.

use crate::OalError;
use std::time::Duration;

use super::platform::wayland::{WaylandHandle, WaylandRuntime};
use crate::engage_ux_oal::traits::oal::PlatformTrait;

/// Concrete OAL instance owned by the embedding application.
pub struct Oal {
	inner: OalInner,
}

enum OalInner {
	Wayland {
		runtime: WaylandRuntime,
		handle: WaylandHandle,
	},
	Unsupported,
}

impl Oal {
	/// Create a new OAL instance for the host platform.
	///
	/// On supported platforms this will instantiate the platform runtime and
	/// return an `Oal` that exposes the platform API. When no supported
	/// platform is available `PlatformNotSupported` is returned.
	pub fn new() -> Result<Self, OalError> {
		// Try Wayland backend first. If initialization fails due to the
		// platform being unsupported, return an `Oal` instance with the
		// `Unsupported` inner so callers may still construct `Oal` and the
		// public API will return `PlatformNotSupported` for platform ops.
		match WaylandRuntime::new() {
			Ok((runtime, handle)) => Ok(Self {
				inner: OalInner::Wayland { runtime, handle },
			}),
			Err(e) => match e {
				OalError::PlatformNotSupported => Ok(Self {
					inner: OalInner::Unsupported,
				}),
				other => Err(other),
			},
		}
	}

	/// Gracefully shutdown the underlying platform runtime, if any.
	///
	/// For Wayland this will ask the runtime to shutdown via the shared
	/// `WaylandHandle::shutdown()` call (invoked on a background thread) while
	/// the owning thread processes runtime messages until the shutdown completes.
	pub fn shutdown(&mut self) -> Result<(), OalError> {
		match &mut self.inner {
			OalInner::Wayland { runtime, handle } => {
				// Call shutdown on the handle from a background thread so the
				// oneshot receiver it waits on can be fulfilled by this thread
				// processing runtime messages. Use a shared AtomicBool to signal
				// completion without accessing the runtime's private globals.
				let h = handle.clone();
				let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
				let done_clone = done.clone();
				let join = std::thread::spawn(move || {
					let _ = h.shutdown();
					done_clone.store(true, std::sync::atomic::Ordering::SeqCst);
				});

				// Process runtime tasks until the background shutdown caller finishes.
				while !done.load(std::sync::atomic::Ordering::SeqCst) {
					runtime.process_main_thread_tasks(std::time::Duration::from_millis(10))?;
				}

				// join the background shutdown caller
				let _ = join.join();
				// Mark this Oal as unsupported now the runtime is down.
				self.inner = OalInner::Unsupported;
				Ok(())
			}
			OalInner::Unsupported => Ok(()),
		}
	}

	/// Queue a task to run on the main thread. This delegates to the
	/// platform runtime/handle.
	pub fn queue_main_thread(
		&self,
		_task: Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
	) -> Result<(), OalError> {
		match &self.inner {
			OalInner::Wayland { handle, .. } => handle.queue_main_thread(_task),
			_ => Err(OalError::PlatformNotSupported),
		}
	}

	/// Queue a main-thread task and receive a oneshot responder you can await.
	pub fn queue_main_thread_with_handle(
		&self,
		_task: Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
	) -> Result<tokio::sync::oneshot::Receiver<Result<(), OalError>>, OalError> {
		match &self.inner {
			OalInner::Wayland { handle, .. } => handle.queue_main_thread_with_handle(_task),
			_ => Err(OalError::PlatformNotSupported),
		}
	}

	/// Process main-thread tasks and platform event queue. This must be called
	/// from the platform main thread (embedding app responsibility). The
	/// semantics and required frequency are platform-dependent.
	pub fn process_main_thread_tasks(&mut self, _budget: Duration) -> Result<(), OalError> {
		match &mut self.inner {
			OalInner::Wayland { runtime, .. } => runtime
				.process_main_thread_tasks(_budget)
				.map_err(|e| OalError::Other(format!("wayland runtime error: {:?}", e))),
			_ => Err(OalError::PlatformNotSupported),
		}
	}

	/// Create a persistent surface (platform dependent). Returns an opaque id
	/// that can be used with the platform's present APIs if supported.
	pub fn create_surface(&self, _width: u32, _height: u32) -> Result<u64, OalError> {
		match &self.inner {
			OalInner::Wayland { handle, .. } => handle.create_surface(_width, _height),
			_ => Err(OalError::PlatformNotSupported),
		}
	}

	/// Destroy a previously created persistent surface id.
	pub fn destroy_surface(&self, _id: u64) -> Result<(), OalError> {
		match &self.inner {
			OalInner::Wayland { handle, .. } => handle.destroy_surface(_id),
			_ => Err(OalError::PlatformNotSupported),
		}
	}

	/// Present a frame to a registered surface id (fire-and-forget).
	pub fn present_to_surface(
		&self,
		_id: u64,
		frame: crate::engage_ux_oal::traits::Frame,
		dirty: Vec<crate::DeviceRect>,
	) -> Result<(), OalError> {
		match &self.inner {
			OalInner::Wayland { handle, .. } => handle.present_to_surface(_id, frame, dirty),
			_ => Err(OalError::PlatformNotSupported),
		}
	}
}
