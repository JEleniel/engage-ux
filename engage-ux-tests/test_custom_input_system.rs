//! Integration tests for custom input device support

use engage_ux_core::input::{CustomInputEvent, CustomInputValue, InputEvent, InputHandler};
use std::collections::HashMap;

// Test component that handles custom input
struct GamepadComponent {
	buttons_pressed: Vec<i64>,
	axis_values: HashMap<i64, f64>,
	events_handled: usize,
}

impl GamepadComponent {
	fn new() -> Self {
		Self {
			buttons_pressed: Vec::new(),
			axis_values: HashMap::new(),
			events_handled: 0,
		}
	}
}

impl InputHandler for GamepadComponent {
	fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
		if event.device_type != "gamepad" {
			return false;
		}

		self.events_handled += 1;

		match event.event_type.as_str() {
			"button_press" => {
				if let Some(button) = event.get_int("button") {
					self.buttons_pressed.push(button);
					true
				} else {
					false
				}
			}
			"axis_move" => {
				if let (Some(axis), Some(value)) = (event.get_int("axis"), event.get_float("value"))
				{
					self.axis_values.insert(axis, value);
					true
				} else {
					false
				}
			}
			_ => false,
		}
	}
}

#[test]
fn test_custom_input_event_creation() {
	let event = CustomInputEvent::new("gamepad", "button_press")
		.with_bool("pressed", true)
		.with_int("button", 0)
		.with_float("pressure", 0.8)
		.with_string("name", "A");

	assert_eq!(event.device_type, "gamepad");
	assert_eq!(event.event_type, "button_press");
	assert_eq!(event.get_bool("pressed"), Some(true));
	assert_eq!(event.get_int("button"), Some(0));
	assert_eq!(event.get_float("pressure"), Some(0.8));
	assert_eq!(event.get_string("name"), Some("A"));
}

#[test]
fn test_custom_input_data_types() {
	let mut event = CustomInputEvent::new("device", "test");

	// Bool
	event = event.with_bool("bool_val", true);
	assert_eq!(event.get_bool("bool_val"), Some(true));

	// Int
	event = event.with_int("int_val", 42);
	assert_eq!(event.get_int("int_val"), Some(42));

	// Float
	event = event.with_float("float_val", 3.14);
	assert_eq!(event.get_float("float_val"), Some(3.14));

	// String
	event = event.with_string("string_val", "test");
	assert_eq!(event.get_string("string_val"), Some("test"));
}

#[test]
fn test_custom_input_missing_values() {
	let event = CustomInputEvent::new("device", "test");

	assert_eq!(event.get_bool("nonexistent"), None);
	assert_eq!(event.get_int("nonexistent"), None);
	assert_eq!(event.get_float("nonexistent"), None);
	assert_eq!(event.get_string("nonexistent"), None);
}

#[test]
fn test_gamepad_input_handling() {
	let mut component = GamepadComponent::new();

	// Button press events
	let button_events = vec![
		CustomInputEvent::new("gamepad", "button_press")
			.with_int("button", 0)
			.with_bool("pressed", true),
		CustomInputEvent::new("gamepad", "button_press")
			.with_int("button", 1)
			.with_bool("pressed", true),
		CustomInputEvent::new("gamepad", "button_press")
			.with_int("button", 2)
			.with_bool("pressed", true),
	];

	for event in button_events {
		assert!(component.handle_custom(&event));
	}

	assert_eq!(component.buttons_pressed.len(), 3);
	assert_eq!(component.buttons_pressed, vec![0, 1, 2]);
	assert_eq!(component.events_handled, 3);
}

#[test]
fn test_gamepad_axis_handling() {
	let mut component = GamepadComponent::new();

	// Axis movement events
	let axis_events = vec![
		CustomInputEvent::new("gamepad", "axis_move")
			.with_int("axis", 0)
			.with_float("value", 0.5),
		CustomInputEvent::new("gamepad", "axis_move")
			.with_int("axis", 1)
			.with_float("value", -0.75),
		CustomInputEvent::new("gamepad", "axis_move")
			.with_int("axis", 0)
			.with_float("value", 1.0), // Update axis 0
	];

	for event in axis_events {
		assert!(component.handle_custom(&event));
	}

	assert_eq!(component.axis_values.len(), 2);
	assert_eq!(component.axis_values.get(&0), Some(&1.0)); // Last value for axis 0
	assert_eq!(component.axis_values.get(&1), Some(&-0.75));
	assert_eq!(component.events_handled, 3);
}

#[test]
fn test_input_event_custom_variant() {
	let custom = CustomInputEvent::new("device", "event").with_int("value", 42);
	let input_event = InputEvent::Custom(custom);

	match input_event {
		InputEvent::Custom(e) => {
			assert_eq!(e.device_type, "device");
			assert_eq!(e.event_type, "event");
			assert_eq!(e.get_int("value"), Some(42));
		}
		_ => panic!("Expected Custom variant"),
	}
}

#[test]
fn test_multiple_device_types() {
	struct MultiDeviceComponent {
		gamepad_events: usize,
		stylus_events: usize,
		motion_events: usize,
	}

	impl InputHandler for MultiDeviceComponent {
		fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
			match event.device_type.as_str() {
				"gamepad" => {
					self.gamepad_events += 1;
					true
				}
				"stylus" => {
					self.stylus_events += 1;
					true
				}
				"motion_sensor" => {
					self.motion_events += 1;
					true
				}
				_ => false,
			}
		}
	}

	let mut component = MultiDeviceComponent {
		gamepad_events: 0,
		stylus_events: 0,
		motion_events: 0,
	};

	let events = vec![
		CustomInputEvent::new("gamepad", "button_press"),
		CustomInputEvent::new("stylus", "press"),
		CustomInputEvent::new("motion_sensor", "accelerometer"),
		CustomInputEvent::new("gamepad", "axis_move"),
		CustomInputEvent::new("stylus", "move"),
	];

	for event in events {
		component.handle_custom(&event);
	}

	assert_eq!(component.gamepad_events, 2);
	assert_eq!(component.stylus_events, 2);
	assert_eq!(component.motion_events, 1);
}

#[test]
fn test_stylus_input() {
	struct StylusComponent {
		position: (f64, f64),
		pressure: f64,
		tilt: (f64, f64),
	}

	impl InputHandler for StylusComponent {
		fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
			if event.device_type != "stylus" {
				return false;
			}

			match event.event_type.as_str() {
				"press" | "move" => {
					if let (Some(x), Some(y)) = (event.get_float("x"), event.get_float("y")) {
						self.position = (x, y);
						if let Some(pressure) = event.get_float("pressure") {
							self.pressure = pressure;
						}
						true
					} else {
						false
					}
				}
				"tilt" => {
					if let (Some(x_tilt), Some(y_tilt)) =
						(event.get_float("x_tilt"), event.get_float("y_tilt"))
					{
						self.tilt = (x_tilt, y_tilt);
						true
					} else {
						false
					}
				}
				_ => false,
			}
		}
	}

	let mut component = StylusComponent {
		position: (0.0, 0.0),
		pressure: 0.0,
		tilt: (0.0, 0.0),
	};

	// Press event
	let press = CustomInputEvent::new("stylus", "press")
		.with_float("x", 100.0)
		.with_float("y", 200.0)
		.with_float("pressure", 0.8);
	assert!(component.handle_custom(&press));
	assert_eq!(component.position, (100.0, 200.0));
	assert_eq!(component.pressure, 0.8);

	// Tilt event
	let tilt = CustomInputEvent::new("stylus", "tilt")
		.with_float("x_tilt", 15.0)
		.with_float("y_tilt", -10.0);
	assert!(component.handle_custom(&tilt));
	assert_eq!(component.tilt, (15.0, -10.0));
}

#[test]
fn test_motion_sensor_input() {
	struct MotionComponent {
		acceleration: (f64, f64, f64),
		rotation: (f64, f64, f64),
	}

	impl InputHandler for MotionComponent {
		fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
			if event.device_type != "motion_sensor" {
				return false;
			}

			if let (Some(x), Some(y), Some(z)) =
				(event.get_float("x"), event.get_float("y"), event.get_float("z"))
			{
				match event.event_type.as_str() {
					"accelerometer" => {
						self.acceleration = (x, y, z);
						true
					}
					"gyroscope" => {
						self.rotation = (x, y, z);
						true
					}
					_ => false,
				}
			} else {
				false
			}
		}
	}

	let mut component = MotionComponent {
		acceleration: (0.0, 0.0, 0.0),
		rotation: (0.0, 0.0, 0.0),
	};

	// Accelerometer
	let accel = CustomInputEvent::new("motion_sensor", "accelerometer")
		.with_float("x", 0.2)
		.with_float("y", 9.8)
		.with_float("z", 0.1);
	assert!(component.handle_custom(&accel));
	assert_eq!(component.acceleration, (0.2, 9.8, 0.1));

	// Gyroscope
	let gyro = CustomInputEvent::new("motion_sensor", "gyroscope")
		.with_float("x", 0.01)
		.with_float("y", -0.02)
		.with_float("z", 0.05);
	assert!(component.handle_custom(&gyro));
	assert_eq!(component.rotation, (0.01, -0.02, 0.05));
}

#[test]
fn test_input_handler_unified_interface() {
	let mut component = GamepadComponent::new();

	// Create custom event
	let custom = CustomInputEvent::new("gamepad", "button_press").with_int("button", 0);

	// Use unified InputEvent
	let input_event = InputEvent::Custom(custom);

	// Handle through InputHandler trait
	assert!(component.handle_input(&input_event));
	assert_eq!(component.buttons_pressed, vec![0]);
}

#[test]
fn test_custom_input_value_types() {
	let values = vec![
		CustomInputValue::Bool(true),
		CustomInputValue::Int(42),
		CustomInputValue::Float(3.14),
		CustomInputValue::String("test".to_string()),
		CustomInputValue::Array(vec![
			CustomInputValue::Int(1),
			CustomInputValue::Int(2),
			CustomInputValue::Int(3),
		]),
	];

	// Verify each type can be created and cloned
	for value in values {
		let cloned = value.clone();
		assert_eq!(value, cloned);
	}
}
