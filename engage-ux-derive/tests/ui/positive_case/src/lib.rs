#![crate_type = "lib"]

use engage_ux_derive::Event;
use serde::Serialize;

#[derive(Serialize, Event)]
#[event(name = "test:positive", crate = "engage_ux_core::events")]
pub enum MyEvent {
	Foo { x: i32 },
}

fn use_event() -> Result<(), MyEventEventError> {
	let e = MyEvent::Foo { x: 1 }.try_to_event(1usize)?;
	// ensure Event is a real type we can drop
	drop(e);
	Ok(())
}

#[test]
fn compile_ok() {
	let _ = use_event();
}

fn main() {
	let _ = use_event();
}
