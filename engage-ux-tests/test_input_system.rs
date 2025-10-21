//! Integration tests for the input system

use engage_ux_core::input::{
	InputEvent, InputHandler, MouseButton, MouseEvent, Touch, TouchEvent, TouchPhase,
};

struct TestComponent {
	keyboard_count: usize,
	mouse_count: usize,
	touch_count: usize,
}

impl InputHandler for TestComponent {
	fn handle_keyboard(&mut self, _event: &KeyboardEvent) -> bool {
		self.keyboard_count += 1;
		true
	}

	fn handle_mouse(&mut self, _event: &MouseEvent) -> bool {
		self.mouse_count += 1;
		true
	}

	fn handle_touch(&mut self, _event: &TouchEvent) -> bool {
		self.touch_count += 1;
		true
	}
}

#[test]
fn test_unified_input_handling() {
	let mut component = TestComponent {
		keyboard_count: 0,
		mouse_count: 0,
		touch_count: 0,
	};

	// Test keyboard input
	let key_event = KeyboardEvent::key_down(KeyCode::Enter, KeyModifiers::empty());
	component.handle_input(&InputEvent::Keyboard(key_event));
	assert_eq!(component.keyboard_count, 1);

	// Test mouse input
	let mouse_event = MouseEvent::button_down(MouseButton::Left, 100.0, 100.0);
	component.handle_input(&InputEvent::Mouse(mouse_event));
	assert_eq!(component.mouse_count, 1);

	// Test touch input
	let touch_event = TouchEvent::new(TouchPhase::Began, vec![Touch::new(1, 100.0, 100.0)]);
	component.handle_input(&InputEvent::Touch(touch_event));
	assert_eq!(component.touch_count, 1);
}

#[test]
fn test_keyboard_modifiers_integration() {
	let mut component = TestComponent {
		keyboard_count: 0,
		mouse_count: 0,
		touch_count: 0,
	};

	// Test Ctrl+A
	let ctrl_a = KeyboardEvent::key_down(KeyCode::Key('a'), KeyModifiers::CTRL);
	assert!(ctrl_a.is_ctrl());
	component.handle_keyboard(&ctrl_a);
	assert_eq!(component.keyboard_count, 1);

	// Test Shift+Tab
	let shift_tab = KeyboardEvent::key_down(KeyCode::Tab, KeyModifiers::SHIFT);
	assert!(shift_tab.is_shift());
	component.handle_keyboard(&shift_tab);
	assert_eq!(component.keyboard_count, 2);
}

#[test]
fn test_multi_touch_gestures() {
	let mut component = TestComponent {
		keyboard_count: 0,
		mouse_count: 0,
		touch_count: 0,
	};

	// Two-finger touch
	let two_finger = TouchEvent::new(
		TouchPhase::Began,
		vec![Touch::new(1, 100.0, 100.0), Touch::new(2, 200.0, 200.0)],
	);

	assert!(two_finger.is_multi_touch());
	component.handle_touch(&two_finger);
	assert_eq!(component.touch_count, 1);
}
