use engage_ux_core::Color;
use engage_ux_core::geometry::Ellipse;
use engage_ux_oal::oal::canvas::{Canvas, Primitive};
use engage_ux_oal::oal::core_adapter;

#[test]
fn emit_ellipse_records_primitive() {
	let canvas = Canvas::new(100.0, 100.0);

	let e = Ellipse::new(engage_ux_core::geometry::Point::new(50.0, 50.0), 20.0, 10.0);
	core_adapter::emit_ellipse(&canvas, &e, Color::from_rgb(0, 0, 0), 1.0, true);

	let cmds = canvas.drain_commands();
	assert_eq!(cmds.len(), 1);

	match &cmds[0] {
		Primitive::Ellipse {
			ellipse,
			color,
			stroke,
			fill,
		} => {
			assert_eq!(ellipse, &e);
			assert_eq!(*color, Color::from_rgb(0, 0, 0));
			assert!((*stroke - 1.0).abs() < f32::EPSILON);
			assert!(*fill);
		}
		other => panic!("expected Ellipse primitive, got: {:?}", other),
	}
}
