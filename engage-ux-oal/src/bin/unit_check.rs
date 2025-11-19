fn main() {
    use engage_ux_oal::oal::{DeviceMetrics, Unit};

    // Metric conversion
    let metrics = DeviceMetrics::new(40.0 * 2.54_f32, 1.0);
    let px = Unit::Metric.to_px(1.0, &metrics);
    assert_eq!(px, 40.0, "Metric conversion failed: got {}", px);

    // Pixel identity
    let metrics = DeviceMetrics::new(1.0 * 2.54_f32, 1.0);
    assert_eq!(Unit::Pixel.to_px(3.0, &metrics), 3.0);

    // Custom
    let metrics = DeviceMetrics::new(1.0 * 2.54_f32, 1.0);
    assert_eq!(Unit::Custom(10.0).to_px(2.0, &metrics), 20.0);

    // Point
    let metrics = DeviceMetrics::new(72.0_f32, 1.0);
    assert_eq!(Unit::Point.to_px(1.0, &metrics), 1.0);

    // DPR scaling
    let metrics = DeviceMetrics::new(25.4_f32, 2.0);
    assert_eq!(Unit::Metric.to_px(1.0, &metrics), 20.0);

    // Round-trip
    let metrics = DeviceMetrics::new(254.0_f32, 1.0);
    let units = 3.5_f32;
    let px = Unit::Imperial.to_px(units, &metrics);
    let back = Unit::Imperial.px_to_units(px, &metrics);
    let diff = (units - back).abs();
    assert!(diff < 1e-6, "round-trip diff {}", diff);

    println!("engage-ux-oal unit checks passed");
}
