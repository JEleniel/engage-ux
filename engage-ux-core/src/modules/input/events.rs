//! Input events and handler implementations. Primary types: `InputEvent`, `CustomInputEvent`, `CustomInputValue`, `InputHandler`.

// Import the concrete input event types from the sibling submodules.
#[allow(unused_imports)]
use crate::input::keyboard::{Key, KeyCode, KeyModifiers, KeyboardEvent, KeyboardState};
#[allow(unused_imports)]
use crate::input::mouse::{MouseButton, MouseEvent, MouseState};
#[allow(unused_imports)]
use crate::input::touch::{Touch, TouchEvent, TouchPhase, TouchState};

/// Unified input event that can represent any input type
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Keyboard input event
    Keyboard(KeyboardEvent),
    /// Mouse input event
    Mouse(MouseEvent),
    /// Touch input event
    Touch(TouchEvent),
    /// Custom input event from other devices
    Custom(CustomInputEvent),
}

/// Custom input event for supporting additional input devices
/// (gamepad, stylus, motion sensors, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct CustomInputEvent {
    /// Device type identifier
    pub device_type: String,
    /// Event type identifier
    pub event_type: String,
    /// Event data as key-value pairs
    pub data: std::collections::HashMap<String, CustomInputValue>,
}

/// Value type for custom input data
#[derive(Debug, Clone, PartialEq)]
pub enum CustomInputValue {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// Float value
    Float(f64),
    /// String value
    String(String),
    /// Array of values
    Array(Vec<CustomInputValue>),
}

impl CustomInputEvent {
    /// Create a new custom input event
    pub fn new(device_type: impl Into<String>, event_type: impl Into<String>) -> Self {
        Self {
            device_type: device_type.into(),
            event_type: event_type.into(),
            data: std::collections::HashMap::new(),
        }
    }

    /// Add boolean data
    pub fn with_bool(mut self, key: impl Into<String>, value: bool) -> Self {
        self.data.insert(key.into(), CustomInputValue::Bool(value));
        self
    }

    /// Add integer data
    pub fn with_int(mut self, key: impl Into<String>, value: i64) -> Self {
        self.data.insert(key.into(), CustomInputValue::Int(value));
        self
    }

    /// Add float data
    pub fn with_float(mut self, key: impl Into<String>, value: f64) -> Self {
        self.data.insert(key.into(), CustomInputValue::Float(value));
        self
    }

    /// Add string data
    pub fn with_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data
            .insert(key.into(), CustomInputValue::String(value.into()));
        self
    }

    /// Get boolean value
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        if let Some(CustomInputValue::Bool(v)) = self.data.get(key) {
            Some(*v)
        } else {
            None
        }
    }

    /// Get integer value
    pub fn get_int(&self, key: &str) -> Option<i64> {
        if let Some(CustomInputValue::Int(v)) = self.data.get(key) {
            Some(*v)
        } else {
            None
        }
    }

    /// Get float value
    pub fn get_float(&self, key: &str) -> Option<f64> {
        if let Some(CustomInputValue::Float(v)) = self.data.get(key) {
            Some(*v)
        } else {
            None
        }
    }

    /// Get string value
    pub fn get_string(&self, key: &str) -> Option<&str> {
        if let Some(CustomInputValue::String(v)) = self.data.get(key) {
            Some(v.as_str())
        } else {
            None
        }
    }
}

/// Input handler trait for components that need to handle input
#[allow(dead_code)]
pub trait InputHandler {
    /// Handle a keyboard event
    fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
        let _ = event;
        false
    }

    /// Handle a mouse event
    fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        let _ = event;
        false
    }

    /// Handle a touch event
    fn handle_touch(&mut self, event: &TouchEvent) -> bool {
        let _ = event;
        false
    }

    /// Handle a custom input event
    fn handle_custom(&mut self, event: &CustomInputEvent) -> bool {
        let _ = event;
        false
    }

    /// Handle any input event
    fn handle_input(&mut self, event: &InputEvent) -> bool {
        match event {
            InputEvent::Keyboard(e) => self.handle_keyboard(e),
            InputEvent::Mouse(e) => self.handle_mouse(e),
            InputEvent::Touch(e) => self.handle_touch(e),
            InputEvent::Custom(e) => self.handle_custom(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        keyboard_handled: bool,
        mouse_handled: bool,
        touch_handled: bool,
    }

    impl InputHandler for TestHandler {
        fn handle_keyboard(&mut self, _event: &KeyboardEvent) -> bool {
            self.keyboard_handled = true;
            true
        }

        fn handle_mouse(&mut self, _event: &MouseEvent) -> bool {
            self.mouse_handled = true;
            true
        }

        fn handle_touch(&mut self, _event: &TouchEvent) -> bool {
            self.touch_handled = true;
            true
        }
    }

    #[test]
    fn test_input_handler_keyboard() {
        let mut handler = TestHandler {
            keyboard_handled: false,
            mouse_handled: false,
            touch_handled: false,
        };

        let event = KeyboardEvent::key_down(KeyCode::Enter, KeyModifiers::empty());
        handler.handle_keyboard(&event);
        assert!(handler.keyboard_handled);
    }

    #[test]
    fn test_input_handler_mouse() {
        let mut handler = TestHandler {
            keyboard_handled: false,
            mouse_handled: false,
            touch_handled: false,
        };

        let event = MouseEvent::button_down(MouseButton::Left, 100.0, 100.0);
        handler.handle_mouse(&event);
        assert!(handler.mouse_handled);
    }

    #[test]
    fn test_input_handler_touch() {
        let mut handler = TestHandler {
            keyboard_handled: false,
            mouse_handled: false,
            touch_handled: false,
        };

        let event = TouchEvent::new(TouchPhase::Began, vec![Touch::new(0, 100.0, 100.0)]);
        handler.handle_touch(&event);
        assert!(handler.touch_handled);
    }

    #[test]
    fn test_custom_input_event() {
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
    fn test_custom_input_event_missing_values() {
        let event = CustomInputEvent::new("stylus", "move");
        assert_eq!(event.get_bool("nonexistent"), None);
        assert_eq!(event.get_int("nonexistent"), None);
        assert_eq!(event.get_float("nonexistent"), None);
        assert_eq!(event.get_string("nonexistent"), None);
    }

    #[test]
    fn test_input_event_custom_variant() {
        let custom = CustomInputEvent::new("device", "event").with_int("value", 42);
        let event = InputEvent::Custom(custom);

        match event {
            InputEvent::Custom(e) => {
                assert_eq!(e.device_type, "device");
                assert_eq!(e.get_int("value"), Some(42));
            }
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn test_custom_input_event_malformed_lookup() {
        // Create an event with an integer stored under "value"
        let event = CustomInputEvent::new("device", "event").with_int("value", 42);

        // Lookup as wrong types should return None rather than panicking or returning garbage
        assert_eq!(event.get_string("value"), None);
        assert_eq!(event.get_float("value"), None);
        assert_eq!(event.get_bool("value"), None);
    }
}
