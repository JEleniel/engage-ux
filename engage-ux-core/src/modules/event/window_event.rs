use serde::{Deserialize, Serialize};

use crate::geometry::Rectangle;

/// Represents various window events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowEvent {
	/// The window has been requested to close.
	CloseRequested,
	/// The window has been resized.
	MovedOrResized {
		/// New window bounds as a `Rectangle`
		rectangle: Rectangle,
	},
	/// The window has been minimized.
	Minimized,
	/// The window has been restored.
	Restored,
	/// The window has been focused.
	Focused,
	/// The window has been unfocused.
	Unfocused,
	/// The window's title was changed.
	TitleChanged {
		/// New title string
		title: String,
	},
}
