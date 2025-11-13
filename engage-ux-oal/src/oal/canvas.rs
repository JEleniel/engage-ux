/// A small wrapper for a logical canvas size and a lightweight command
/// recording surface used by controls to emit primitive drawing commands.
///
/// The `Canvas` stores a logical size in OAL units and an interior-mutable
/// command list. Controls can obtain `&Canvas` and call drawing helpers
/// (e.g. `line`, `rect`) which record commands for the renderer to consume.
use std::sync::Mutex;

use engage_ux_core::Color;

#[derive(Debug, Clone)]
pub enum Primitive {
	Point {
		pos: (f32, f32),
		color: Color,
	},
	Line {
		a: (f32, f32),
		b: (f32, f32),
		color: Color,
		stroke: f32,
	},
	Circle {
		center: (f32, f32),
		radius: f32,
		color: Color,
		stroke: f32,
		fill: bool,
	},
	Oval {
		center: (f32, f32),
		radii: (f32, f32),
		color: Color,
		stroke: f32,
		fill: bool,
	},
	Rect {
		origin: (f32, f32),
		size: (f32, f32),
		color: Color,
		stroke: f32,
		fill: bool,
	},
	RoundedRect {
		origin: (f32, f32),
		size: (f32, f32),
		radii: (f32, f32),
		color: Color,
		stroke: f32,
		fill: bool,
	},
}

#[derive(Debug)]
pub struct Canvas {
	/// Size in logical units
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
	pub fn point(&self, pos: (f32, f32), color: Color) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Point { pos, color });
	}

	/// Record a line primitive.
	pub fn line(&self, a: (f32, f32), b: (f32, f32), color: Color, stroke: f32) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Line {
			a,
			b,
			color,
			stroke,
		});
	}

	/// Record a circle primitive.
	pub fn circle(&self, center: (f32, f32), radius: f32, color: Color, stroke: f32, fill: bool) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Circle {
			center,
			radius,
			color,
			stroke,
			fill,
		});
	}

	/// Record an oval primitive.
	pub fn oval(
		&self,
		center: (f32, f32),
		radii: (f32, f32),
		color: Color,
		stroke: f32,
		fill: bool,
	) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Oval {
			center,
			radii,
			color,
			stroke,
			fill,
		});
	}

	/// Record an axis-aligned rectangle.
	pub fn rect(
		&self,
		origin: (f32, f32),
		size: (f32, f32),
		color: Color,
		stroke: f32,
		fill: bool,
	) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::Rect {
			origin,
			size,
			color,
			stroke,
			fill,
		});
	}

	/// Record a rounded rectangle.
	pub fn rounded_rect(
		&self,
		origin: (f32, f32),
		size: (f32, f32),
		radii: (f32, f32),
		color: Color,
		stroke: f32,
		fill: bool,
	) {
		let mut guard = self.commands.lock().unwrap();
		guard.push(Primitive::RoundedRect {
			origin,
			size,
			radii,
			color,
			stroke,
			fill,
		});
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
