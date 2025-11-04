//! Custom input device demonstration
//!
//! Shows how to create and handle custom input events from devices
//! like gamepads, stylus, motion sensors, etc.

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::input::InputEvent;

// Provider-specific example event types used by this demo (gamepad, stylus, motion)
#[derive(Debug, Clone)]
pub struct GamepadEvent {
	pub event_type: String,
	pub button: Option<i64>,
	pub value: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct StylusEvent {
	pub event_type: String,
	pub x: Option<f64>,
	pub y: Option<f64>,
	pub pressure: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct MotionEvent {
	pub event_type: String,
	pub x: Option<f64>,
	pub y: Option<f64>,
	pub z: Option<f64>,
}

// Example component that handles provider-specific input
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

	fn handle_gamepad(&mut self, event: &GamepadEvent) {
		match event.event_type.as_str() {
			"button_press" => {
				if let Some(button) = event.button {
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
				if let Some(value) = event.value {
					println!("  Axis moved to {:.2}", value);
					// For demo, move player by value
					self.player_x += value as f32 * 5.0;
				}
			}
			_ => {}
		}
	}

	fn handle_stylus(&mut self, event: &StylusEvent) {
		match event.event_type.as_str() {
			"press" => {
				if let (Some(x), Some(y), Some(pressure)) = (event.x, event.y, event.pressure) {
					println!(
						"  Stylus press at ({:.1}, {:.1}) with pressure {:.2}",
						x, y, pressure
					);
				}
			}
			"move" => {
				if let (Some(x), Some(y)) = (event.x, event.y) {
					println!("  Stylus move to ({:.1}, {:.1})", x, y);
				}
			}
			"tilt" => {
				println!("  Stylus tilt event")
			}
			_ => {}
		}
	}

	fn handle_motion(&mut self, event: &MotionEvent) {
		match event.event_type.as_str() {
			"accelerometer" => {
				if let (Some(x), Some(y), Some(z)) = (event.x, event.y, event.z) {
					println!("  Accelerometer: x={:.2}, y={:.2}, z={:.2}", x, y, z);
				}
			}
			"gyroscope" => {
				if let (Some(x), Some(y), Some(z)) = (event.x, event.y, event.z) {
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

// This example handles provider-specific events directly; it does not
// implement the generic InputHandler::handle_custom method (removed in the
// migration). The following demos construct provider-specific events and call
// the matching handlers above.

fn main() {
	println!("Engage UX - Custom Input Device Demo\n");
	println!("=====================================\n");

	let mut game = GameComponent::new(1);

	// Demo 1: Gamepad input
	println!("Demo 1: Gamepad Input");
	println!("---------------------");

	let gamepad_events = vec![
		GamepadEvent {
			event_type: "button_press".to_string(),
			button: Some(0),
			value: None,
		},
		GamepadEvent {
			event_type: "button_press".to_string(),
			button: Some(1),
			value: None,
		},
		GamepadEvent {
			event_type: "axis_move".to_string(),
			button: None,
			value: Some(0.75),
		},
		GamepadEvent {
			event_type: "axis_move".to_string(),
			button: None,
			value: Some(-0.5),
		},
	];

	for event in gamepad_events {
		game.handle_gamepad(&event);
	}
	println!();

	// Demo 2: Stylus input
	println!("Demo 2: Stylus Input");
	println!("--------------------");

	let stylus_events = vec![
		StylusEvent {
			event_type: "press".to_string(),
			x: Some(100.0),
			y: Some(200.0),
			pressure: Some(0.8),
		},
		StylusEvent {
			event_type: "move".to_string(),
			x: Some(105.0),
			y: Some(205.0),
			pressure: Some(0.75),
		},
		StylusEvent {
			event_type: "tilt".to_string(),
			x: None,
			y: None,
			pressure: None,
		},
	];

	for event in stylus_events {
		game.handle_stylus(&event);
	}
	println!();

	// Demo 3: Motion sensor input
	println!("Demo 3: Motion Sensor Input");
	println!("---------------------------");

	let motion_events = vec![
		MotionEvent {
			event_type: "accelerometer".to_string(),
			x: Some(0.2),
			y: Some(9.8),
			z: Some(0.1),
		},
		MotionEvent {
			event_type: "gyroscope".to_string(),
			x: Some(0.01),
			y: Some(-0.02),
			z: Some(0.05),
		},
	];

	for event in motion_events {
		game.handle_motion(&event);
	}
	println!();

	// Demo 4: Generic InputEvent handling
	println!("Demo 4: Using InputEvent Enum");
	println!("------------------------------");

	// Demo: show InputEvent usage with mouse event as example
	let mouse_input = InputEvent::Mouse(engage_ux_core::input::mouse::MouseEvent::move_event(
		12.0, 34.0,
	));
	game.handle_input(&mouse_input);
	println!();

	// Demo 5: Custom device - MIDI controller
	println!("Demo 5: Custom Device - MIDI Controller");
	println!("----------------------------------------");

	let midi_events = vec![
		GamepadEvent {
			event_type: "note_on".to_string(),
			button: Some(60),
			value: Some(100.0),
		},
		GamepadEvent {
			event_type: "control_change".to_string(),
			button: Some(1),
			value: Some(64.0),
		},
	];

	for event in midi_events {
		println!("\nReceived MIDI-like input:");
		println!("  Event: {}", event.event_type);
		if let Some(note) = event.button {
			println!("  Note/Controller: {}", note);
		}
		if let Some(value) = event.value {
			println!("  Value: {}", value);
		}
	}
	println!();

	// Demo 6: Custom device - Eye tracker
	println!("Demo 6: Custom Device - Eye Tracker");
	println!("------------------------------------");

	let eye_events = vec![
		StylusEvent {
			event_type: "gaze_point".to_string(),
			x: Some(512.0),
			y: Some(384.0),
			pressure: None,
		},
		StylusEvent {
			event_type: "blink".to_string(),
			x: None,
			y: None,
			pressure: None,
		},
	];

	for event in eye_events {
		println!("\nReceived eye tracker input:");
		println!("  Event: {}", event.event_type);
		if let (Some(x), Some(y)) = (event.x, event.y) {
			println!("  Gaze point: ({:.1}, {:.1})", x, y);
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
