use engage_ux_components as components;
use engage_ux_core::component::{Component as ComponentTrait, ComponentId};
use engage_ux_themes as themes;

fn main() {
	println!("Engage UX demo — instantiating themes and components...");

	// Load built-in themes (light/dark)
	let light = themes::Theme::light().expect("build light theme");
	let dark = themes::Theme::dark().expect("build dark theme");
	println!("Loaded themes: '{}' and '{}'", light.name, dark.name);

	// Create a small collection of representative components to exercise APIs
	let mut next_id: ComponentId = 1u128;
	let mut components_box: Vec<Box<dyn ComponentTrait>> = Vec::new();

	components_box.push(Box::new(components::Button::new(next_id, "Press me")));
	next_id += 1;
	components_box.push(Box::new(components::Label::new(
		next_id,
		"Hello, Engage UX!",
	)));
	next_id += 1;
	components_box.push(Box::new(components::Checkbox::new(next_id, "Accept terms")));
	next_id += 1;
	components_box.push(Box::new(components::Slider::new(next_id, 0.0, 100.0)));
	next_id += 1;
	components_box.push(Box::new(components::TextInput::new(next_id)));
	next_id += 1;
	components_box.push(Box::new(components::Video::new(next_id, "")));
	next_id += 1;
	components_box.push(Box::new(components::Toggle::with_label(
		next_id,
		"Enable feature",
	)));
	next_id += 1;
	components_box.push(Box::new(components::Container::new(next_id)));
	next_id += 1;
	components_box.push(Box::new(
		components::Card::new(next_id).with_title("Demo Card"),
	));

	println!("Instantiated {} components:", components_box.len());
	for c in &components_box {
		// Print runtime type name plus id
		let type_name = std::any::type_name_of_val(&**c);
		println!(" - {} (id={})", type_name, c.id());
	}

	// Print a short snippet of the light theme JSON to demonstrate serialization
	if let Ok(json) = light.to_json() {
		let snippet: String = json.chars().take(200).collect();
		println!("Light theme JSON (snippet): {}", snippet);
	}

	println!(
		"Demo complete — no UI shown (headless demo). To run a visual demo, integrate with an OAL \
		 backend and a renderer."
	);
}
