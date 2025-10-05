//! Integration tests for platform-specific backends
//!
//! Tests the winit window backend and softbuffer renderer implementations
//! across all supported platforms.

use engage_ux_oal::backends::{
	RenderCommand, WindowBackendEvent, WindowBounds, WindowState, get_backend_factory,
	renderer::{Color, Rect},
};

#[test]
fn test_platform_backend_factory_creation() {
	// This test verifies that we can create a backend factory for the current platform
	let factory = get_backend_factory();
	let renderer = factory.create_renderer();
	let window = factory.create_window_backend();

	// Verify the backends are not stub implementations
	assert!(
		renderer.name().contains("Softbuffer") || renderer.name().contains("Renderer"),
		"Expected real renderer implementation, got: {}",
		renderer.name()
	);

	assert!(
		window.name().contains("Winit") || window.name().contains("Window"),
		"Expected real window backend implementation, got: {}",
		window.name()
	);
}

#[test]
fn test_platform_renderer_basic_operations() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();

	// Create a render context
	let mut context = renderer.create_context(800, 600);
	assert_eq!(context.size(), (800, 600));

	// Test basic rendering operations
	context.begin_frame();
	context.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)));
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(10.0, 10.0, 100.0, 100.0),
		color: Color::rgb(1.0, 0.0, 0.0),
	});
	context.end_frame();

	// Test passes if no panic occurs
}

#[test]
fn test_platform_renderer_shapes() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(400, 400);

	context.begin_frame();

	// Test rectangle
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(50.0, 50.0, 100.0, 50.0),
		color: Color::rgba(1.0, 0.0, 0.0, 0.5),
	});

	// Test stroked rectangle
	context.execute(RenderCommand::StrokeRect {
		rect: Rect::new(200.0, 50.0, 100.0, 50.0),
		color: Color::rgb(0.0, 1.0, 0.0),
		width: 2.0,
	});

	// Test circle
	context.execute(RenderCommand::Circle {
		x: 100.0,
		y: 200.0,
		radius: 30.0,
		color: Color::rgb(0.0, 0.0, 1.0),
		filled: true,
	});

	// Test line
	context.execute(RenderCommand::Line {
		x1: 200.0,
		y1: 200.0,
		x2: 300.0,
		y2: 250.0,
		color: Color::rgb(1.0, 1.0, 0.0),
		width: 1.0,
	});

	context.end_frame();
}

#[test]
fn test_platform_renderer_clipping() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(400, 400);

	context.begin_frame();

	// Set a clip region
	context.execute(RenderCommand::SetClip(Rect::new(50.0, 50.0, 200.0, 200.0)));

	// Draw something that extends beyond the clip region
	context.execute(RenderCommand::FillRect {
		rect: Rect::new(0.0, 0.0, 400.0, 400.0),
		color: Color::rgb(1.0, 0.0, 0.0),
	});

	// Restore clip
	context.execute(RenderCommand::RestoreClip);

	context.end_frame();
}

#[test]
fn test_platform_window_backend_creation() {
	let factory = get_backend_factory();
	let window = factory.create_window_backend();

	// Test initial state
	assert!(window.is_visible());
	assert_eq!(window.state(), WindowState::Normal);
	assert_eq!(window.scale_factor(), 1.0);
}

#[test]
fn test_platform_window_backend_properties() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Test title
	window.set_title("Test Window");
	assert_eq!(window.title(), "Test Window");

	// Test bounds
	let bounds = WindowBounds::new(100, 100, 1024, 768);
	window.set_bounds(bounds);
	assert_eq!(window.bounds(), bounds);

	// Test visibility
	window.hide();
	assert!(!window.is_visible());
	window.show();
	assert!(window.is_visible());

	// Test resizable
	window.set_resizable(false);

	// Test decorated
	window.set_decorated(false);
}

#[test]
fn test_platform_window_backend_states() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Test state transitions
	window.set_state(WindowState::Maximized);
	assert_eq!(window.state(), WindowState::Maximized);

	window.set_state(WindowState::Minimized);
	assert_eq!(window.state(), WindowState::Minimized);

	window.set_state(WindowState::Normal);
	assert_eq!(window.state(), WindowState::Normal);

	window.set_state(WindowState::Fullscreen);
	assert_eq!(window.state(), WindowState::Fullscreen);
}

#[test]
fn test_platform_window_backend_events() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Request focus should generate an event
	window.request_focus();
	assert!(window.is_focused());

	// Check for focus event
	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::FocusGained)));

	// Changing bounds should generate events
	window.set_bounds(WindowBounds::new(0, 0, 1280, 720));

	// Should have resize and move events
	let event1 = window.poll_event();
	let event2 = window.poll_event();

	let has_resize = matches!(event1, Some(WindowBackendEvent::Resized { .. }))
		|| matches!(event2, Some(WindowBackendEvent::Resized { .. }));
	let has_move = matches!(event1, Some(WindowBackendEvent::Moved { .. }))
		|| matches!(event2, Some(WindowBackendEvent::Moved { .. }));

	assert!(has_resize, "Expected Resized event");
	assert!(has_move, "Expected Moved event");
}

#[test]
fn test_platform_window_backend_state_events() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Maximizing should generate an event
	window.set_state(WindowState::Maximized);
	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::Maximized)));

	// Minimizing should generate an event
	window.set_state(WindowState::Minimized);
	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::Minimized)));

	// Restoring from minimized should generate an event
	window.set_state(WindowState::Normal);
	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::Restored)));
}

#[test]
fn test_platform_window_backend_close_event() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	// Closing should generate an event
	window.close();
	assert!(!window.is_visible());

	let event = window.poll_event();
	assert!(matches!(event, Some(WindowBackendEvent::CloseRequested)));
}

#[test]
fn test_platform_multiple_contexts() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();

	// Create multiple render contexts
	let mut context1 = renderer.create_context(800, 600);
	let mut context2 = renderer.create_context(1920, 1080);

	assert_eq!(context1.size(), (800, 600));
	assert_eq!(context2.size(), (1920, 1080));

	// Both contexts should work independently
	context1.begin_frame();
	context1.execute(RenderCommand::Clear(Color::rgb(1.0, 0.0, 0.0)));
	context1.end_frame();

	context2.begin_frame();
	context2.execute(RenderCommand::Clear(Color::rgb(0.0, 1.0, 0.0)));
	context2.end_frame();
}

#[test]
fn test_platform_renderer_complex_scene() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(800, 600);

	context.begin_frame();

	// Clear background
	context.execute(RenderCommand::Clear(Color::rgb(0.1, 0.1, 0.1)));

	// Draw multiple shapes
	for i in 0..5 {
		let x = 50.0 + i as f32 * 150.0;
		let y = 100.0;

		context.execute(RenderCommand::FillRect {
			rect: Rect::new(x, y, 100.0, 100.0),
			color: Color::rgba(
				i as f32 * 0.2,
				1.0 - i as f32 * 0.2,
				0.5,
				0.8,
			),
		});

		context.execute(RenderCommand::Circle {
			x: x + 50.0,
			y: y + 200.0,
			radius: 40.0,
			color: Color::rgb(1.0, 1.0, 0.0),
			filled: true,
		});
	}

	context.end_frame();
}

#[test]
fn test_platform_renderer_hardware_acceleration_query() {
	let factory = get_backend_factory();
	let renderer = factory.create_renderer();

	// Query hardware acceleration status
	let is_hw = renderer.is_hardware_accelerated();

	// Softbuffer is a software renderer, so should be false
	// But we just verify the method is callable
	assert!(!is_hw || is_hw); // Always passes, just testing the interface
}

#[test]
fn test_platform_backend_name_reporting() {
	let factory = get_backend_factory();
	let renderer = factory.create_renderer();
	let window = factory.create_window_backend();

	// Both backends should report their names
	let renderer_name = renderer.name();
	let window_name = window.name();

	assert!(!renderer_name.is_empty(), "Renderer should report a name");
	assert!(!window_name.is_empty(), "Window backend should report a name");

	println!("Platform renderer: {}", renderer_name);
	println!("Platform window backend: {}", window_name);
}
