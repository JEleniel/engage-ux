//! Touch input handling for multi-touch devices

use std::collections::HashMap;

/// Touch phase during a touch sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TouchPhase {
	/// Touch started
	Began,
	/// Touch moved
	Moved,
	/// Touch ended
	Ended,
	/// Touch cancelled (e.g., by system)
	Cancelled,
}

/// Represents a single touch point
#[derive(Debug, Clone, PartialEq)]
pub struct Touch {
	/// Unique identifier for this touch
	pub id: u64,
	/// X coordinate
	pub x: f32,
	/// Y coordinate
	pub y: f32,
	/// Touch pressure (0.0 to 1.0, if available)
	pub pressure: Option<f32>,
	/// Touch radius (if available)
	pub radius: Option<f32>,
}

impl Touch {
	/// Create a new touch point
	pub fn new(id: u64, x: f32, y: f32) -> Self {
		Self {
			id,
			x,
			y,
			pressure: None,
			radius: None,
		}
	}

	/// Create a new touch with pressure
	pub fn with_pressure(id: u64, x: f32, y: f32, pressure: f32) -> Self {
		Self {
			id,
			x,
			y,
			pressure: Some(pressure),
			radius: None,
		}
	}

	/// Get position as tuple
	pub fn position(&self) -> (f32, f32) {
		(self.x, self.y)
	}

	/// Calculate distance to another touch
	pub fn distance_to(&self, other: &Touch) -> f32 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		(dx * dx + dy * dy).sqrt()
	}
}

/// Touch event containing one or more touch points
#[derive(Debug, Clone, PartialEq)]
pub struct TouchEvent {
	/// Phase of the touch event
	pub phase: TouchPhase,
	/// All active touch points
	pub touches: Vec<Touch>,
	/// Timestamp of the event (if available)
	pub timestamp: Option<f64>,
}

impl TouchEvent {
	/// Create a new touch event
	pub fn new(phase: TouchPhase, touches: Vec<Touch>) -> Self {
		Self {
			phase,
			touches,
			timestamp: None,
		}
	}

	/// Create a touch event with timestamp
	pub fn with_timestamp(phase: TouchPhase, touches: Vec<Touch>, timestamp: f64) -> Self {
		Self {
			phase,
			touches,
			timestamp: Some(timestamp),
		}
	}

	/// Get the primary touch (first touch point)
	pub fn primary_touch(&self) -> Option<&Touch> {
		self.touches.first()
	}

	/// Get number of active touches
	pub fn touch_count(&self) -> usize {
		self.touches.len()
	}

	/// Check if this is a multi-touch gesture
	pub fn is_multi_touch(&self) -> bool {
		self.touches.len() > 1
	}
}

/// Touch state tracker for managing multi-touch interactions
#[derive(Debug, Default)]
pub struct TouchState {
	/// Currently active touches
	active_touches: HashMap<u64, Touch>,
}

impl TouchState {
	/// Create a new touch state
	pub fn new() -> Self {
		Self::default()
	}

	/// Update state with a touch event
	pub fn update(&mut self, event: &TouchEvent) {
		match event.phase {
			TouchPhase::Began => {
				for touch in &event.touches {
					self.active_touches.insert(touch.id, touch.clone());
				}
			}
			TouchPhase::Moved => {
				for touch in &event.touches {
					if let Some(existing) = self.active_touches.get_mut(&touch.id) {
						*existing = touch.clone();
					}
				}
			}
			TouchPhase::Ended | TouchPhase::Cancelled => {
				for touch in &event.touches {
					self.active_touches.remove(&touch.id);
				}
			}
		}
	}

	/// Get a specific active touch by ID
	pub fn get_touch(&self, id: u64) -> Option<&Touch> {
		self.active_touches.get(&id)
	}

	/// Get all active touches
	pub fn active_touches(&self) -> Vec<&Touch> {
		self.active_touches.values().collect()
	}

	/// Get number of active touches
	pub fn touch_count(&self) -> usize {
		self.active_touches.len()
	}

	/// Check if there are any active touches
	pub fn has_active_touches(&self) -> bool {
		!self.active_touches.is_empty()
	}

	/// Clear all active touches
	pub fn clear(&mut self) {
		self.active_touches.clear();
	}

	/// Calculate centroid of all active touches
	pub fn centroid(&self) -> Option<(f32, f32)> {
		if self.active_touches.is_empty() {
			return None;
		}

		let mut sum_x = 0.0;
		let mut sum_y = 0.0;
		let count = self.active_touches.len() as f32;

		for touch in self.active_touches.values() {
			sum_x += touch.x;
			sum_y += touch.y;
		}

		Some((sum_x / count, sum_y / count))
	}
}

/// Gesture recognizer for common touch gestures
#[derive(Debug)]
#[allow(dead_code)]
pub struct GestureRecognizer {
	initial_distance: Option<f32>,
	initial_centroid: Option<(f32, f32)>,
}

impl GestureRecognizer {
	/// Create a new gesture recognizer
	pub fn new() -> Self {
		Self {
			initial_distance: None,
			initial_centroid: None,
		}
	}

	/// Detect pinch gesture (two-finger zoom)
	pub fn detect_pinch(&mut self, event: &TouchEvent) -> Option<f32> {
		if event.touches.len() != 2 {
			self.initial_distance = None;
			return None;
		}

		let distance = event.touches[0].distance_to(&event.touches[1]);

		match event.phase {
			TouchPhase::Began => {
				self.initial_distance = Some(distance);
				None
			}
			TouchPhase::Moved => self.initial_distance.map(|initial| distance / initial),
			_ => {
				self.initial_distance = None;
				None
			}
		}
	}

	/// Detect pan gesture
	pub fn detect_pan(&mut self, event: &TouchEvent) -> Option<(f32, f32)> {
		if event.touches.is_empty() {
			self.initial_centroid = None;
			return None;
		}

		let centroid = calculate_centroid(&event.touches);

		match event.phase {
			TouchPhase::Began => {
				self.initial_centroid = Some(centroid);
				None
			}
			TouchPhase::Moved => self
				.initial_centroid
				.map(|initial| (centroid.0 - initial.0, centroid.1 - initial.1)),
			_ => {
				self.initial_centroid = None;
				None
			}
		}
	}

	/// Reset gesture state
	pub fn reset(&mut self) {
		self.initial_distance = None;
		self.initial_centroid = None;
	}
}

impl Default for GestureRecognizer {
	fn default() -> Self {
		Self::new()
	}
}

fn calculate_centroid(touches: &[Touch]) -> (f32, f32) {
	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let count = touches.len() as f32;

	for touch in touches {
		sum_x += touch.x;
		sum_y += touch.y;
	}

	(sum_x / count, sum_y / count)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_touch_creation() {
		let touch = Touch::new(1, 100.0, 200.0);
		assert_eq!(touch.id, 1);
		assert_eq!(touch.position(), (100.0, 200.0));
	}

	#[test]
	fn test_touch_distance() {
		let touch1 = Touch::new(1, 0.0, 0.0);
		let touch2 = Touch::new(2, 3.0, 4.0);
		assert_eq!(touch1.distance_to(&touch2), 5.0);
	}

	#[test]
	fn test_touch_event() {
		let touches = vec![Touch::new(1, 100.0, 100.0)];
		let event = TouchEvent::new(TouchPhase::Began, touches);
		assert_eq!(event.phase, TouchPhase::Began);
		assert_eq!(event.touch_count(), 1);
		assert!(!event.is_multi_touch());
	}

	#[test]
	fn test_touch_state() {
		let mut state = TouchState::new();

		let began = TouchEvent::new(TouchPhase::Began, vec![Touch::new(1, 100.0, 100.0)]);
		state.update(&began);
		assert_eq!(state.touch_count(), 1);
		assert!(state.has_active_touches());

		let ended = TouchEvent::new(TouchPhase::Ended, vec![Touch::new(1, 100.0, 100.0)]);
		state.update(&ended);
		assert_eq!(state.touch_count(), 0);
		assert!(!state.has_active_touches());
	}

	#[test]
	fn test_multi_touch_state() {
		let mut state = TouchState::new();

		let began = TouchEvent::new(
			TouchPhase::Began,
			vec![Touch::new(1, 100.0, 100.0), Touch::new(2, 200.0, 200.0)],
		);
		state.update(&began);
		assert_eq!(state.touch_count(), 2);
	}

	#[test]
	fn test_centroid() {
		let mut state = TouchState::new();

		let began = TouchEvent::new(
			TouchPhase::Began,
			vec![Touch::new(1, 0.0, 0.0), Touch::new(2, 100.0, 100.0)],
		);
		state.update(&began);

		let centroid = state.centroid().unwrap();
		assert_eq!(centroid, (50.0, 50.0));
	}

	#[test]
	fn test_gesture_recognizer_pinch() {
		let mut recognizer = GestureRecognizer::new();

		let began = TouchEvent::new(
			TouchPhase::Began,
			vec![Touch::new(1, 0.0, 0.0), Touch::new(2, 100.0, 0.0)],
		);
		recognizer.detect_pinch(&began);

		let moved = TouchEvent::new(
			TouchPhase::Moved,
			vec![Touch::new(1, 0.0, 0.0), Touch::new(2, 200.0, 0.0)],
		);
		let scale = recognizer.detect_pinch(&moved);
		assert!(scale.is_some());
		assert_eq!(scale.unwrap(), 2.0);
	}
}
