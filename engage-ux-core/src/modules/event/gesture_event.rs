use serde::{Deserialize, Serialize};

/// Represents a gesture event with its associated data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureEvent {
	/// The type of gesture event.
	pub kind: GestureKind,
	/// The scale factor of the gesture.
	pub scale: f32,
	/// The rotation angle of the gesture.
	pub rotation: f32,
	/// The translation offset of the gesture.
	pub translation: (f32, f32),
}

/// Enum representing different kinds of gesture events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GestureKind {
	/// A pinch gesture, typically used for zooming.
	Pinch,
	/// A rotate gesture, typically used for rotating objects.
	Rotate,
	/// A swipe gesture, typically used for scrolling or navigation.
	Swipe,
	/// Other gesture types not covered by the predefined variants.
	Other(String),
}
