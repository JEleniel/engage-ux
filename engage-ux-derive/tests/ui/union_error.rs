// This UI test should fail: the `Event` derive is only supported for structs and enums.
// The derive macro emits a syn::Error with the text "Event derive is only supported for structs and enums".

use engage_ux_derive::Event;

#[derive(Event)]
union BadEvent {
	a: u8,
}
