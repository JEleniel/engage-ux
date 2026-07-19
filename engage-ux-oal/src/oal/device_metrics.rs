/// Device metrics describing pixel density for the device.
///
/// We store DPI (dots per inch) and DPR (device pixel ratio). DPCM can be
/// derived from DPI when needed via dpi / 2.54.
#[derive(Debug, Clone)]
pub struct DeviceMetrics {
	/// Dots (pixels) per inch.
	pub dpi: f32,
	/// Device pixel ratio (logical -> physical pixel scaling).
	pub dpr: f32,
}

impl DeviceMetrics {
	/// Create `DeviceMetrics` with explicit dpi and dpr values.
	pub fn new(dpi: f32, dpr: f32) -> Self {
		Self { dpi, dpr }
	}

	/// Convenience constructor with DPR defaulting to 1.0.
	pub fn with_dpi(dpi: f32) -> Self {
		Self { dpi, dpr: 1.0 }
	}

	/// Backwards-compatible constructor used by existing tests/examples.
	/// Interprets the integer as pixels per centimeter (pixels per 1U where
	/// 1U == 1cm) and converts it into DPI. DPR defaults to 1.0.
	pub fn with_pixels(pixels_per_unit: i32) -> Self {
		let dpcm = pixels_per_unit as f32;
		let dpi = dpcm * 2.54_f32;
		Self { dpi, dpr: 1.0 }
	}
}

// Public docs: DeviceMetrics describes device density. Prefer platform
// provided metrics where available (e.g. via the windowing backend). Tests
// can construct simple metrics using `with_dpi` or `new`.
