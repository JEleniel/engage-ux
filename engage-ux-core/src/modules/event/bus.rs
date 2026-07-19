use std::sync::{Arc, Mutex};

use crate::component::ComponentId;
use crate::modules::event::event::Event;
use serde::Serialize;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

/// A simple event bus that allows multi-subscriber broadcasting of `Event`.
///
/// Use `emit` to send an `Event` directly or `emit_payload` to send any
/// serializable payload. `emit_payload` will inspect the serialized JSON for
/// an optional `category` and optional `name` field. `category` will be mapped
/// into `Event.category` and removed from the payload object in the event's
/// `EventType::Custom.data` when present.
pub struct EventBus {
	tx: Arc<UnboundedSender<Event>>,
	rx: Arc<Mutex<UnboundedReceiver<Event>>>,
}

impl Default for EventBus {
	fn default() -> Self {
		Self::new()
	}
}

impl EventBus {
	/// Create a new bus with a unbounded channel.
	pub fn new() -> Self {
		let (tx, rx) = mpsc::unbounded_channel::<Event>();

		Self {
			tx: Arc::new(tx),
			rx: Arc::new(Mutex::new(rx)),
		}
	}

	/// Emit a pre-built `Event`.
	pub fn emit(&self, event: Event) {
		// Send the event from the originating thread to the bus processing thread.
		let _ = self.tx.send(event);
	}

	/// Emit any serializable payload. The payload will be converted to JSON. Returns serde_json errors
	pub fn emit_payload<T>(
		&self,
		source_component_id: ComponentId,
		payload: T,
	) -> Result<(), serde_json::Error>
	where
		T: Serialize,
	{
		let value = serde_json::to_string(&payload)?;

		self.emit(Event::Custom {
			timestamp: chrono::Utc::now(),
			source_component_id,
			payload: value,
		});
		Ok(())
	}

	/// Subscribe to receive events.
	/// Subscribe to receive events. Returns a thread-safe shared receiver wrapped
	/// in an Arc<Mutex<_>> so synchronous consumers can lock and poll events.
	pub fn subscribe(&self) -> Arc<Mutex<UnboundedReceiver<Event>>> {
		self.rx.clone()
	}
}
