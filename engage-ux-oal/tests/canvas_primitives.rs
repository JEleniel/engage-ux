use engage_ux_core::Color;
use engage_ux_core::geometry::{Line, Point, Polygon, Polyline, Rectangle, Text};
use engage_ux_oal::oal::canvas::{Canvas, Primitive};
use engage_ux_oal::oal::core_adapter;

#[test]
fn emit_point_records() {
	let canvas = Canvas::new(10.0, 10.0);
	let p = Point::new(1.0, 2.0);
	core_adapter::emit_point(&canvas, &p, Color::from_rgb(10, 20, 30));
	let cmds = canvas.drain_commands();
	assert_eq!(cmds.len(), 1);
	match &cmds[0] {
		Primitive::Point { pos, color } => {
			assert_eq!(pos, &p);
			assert_eq!(*color, Color::from_rgb(10, 20, 30));
		}
		other => panic!("expected Point, got {:?}", other),
	}
}

#[test]
fn emit_line_records() {
	let canvas = Canvas::new(10.0, 10.0);
	let a = Point::new(0.0, 0.0);
	let b = Point::new(3.0, 4.0);
	let line = Line::new(a.clone(), b.clone());
	core_adapter::emit_line(&canvas, &line, Color::from_rgb(0, 128, 255), 2.0);
	let cmds = canvas.drain_commands();
	assert_eq!(cmds.len(), 1);
	match &cmds[0] {
		Primitive::Line {
			line: l,
			color,
			stroke,
		} => {
			assert_eq!(l.start, a);
			assert_eq!(l.end, b);
			assert_eq!(*color, Color::from_rgb(0, 128, 255));
			assert!((*stroke - 2.0).abs() < f32::EPSILON);
		}
		other => panic!("expected Line, got {:?}", other),
	}
}

#[test]
fn emit_rectangle_records() {
	let canvas = Canvas::new(200.0, 200.0);
	let top_left = Point::new(10.0, 10.0);
	let rect = Rectangle::new(top_left.clone(), 50.0, 60.0);
	core_adapter::emit_rectangle(&canvas, &rect, Color::from_rgb(5, 6, 7), 1.0, true);
	let cmds = canvas.drain_commands();
	assert_eq!(cmds.len(), 1);
	match &cmds[0] {
		Primitive::RoundedRect {
			rect: r,
			color,
			stroke,
			fill,
		} => {
			assert_eq!(r.top_left, top_left);
			assert_eq!(r.width, 50.0);
			assert_eq!(r.height, 60.0);
			assert_eq!(*color, Color::from_rgb(5, 6, 7));
			assert!((*stroke - 1.0).abs() < f32::EPSILON);
			assert!(*fill);
		}
		other => panic!("expected RoundedRect, got {:?}", other),
	}
}

#[test]
fn emit_polygon_and_polyline_records() {
	let canvas = Canvas::new(100.0, 100.0);
	let pts = vec![
		Point::new(0.0, 0.0),
		Point::new(10.0, 0.0),
		Point::new(10.0, 10.0),
	];
	let poly = Polygon::new(pts.clone());
	core_adapter::emit_polygon(&canvas, &poly, Color::from_rgb(1, 2, 3), 0.5, true);
	let pl = Polyline::new(pts.clone());
	core_adapter::emit_polyline(&canvas, &pl, Color::from_rgb(10, 11, 12), 0.8);
	let cmds = canvas.drain_commands();
	assert_eq!(cmds.len(), 2);
	match &cmds[0] {
		Primitive::Polygon {
			polygon,
			color,
			stroke,
			fill,
		} => {
			assert_eq!(polygon.points.len(), 3);
			assert_eq!(*color, Color::from_rgb(1, 2, 3));
			assert!((*stroke - 0.5).abs() < f32::EPSILON);
			assert!(*fill);
		}
		other => panic!("expected Polygon, got {:?}", other),
	}
	match &cmds[1] {
		Primitive::Polyline {
			polyline,
			color,
			stroke,
		} => {
			assert_eq!(polyline.points.len(), 3);
			assert_eq!(*color, Color::from_rgb(10, 11, 12));
			assert!((*stroke - 0.8).abs() < f32::EPSILON);
		}
		other => panic!("expected Polyline, got {:?}", other),
	}
}

#[test]
fn emit_text_records() {
	let canvas = Canvas::new(200.0, 200.0);
	let t = Text::new(Point::new(5.0, 5.0), "Hi".to_string(), 12.0);
	core_adapter::emit_text(&canvas, &t, Color::from_rgb(7, 8, 9));
	let cmds = canvas.drain_commands();
	assert_eq!(cmds.len(), 1);
	match &cmds[0] {
		Primitive::Text { text, color } => {
			assert_eq!(text.position.x, 5.0);
			assert_eq!(text.content, "Hi");
			assert_eq!(*color, Color::from_rgb(7, 8, 9));
		}
		other => panic!("expected Text, got {:?}", other),
	}
}
