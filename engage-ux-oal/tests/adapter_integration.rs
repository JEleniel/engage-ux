//! Adapter integration tests for the OAL backends.
//!
//! These exercises verify recording and backend adapter behavior in a
//! headless or test environment.

use engage_ux_core::Color;
use engage_ux_core::geometry::{Circle, Line, Point, Polygon, Polyline, Rectangle, Text};
use engage_ux_oal::Canvas;
use engage_ux_oal::Primitive;
use engage_ux_oal::core_adapter;

#[test]
fn adapter_records_primitives_to_canvas() {
	// Create a canvas in logical units
	let c = Canvas::new(200.0, 200.0);

	// Prepare core primitives
	let line = Line::new(
		Point {
			x: 10.0,
			y: 10.0,
			style: None,
		},
		Point {
			x: 100.0,
			y: 100.0,
			style: None,
		},
	);
	let circle = Circle::new(
		Point {
			x: 50.0,
			y: 50.0,
			style: None,
		},
		20.0,
	);
	let rect = Rectangle::new(
		Point {
			x: 20.0,
			y: 20.0,
			style: None,
		},
		80.0,
		60.0,
	);
	let poly = Polygon::new(vec![
		Point {
			x: 0.0,
			y: 0.0,
			style: None,
		},
		Point {
			x: 10.0,
			y: 0.0,
			style: None,
		},
		Point {
			x: 10.0,
			y: 10.0,
			style: None,
		},
	]);
	let pline = Polyline::new(vec![
		Point {
			x: 5.0,
			y: 5.0,
			style: None,
		},
		Point {
			x: 15.0,
			y: 5.0,
			style: None,
		},
		Point {
			x: 25.0,
			y: 15.0,
			style: None,
		},
	]);
	let text = Text::new(
		Point {
			x: 12.0,
			y: 12.0,
			style: None,
		},
		"Hello".to_string(),
		12.0,
	);

	// Styling
	let red = Color::from_rgb(255, 0, 0);
	let blue = Color::from_rgb(0, 0, 255);

	// Emit using adapter
	core_adapter::emit_line(&c, &line, red, 2.0);
	core_adapter::emit_circle(&c, &circle, blue, 1.0, true);
	core_adapter::emit_rectangle(&c, &rect, red, 0.0, true);
	core_adapter::emit_polygon(&c, &poly, blue, 1.0, true);
	core_adapter::emit_polyline(&c, &pline, red, 1.5);
	core_adapter::emit_text(&c, &text, blue);

	// Drain and inspect
	let prims = c.drain_commands();
	assert_eq!(prims.len(), 6, "expected 6 recorded primitives");

	// Check variants in order
	assert!(matches!(prims[0], Primitive::Line { .. }));
	assert!(matches!(prims[1], Primitive::Circle { .. }));
	assert!(matches!(
		prims[2],
		Primitive::RoundedRect { .. } | Primitive::Rect { .. }
	));
	assert!(matches!(prims[3], Primitive::Polygon { .. }));
	assert!(matches!(prims[4], Primitive::Polyline { .. }));
	assert!(matches!(prims[5], Primitive::Text { .. }));
}
