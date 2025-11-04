//! Integration tests migrated from generic custom input support to provider-specific events

use engage_ux_core::input::InputEvent;
use std::collections::HashMap;

// Provider-specific types for tests
#[derive(Clone, Debug)]
struct GamepadEvent {
	event_type: &'static str,
	button: Option<i64>,
	value: Option<f64>,
}

impl GamepadEvent {
	fn button_press(button: i64) -> Self {
		Self {
			event_type: "button_press",
			button: Some(button),
			value: None,
		}
	}
	fn axis_move(value: f64) -> Self {
		Self {
			event_type: "axis_move",
			button: None,
			value: Some(value),
		}
	}
}

// Test component that handles gamepad-like events directly
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

	fn handle_gamepad(&mut self, event: &GamepadEvent) -> bool {
		self.events_handled += 1;
		match event.event_type {
			"button_press" => {
				if let Some(button) = event.button {
					self.buttons_pressed.push(button);
					true
				} else {
					false
				}
			}
			"axis_move" => {
				if let Some(value) = event.value {
					// use axis index 0 for tests
					self.axis_values.insert(0, value);
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
fn test_gamepad_input_handling() {
	let mut component = GamepadComponent::new();

	// Button press events
	let button_events = vec![
		GamepadEvent::button_press(0),
		GamepadEvent::button_press(1),
		GamepadEvent::button_press(2),
	];

	for event in button_events {
		assert!(component.handle_gamepad(&event));
	}

	assert_eq!(component.buttons_pressed.len(), 3);
	assert_eq!(component.buttons_pressed, vec![0, 1, 2]);
	assert_eq!(component.events_handled, 3);
}

#[test]
fn test_gamepad_axis_handling() {
	let mut component = GamepadComponent::new();

	let axis_events = vec![
		GamepadEvent::axis_move(0.5),
		GamepadEvent::axis_move(-0.75),
		GamepadEvent::axis_move(1.0),
	];

	for event in axis_events {
		assert!(component.handle_gamepad(&event));
	}

	assert_eq!(component.axis_values.len(), 1);
	assert_eq!(component.axis_values.get(&0), Some(&1.0));
	assert_eq!(component.events_handled, 3);
}

#[test]
fn test_input_event_mouse_variant() {
	// Verify InputEvent still supports mouse variant
	let mouse = engage_ux_core::input::mouse::MouseEvent::move_event(1.0, 2.0);
	let input_event = InputEvent::Mouse(mouse);

	match input_event {
		InputEvent::Mouse(e) => assert_eq!(e.position(), (1.0, 2.0)),
		_ => panic!("Expected Mouse variant"),
	}
}
