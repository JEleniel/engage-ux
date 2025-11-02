#![crate_type = "lib"]

use engage_ux_derive::Event;

#[derive(Event)]
#[event(crate = "engage_ux_core::events")]
pub enum BadEvent {
	A,
}

fn _f() {
	// The derive should fail to compile because `BadEvent` does not implement `serde::Serialize`.
	let _ = BadEvent::A;
}

fn main() {}
