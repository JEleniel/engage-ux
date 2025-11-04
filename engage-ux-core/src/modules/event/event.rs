use crate::modules::event::event_type::EventType;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// An event with its target component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	/// Target component ID
	pub source_component_id: u128,
	/// Type of event
	pub event_type: EventType,
	/// Timestamp of the event
	pub timestamp: DateTime<Utc>,
}

impl Event {
	/// Create a new event
	pub fn new(source_component_id: u128, event_type: EventType) -> Self {
		Self {
			source_component_id,
			event_type,
			timestamp: Utc::now(),
		}
	}
}
