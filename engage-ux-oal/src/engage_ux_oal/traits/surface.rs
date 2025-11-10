//! Surface/back-buffer traits and builder.

use crate::{
	OalError,
	engage_ux_oal::types::{DeviceRect, DeviceSize},
};

use super::Frame;

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

	// Build a platform surface. Platform crates will provide a concrete implementation.
	//
	// The platform-specific `build` implementation is provided by the backend
	// (for example the Wayland backend) and will be available unconditionally
	// after feature-gate removal.
}
