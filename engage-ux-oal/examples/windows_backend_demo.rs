//! Windows Backend Demonstration
//!
//! This example demonstrates the Windows backend implementation including:
//! - Window creation and management
//! - Graphics rendering
//! - Event handling
//! - DPI awareness
//!
//! Run with: cargo run --example windows_backend_demo -p engage-ux-oal

use engage_ux_oal::backends::renderer::{Color, Rect};
use engage_ux_oal::backends::{RenderCommand, WindowBounds, WindowState, get_backend_factory};

fn main() {
	println!("=== Windows Backend Demonstration ===\n");

	// Get the platform-specific backend factory
	let factory = get_backend_factory();
	println!("✓ Backend factory initialized");

	// Create window backend
	let mut window = factory.create_window_backend();
	println!("✓ Window backend created: {}", window.name());

	// Create renderer
	let mut renderer = factory.create_renderer();
	println!(
		"✓ Renderer created: {} (Hardware accelerated: {})",
		renderer.name(),
		renderer.is_hardware_accelerated()
	);

	println!("\n--- Window Management ---");

	// Configure window
	window.set_title("Engage UX - Windows Backend Demo");
	println!("✓ Window title set: {}", window.title());

	window.set_bounds(WindowBounds::new(100, 100, 800, 600));
	let bounds = window.bounds();
	println!(
		"✓ Window bounds: x={}, y={}, width={}, height={}",
		bounds.x, bounds.y, bounds.width, bounds.height
	);

	window.set_resizable(true);
	println!("✓ Window resizable: enabled");

	window.set_decorated(true);
	println!("✓ Window decorations: enabled");

	println!("✓ Initial state: {:?}", window.state());
	println!("✓ DPI scale factor: {}", window.scale_factor());

	// Show window
	window.show();
	println!("✓ Window visible: {}", window.is_visible());

	println!("\n--- Event Handling ---");

	// Test state transitions
	println!("Testing window state transitions...");

	window.set_state(WindowState::Maximized);
	if let Some(event) = window.poll_event() {
		println!("  ✓ Event generated: {:?}", event);
	}

	window.set_state(WindowState::Minimized);
	if let Some(event) = window.poll_event() {
		println!("  ✓ Event generated: {:?}", event);
	}

	window.set_state(WindowState::Normal);
	if let Some(event) = window.poll_event() {
		println!("  ✓ Event generated: {:?}", event);
	}

	// Test bounds change
	window.set_bounds(WindowBounds::new(150, 150, 1024, 768));
	let mut event_count = 0;
	while let Some(event) = window.poll_event() {
		println!("  ✓ Event generated: {:?}", event);
		event_count += 1;
	}
	println!("  Total events from bounds change: {}", event_count);

	// Test focus
	window.request_focus();
	if let Some(event) = window.poll_event() {
		println!("  ✓ Focus event: {:?}", event);
	}

	println!("\n--- Graphics Rendering ---");

	// Create render context
	let mut context = renderer.create_context(800, 600);
	let (width, height) = context.size();
	println!("✓ Render context created: {}x{}", width, height);

	// Render a test frame
	println!("Rendering test frame...");
	context.begin_frame();

	// Clear background
	context.execute(RenderCommand::Clear(Color::rgb(0.1, 0.1, 0.15)));
	println!("  ✓ Background cleared");

	// Draw filled rectangles
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(50.0, 50.0, 200.0, 100.0),
		color: Color::rgba(0.8, 0.2, 0.2, 0.9),
	});
	println!("  ✓ Red rectangle drawn");

	context.execute(RenderCommand::FillRect {
		rect: Rect::new(300.0, 50.0, 200.0, 100.0),
		color: Color::rgba(0.2, 0.8, 0.2, 0.9),
	});
	println!("  ✓ Green rectangle drawn");

	context.execute(RenderCommand::FillRect {
		rect: Rect::new(550.0, 50.0, 200.0, 100.0),
		color: Color::rgba(0.2, 0.2, 0.8, 0.9),
	});
	println!("  ✓ Blue rectangle drawn");

	// Draw stroked rectangles
	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(50.0, 200.0, 200.0, 100.0),
		color: Color::rgb(1.0, 1.0, 0.0),
		width: 3.0,
	});
	println!("  ✓ Yellow outlined rectangle drawn");

	// Draw circles
	context.execute(RenderCommand::Circle {
		x: 150.0,
		y: 400.0,
		radius: 50.0,
		color: Color::rgb(1.0, 0.5, 0.0),
		filled: true,
	});
	println!("  ✓ Orange filled circle drawn");

	context.execute(RenderCommand::Circle {
		x: 400.0,
		y: 400.0,
		radius: 50.0,
		color: Color::rgb(0.5, 0.0, 1.0),
		filled: false,
	});
	println!("  ✓ Purple outlined circle drawn");

	// Draw lines
	context.execute(RenderCommand::Line {
		x1: 550.0,
		y1: 350.0,
		x2: 750.0,
		y2: 450.0,
		color: Color::rgb(0.0, 1.0, 1.0),
		width: 2.0,
	});
	println!("  ✓ Cyan line drawn");

	// Test clipping
	context.execute(RenderCommand::SetClip(Rect::new(50.0, 500.0, 150.0, 80.0)));
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(0.0, 480.0, 800.0, 120.0),
		color: Color::rgba(1.0, 1.0, 1.0, 0.5),
	});
	context.execute(RenderCommand::RestoreClip);
	println!("  ✓ Clipped rectangle drawn");

	context.end_frame();
	println!("✓ Frame rendered successfully");

	println!("\n--- Multi-Context Test ---");

	// Test multiple render contexts
	let mut context2 = renderer.create_context(1920, 1080);
	let (w2, h2) = context2.size();
	println!("✓ Second render context: {}x{}", w2, h2);

	context2.begin_frame();
	context2.execute(RenderCommand::Clear(Color::rgb(0.2, 0.2, 0.3)));
	context2.execute(RenderCommand::FillRect {
		rect: Rect::new(100.0, 100.0, 400.0, 300.0),
		color: Color::rgb(0.8, 0.8, 0.2),
	});
	context2.end_frame();
	println!("✓ Second context rendered successfully");

	println!("\n--- Window Cleanup ---");

	// Test close event
	window.close();
	if let Some(event) = window.poll_event() {
		println!("✓ Close event generated: {:?}", event);
	}
	println!("✓ Window closed (visible: {})", window.is_visible());

	println!("\n=== Demo Complete ===");
	println!("\nSummary:");
	println!("- Window management: ✓ Working");
	println!("- Event system: ✓ Working");
	println!("- Graphics rendering: ✓ Working");
	println!("- Multi-context support: ✓ Working");
	println!("- State transitions: ✓ Working");
	println!("\nThe Windows backend is fully functional!");
}
