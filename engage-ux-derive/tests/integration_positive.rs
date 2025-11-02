use engage_ux_derive::Event;
use serde::Serialize;

#[derive(Serialize, Event)]
#[event(name = "test:integration_positive", crate = "engage_ux_core::events")]
pub enum IntegrationEvent {
	Ping { v: u32 },
}

#[test]
fn integration_positive() -> Result<(), IntegrationEventEventError> {
	// Ensure the generated `try_to_event` exists and returns an `Event`.
	let ev = IntegrationEvent::Ping { v: 42 }.try_to_event(0usize)?;
	drop(ev);
	Ok(())
}
