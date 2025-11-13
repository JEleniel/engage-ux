/// Device metrics describing the physical and pixel density of the device.
///
/// This exposes DPCM (dots per centimeter), DPI (dots per inch), and DPR
/// (device pixel ratio) as floating point values. These are used by the
/// `Unit` conversion helpers to translate logical units into pixels.
#[derive(Debug, Clone)]
pub struct DeviceMetrics {
	/// Dots (pixels) per centimeter.
	pub dpcm: f32,
	/// Dots (pixels) per inch.
	pub dpi: f32,
	/// Device pixel ratio (logical -> physical pixel scaling).
	pub dpr: f32,
}

impl DeviceMetrics {
	/// Create `DeviceMetrics` with explicit dpcm, dpi and dpr values.
	pub fn new(dpcm: f32, dpi: f32, dpr: f32) -> Self {
		Self { dpcm, dpi, dpr }
	}

	/// Convenience constructor from pixels-per-centimeter only. DPR defaults to 1.0.
	pub fn with_dpcm(dpcm: f32) -> Self {
		// derive dpi from dpcm: 1 inch = 2.54 cm
		Self {
			dpcm,
			dpi: dpcm * 2.54_f32,
			dpr: 1.0,
		}
	}

	/// Backwards-compatible constructor used by existing tests/examples.
	/// Interprets the integer as pixels per (logical) unit and maps that
	/// to dpcm. DPR defaults to 1.0.
	pub fn with_pixels(pixels_per_unit: i32) -> Self {
		let dpcm = pixels_per_unit as f32;
		Self {
			dpcm,
			dpi: dpcm * 2.54_f32,
			dpr: 1.0,
		}
	}
}

// Public docs: DeviceMetrics describes device density. Prefer platform
// provided metrics where available (e.g. via the windowing backend). Tests
// can construct simple metrics using `with_dpcm` or `new`.
