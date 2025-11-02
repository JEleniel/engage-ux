use engage_ux_derive::Event;

// Missing serde::Serialize should produce a trait-bound error from generated code
#[derive(Event)]
#[event("no_serialize")]
struct NotSerializable {
	x: u32,
}
