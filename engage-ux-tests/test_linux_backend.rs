//! Linux-specific backend integration tests
//!
//! Tests the tiny-skia renderer and AT-SPI accessibility infrastructure
//! specific to Linux platforms.

#![cfg(target_os = "linux")]

use engage_ux_core::accessibility::{AccessibilityProps, AriaRole};
use engage_ux_oal::backends::{
	AtSpiAccessibilityBridge, AtSpiState, RenderCommand, TinySkiaRenderer, get_backend_factory,
	renderer::{Color, Rect, RenderBackend},
};

#[test]
fn test_linux_backend_factory() {
	// Verify Linux backend uses tiny-skia renderer
	let factory = get_backend_factory();
	let renderer = factory.create_renderer();

	assert_eq!(
		renderer.name(),
		"Tiny-Skia Renderer",
		"Linux should use Tiny-Skia renderer"
	);
	assert!(!renderer.is_hardware_accelerated());
}

#[test]
fn test_tiny_skia_renderer_direct() {
	// Test direct creation of tiny-skia renderer
	let mut renderer = TinySkiaRenderer::new();
	assert_eq!(renderer.name(), "Tiny-Skia Renderer");

	let context = renderer.create_context(800, 600);
	assert_eq!(context.size(), (800, 600));
}

#[test]
fn test_tiny_skia_rendering_all_shapes() {
	let mut renderer = TinySkiaRenderer::new();
	let mut context = renderer.create_context(1000, 800);

	context.begin_frame();

	// Clear background
	context.execute(RenderCommand::Clear(Color::rgb(0.1, 0.1, 0.1)));

	// Test filled rectangles with transparency
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(50.0, 50.0, 150.0, 100.0),
		color: Color::rgba(1.0, 0.0, 0.0, 0.8),
	});

	context.execute(RenderCommand::FillRect {
		rect: Rect::new(250.0, 50.0, 150.0, 100.0),
		color: Color::rgba(0.0, 1.0, 0.0, 0.6),
	});

	// Test stroked rectangles with various widths
	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(450.0, 50.0, 150.0, 100.0),
		color: Color::rgb(0.0, 0.0, 1.0),
		width: 1.0,
	});

	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(650.0, 50.0, 150.0, 100.0),
		color: Color::rgb(1.0, 1.0, 0.0),
		width: 3.0,
	});

	// Test filled circles
	context.execute(RenderCommand::Circle {
		x: 125.0,
		y: 300.0,
		radius: 50.0,
		color: Color::rgb(1.0, 0.5, 0.0),
		filled: true,
	});

	context.execute(RenderCommand::Circle {
		x: 325.0,
		y: 300.0,
		radius: 60.0,
		color: Color::rgba(0.5, 0.0, 1.0, 0.7),
		filled: true,
	});

	// Test stroked circles
	context.execute(RenderCommand::Circle {
		x: 525.0,
		y: 300.0,
		radius: 50.0,
		color: Color::rgb(0.0, 1.0, 1.0),
		filled: false,
	});

	// Test lines with various widths
	context.execute(RenderCommand::Line {
		x1: 50.0,
		y1: 450.0,
		x2: 300.0,
		y2: 450.0,
		color: Color::rgb(1.0, 1.0, 1.0),
		width: 1.0,
	});

	context.execute(RenderCommand::Line {
		x1: 50.0,
		y1: 480.0,
		x2: 300.0,
		y2: 480.0,
		color: Color::rgb(1.0, 0.0, 1.0),
		width: 2.0,
	});

	context.execute(RenderCommand::Line {
		x1: 50.0,
		y1: 520.0,
		x2: 300.0,
		y2: 520.0,
		color: Color::rgb(0.0, 1.0, 0.5),
		width: 5.0,
	});

	// Test diagonal lines
	context.execute(RenderCommand::Line {
		x1: 400.0,
		y1: 450.0,
		x2: 600.0,
		y2: 550.0,
		color: Color::rgb(1.0, 1.0, 0.0),
		width: 2.0,
	});

	context.end_frame();

	// Test passes if no panic occurs
}

#[test]
fn test_tiny_skia_clipping_operations() {
	let mut renderer = TinySkiaRenderer::new();
	let mut context = renderer.create_context(500, 500);

	context.begin_frame();

	// Set a clip region
	context.execute(RenderCommand::SetClip(Rect::new(
		100.0, 100.0, 300.0, 300.0,
	)));

	// Draw shapes that extend beyond clip region
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(0.0, 0.0, 500.0, 500.0),
		color: Color::rgb(1.0, 0.0, 0.0),
	});

	// Restore clip
	context.execute(RenderCommand::RestoreClip);

	// Draw without clipping
	context.execute(RenderCommand::Circle {
		x: 250.0,
		y: 250.0,
		radius: 50.0,
		color: Color::rgb(0.0, 1.0, 0.0),
		filled: true,
	});

	context.end_frame();
}

#[test]
fn test_tiny_skia_multiple_contexts() {
	let mut renderer = TinySkiaRenderer::new();

	// Create multiple render contexts
	let mut context1 = renderer.create_context(800, 600);
	let mut context2 = renderer.create_context(1024, 768);
	let mut context3 = renderer.create_context(1920, 1080);

	assert_eq!(context1.size(), (800, 600));
	assert_eq!(context2.size(), (1024, 768));
	assert_eq!(context3.size(), (1920, 1080));

	// Render to each context independently
	context1.begin_frame();
	context1.execute(RenderCommand::Clear(Color::rgb(1.0, 0.0, 0.0)));
	context1.end_frame();

	context2.begin_frame();
	context2.execute(RenderCommand::Clear(Color::rgb(0.0, 1.0, 0.0)));
	context2.end_frame();

	context3.begin_frame();
	context3.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 1.0)));
	context3.end_frame();
}

#[test]
fn test_atspi_bridge_initialization() {
	let mut bridge = AtSpiAccessibilityBridge::new("Test Linux Application");
	assert!(!bridge.is_initialized());

	// Initialize the bridge
	assert!(bridge.initialize().is_ok());
	assert!(bridge.is_initialized());

	// Double initialization should be safe
	assert!(bridge.initialize().is_ok());

	// Shutdown
	bridge.shutdown();
	assert!(!bridge.is_initialized());
}

#[test]
fn test_atspi_element_lifecycle() {
	let mut bridge = AtSpiAccessibilityBridge::new("Test App");
	bridge.initialize().unwrap();

	// Create accessibility props for a button
	let button_props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Click Me")
		.with_description("A clickable button");

	// Register the element
	assert!(bridge.register_element(1, &button_props).is_ok());

	// Update the element
	let updated_props = button_props.clone().with_label("Updated Button");
	assert!(bridge.update_element(1, &updated_props).is_ok());

	// Unregister the element
	assert!(bridge.unregister_element(1).is_ok());
}

#[test]
fn test_atspi_multiple_elements() {
	let mut bridge = AtSpiAccessibilityBridge::new("Multi-Element App");
	bridge.initialize().unwrap();

	// Register multiple UI elements
	let button = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Submit");

	let textbox = AccessibilityProps::new()
		.with_role(AriaRole::Textbox)
		.with_label("Username");

	let checkbox = AccessibilityProps::new()
		.with_role(AriaRole::Checkbox)
		.with_label("Remember me");

	assert!(bridge.register_element(1, &button).is_ok());
	assert!(bridge.register_element(2, &textbox).is_ok());
	assert!(bridge.register_element(3, &checkbox).is_ok());

	// Unregister in different order
	assert!(bridge.unregister_element(2).is_ok());
	assert!(bridge.unregister_element(1).is_ok());
	assert!(bridge.unregister_element(3).is_ok());
}

#[test]
fn test_atspi_focus_notifications() {
	let mut bridge = AtSpiAccessibilityBridge::new("Focus Test App");
	bridge.initialize().unwrap();

	let button = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Focusable Button");

	bridge.register_element(1, &button).unwrap();

	// Notify focus change
	assert!(bridge.notify_focus_changed(1).is_ok());

	// Notify state change
	assert!(
		bridge
			.notify_state_changed(1, AtSpiState::Focused, true)
			.is_ok()
	);
	assert!(
		bridge
			.notify_state_changed(1, AtSpiState::Focused, false)
			.is_ok()
	);
}

#[test]
fn test_atspi_state_notifications() {
	let mut bridge = AtSpiAccessibilityBridge::new("State Test App");
	bridge.initialize().unwrap();

	let checkbox = AccessibilityProps::new()
		.with_role(AriaRole::Checkbox)
		.with_label("Test Checkbox");

	bridge.register_element(1, &checkbox).unwrap();

	// Test all state changes
	let states = [
		AtSpiState::Enabled,
		AtSpiState::Focusable,
		AtSpiState::Focused,
		AtSpiState::Selected,
		AtSpiState::Checked,
		AtSpiState::Expanded,
		AtSpiState::Pressed,
		AtSpiState::Visible,
		AtSpiState::Showing,
		AtSpiState::Active,
		AtSpiState::Required,
		AtSpiState::ReadOnly,
	];

	for state in states.iter() {
		assert!(bridge.notify_state_changed(1, *state, true).is_ok());
		assert!(bridge.notify_state_changed(1, *state, false).is_ok());
	}
}

#[test]
fn test_atspi_property_notifications() {
	let mut bridge = AtSpiAccessibilityBridge::new("Property Test App");
	bridge.initialize().unwrap();

	let button = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Dynamic Button");

	bridge.register_element(1, &button).unwrap();

	// Notify various property changes
	assert!(bridge.notify_property_changed(1, "name").is_ok());
	assert!(bridge.notify_property_changed(1, "description").is_ok());
	assert!(bridge.notify_property_changed(1, "value").is_ok());
}

#[test]
fn test_atspi_error_handling() {
	use engage_ux_oal::backends::AccessibilityError;

	let bridge = AtSpiAccessibilityBridge::new("Error Test App");

	// Operations should fail when not initialized
	let props = AccessibilityProps::new();

	assert_eq!(
		bridge.register_element(1, &props),
		Err(AccessibilityError::NotInitialized)
	);
	assert_eq!(
		bridge.update_element(1, &props),
		Err(AccessibilityError::NotInitialized)
	);
	assert_eq!(
		bridge.unregister_element(1),
		Err(AccessibilityError::NotInitialized)
	);
	assert_eq!(
		bridge.notify_focus_changed(1),
		Err(AccessibilityError::NotInitialized)
	);
}

#[test]
fn test_linux_backend_integration() {
	// Test that Linux backend properly integrates renderer and window backend
	let factory = get_backend_factory();

	let mut renderer = factory.create_renderer();
	let window = factory.create_window_backend();

	// Verify both backends work together
	assert!(renderer.name().contains("Skia") || renderer.name().contains("Renderer"));
	assert!(window.name().contains("Winit") || window.name().contains("Window"));

	// Create a render context
	let mut context = renderer.create_context(800, 600);

	// Perform a simple render
	context.begin_frame();
	context.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)));
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(100.0, 100.0, 200.0, 150.0),
		color: Color::rgb(1.0, 1.0, 1.0),
	});
	context.end_frame();

	// Verify window properties
	assert_eq!(window.bounds().width, 800);
	assert_eq!(window.bounds().height, 600);
}

#[test]
fn test_linux_x11_wayland_compatibility() {
	// This test verifies that the backend can be created on Linux
	// regardless of whether X11 or Wayland is being used
	let factory = get_backend_factory();

	let renderer = factory.create_renderer();
	let window = factory.create_window_backend();

	// Both should be successfully created
	assert_eq!(renderer.name(), "Tiny-Skia Renderer");
	assert!(window.name().contains("Winit"));
}
