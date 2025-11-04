use crate::modules::event::category::EventCategory;
use crate::modules::event::event::Event;
use tokio::sync::{broadcast, mpsc};

/// A simple event bus that allows multi-subscriber broadcasting of `Event`.
///
/// Use `emit` to send an `Event` directly or `emit_payload` to send any type
/// convertible into `EventType` (e.g. types that `#[event]` derive generates).
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

	/// Emit a payload convertible into `EventType`.
	pub fn emit_payload<T, E>(&self, payload: T, target: u128) -> Result<(), E>
	where
		T: std::convert::TryInto<crate::modules::event::event_type::EventType, Error = E>,
	{
		let evt_type = payload.try_into()?;
		self.emit(Event::new(target, evt_type));
		Ok(())
	}

	/// Subscribe to receive ALL events. Each subscriber receives all future events.
	pub fn subscribe(&self) -> broadcast::Receiver<Event> {
		self.sender.subscribe()
	}

	/// Subscribe with a category filter. Returns an mpsc receiver that will
	/// receive only events whose type matches `category`.
	///
	/// This spawns a background task that forwards matching events from the
	/// internal broadcast receiver to the returned mpsc receiver.
	pub fn subscribe_category(&self, category: EventCategory) -> mpsc::Receiver<Event> {
		let mut rx = self.sender.subscribe();
		let (tx, rx_out) = mpsc::channel(16);

		// Spawn a background task to forward filtered events. We intentionally
		// detach this; subscribers receive an mpsc::Receiver and can await on it.
		tokio::spawn(async move {
			while let Ok(evt) = rx.recv().await {
				if evt.event_type.category() == category {
					// If the receiver is closed, stop the task.
					if tx.send(evt).await.is_err() {
						break;
					}
				}
			}
		});

		rx_out
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
	use crate::modules::event::event_type::EventType;

	#[test]
	fn test_event_creation() {
		let event = Event::new(1, EventType::Click);
		assert_eq!(event.source_component_id, 1);
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
		assert_eq!(got.source_component_id, 1);
	}

	// Async test to validate category subscription. Uses tokio runtime.
	#[tokio::test]
	async fn test_subscribe_category() {
		let bus = EventBus::default();
		let mut rx_filtered =
			bus.subscribe_category(crate::modules::event::category::EventCategory::Mouse);

		// Emit a mouse event and a non-mouse event (Resize)
		bus.emit(Event::new(1, EventType::Click));
		bus.emit(Event::new(
			2,
			EventType::Resize {
				width: 100.0,
				height: 200.0,
			},
		));

		// Should receive the mouse event via the filtered receiver
		if let Some(evt) = rx_filtered.recv().await {
			assert_eq!(evt.source_component_id, 1);
			assert_eq!(
				evt.event_type.category(),
				crate::modules::event::category::EventCategory::Mouse
			);
		} else {
			panic!("expected filtered event");
		}
	}

	// A simple demonstration of how to use the `Event` derive is provided in
	// the repository docs; deriving `Event` on user types generates a
	// `TryFrom<YourType> for events::EventType` impl which `EventBus::emit_payload`
	// will accept. We avoid compiling a derive-using test inside the same
	// crate to prevent macro path-resolution complexities; see documentation
	// or examples for usage.
}
