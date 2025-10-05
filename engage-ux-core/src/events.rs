//! Event system using Tokio signals
//!
//! Provides thread-safe event handling using Tokio's async runtime.

use crate::component::ComponentId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Types of events that can occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
	/// Mouse button pressed
	MouseDown { x: f32, y: f32, button: MouseButton },
	/// Mouse button released
	MouseUp { x: f32, y: f32, button: MouseButton },
	/// Mouse moved
	MouseMove { x: f32, y: f32 },
	/// Mouse wheel scrolled
	MouseWheel { delta_x: f32, delta_y: f32 },
	/// Key pressed
	KeyDown {
		key: String,
		modifiers: KeyModifiers,
	},
	/// Key released
	KeyUp {
		key: String,
		modifiers: KeyModifiers,
	},
	/// Text input
	TextInput { text: String },
	/// Component gained focus
	FocusGained,
	/// Component lost focus
	FocusLost,
	/// Component was clicked
	Click,
	/// Component value changed
	ValueChanged,
	/// Window resized
	Resize { width: f32, height: f32 },
	/// Custom event
	Custom { name: String, data: String },
}

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
	Left,
	Right,
	Middle,
}

/// Keyboard modifiers
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct KeyModifiers {
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
	pub meta: bool,
}

/// An event with its target component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	pub target: ComponentId,
	pub event_type: EventType,
	pub timestamp: u64,
}

impl Event {
	pub fn new(target: ComponentId, event_type: EventType) -> Self {
		Self {
			target,
			event_type,
			timestamp: std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.unwrap_or_default()
				.as_millis() as u64,
		}
	}
}

/// Event handler callback type
pub type EventCallback = Arc<dyn Fn(&Event) + Send + Sync>;

/// Event handler for managing event subscriptions
pub struct EventHandler {
	sender: broadcast::Sender<Event>,
}

impl EventHandler {
	/// Create a new event handler
	pub fn new() -> Self {
		let (sender, _) = broadcast::channel(100);
		Self { sender }
	}

	/// Emit an event
	pub fn emit(&self, event: Event) {
		let _ = self.sender.send(event);
	}

	/// Subscribe to events
	pub fn subscribe(&self) -> broadcast::Receiver<Event> {
		self.sender.subscribe()
	}
}

impl Default for EventHandler {
	fn default() -> Self {
		Self::new()
	}
}

impl Clone for EventHandler {
	fn clone(&self) -> Self {
		Self {
			sender: self.sender.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_event_creation() {
		let event = Event::new(1, EventType::Click);
		assert_eq!(event.target, 1);
		assert!(matches!(event.event_type, EventType::Click));
	}

	#[test]
	fn test_event_handler() {
		let handler = EventHandler::new();
		let _receiver = handler.subscribe();

		let event = Event::new(1, EventType::Click);
		handler.emit(event.clone());

		// Note: In a real test, we'd use tokio runtime to test async
		// For now, just verify the handler was created
		assert!(handler.sender.receiver_count() > 0);
	}

	#[test]
	fn test_key_modifiers() {
		let modifiers = KeyModifiers {
			shift: true,
			ctrl: false,
			alt: false,
			meta: false,
		};
		assert!(modifiers.shift);
		assert!(!modifiers.ctrl);
	}
}
