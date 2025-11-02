//! Event system using Tokio signals
//!
//! Provides thread-safe event handling using Tokio's async runtime.

use crate::Point;
use crate::input::mouse::MouseButton;
use crate::types::ComponentId;
use serde::{Deserialize, Serialize};
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

/// A simple event bus that allows multi-subscriber broadcasting of `Event`.
///
/// This is intentionally small and ergonomic for use with the derive crate
/// which generates `TryFrom<YourType> for EventType` implementations. Use
/// `emit` to send an `Event` directly or `emit_payload` to send any type
/// convertible into `EventType` (e.g. types that `#[derive(Event)]`).
pub struct EventBus {
	sender: broadcast::Sender<Event>,
}

impl EventBus {
	/// Create a new bus with a bounded channel capacity.
	pub fn new(capacity: usize) -> Self {
		let (sender, _) = broadcast::channel(capacity);
		Self { sender }
	}

	/// Create a new bus with a default capacity (100).
	pub fn default() -> Self {
		Self::new(100)
	}

	/// Emit a pre-built `Event`.
	pub fn emit(&self, event: Event) {
		let _ = self.sender.send(event);
	}

	/// Emit a payload convertible into `EventType`. This is the idiomatic
	/// integration point with `#[derive(Event)]` — the derive macro implements
	/// `TryFrom<T> for EventType` so this method will accept your typed event
	/// payload and construct an `Event` for the given target.
	pub fn emit_payload<T, E>(&self, payload: T, target: ComponentId) -> Result<(), E>
	where
		T: std::convert::TryInto<EventType, Error = E>,
	{
		// The derive crate generates `TryFrom<T> for EventType` which yields
		// a `TryInto<EventType>` implementation. Use that here to convert
		// typed payloads (from `#[derive(Event)]`) into the core EventType.
		let evt_type = payload.try_into()?;
		self.emit(Event::new(target, evt_type));
		Ok(())
	}

	/// Subscribe to events. Each subscriber receives all future events.
	pub fn subscribe(&self) -> broadcast::Receiver<Event> {
		self.sender.subscribe()
	}

	/// Number of active receivers.
	pub fn receiver_count(&self) -> usize {
		self.sender.receiver_count()
	}
}

impl Clone for EventBus {
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
		let bus = EventBus::default();
		let mut rx = bus.subscribe();

		let event = Event::new(1, EventType::Click);
		bus.emit(event.clone());

		// The subscriber should receive the event we just emitted
		let got = rx.try_recv().expect("expected event");
		assert_eq!(got.target, 1);
	}
}
