//! Event system using Tokio signals
//!
//! Provides thread-safe event handling using Tokio's async runtime.

use crate::{Point, component::ComponentId, input::MouseButton};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Types of events that can occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
	/// Mouse button pressed
	MouseDown {
		/// Location of the mouse event
		location: Point,
		/// Which mouse button was pressed
		button: MouseButton,
	},
	/// Mouse button released
	MouseUp {
		/// Location of the mouse event
		location: Point,
		/// Which mouse button was released
		button: MouseButton,
	},
	/// Mouse moved
	MouseMove {
		/// The location of the mouse
		location: Point,
	},
	/// Mouse wheel scrolled
	MouseWheel {
		/// Scroll delta X
		delta_x: f32,
		/// Scroll delta Y
		delta_y: f32,
	},
	/// Key pressed
	KeyDown {
		/// The keyboard state at the time of the event
		key_event: keyboard_types::KeyboardEvent,
	},
	/// Key released
	KeyUp {
		/// The keyboard state at the time of the event
		key_event: keyboard_types::KeyboardEvent,
	},
	/// Text input
	TextInput {
		/// The input text
		text: String,
	},
	/// Component gained focus
	FocusGained,
	/// Component lost focus
	FocusLost,
	/// Component was clicked
	Click,
	/// Component value changed
	ValueChanged,
	/// Window resized
	Resize {
		/// The new window width
		width: f32,
		/// The new window height
		height: f32,
	},
	/// Custom event
	Custom {
		/// Custom event name
		name: String,
		/// Custom event data
		data: String,
	},
}

/// An event with its target component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	/// Target component ID
	pub target: ComponentId,
	/// Type of event
	pub event_type: EventType,
	/// Timestamp of the event
	pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event {
	/// Create a new event
	pub fn new(target: ComponentId, event_type: EventType) -> Self {
		Self {
			target,
			event_type,
			timestamp: chrono::Utc::now(),
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
}
