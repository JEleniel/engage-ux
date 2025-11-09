/// Device-space rectangle (device pixels). Integer coordinates to match OS surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceRect {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

impl DeviceRect {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}
}

/// Device size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceSize {
	pub width: u32,
	pub height: u32,
}

impl DeviceSize {
	pub fn new(width: u32, height: u32) -> Self {
		Self { width, height }
	}
}

/// Minimal accessibility event payload used by the OAL bridge. This mirrors the
/// platform-agnostic accessibility events in `engage-ux-core` and is intentionally
/// shallow so the OAL implementation can map to OS-specific accessibility APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilityPayload {
	StateChanged { name: String },
	Announcement { message: String },
}
