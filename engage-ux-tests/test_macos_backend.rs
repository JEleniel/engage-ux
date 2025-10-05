//! macOS-specific backend tests
//!
//! These tests verify that the macOS backend implementation meets
//! platform-specific requirements including window management, rendering,
//! event handling, and accessibility readiness.

#![cfg(target_os = "macos")]

use engage_ux_oal::backends::{
	RenderCommand, WindowBackendEvent, WindowBounds, WindowState, get_backend_factory,
	renderer::{Color, Rect},
};

#[test]
fn test_macos_backend_factory() {
	// Verify that on macOS, we get the MacOSBackendFactory
	let factory = get_backend_factory();
	let renderer = factory.create_renderer();
	let window = factory.create_window_backend();

	// Backends should not be stubs on macOS
	assert_ne!(
		renderer.name(),
		"Stub Renderer",
		"macOS should use real renderer implementation"
	);
	assert_ne!(
		window.name(),
		"Stub Window Backend",
		"macOS should use real window backend"
	);
}

#[test]
fn test_macos_renderer_core_graphics_compatibility() {
	// Test that the renderer can handle Core Graphics-like operations
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(800, 600);

	context.begin_frame();

	// Core Graphics-style operations
	// Clear with background color
	context.execute(RenderCommand::Clear(Color::rgb(1.0, 1.0, 1.0)));

	// Draw filled rect (CGContextFillRect equivalent)
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(10.0, 10.0, 100.0, 100.0),
		color: Color::rgba(1.0, 0.0, 0.0, 0.5),
	});

	// Draw stroked rect (CGContextStrokeRect equivalent)
	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(120.0, 10.0, 100.0, 100.0),
		color: Color::rgb(0.0, 1.0, 0.0),
		width: 2.0,
	});

	// Draw circle (CGContextFillEllipseInRect equivalent)
	context.execute(RenderCommand::Circle {
		x: 300.0,
		y: 60.0,
		radius: 50.0,
		color: Color::rgb(0.0, 0.0, 1.0),
		filled: true,
	});

	// Draw line (CGContextStrokePath equivalent)
	context.execute(RenderCommand::Line {
		x1: 400.0,
		y1: 10.0,
		x2: 500.0,
		y2: 110.0,
		color: Color::rgb(1.0, 1.0, 0.0),
		width: 1.0,
	});

	// Clipping support (CGContextClip equivalent)
	context.execute(RenderCommand::SetClip(Rect::new(50.0, 200.0, 300.0, 200.0)));
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(0.0, 150.0, 800.0, 300.0),
		color: Color::rgba(0.5, 0.5, 0.5, 0.3),
	});
	context.execute(RenderCommand::RestoreClip);

	context.end_frame();

	// Test passes if rendering completes without panic
}

#[test]
fn test_macos_window_cocoa_compatibility() {
	// Test that the window backend supports Cocoa/AppKit-like operations
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// NSWindow-like operations
	// Set window title (setTitle: equivalent)
	window.set_title("macOS Test Window");
	assert_eq!(window.title(), "macOS Test Window");

	// Window visibility (orderFront:/orderOut: equivalent)
	window.show();
	assert!(window.is_visible());
	window.hide();
	assert!(!window.is_visible());
	window.show();

	// Window frame (setFrame:display: equivalent)
	let bounds = WindowBounds::new(100, 100, 800, 600);
	window.set_bounds(bounds);
	assert_eq!(window.bounds(), bounds);

	// Window level/state (setLevel: equivalent)
	window.set_state(WindowState::Normal);
	assert_eq!(window.state(), WindowState::Normal);

	window.set_state(WindowState::Maximized);
	assert_eq!(window.state(), WindowState::Maximized);

	window.set_state(WindowState::Minimized);
	assert_eq!(window.state(), WindowState::Minimized);

	// Window focus (makeKeyAndOrderFront: equivalent)
	window.request_focus();
	assert!(window.is_focused());

	// Window style (setStyleMask: equivalent)
	window.set_decorated(true);
	window.set_resizable(true);
	window.set_decorated(false);
	window.set_resizable(false);
}

#[test]
fn test_macos_window_events() {
	// Test macOS-specific window event handling
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Test window resize events (NSWindowDidResizeNotification equivalent)
	let new_bounds = WindowBounds::new(0, 0, 1024, 768);
	window.set_bounds(new_bounds);

	let event1 = window.poll_event();
	let event2 = window.poll_event();

	assert!(
		matches!(event1, Some(WindowBackendEvent::Resized { .. }))
			|| matches!(event2, Some(WindowBackendEvent::Resized { .. })),
		"Should receive resize event"
	);

	// Test window move events (NSWindowDidMoveNotification equivalent)
	assert!(
		matches!(event1, Some(WindowBackendEvent::Moved { .. }))
			|| matches!(event2, Some(WindowBackendEvent::Moved { .. })),
		"Should receive move event"
	);

	// Test window focus events (NSWindowDidBecomeKeyNotification equivalent)
	window.request_focus();
	let event = window.poll_event();
	assert!(
		matches!(event, Some(WindowBackendEvent::FocusGained)),
		"Should receive focus gained event"
	);

	// Test window state change events
	window.set_state(WindowState::Maximized);
	let event = window.poll_event();
	assert!(
		matches!(event, Some(WindowBackendEvent::Maximized)),
		"Should receive maximized event"
	);

	window.set_state(WindowState::Minimized);
	let event = window.poll_event();
	assert!(
		matches!(event, Some(WindowBackendEvent::Minimized)),
		"Should receive minimized event"
	);

	// Test window close events (NSWindowWillCloseNotification equivalent)
	window.close();
	let event = window.poll_event();
	assert!(
		matches!(event, Some(WindowBackendEvent::CloseRequested)),
		"Should receive close requested event"
	);
}

#[test]
fn test_macos_dpi_scaling() {
	// Test DPI scaling for Retina displays
	let factory = get_backend_factory();
	let window = factory.create_window_backend();

	// On macOS, scale factor should be accessible
	let scale = window.scale_factor();
	assert!(
		scale >= 1.0 && scale <= 3.0,
		"Scale factor should be reasonable (1.0 for normal, 2.0 for Retina, etc.)"
	);

	// DPI-aware rendering should work at any scale
	let mut renderer = factory.create_renderer();
	let logical_size = (800, 600);
	let physical_size = (
		(logical_size.0 as f32 * scale) as u32,
		(logical_size.1 as f32 * scale) as u32,
	);

	let mut context = renderer.create_context(physical_size.0, physical_size.1);
	assert_eq!(context.size(), physical_size);

	context.begin_frame();
	context.execute(RenderCommand::Clear(Color::rgb(1.0, 1.0, 1.0)));
	context.end_frame();
}

#[test]
fn test_macos_multiple_windows() {
	// Test support for multiple windows (common macOS pattern)
	let factory = get_backend_factory();

	let mut window1 = factory.create_window_backend();
	let mut window2 = factory.create_window_backend();

	window1.set_title("Window 1");
	window2.set_title("Window 2");

	window1.set_bounds(WindowBounds::new(100, 100, 800, 600));
	window2.set_bounds(WindowBounds::new(200, 200, 600, 400));

	assert_eq!(window1.title(), "Window 1");
	assert_eq!(window2.title(), "Window 2");
	assert!(window1.bounds() != window2.bounds());

	// Both windows should be independent
	window1.hide();
	assert!(!window1.is_visible());
	assert!(window2.is_visible());

	window1.show();
	assert!(window1.is_visible());
	assert!(window2.is_visible());
}

#[test]
fn test_macos_accessibility_readiness() {
	// Test that the backend structure supports accessibility
	// While we can't test NSAccessibility directly without unsafe code,
	// we can verify that the abstractions are in place
	let factory = get_backend_factory();
	let window = factory.create_window_backend();

	// Window properties that map to NSAccessibility attributes
	assert!(
		!window.title().is_empty(),
		"Title should be set for accessibility"
	);
	assert!(window.is_visible(), "Visibility state should be trackable");

	let bounds = window.bounds();
	assert!(
		bounds.width > 0 && bounds.height > 0,
		"Size should be valid for accessibility"
	);

	// Window state is trackable (important for NSAccessibilityWindowRole)
	let state = window.state();
	assert!(
		matches!(
			state,
			WindowState::Normal
				| WindowState::Minimized
				| WindowState::Maximized
				| WindowState::Fullscreen
		),
		"Window state should be defined"
	);
}

#[test]
fn test_macos_renderer_color_space() {
	// Test color handling compatible with macOS color spaces
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(400, 400);

	context.begin_frame();

	// Test RGBA color support (NSColor equivalent)
	let colors = [
		Color::rgba(1.0, 0.0, 0.0, 1.0),  // Red
		Color::rgba(0.0, 1.0, 0.0, 1.0),  // Green
		Color::rgba(0.0, 0.0, 1.0, 1.0),  // Blue
		Color::rgba(1.0, 1.0, 1.0, 1.0),  // White
		Color::rgba(0.0, 0.0, 0.0, 1.0),  // Black
		Color::rgba(0.5, 0.5, 0.5, 1.0),  // Gray
		Color::rgba(1.0, 0.0, 0.0, 0.5),  // Transparent red
		Color::rgba(0.0, 1.0, 0.0, 0.25), // Very transparent green
	];

	for (i, color) in colors.iter().enumerate() {
		context.execute(RenderCommand::FillRect {
			rect: Rect::new((i * 50) as f32, 0.0, 40.0, 40.0),
			color: *color,
		});
	}

	context.end_frame();

	// Test passes if color rendering completes without panic
}

#[test]
fn test_macos_window_fullscreen_mode() {
	// Test fullscreen mode (common on macOS)
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Enter fullscreen
	window.set_state(WindowState::Fullscreen);
	assert_eq!(window.state(), WindowState::Fullscreen);

	// Exit fullscreen
	window.set_state(WindowState::Normal);
	assert_eq!(window.state(), WindowState::Normal);
}

#[test]
fn test_macos_backend_integration() {
	// Integration test: Create window and renderer, perform typical operations
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();
	let mut renderer = factory.create_renderer();

	// Setup window
	window.set_title("macOS Integration Test");
	window.set_bounds(WindowBounds::new(0, 0, 1024, 768));
	window.show();

	// Create render context matching window size
	let bounds = window.bounds();
	let mut context = renderer.create_context(bounds.width, bounds.height);

	// Render a test scene
	context.begin_frame();

	// Background
	context.execute(RenderCommand::Clear(Color::rgb(0.95, 0.95, 0.95)));

	// Title bar-like element
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(0.0, 0.0, bounds.width as f32, 28.0),
		color: Color::rgb(0.9, 0.9, 0.9),
	});

	// Content area
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(20.0, 50.0, 500.0, 400.0),
		color: Color::rgb(1.0, 1.0, 1.0),
	});

	// Border
	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(20.0, 50.0, 500.0, 400.0),
		color: Color::rgb(0.7, 0.7, 0.7),
		width: 1.0,
	});

	context.end_frame();

	// Verify window is still operational
	assert!(window.is_visible());
	assert_eq!(window.state(), WindowState::Normal);
}
