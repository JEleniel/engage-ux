use engage_ux_derive::Event;
use serde::Serialize;

// Invalid crate path string should be rejected by the macro when parsing into tokens
#[derive(Serialize, Event)]
#[event(crate = "!!not::a::path")]
struct BadCrate {
	x: u8,
}
