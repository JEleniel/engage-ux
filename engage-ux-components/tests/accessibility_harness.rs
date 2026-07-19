//! Accessibility test harness
//!
//! Verifies that common interactive components expose basic accessibility
//! metadata (labels, alt text, placeholders) via their public APIs.

use engage_ux_components::*;

#[test]
fn accessibility_smoke() {
	// Button
	let btn = Button::new(100, "Submit");
	assert_eq!(btn.text(), "Submit");

	// Checkbox
	let cb = Checkbox::new(101, "Agree");
	assert_eq!(cb.label(), "Agree");

	// Text input (placeholder)
	let mut ti = TextInput::new(102);
	ti.set_placeholder("Enter name");
	assert_eq!(ti.placeholder(), "Enter name");

	// Label
	let lbl = Label::new(103, "Display");
	assert_eq!(lbl.text(), "Display");

	// Image alt text
	let mut img = Image::new(104, "img.png");
	img.set_alt_text("Alt text");
	assert_eq!(img.alt_text(), "Alt text");
}
