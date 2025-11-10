//! Window-related traits and builder.

use crate::{
	OalError,
	engage_ux_oal::types::{DeviceRect, DeviceSize, UnitScale, UnitSize, View},
};
use engage_ux_core::event::EventBus;

use super::Frame;

/// Windowing surface / top-level window abstraction.
pub trait Window {
	/// Present a frame (delegates to an underlying Surface).
	fn present(&self, frame: Frame, dirty: &[DeviceRect]) -> Result<(), OalError>;

	/// Poll platform events. The embedder should call this from the main thread.
	fn poll_events(&self) -> Result<(), OalError>;

	/// Process main-thread-only tasks queued via `queue_main_thread`.
	fn process_main_thread_tasks(&self) -> Result<(), OalError>;

	/// Set the window title (top-level windows only).
	fn set_title(&self, title: &str) -> Result<(), OalError>;

	/// Enable or disable OS decorations for the window.
	fn set_decorations(&self, decorated: bool) -> Result<(), OalError>;

	/// Resize the window to the provided device size.
	fn set_size(&self, size: DeviceSize) -> Result<(), OalError>;

	/// Get the current window size in device pixels. Implementations that
	/// maintain a cached size should return it here. Default is Unsupported.
	fn size(&self) -> Result<DeviceSize, OalError> {
		Err(OalError::UnsupportedOperation)
	}

	/// Set the window size in logical Units. This default implementation
	/// converts Units -> device pixels using the provided UnitScale and calls
	/// `set_size`. Implementations may override for optimized handling.
	fn set_size_units(&self, size: UnitSize, scale: UnitScale) -> Result<(), OalError> {
		let ds = scale.units_to_device_size(size);
		self.set_size(ds)
	}

	/// Query the window size expressed in Units using the given scale.
	fn size_units(&self, scale: UnitScale) -> Result<UnitSize, OalError> {
		let ds = self.size()?;
		Ok(scale.device_to_unit_size(ds))
	}

	/// Show or hide the window.
	fn set_visible(&self, visible: bool) -> Result<(), OalError>;

	/// Minimize or restore the window.
	fn set_minimized(&self, minimized: bool) -> Result<(), OalError>;

	/// Maximize or restore the window.
	fn set_maximized(&self, maximized: bool) -> Result<(), OalError>;

	/// Request that the window be closed by the platform/compositor.
	fn close(&self) -> Result<(), OalError>;

	/// Attach an `EventBus` so platform input events (pointer/keyboard/touch)
	/// are published to the provided bus. This allows the core renderer and
	/// input modules to subscribe to window-local events.
	fn attach_event_bus(&self, bus: EventBus) -> Result<(), OalError>;

	/// Set the movable view (viewport) on the window's Canvas. By default
	/// this operation is unsupported; platform implementations should store a
	/// `View` and apply conversions when presenting frames.
	fn set_view(&self, _view: View) -> Result<(), OalError> {
		Err(OalError::UnsupportedOperation)
	}

	/// Get the current view (viewport) onto the canvas. Default returns Unsupported.
	fn get_view(&self) -> Result<View, OalError> {
		Err(OalError::UnsupportedOperation)
	}

	/// Move the view by units (dx, dy). Right/down are positive.
	fn move_view_by(&self, _dx: i32, _dy: i32) -> Result<(), OalError> {
		Err(OalError::UnsupportedOperation)
	}
}

/// Builder used to create a top-level `Window` (for platforms that support
/// native toplevels/decorations). By default the `build()` method returns
/// `PlatformNotSupported` unless the target platform provides a concrete
/// implementation behind a feature flag (for example `wayland`).
pub struct WindowBuilder {
	pub width: u32,
	pub height: u32,
	pub title: Option<String>,
	pub decorated: bool,
	pub resizable: bool,
	pub app_id: Option<String>,
}

impl Default for WindowBuilder {
	fn default() -> Self {
		Self {
			width: 800,
			height: 600,
			title: None,
			decorated: true,
			resizable: true,
			app_id: None,
		}
	}
}

impl WindowBuilder {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			..Default::default()
		}
	}

	pub fn title(mut self, t: impl Into<String>) -> Self {
		self.title = Some(t.into());
		self
	}

	pub fn decorated(mut self, v: bool) -> Self {
		self.decorated = v;
		self
	}

	pub fn resizable(mut self, v: bool) -> Self {
		self.resizable = v;
		self
	}

	pub fn app_id(mut self, id: impl Into<String>) -> Self {
		self.app_id = Some(id.into());
		self
	}

	/// Build a platform `Window`. Platform crates should provide a concrete
	/// implementation when the relevant feature is enabled.
	#[cfg(not(feature = "wayland"))]
	pub fn build(self) -> Result<Box<dyn Window>, OalError> {
		Err(OalError::PlatformNotSupported)
	}
}
