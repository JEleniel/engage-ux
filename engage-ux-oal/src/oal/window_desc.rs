/// Window descriptor used to request a new logical window from the OAL.
#[derive(Debug, Clone)]
pub struct WindowDesc {
	/// Initial window title.
	pub title: String,
	/// Initial size of the window in logical OAL units (width, height).
	pub size_in_units: (f32, f32),
}

impl WindowDesc {
	/// Create a new `WindowDesc` with the given title and logical size.
	pub fn new<T: Into<String>>(title: T, size_in_units: (f32, f32)) -> Self {
		Self {
			title: title.into(),
			size_in_units,
		}
	}
}
