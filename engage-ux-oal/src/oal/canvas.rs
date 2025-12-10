//! Lightweight command recording surface used by UI controls to emit
//! drawing primitives. The `Canvas` is intentionally small and stores a
//! logical size (in OAL units) and an interior-mutable command list that
//! consumers may record into from a shared reference.
use std::sync::Mutex;

use engage_ux_core::Color;
use engage_ux_core::geometry::{Circle, Ellipse, Line, Point, Polygon, Polyline, Rectangle, Text};
// styling structs are represented by primitive fields; no direct import needed

/// Primitive drawing commands recorded into a `Canvas`.
///
/// Each variant holds the minimal data the renderer needs. Coordinates are
/// expressed in OAL logical units (device-independent f32s). Renderers are
/// responsible for layout, font shaping and rasterization details.
#[derive(Debug, Clone)]
pub enum Primitive {
	/// A single point at `pos` with `color`.
	Point {
		/// Logical coordinates of the point.
		pos: Point,
		/// Color of the point
		color: Color,
	},

	/// A stroked line with stroke width and color.
	Line {
		/// Geometric line (start/end) from core geometry.
		line: Line,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
	},

	/// A circle based on the core `Circle` primitive.
	Circle {
		/// Core circle (center + radius).
		circle: Circle,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
		/// Filled
		fill: bool,
	},

	/// An ellipse primitive. Uses core `Ellipse` for center and radii.
	Ellipse {
		/// Core ellipse (center + rx/ry).
		ellipse: Ellipse,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
		/// Filled
		fill: bool,
	},

	/// An axis-aligned rectangle. Uses the core `Rectangle` type which also
	/// includes per-corner radii if needed.
	Rect {
		/// Core rectangle describing position, size and corner radii.
		rect: Rectangle,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
		/// Filled
		fill: bool,
	},

	/// A rounded rectangle. Internally uses the core `Rectangle` which has
	/// corner radii fields; this variant exists for clarity in recordings.
	RoundedRect {
		/// Core rectangle with corner radii set.
		rect: Rectangle,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
		/// Filled
		fill: bool,
	},

	/// A closed polygon. Uses the core `Polygon` type for point storage and
	/// polygon operations.
	Polygon {
		/// Core polygon.
		polygon: Polygon,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
		/// Filled
		fill: bool,
	},

	/// An open polyline composed of straight segments. Uses core `Polyline`.
	Polyline {
		/// Core polyline.
		polyline: Polyline,
		/// Stroke color
		color: Color,
		/// Stroke width
		stroke: f32,
	},

	/// Text to be drawn. Uses the core `Text` primitive which contains
	/// position, content and font size fields. Renderer performs shaping.
	Text {
		/// Core text primitive.
		text: Text,
		/// Text color
		color: Color,
	},
}

/// A thread-safe, interior-mutable canvas used to record drawing commands.
///
/// `Canvas` is intentionally minimal: it exposes helpers to record primitives
/// and a `drain_commands` method that returns and clears the recorded
/// commands for consumption by a renderer.
#[derive(Debug)]
pub struct Canvas {
	/// Size in logical units (width, height).
	pub size_in_units: (f32, f32),
	/// Recorded draw commands. Interior-mutable so callers with `&Canvas`
	/// can emit commands without requiring a mutable borrow of the window.
	commands: Mutex<Vec<Primitive>>,
}

impl Clone for Canvas {
	fn clone(&self) -> Self {
		let cmds = match self.commands.lock() {
			Ok(g) => g.clone(),
			Err(_) => Vec::new(),
		};
		Self {
			size_in_units: self.size_in_units,
			commands: Mutex::new(cmds),
		}
	}
}

impl Canvas {
	/// Create a new `Canvas` with the given logical width and height.
	pub fn new(w: f32, h: f32) -> Self {
		Self {
			size_in_units: (w, h),
			commands: Mutex::new(Vec::new()),
		}
	}

	/// Return the logical size of the canvas.
	pub fn logical_size(&self) -> (f32, f32) {
		self.size_in_units
	}

	/// Resize the canvas to the supplied logical size.
	pub fn resize(&mut self, new_size: (f32, f32)) {
		self.size_in_units = new_size;
	}

	/// Record a point primitive.
	pub fn point(&self, mut pos: Point, color: Color) {
		let mut guard = self.commands.lock().unwrap();
		// Point has no inline style field in core; record color with the primitive.
		guard.push(Primitive::Point { pos, color });
	}

	/// Record a line primitive.
	pub fn line(&self, line: Line, color: Color, stroke: f32) {
		let mut guard = self.commands.lock().unwrap();
		// Do not mutate core geometry: record styling in the primitive
		guard.push(Primitive::Line {
			line,
			color,
			stroke,
		});
	}

	/// Record a circle primitive.
	pub fn circle(&self, circle: Circle, color: Color, stroke: f32, fill: bool) {
		let mut guard = self.commands.lock().unwrap();
		// Preserve original core geometry; styling is carried by the
		// primitive's `color`/`stroke`/`fill` fields.
		guard.push(Primitive::Circle {
			circle,
			color,
			stroke,
			fill,
		});
	}

	/// Record an ellipse primitive.
	pub fn ellipse(&self, ellipse: Ellipse, color: Color, stroke: f32, fill: bool) {
		let mut guard = self.commands.lock().unwrap();
		// Do not alter the provided `Ellipse`; styling is recorded
		// separately on the primitive.
		guard.push(Primitive::Ellipse {
			ellipse,
			color,
			stroke,
			fill,
		});
	}

	/// Record an axis-aligned rectangle.
	pub fn rect(&self, rect: Rectangle, color: Color, stroke: f32, fill: bool) {
		let mut guard = self.commands.lock().unwrap();
		// Preserve core rectangle data; styling is attached to the primitive.
		guard.push(Primitive::Rect {
			rect,
			color,
			stroke,
			fill,
		});
	}

	/// Record a rounded rectangle.
	pub fn rounded_rect(&self, rect: Rectangle, color: Color, stroke: f32, fill: bool) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::RoundedRect {
			rect,
			color,
			stroke,
			fill,
		});
	}

	/// Record a polygon (closed) primitive. Points should be in logical units
	/// and will be interpreted in order; the polygon will be closed automatically
	/// by the renderer.
	pub fn polygon(&self, polygon: Polygon, color: Color, stroke: f32, fill: bool) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Polygon {
			polygon,
			color,
			stroke,
			fill,
		});
	}

	/// Record a polyline (open) primitive.
	pub fn polyline(&self, polyline: Polyline, color: Color, stroke: f32) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Polyline {
			polyline,
			color,
			stroke,
		});
	}

	/// Record a text primitive. `text` is owned; font handling/layout is the
	/// responsibility of the renderer implementation.
	pub fn text(&self, text: Text, color: Color) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Text { text, color });
	}

	/// Drain and return recorded commands. The renderer can call this to
	/// obtain the current list of primitives to rasterize and present.
	pub fn drain_commands(&self) -> Vec<Primitive> {
		match self.commands.lock() {
			Ok(mut g) => std::mem::take(&mut *g),
			Err(_) => Vec::new(),
		}
	}
}
