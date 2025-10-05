//! Basic components example
//!
//! This example demonstrates how to create and use basic Engage UX components.
//!
//! Run with: cargo run --example basic_components

use engage_ux_components::{Button, Checkbox, Container, Label, TextInput};
use engage_ux_core::color::Color;
use engage_ux_core::component::Component;
use engage_ux_core::events::{Event, EventType};
use engage_ux_themes::Theme;

fn main() {
	println!("=== Engage UX - Basic Components Example ===\n");

	// Create a theme
	let theme = Theme::light();
	println!("Using theme: {}", theme.name);
	println!("Primary color: {:?}\n", theme.colors.primary);

	// Create a label
	let mut label = Label::new(1, "Welcome to Engage UX!");
	label.set_font_size(24.0);
	label.set_color(theme.colors.primary.clone());
	println!("Created Label:");
	println!("  ID: {}", label.id());
	println!("  Text: {}", label.text());
	println!("  Font Size: {}\n", label.font_size());

	// Create a button
	let mut button = Button::new(2, "Click Me!");
	button.set_background_color(theme.colors.primary.clone());
	button.set_on_click(|event| {
		println!("Button clicked! Event target: {}", event.target);
	});
	println!("Created Button:");
	println!("  ID: {}", button.id());
	println!("  Text: {}\n", button.text());

	// Simulate a button click
	let click_event = Event::new(2, EventType::Click);
	button.handle_click(&click_event);

	// Create a checkbox
	let mut checkbox = Checkbox::new(3, "Enable notifications");
	println!("\nCreated Checkbox:");
	println!("  ID: {}", checkbox.id());
	println!("  Checked: {}", checkbox.is_checked());

	checkbox.set_checked(true);
	println!("  After toggle: {}", checkbox.is_checked());

	// Create a text input
	let mut text_input = TextInput::new(4);
	text_input.set_value("Enter your name...");
	println!("\nCreated TextInput:");
	println!("  ID: {}", text_input.id());
	println!("  Value: {}", text_input.value());

	// Create a container to hold components
	let mut container = Container::new(5);
	container.add_child(label.id());
	container.add_child(button.id());
	container.add_child(checkbox.id());
	container.add_child(text_input.id());
	println!("\nCreated Container:");
	println!("  ID: {}", container.id());
	println!("  Children: {:?}", container.children());

	// Demonstrate color system
	println!("\n=== Color System Demo ===");
	let red = Color::rgb(1.0, 0.0, 0.0, 1.0);
	let red_hsl = red.to_hsl();
	println!("Red in RGB: {:?}", red.components());
	println!("Red in HSL: {:?}", red_hsl.components());

	let blue_hex = Color::from_hex("#0000FF").unwrap();
	println!("Blue from hex: {:?}", blue_hex.components());

	// Demonstrate theme system
	println!("\n=== Theme System Demo ===");
	let dark_theme = Theme::dark();
	println!("Dark theme name: {}", dark_theme.name);
	println!("Dark theme background: {:?}", dark_theme.colors.background);
	println!("Dark theme text: {:?}", dark_theme.colors.text_primary);

	// Serialize theme to JSON
	if let Ok(json) = theme.to_json() {
		println!("\nTheme serialized to JSON (first 200 chars):");
		println!("{}", &json[..json.len().min(200)]);
		println!("...");
	}

	println!("\n=== Example Complete ===");
	println!("This example demonstrated:");
	println!("  ✓ Creating UI components");
	println!("  ✓ Using the theme system");
	println!("  ✓ Working with colors");
	println!("  ✓ Event handling");
	println!("  ✓ Component composition with containers");
}
