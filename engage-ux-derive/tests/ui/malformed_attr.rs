use engage_ux_derive::Event;
use serde::Serialize;

// Malformed attribute value (non-literal) should cause a parse error
#[derive(Serialize, Event)]
#[event(name = 123)]
struct BadAttr {
	x: u8,
}
