//! Custom input device demonstration
//!
//! Shows how to create and handle custom input events from devices
//! like gamepads, stylus, motion sensors, etc.

use engage_ux_core::component::{Component, ComponentId, ComponentProperties, Rect};
use engage_ux_core::input::{CustomInputEvent, InputEvent, InputHandler};

// Example component that handles custom input
struct GameComponent {
	properties: ComponentProperties,
	player_x: f32,
	player_y: f32,
	score: i32,
}

impl GameComponent {
	fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			player_x: 0.0,
			player_y: 0.0,
			score: 0,
		}
	}

	fn handle_gamepad(&mut self, event: &CustomInputEvent) {
		match event.event_type.as_str() {
			"button_press" => {
				if let Some(button) = event.get_int("button") {
					println!("  Button {} pressed!", button);
					match button {
						0 => {
							// A button
							self.score += 10;
							println!("  Score increased to {}", self.score);
						}
						1 => {
							// B button
							println!("  Jump!");
						}
						_ => {}
					}
				}
			}
			"axis_move" => {
				if let Some(axis) = event.get_int("axis") {
					if let Some(value) = event.get_float("value") {
						println!("  Axis {} moved to {:.2}", axis, value);
						match axis {
							0 => {
								// Left stick X
								self.player_x += value as f32 * 5.0;
								println!("  Player X: {:.1}", self.player_x);
							}
							1 => {
								// Left stick Y
								self.player_y += value as f32 * 5.0;
								println!("  Player Y: {:.1}", self.player_y);
							}
							_ => {}
						}
					}
				}
			}
			_ => {}
		}
	}

	fn handle_stylus(&mut self, event: &CustomInputEvent) {
		match event.event_type.as_str() {
			"press" => {
				if let (Some(x), Some(y), Some(pressure)) = (
					event.get_float("x"),
					event.get_float("y"),
					event.get_float("pressure"),
				) {
					println!(
						"  Stylus press at ({:.1}, {:.1}) with pressure {:.2}",
						x, y, pressure
					);
				}
			}
			"move" => {
				if let (Some(x), Some(y)) = (event.get_float("x"), event.get_float("y")) {
					println!("  Stylus move to ({:.1}, {:.1})", x, y);
				}
			}
			"tilt" => {
				if let (Some(x_tilt), Some(y_tilt)) =
					(event.get_float("x_tilt"), event.get_float("y_tilt"))
				{
					println!("  Stylus tilt: ({:.2}, {:.2})", x_tilt, y_tilt);
				}
			}
			_ => {}
		}
	}

	fn handle_motion(&mut self, event: &CustomInputEvent) {
		match event.event_type.as_str() {
			"accelerometer" => {
				if let (Some(x), Some(y), Some(z)) = (
					event.get_float("x"),
					event.get_float("y"),
					event.get_float("z"),
				) {
					println!("  Accelerometer: x={:.2}, y={:.2}, z={:.2}", x, y, z);
				}
			}
			"gyroscope" => {
				if let (Some(x), Some(y), Some(z)) = (
					event.get_float("x"),
					event.get_float("y"),
					event.get_float("z"),
				) {
					println!("  Gyroscope: x={:.2}, y={:.2}, z={:.2}", x, y, z);
				}
			}
			_ => {}
		}
	}
}

impl Component for GameComponent {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn properties(&self) -> &ComponentProperties {
		&self.properties
	}

	fn properties_mut(&mut self) -> &mut ComponentProperties {
		&mut self.properties
	}
}

impl InputHandler for GameComponent {
	fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
		println!("\nReceived custom input from: {}", event.device_type);

		match event.device_type.as_str() {
			"gamepad" => {
				self.handle_gamepad(event);
				true
			}
			"stylus" => {
				self.handle_stylus(event);
				true
			}
			"motion_sensor" => {
				self.handle_motion(event);
				true
			}
			_ => {
				println!("  Unknown device type");
				false
			}
		}
	}
}

fn main() {
	println!("Engage UX - Custom Input Device Demo\n");
	println!("=====================================\n");

	let mut game = GameComponent::new(1);

	// Demo 1: Gamepad input
	println!("Demo 1: Gamepad Input");
	println!("---------------------");

	let gamepad_events = vec![
		CustomInputEvent::new("gamepad", "button_press")
			.with_int("button", 0)
			.with_bool("pressed", true)
			.with_string("button_name", "A"),
		CustomInputEvent::new("gamepad", "button_press")
			.with_int("button", 1)
			.with_bool("pressed", true)
			.with_string("button_name", "B"),
		CustomInputEvent::new("gamepad", "axis_move")
			.with_int("axis", 0)
			.with_float("value", 0.75)
			.with_string("axis_name", "left_x"),
		CustomInputEvent::new("gamepad", "axis_move")
			.with_int("axis", 1)
			.with_float("value", -0.5)
			.with_string("axis_name", "left_y"),
	];

	for event in gamepad_events {
		game.handle_custom(&event);
	}
	println!();

	// Demo 2: Stylus input
	println!("Demo 2: Stylus Input");
	println!("--------------------");

	let stylus_events = vec![
		CustomInputEvent::new("stylus", "press")
			.with_float("x", 100.0)
			.with_float("y", 200.0)
			.with_float("pressure", 0.8),
		CustomInputEvent::new("stylus", "move")
			.with_float("x", 105.0)
			.with_float("y", 205.0)
			.with_float("pressure", 0.75),
		CustomInputEvent::new("stylus", "tilt")
			.with_float("x_tilt", 15.0)
			.with_float("y_tilt", -10.0),
	];

	for event in stylus_events {
		game.handle_custom(&event);
	}
	println!();

	// Demo 3: Motion sensor input
	println!("Demo 3: Motion Sensor Input");
	println!("---------------------------");

	let motion_events = vec![
		CustomInputEvent::new("motion_sensor", "accelerometer")
			.with_float("x", 0.2)
			.with_float("y", 9.8)
			.with_float("z", 0.1),
		CustomInputEvent::new("motion_sensor", "gyroscope")
			.with_float("x", 0.01)
			.with_float("y", -0.02)
			.with_float("z", 0.05),
	];

	for event in motion_events {
		game.handle_custom(&event);
	}
	println!();

	// Demo 4: Generic InputEvent handling
	println!("Demo 4: Using InputEvent Enum");
	println!("------------------------------");

	let custom_event = CustomInputEvent::new("gamepad", "button_press")
		.with_int("button", 0)
		.with_bool("pressed", true);

	let input_event = InputEvent::Custom(custom_event);
	game.handle_input(&input_event);
	println!();

	// Demo 5: Custom device - MIDI controller
	println!("Demo 5: Custom Device - MIDI Controller");
	println!("----------------------------------------");

	let midi_events = vec![
		CustomInputEvent::new("midi_controller", "note_on")
			.with_int("note", 60)
			.with_int("velocity", 100)
			.with_int("channel", 0),
		CustomInputEvent::new("midi_controller", "control_change")
			.with_int("controller", 1)
			.with_int("value", 64)
			.with_int("channel", 0),
	];

	for event in midi_events {
		println!("\nReceived MIDI input:");
		println!("  Event: {}", event.event_type);
		if let Some(note) = event.get_int("note") {
			println!("  Note: {}", note);
		}
		if let Some(controller) = event.get_int("controller") {
			println!("  Controller: {}", controller);
		}
		if let Some(value) = event.get_int("value") {
			println!("  Value: {}", value);
		}
	}
	println!();

	// Demo 6: Custom device - Eye tracker
	println!("Demo 6: Custom Device - Eye Tracker");
	println!("------------------------------------");

	let eye_events = vec![
		CustomInputEvent::new("eye_tracker", "gaze_point")
			.with_float("x", 512.0)
			.with_float("y", 384.0)
			.with_bool("left_eye_open", true)
			.with_bool("right_eye_open", true),
		CustomInputEvent::new("eye_tracker", "blink").with_string("eye", "right"),
	];

	for event in eye_events {
		println!("\nReceived eye tracker input:");
		println!("  Event: {}", event.event_type);
		if let (Some(x), Some(y)) = (event.get_float("x"), event.get_float("y")) {
			println!("  Gaze point: ({:.1}, {:.1})", x, y);
		}
		if let Some(eye) = event.get_string("eye") {
			println!("  Eye: {}", eye);
		}
	}
	println!();

	println!("Custom input demo complete!");
	println!("\nFinal game state:");
	println!(
		"  Player position: ({:.1}, {:.1})",
		game.player_x, game.player_y
	);
	println!("  Score: {}", game.score);
}
