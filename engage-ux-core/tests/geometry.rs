use engage_ux_core::geometry::{
	Border, Circle, Ellipse, Line, Move, Point, Polygon, Polyline, Rectangle, Text,
};

#[test]
fn test_point_and_offset_new() {
	let p = Point::new(1.5, 2.5);
	assert_eq!(p.x, 1.5);
	assert_eq!(p.y, 2.5);

	let o = Move::new(3.0, 4.0);
	assert_eq!(o.x, 3.0);
	assert_eq!(o.y, 4.0);
}

#[test]
fn test_rectangle_new_and_shrink() {
	let top_left = Point::new(0.0, 0.0);
	let r = Rectangle::new(top_left.clone(), 100.0, 50.0);
	assert_eq!(r.top_left, top_left);
	assert_eq!(r.width, 100.0);
	assert_eq!(r.height, 50.0);

	let border = Border {
		top: 2.0,
		right: 3.0,
		bottom: 4.0,
		left: 5.0,
	};
	let shrunk = r.shrink(&border);
	assert_eq!(shrunk.top_left.x, 5.0);
	assert_eq!(shrunk.top_left.y, 2.0);
	assert_eq!(shrunk.width, 92.0);
	assert_eq!(shrunk.height, 44.0);
}

#[test]
fn test_line_length_and_contains() {
	let a = Point::new(0.0, 0.0);
	let b = Point::new(3.0, 4.0);
	let line = Line::new(a.clone(), b.clone());
	let len = line.length();
	assert!((len - 5.0).abs() < 1e-6);

	let mid = Point::new(1.5, 2.0);
	assert!(line.contains_point(&mid));
	let off = Point::new(1.5, 2.1);
	assert!(!line.contains_point(&off));
}

#[test]
fn test_circle_area_and_contains() {
	let c = Circle::new(Point::new(0.0, 0.0), 2.0);
	let area = c.area();
	assert!((area - std::f32::consts::PI * 4.0).abs() < 1e-6);

	assert!(c.contains_point(&Point::new(1.0, 1.0)));
	assert!(!c.contains_point(&Point::new(3.0, 0.0)));
}

#[test]
fn test_polygon_contains_and_bbox() {
	// square from (0,0) to (10,10)
	let pts = vec![
		Point::new(0.0, 0.0),
		Point::new(10.0, 0.0),
		Point::new(10.0, 10.0),
		Point::new(0.0, 10.0),
	];
	let poly = Polygon::new(pts);
	assert!(poly.contains_point(&Point::new(5.0, 5.0)));
	assert!(!poly.contains_point(&Point::new(15.0, 5.0)));

	let bbox = poly.bounding_box().expect("bbox");
	assert_eq!(bbox.top_left.x, 0.0);
	assert_eq!(bbox.top_left.y, 0.0);
	assert_eq!(bbox.width, 10.0);
	assert_eq!(bbox.height, 10.0);
}

#[test]
fn test_polyline_length() {
	let pts = vec![
		Point::new(0.0, 0.0),
		Point::new(3.0, 4.0),
		Point::new(6.0, 4.0),
	];
	let pl = Polyline::new(pts);
	let len = pl.length();
	// segments: 5.0 and 3.0 => total 8.0
	assert!((len - 8.0).abs() < 1e-6);
}

#[test]
fn test_text_estimated_bbox() {
	let txt = Text::new(Point::new(0.0, 0.0), "Hello".to_string(), 12.0);
	let (pos, w, h) = txt.estimated_bbox();
	assert_eq!(pos.x, 0.0);
	assert_eq!(pos.y, 0.0);
	assert!((w - (0.6 * 12.0 * 5.0)).abs() < 1e-6);
	assert!((h - 12.0).abs() < 1e-6);
}

#[test]
fn test_ellipse_area_and_contains() {
	let e = Ellipse::new(Point::new(0.0, 0.0), 2.0, 1.0);
	let area = e.area();
	assert!((area - std::f32::consts::PI * 2.0).abs() < 1e-6);

	assert!(e.contains_point(&Point::new(1.0, 0.0)));
	assert!(e.contains_point(&Point::new(0.0, 0.5)));
	assert!(!e.contains_point(&Point::new(3.0, 0.0)));
}
