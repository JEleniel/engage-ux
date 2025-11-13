use crate::{DeviceMetrics, Unit};

#[test]
fn metric_converts_using_dpcm() {
	// 1U (1cm) should map to dpcm pixels
	let metrics = DeviceMetrics::new(40.0, 40.0 * 2.54_f32, 1.0);
	let px = Unit::Metric.to_px(1.0, &metrics);
	assert_eq!(px, 40.0);
}

#[test]
fn pixel_is_one_to_one_when_dpr_is_one() {
	let metrics = DeviceMetrics::new(1.0, 1.0 * 2.54_f32, 1.0);
	assert_eq!(Unit::Pixel.to_px(3.0, &metrics), 3.0);
}

#[test]
fn custom_uses_value() {
	let metrics = DeviceMetrics::new(1.0, 1.0 * 2.54_f32, 1.0);
	assert_eq!(Unit::Custom(10.0).to_px(2.0, &metrics), 20.0);
}

#[test]
fn point_uses_dpi_div_72() {
	// construct metrics with dpi=72 so 1pt -> 1px
	let metrics = DeviceMetrics::new(28.3464567_f32, 72.0_f32, 1.0);
	assert_eq!(Unit::Point.to_px(1.0, &metrics), 1.0);
}

#[test]
fn dpr_scales_pixels() {
	// With DPR=2.0, pixel values should double
	let metrics = DeviceMetrics::new(10.0, 25.4_f32, 2.0);
	// Metric uses dpcm (10), times dpr 2.0 => 20
	assert_eq!(Unit::Metric.to_px(1.0, &metrics), 20.0);
}

#[test]
fn zero_and_negative_units() {
	let metrics = DeviceMetrics::new(10.0, 25.4_f32, 1.0);
	assert_eq!(Unit::Metric.to_px(0.0, &metrics), 0.0);
	// Negative units are allowed mathematically; ensure sign is preserved
	assert_eq!(Unit::Metric.to_px(-2.0, &metrics), -20.0);
}

#[test]
fn round_trip_px_to_units() {
	let metrics = DeviceMetrics::new(10.0, 254.0_f32, 1.0);
	let units = 3.5_f32;
	let px = Unit::Imperial.to_px(units, &metrics);
	let back = Unit::Imperial.px_to_units(px, &metrics);
	// Allow a tiny epsilon for floating point round-trip
	let diff = (units - back).abs();
	assert!(diff < 1e-6, "round-trip diff {}", diff);
}
