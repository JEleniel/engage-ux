//! Frame types for the OAL traits module.
//!
//! Lightweight container used when presenting pixel data to platform surfaces.

use crate::engage_ux_oal::types::DeviceSize;

/// A lightweight frame placeholder. Concrete implementations will provide
/// renderer-specific frame payloads (GPU surface, image buffer, etc.).
pub struct Frame {
	/// Pixel buffer in core RGBA8 (premultiplied) layout. Implementations may accept
	/// `None` and pull from a renderer-specific mechanism instead.
	pub rgba_pixels: Option<Vec<u8>>,
	pub size: DeviceSize,
}
