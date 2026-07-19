use crate::oal::device_metrics::DeviceMetrics;

/// Units available for the OAL. These map logical "Units (U)" into pixels
/// using the device metrics supplied at runtime.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
	/// Metric: 1U = 1cm
	Metric,
	/// Imperial: 1U = 1in
	Imperial,
	/// Point: 1U = 1pt = 1/72in
	Point,
	/// Pixel: 1U = 1px
	Pixel,
	/// Custom pixels per unit (non-integer allowed)
	Custom(f32),
}

impl Unit {
	/// Convert units (U) to pixels using the provided device metrics.
	/// Returns a floating point pixel value; callers may round if they
	/// require integer pixels.
	pub fn to_px(&self, units: f32, metrics: &DeviceMetrics) -> f32 {
		// Derive DPCM on demand from DPI to keep DeviceMetrics minimal.
		let dpcm = metrics.dpi / 2.54_f32;
		let px_per_u = match self {
			Unit::Metric => dpcm,
			Unit::Imperial => metrics.dpi,
			Unit::Point => metrics.dpi / 72.0_f32,
			Unit::Pixel => 1.0_f32,
			Unit::Custom(p) => *p,
		};

		units * px_per_u * metrics.dpr
	}

	/// Convert pixel values into logical units using device metrics.
	pub fn px_to_units(&self, px: f32, metrics: &DeviceMetrics) -> f32 {
		let px = px / metrics.dpr;
		let dpcm = metrics.dpi / 2.54_f32;

		let px_per_u = match self {
			Unit::Metric => dpcm,
			Unit::Imperial => metrics.dpi,
			Unit::Point => metrics.dpi / 72.0_f32,
			Unit::Pixel => 1.0_f32,
			Unit::Custom(p) => *p,
		};

		px / px_per_u
	}
}

// Public docs: `Unit` maps logical units to pixels. Metric uses DPCM, Imperial
// uses DPI, Point uses DPI/72, Pixel is 1:1, and Custom accepts an explicit
// px-per-unit floating value.
