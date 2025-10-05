//! Layout System Demo
//!
//! Demonstrates the layout system with relative units and positioning.

use engage_ux_core::layout::{Layout, Size, Unit};
use engage_ux_oal::{Monitor, MonitorConfiguration, MonitorLayoutMode};

fn main() {
	println!("=== Engage UX Layout System Demo ===\n");

	// Demo 1: Relative Units
	println!("1. Relative Unit System:");
	println!("   - Pixels: 100px = absolute 100 pixels");
	println!("   - Relative to Base (rb): 2rb = 2 * theme base size");
	println!("   - Relative to Parent (rp): 1.5rp = 1.5 * inherited size");
	println!("   - Percentage (%): 50% = 50% of parent dimension\n");

	// Example conversions
	let parent_width = 800.0;
	let base_size = 16.0;
	let inherited_size = 20.0;

	let pixel_unit = Unit::pixels(100.0);
	let rb_unit = Unit::rb(2.0);
	let rp_unit = Unit::rp(1.5);
	let percent_unit = Unit::percent(50.0);

	println!("   Example conversions (parent=800px, base=16px, inherited=20px):");
	println!(
		"   - 100px     → {}px",
		pixel_unit.to_pixels(parent_width, base_size, inherited_size)
	);
	println!(
		"   - 2rb       → {}px (2 × 16)",
		rb_unit.to_pixels(parent_width, base_size, inherited_size)
	);
	println!(
		"   - 1.5rp     → {}px (1.5 × 20)",
		rp_unit.to_pixels(parent_width, base_size, inherited_size)
	);
	println!(
		"   - 50%       → {}px (50% of 800)",
		percent_unit.to_pixels(parent_width, base_size, inherited_size)
	);
	println!();

	// Demo 2: Layout System
	println!("2. Layout System:");

	// Fixed size layout
	let fixed_layout = Layout::new()
		.with_left(Unit::pixels(10.0))
		.with_top(Unit::pixels(20.0))
		.with_width(Size::Fixed(Unit::pixels(200.0)))
		.with_height(Size::Fixed(Unit::pixels(100.0)));

	println!("   Fixed Size Layout:");
	println!("   - Position: left=10px, top=20px");
	println!("   - Size: width=200px, height=100px");

	let bounds = fixed_layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);
	println!(
		"   → Calculated: x={}, y={}, w={}, h={}\n",
		bounds.x, bounds.y, bounds.width, bounds.height
	);

	// Percentage-based layout
	let percent_layout = Layout::new()
		.with_left(Unit::percent(10.0))
		.with_top(Unit::percent(10.0))
		.with_width(Size::Fixed(Unit::percent(80.0)))
		.with_height(Size::Fixed(Unit::percent(80.0)));

	println!("   Percentage Layout:");
	println!("   - Position: left=10%, top=10%");
	println!("   - Size: width=80%, height=80%");

	let bounds = percent_layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);
	println!(
		"   → Calculated: x={}, y={}, w={}, h={}\n",
		bounds.x, bounds.y, bounds.width, bounds.height
	);

	// Fill layout with constraints
	let constrained_layout = Layout::new()
		.with_width(Size::Fill)
		.with_height(Size::Fill)
		.with_min_width(Unit::pixels(200.0))
		.with_max_width(Unit::pixels(600.0))
		.with_min_height(Unit::pixels(150.0))
		.with_max_height(Unit::pixels(400.0));

	println!("   Fill Layout with Constraints:");
	println!("   - Size: Fill parent");
	println!("   - Constraints: width 200-600px, height 150-400px");

	let bounds = constrained_layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);
	println!("   → Calculated: w={}, h={}\n", bounds.width, bounds.height);

	// Edge-based sizing
	let edge_layout = Layout::new()
		.with_left(Unit::pixels(20.0))
		.with_right(Unit::pixels(20.0))
		.with_top(Unit::pixels(30.0))
		.with_bottom(Unit::pixels(30.0));

	println!("   Edge-Based Layout:");
	println!("   - left=20px, right=20px, top=30px, bottom=30px");

	let bounds = edge_layout.calculate_bounds(800.0, 600.0, 16.0, 16.0);
	println!(
		"   → Calculated: x={}, y={}, w={}, h={}\n",
		bounds.x, bounds.y, bounds.width, bounds.height
	);

	// Demo 3: Multi-Monitor Configuration
	println!("3. Multi-Monitor Configuration:");

	let mut config = MonitorConfiguration::new(MonitorLayoutMode::Unified);

	// Add primary monitor
	let primary = Monitor::new(1, "Primary Display".to_string(), (2560, 1440))
		.with_position(0, 0)
		.with_scale_factor(1.5)
		.as_primary()
		.with_refresh_rate(144);
	config.add_monitor(primary);

	// Add secondary monitor
	let secondary = Monitor::new(2, "Secondary Display".to_string(), (1920, 1080))
		.with_position(2560, 0)
		.with_scale_factor(1.0)
		.with_refresh_rate(60);
	config.add_monitor(secondary);

	println!("   Configuration:");
	println!("   - Mode: {:?}", config.layout_mode);
	println!("   - Monitors: {}", config.monitors.len());
	println!();

	for monitor in &config.monitors {
		println!("   Monitor {}:", monitor.id);
		println!("     Name: {}", monitor.name);
		println!(
			"     Resolution: {}x{}",
			monitor.resolution.0, monitor.resolution.1
		);
		println!(
			"     Position: ({}, {})",
			monitor.position.0, monitor.position.1
		);
		println!("     Scale: {}x", monitor.scale_factor);
		println!("     Refresh: {}Hz", monitor.refresh_rate.unwrap_or(0));
		println!(
			"     Primary: {}",
			if monitor.is_primary { "Yes" } else { "No" }
		);
		println!();
	}

	// Virtual bounds for unified mode
	if let Some(virtual_bounds) = config.virtual_bounds() {
		println!("   Virtual Screen (Unified Mode):");
		println!(
			"   - Total Size: {}x{}",
			virtual_bounds.width, virtual_bounds.height
		);
		println!(
			"   - Position: ({}, {})\n",
			virtual_bounds.x, virtual_bounds.y
		);
	}

	// Test point location
	let test_point = (3000, 500);
	if let Some(monitor) = config.monitor_at_point(test_point.0, test_point.1) {
		println!(
			"   Point ({}, {}) is on: {}",
			test_point.0, test_point.1, monitor.name
		);
	}

	println!("\n=== Demo Complete ===");
	println!("All features demonstrated successfully!");
}
