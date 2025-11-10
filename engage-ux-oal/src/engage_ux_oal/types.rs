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

/// Logical units used by the embedding application. The OAL exposes sizes and
/// coordinates in `Units` so embedder code does not need to reason about pixels.
/// By default 1 Unit (1U) == 1cm; the runtime scales Units -> device pixels
/// using the configured `UnitScale` which typically derives from the display's
/// dots-per-centimeter (DPCm).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitSize {
	pub width: i32,
	pub height: i32,
}

impl UnitSize {
	pub fn new(width: i32, height: i32) -> Self {
		Self { width, height }
	}
}

/// Rectangle expressed in Units. Coordinates and sizes are i32 to allow both
/// positive and negative canvas coordinates as required by the Canvas model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitRect {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

impl UnitRect {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}
}

/// Scale used to convert between Units and device pixels. `pixels_per_unit`
/// expresses how many device pixels represent 1U. Default assumes 1U == 1cm
/// and a reasonable default DPCm (dots per cm).
#[derive(Debug, Clone, Copy)]
pub struct UnitScale {
	pub pixels_per_unit: f32,
}

impl UnitScale {
	/// Default DPCm approximation (approx 96 DPI / 2.54cm per inch).
	pub fn default() -> Self {
		Self {
			pixels_per_unit: 96.0 / 2.54,
		}
	}

	pub fn from_dpcm(dpcm: f32) -> Self {
		Self {
			pixels_per_unit: dpcm,
		}
	}

	pub fn units_to_device_size(&self, u: UnitSize) -> DeviceSize {
		let w = (u.width as f32 * self.pixels_per_unit).round();
		let h = (u.height as f32 * self.pixels_per_unit).round();
		DeviceSize::new(w.max(0.0) as u32, h.max(0.0) as u32)
	}

	pub fn device_to_unit_size(&self, d: DeviceSize) -> UnitSize {
		let w = (d.width as f32 / self.pixels_per_unit).round() as i32;
		let h = (d.height as f32 / self.pixels_per_unit).round() as i32;
		UnitSize::new(w, h)
	}

	pub fn units_to_device_rect(&self, r: UnitRect) -> DeviceRect {
		let x = (r.x as f32 * self.pixels_per_unit).round() as i32;
		let y = (r.y as f32 * self.pixels_per_unit).round() as i32;
		let w = (r.width as f32 * self.pixels_per_unit).round() as i32;
		let h = (r.height as f32 * self.pixels_per_unit).round() as i32;
		DeviceRect::new(x, y, w, h)
	}

	pub fn device_to_unit_rect(&self, r: DeviceRect) -> UnitRect {
		let x = (r.x as f32 / self.pixels_per_unit).round() as i32;
		let y = (r.y as f32 / self.pixels_per_unit).round() as i32;
		let w = (r.width as f32 / self.pixels_per_unit).round() as i32;
		let h = (r.height as f32 / self.pixels_per_unit).round() as i32;
		UnitRect::new(x, y, w, h)
	}
}

/// A Canvas is a logical drawing surface expressed in Units. Coordinates may be
/// positive or negative (i32 range). The Canvas describes the valid coordinate
/// bounds which viewports (windows) can move around on.
#[derive(Debug, Clone, Copy)]
pub struct Canvas {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

impl Canvas {
	pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
		Self {
			left,
			top,
			right,
			bottom,
		}
	}

	pub fn width(&self) -> i32 {
		self.right - self.left
	}
	pub fn height(&self) -> i32 {
		self.bottom - self.top
	}
}

/// A movable view (window) onto a Canvas. `offset_x`/`offset_y` define the
/// canvas coordinate that corresponds to the top-left of the window. Right and
/// down are positive.
#[derive(Debug, Clone, Copy)]
pub struct View {
	pub offset_x: i32,
	pub offset_y: i32,
	pub size: UnitSize,
}

impl View {
	pub fn new(offset_x: i32, offset_y: i32, size: UnitSize) -> Self {
		Self {
			offset_x,
			offset_y,
			size,
		}
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
