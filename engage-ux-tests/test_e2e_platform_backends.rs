//! End-to-end functional tests for platform-specific backends
//!
//! Tests comprehensive workflows across all supported platforms,
//! validating the complete integration of rendering, window management,
//! and input handling.

use engage_ux_oal::backends::{
	RenderCommand, WindowBackendEvent, WindowBounds, WindowState, get_backend_factory,
	renderer::{Color, Rect},
};

/// Test complete window creation and rendering workflow
#[test]
fn test_e2e_window_creation_and_rendering() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();
	let mut renderer = factory.create_renderer();

	// Window setup workflow
	window.set_title("E2E Test Window");
	window.set_bounds(WindowBounds::new(100, 100, 800, 600));
	window.show();

	assert_eq!(window.title(), "E2E Test Window");
	assert_eq!(window.bounds(), WindowBounds::new(100, 100, 800, 600));
	assert!(window.is_visible());

	// Rendering workflow
	let mut context = renderer.create_context(800, 600);
	context.begin_frame();

	// Clear background
	context.execute(RenderCommand::Clear(Color::rgb(0.1, 0.1, 0.1)));

	// Render UI elements
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(50.0, 50.0, 200.0, 100.0),
		color: Color::rgb(0.2, 0.4, 0.8),
	});

	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(50.0, 50.0, 200.0, 100.0),
		color: Color::rgb(1.0, 1.0, 1.0),
		width: 2.0,
	});

	context.end_frame();

	// Verify workflow completed
	assert_eq!(context.size(), (800, 600));
}

/// Test window state transitions in real-world scenarios
#[test]
fn test_e2e_window_state_lifecycle() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Start in normal state
	assert_eq!(window.state(), WindowState::Normal);

	// User maximizes window
	window.set_state(WindowState::Maximized);
	assert_eq!(window.state(), WindowState::Maximized);

	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::Maximized)));

	// User minimizes window
	window.set_state(WindowState::Minimized);
	assert_eq!(window.state(), WindowState::Minimized);

	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::Minimized)));

	// User restores window
	window.set_state(WindowState::Normal);
	assert_eq!(window.state(), WindowState::Normal);

	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::Restored)));

	// User toggles fullscreen
	window.set_state(WindowState::Fullscreen);
	assert_eq!(window.state(), WindowState::Fullscreen);
}

/// Test multi-window scenario
#[test]
fn test_e2e_multi_window_rendering() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();

	// Create multiple windows with different contexts
	let mut window1 = factory.create_window_backend();
	window1.set_title("Window 1");
	window1.set_bounds(WindowBounds::new(0, 0, 640, 480));

	let mut window2 = factory.create_window_backend();
	window2.set_title("Window 2");
	window2.set_bounds(WindowBounds::new(650, 0, 640, 480));

	// Create separate render contexts
	let mut context1 = renderer.create_context(640, 480);
	let mut context2 = renderer.create_context(640, 480);

	// Render different content to each window
	context1.begin_frame();
	context1.execute(RenderCommand::Clear(Color::rgb(1.0, 0.0, 0.0)));
	context1.end_frame();

	context2.begin_frame();
	context2.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 1.0)));
	context2.end_frame();

	// Verify both contexts work independently
	assert_eq!(context1.size(), (640, 480));
	assert_eq!(context2.size(), (640, 480));
	assert_eq!(window1.title(), "Window 1");
	assert_eq!(window2.title(), "Window 2");
}

/// Test complex rendering workflow
#[test]
fn test_e2e_complex_rendering_workflow() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(1920, 1080);

	context.begin_frame();

	// Background
	context.execute(RenderCommand::Clear(Color::rgb(0.95, 0.95, 0.95)));

	// Title bar
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(0.0, 0.0, 1920.0, 60.0),
		color: Color::rgb(0.2, 0.2, 0.2),
	});

	// Content area with clipping
	context.execute(RenderCommand::SetClip(Rect::new(20.0, 80.0, 1880.0, 980.0)));

	// Multiple UI elements
	for i in 0..5 {
		let y = 100.0 + i as f32 * 120.0;

		// Card background
		context.execute(RenderCommand::FillRect {
			rect: Rect::new(50.0, y, 400.0, 100.0),
			color: Color::rgba(1.0, 1.0, 1.0, 0.95),
		});

		// Card border
		context.execute(RenderCommand::StrokeRect {
			rect: Rect::new(50.0, y, 400.0, 100.0),
			color: Color::rgb(0.8, 0.8, 0.8),
			width: 1.0,
		});

		// Icon circle
		context.execute(RenderCommand::Circle {
			x: 80.0,
			y: y + 50.0,
			radius: 20.0,
			color: Color::rgb(0.2, 0.6, 1.0),
			filled: true,
		});
	}

	context.execute(RenderCommand::RestoreClip);
	context.end_frame();
}

/// Test window focus and event handling
#[test]
fn test_e2e_window_focus_workflow() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Initial state
	window.set_title("Focus Test");
	window.show();

	// Request focus
	window.request_focus();
	assert!(window.is_focused());

	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::FocusGained)));

	// Simulate focus loss (in real app, this would happen from OS)
	// For testing, we verify the event mechanism works
}

/// Test window resize workflow with rendering
#[test]
fn test_e2e_window_resize_workflow() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();
	let mut renderer = factory.create_renderer();

	// Initial size
	window.set_bounds(WindowBounds::new(0, 0, 800, 600));
	let mut context = renderer.create_context(800, 600);

	// Render at initial size
	context.begin_frame();
	context.execute(RenderCommand::Clear(Color::rgb(0.5, 0.5, 0.5)));
	context.end_frame();

	// Resize window
	window.set_bounds(WindowBounds::new(0, 0, 1024, 768));

	// Check for resize event
	let event = window.poll_event();
	assert!(
		matches!(event, Some(WindowBackendEvent::Resized { .. })),
		"Expected resize event"
	);

	// Create new context for new size
	let mut context = renderer.create_context(1024, 768);

	// Render at new size
	context.begin_frame();
	context.execute(RenderCommand::Clear(Color::rgb(0.5, 0.5, 0.5)));
	context.end_frame();

	assert_eq!(context.size(), (1024, 768));
}

/// Test platform-specific scaling factors
#[test]
fn test_e2e_platform_scaling() {
	let factory = get_backend_factory();
	let window = factory.create_window_backend();

	// Get platform scale factor
	let scale = window.scale_factor();

	// Scale factor should be positive
	assert!(scale > 0.0);

	// Common scale factors: 1.0 (standard), 1.5, 2.0 (HiDPI)
	assert!(scale >= 1.0 && scale <= 3.0);

	// Verify scaled rendering works
	let mut renderer = factory.create_renderer();
	let logical_width = 800;
	let logical_height = 600;

	let physical_width = (logical_width as f32 * scale) as u32;
	let physical_height = (logical_height as f32 * scale) as u32;

	let mut context = renderer.create_context(physical_width, physical_height);

	context.begin_frame();
	context.execute(RenderCommand::Clear(Color::rgb(1.0, 1.0, 1.0)));
	context.end_frame();

	assert_eq!(context.size(), (physical_width, physical_height));
}

/// Test complete UI rendering pipeline
#[test]
fn test_e2e_ui_rendering_pipeline() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(800, 600);

	context.begin_frame();

	// Clear background
	context.execute(RenderCommand::Clear(Color::rgb(0.1, 0.1, 0.15)));

	// Button rendering
	let button_rect = Rect::new(300.0, 250.0, 200.0, 60.0);

	// Button background
	context.execute(RenderCommand::FillRect {
		rect: button_rect,
		color: Color::rgb(0.2, 0.4, 0.8),
	});

	// Button border
	context.execute(RenderCommand::StrokeRect {
		rect: button_rect,
		color: Color::rgb(0.3, 0.5, 0.9),
		width: 2.0,
	});

	// Button hover state (simulated)
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(305.0, 255.0, 190.0, 50.0),
		color: Color::rgba(1.0, 1.0, 1.0, 0.1),
	});

	// Input field rendering
	let input_rect = Rect::new(250.0, 150.0, 300.0, 40.0);

	// Input background
	context.execute(RenderCommand::FillRect {
		rect: input_rect,
		color: Color::rgb(1.0, 1.0, 1.0),
	});

	// Input border
	context.execute(RenderCommand::StrokeRect {
		rect: input_rect,
		color: Color::rgb(0.7, 0.7, 0.7),
		width: 1.0,
	});

	// Cursor (blinking simulated)
	context.execute(RenderCommand::Line {
		x1: 260.0,
		y1: 160.0,
		x2: 260.0,
		y2: 180.0,
		color: Color::rgb(0.0, 0.0, 0.0),
		width: 2.0,
	});

	context.end_frame();
}

/// Test window close workflow
#[test]
fn test_e2e_window_close_workflow() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	window.set_title("Closing Window");
	window.show();

	assert!(window.is_visible());

	// User closes window
	window.close();

	assert!(!window.is_visible());

	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::CloseRequested)));
}

/// Test decorated vs undecorated window modes
#[test]
fn test_e2e_window_decoration_modes() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Default should be decorated
	window.set_title("Decorated Window");
	window.show();

	// Switch to undecorated (custom title bar)
	window.set_decorated(false);

	// Switch back to decorated
	window.set_decorated(true);

	// Window should still be functional
	assert!(window.is_visible());
	assert_eq!(window.title(), "Decorated Window");
}

/// Test window resizable property
#[test]
fn test_e2e_window_resizable_modes() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	window.set_bounds(WindowBounds::new(100, 100, 640, 480));

	// Make non-resizable (fixed size dialog)
	window.set_resizable(false);

	// Make resizable again
	window.set_resizable(true);

	// Window should maintain bounds
	assert_eq!(window.bounds(), WindowBounds::new(100, 100, 640, 480));
}

/// Test rendering performance with many draw calls
#[test]
fn test_e2e_rendering_performance() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(1920, 1080);

	context.begin_frame();

	// Background
	context.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)));

	// Render many UI elements (stress test)
	for i in 0..100 {
		let x = (i % 10) as f32 * 192.0;
		let y = (i / 10) as f32 * 108.0;

		context.execute(RenderCommand::FillRect {
			rect: Rect::new(x + 5.0, y + 5.0, 180.0, 95.0),
			color: Color::rgba(
				(i as f32 * 0.01) % 1.0,
				(i as f32 * 0.02) % 1.0,
				(i as f32 * 0.03) % 1.0,
				0.9,
			),
		});

		context.execute(RenderCommand::StrokeRect {
			rect: Rect::new(x + 5.0, y + 5.0, 180.0, 95.0),
			color: Color::rgb(1.0, 1.0, 1.0),
			width: 1.0,
		});
	}

	context.end_frame();

	// Test passes if no panic or crash occurs
}

/// Test window move events
#[test]
fn test_e2e_window_move_workflow() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	window.set_bounds(WindowBounds::new(100, 100, 800, 600));

	// Move window
	window.set_bounds(WindowBounds::new(200, 150, 800, 600));

	// Check for move event
	let event = window.poll_event();
	assert!(
		matches!(event, Some(WindowBackendEvent::Moved { .. })),
		"Expected move event"
	);

	assert_eq!(window.bounds().x, 200);
	assert_eq!(window.bounds().y, 150);
}

/// Test rendering with transparency
#[test]
fn test_e2e_transparency_rendering() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(800, 600);

	context.begin_frame();

	// Clear to transparent black
	context.execute(RenderCommand::Clear(Color::rgba(0.0, 0.0, 0.0, 0.0)));

	// Render semi-transparent layers
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(100.0, 100.0, 300.0, 200.0),
		color: Color::rgba(1.0, 0.0, 0.0, 0.5),
	});

	context.execute(RenderCommand::FillRect {
		rect: Rect::new(200.0, 150.0, 300.0, 200.0),
		color: Color::rgba(0.0, 1.0, 0.0, 0.5),
	});

	context.execute(RenderCommand::FillRect {
		rect: Rect::new(300.0, 200.0, 300.0, 200.0),
		color: Color::rgba(0.0, 0.0, 1.0, 0.5),
	});

	context.end_frame();
}
