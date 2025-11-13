use crate::oal::transform::Transform;

/// `View` describes a logical sub-region of a `Window`'s `Canvas` together
/// with an optional `Transform` that renderers can apply when presenting.
#[derive(Debug, Clone)]
pub struct View {
	/// Top-left origin of the view in logical units.
	pub origin: (f32, f32),
	/// Size of the view (width, height) in logical units.
	pub size: (f32, f32),
	/// Optional transform applied to this view when rendering.
	pub transform: Option<Transform>,
}

impl View {
	/// Set the position of the view's origin.
	pub fn set_position(&mut self, pos: (f32, f32)) {
		self.origin = pos;
	}

	/// Set the view's logical size.
	pub fn set_size(&mut self, size: (f32, f32)) {
		self.size = size;
	}

	/// Replace the view transform with `transform`.
	pub fn set_transform(&mut self, transform: Option<Transform>) {
		self.transform = transform;
	}
}
