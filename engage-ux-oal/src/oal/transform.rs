/// A simple 2D transform for a View. Kept minimal for now.
#[derive(Debug, Clone)]
pub struct Transform {
	/// Translate the view by (x, y) in units
	pub translate: (f32, f32),
	/// Scale the view by (sx, sy)
	pub scale: (f32, f32),
	/// Rotation in degrees clockwise
	pub rotation_degrees: f32,
}

impl Transform {
	pub fn identity() -> Self {
		Self {
			translate: (0.0, 0.0),
			scale: (1.0, 1.0),
			rotation_degrees: 0.0,
		}
	}
}
