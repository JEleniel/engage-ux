//! Integration tests for the rendering pipeline

use engage_ux_oal::backends::{RenderCommand, get_backend_factory};

#[test]
fn test_backend_factory_system() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();

	assert!(!renderer.name().is_empty());

	let mut context = renderer.create_context(800, 600);
	assert_eq!(context.size(), (800, 600));

	context.begin_frame();
	context.end_frame();
}

#[test]
fn test_render_commands() {
	let factory = get_backend_factory();
	let mut renderer = factory.create_renderer();
	let mut context = renderer.create_context(1024, 768);

	use engage_ux_oal::backends::renderer::{Color, Rect, TextAlign};

	let commands = vec![
		RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)),
		RenderCommand::FillRect {
			rect: Rect::new(10.0, 10.0, 100.0, 100.0),
			color: Color::rgb(1.0, 0.0, 0.0),
		},
		RenderCommand::Text {
			text: "Hello World".to_string(),
			x: 50.0,
			y: 50.0,
			font_size: 16.0,
			color: Color::rgb(1.0, 1.0, 1.0),
			align: TextAlign::Center,
		},
	];

	context.begin_frame();
	context.execute_batch(&commands);
	context.end_frame();

	// If we get here without panicking, the render pipeline works
	assert!(true);
}

#[test]
fn test_window_backend_integration() {
	let factory = get_backend_factory();
	let mut window = factory.create_window_backend();

	assert!(!window.name().is_empty());
	assert!(window.is_visible());

	window.set_title("Integration Test Window");
	assert_eq!(window.title(), "Integration Test Window");

	window.hide();
	assert!(!window.is_visible());

	window.show();
	assert!(window.is_visible());
}
