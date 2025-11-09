//! OS Abstraction Layer (OAL) traits and a short API contract.
//!
//! Contract (short):
//! - Inputs: callers primarily provide `Frame` payloads and device-space rectangles
//!   (`DeviceRect`) representing damaged regions. Sizes are expressed in `DeviceSize`.
//! - Outputs: platform operations return `Result<_, OalError>`; side-effects include
//!   presenting buffers to the platform compositor and publishing accessibility events.
//! - Error modes: use `OalError` for unified error handling. Implementations must map
//!   platform-specific failures into `OalError::Platform` or other appropriate variants.
//! - Thread affinity: runtime- and windowing-related APIs are main-thread-affine.
//!   Background threads may enqueue present operations via `Surface`/`Window` helpers
//!   provided by platform-specific implementations, but the actual buffer submission
//!   and event polling must occur on the main thread.
//!
//! Minimal acceptance criteria documented here and referenced by unit tests:
//! - `SurfaceBuilder::build()` returns `PlatformNotSupported` when a platform feature
//!   is not enabled.
//! - Implementations expose a `process_main_thread_tasks`/`run_once` API to drain
//!   main-thread work with bounded budgets and support graceful shutdown.
//!
//! Types in this module express the lightweight contract; platform crates should
//! provide concrete implementations and tests that validate the invariants above.
use crate::{
	OalError,
	engage_ux_oal::types::{DeviceRect, DeviceSize},
};
// Arc is intentionally not required by the revised Window API (main-thread-only).

/// A lightweight frame placeholder. Concrete implementations will provide
/// renderer-specific frame payloads (GPU surface, image buffer, etc.).
pub struct Frame {
	/// Pixel buffer in core RGBA8 (premultiplied) layout. Implementations may accept
	/// `None` and pull from a renderer-specific mechanism instead.
	pub rgba_pixels: Option<Vec<u8>>,
	pub size: DeviceSize,
}

/// Surface represents a platform back buffer or render target.
pub trait Surface: Send + Sync {
	/// Present the frame to the platform. `dirty` lists device-space rectangles to update.
	fn present_frame(&self, frame: Frame, dirty: &[DeviceRect]) -> Result<(), OalError>;

	/// Invalidate region(s) so that the next frame includes them.
	fn invalidate_region(&self, rects: &[DeviceRect]) -> Result<(), OalError>;

	/// Query surface size.
	fn size(&self) -> DeviceSize;
}

/// Builder used to create a `Surface` implementation.
pub struct SurfaceBuilder {
	pub width: u32,
	pub height: u32,
	/// None => automatic detection. Some(true/false) forces GPU preference.
	pub prefer_gpu: Option<bool>,
	pub debug: bool,
}

impl Default for SurfaceBuilder {
	fn default() -> Self {
		Self {
			width: 800,
			height: 600,
			prefer_gpu: None,
			debug: false,
		}
	}
}

impl SurfaceBuilder {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			prefer_gpu: None,
			debug: false,
		}
	}

	pub fn prefer_gpu(mut self, v: Option<bool>) -> Self {
		self.prefer_gpu = v;
		self
	}

	pub fn debug(mut self, v: bool) -> Self {
		self.debug = v;
		self
	}

	/// Build a platform surface. Platform crates will provide a concrete implementation.
	#[cfg(not(feature = "wayland"))]
	pub fn build(self) -> Result<Box<dyn Surface>, OalError> {
		Err(OalError::PlatformNotSupported)
	}

	// When the `wayland` feature is enabled a platform-specific implementation
	// will provide a concrete `build` method that returns a Wayland-backed
	// Surface. That impl lives in the platform module to avoid conditional
	// compilation cruft in this file.
}

/// Accessibility bridge trait. Implementations are expected to be main-thread-only.
///
/// The core crate exposes `AccessibilityEvent` as a platform-agnostic event type; the
/// OAL bridge accepts those events and forwards them to the OS-specific accessibility APIs.
pub trait AccessibilityBridge: Send + Sync {
	fn publish_event(
		&self,
		event: crate::engage_ux_oal::types::AccessibilityPayload,
	) -> Result<(), OalError>;
}

/// Windowing surface / top-level window abstraction.
pub trait Window {
	/// Present a frame (delegates to an underlying Surface).
	fn present(&self, frame: Frame, dirty: &[DeviceRect]) -> Result<(), OalError>;

	/// Poll platform events. The embedder should call this from the main thread.
	fn poll_events(&self) -> Result<(), OalError>;

	/// Process main-thread-only tasks queued via `queue_main_thread`.
	fn process_main_thread_tasks(&self) -> Result<(), OalError>;
}

/// Top-level OAL handle. Platform crates should provide a concrete type and constructors.
pub trait Oal: Send + Sync {
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
