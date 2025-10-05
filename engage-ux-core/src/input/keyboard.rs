//! Keyboard input handling with full accessibility support

use bitflags::bitflags;
use std::fmt;

bitflags! {
	/// Keyboard modifiers (Shift, Ctrl, Alt, Meta)
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct KeyModifiers: u8 {
		const SHIFT = 0b0001;
		const CTRL = 0b0010;
		const ALT = 0b0100;
		const META = 0b1000;
	}
}

/// Physical key codes for keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
	// Navigation keys
	Enter,
	Tab,
	Escape,
	Space,
	Backspace,
	Delete,
	
	// Arrow keys
	ArrowUp,
	ArrowDown,
	ArrowLeft,
	ArrowRight,
	
	// Page navigation
	Home,
	End,
	PageUp,
	PageDown,
	
	// Function keys
	F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
	
	// Alphanumeric keys
	Key(char),
	
	// Modifier keys
	Shift,
	Control,
	Alt,
	Meta,
	
	// Other
	Unknown,
}

/// Keyboard event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
	/// Key was pressed down
	Down,
	/// Key was released
	Up,
	/// Character input (after key translation)
	Char,
}

/// Keyboard event with full context
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardEvent {
	/// Type of keyboard event
	pub event_type: KeyEventType,
	/// The key code
	pub key_code: KeyCode,
	/// Active modifiers
	pub modifiers: KeyModifiers,
	/// Character representation (for text input)
	pub character: Option<char>,
	/// Whether this is a repeat event
	pub repeat: bool,
}

impl KeyboardEvent {
	/// Create a key down event
	pub fn key_down(key_code: KeyCode, modifiers: KeyModifiers) -> Self {
		Self {
			event_type: KeyEventType::Down,
			key_code,
			modifiers,
			character: None,
			repeat: false,
		}
	}

	/// Create a key up event
	pub fn key_up(key_code: KeyCode, modifiers: KeyModifiers) -> Self {
		Self {
			event_type: KeyEventType::Up,
			key_code,
			modifiers,
			character: None,
			repeat: false,
		}
	}

	/// Create a character input event
	pub fn char_input(character: char, modifiers: KeyModifiers) -> Self {
		Self {
			event_type: KeyEventType::Char,
			key_code: KeyCode::Key(character),
			modifiers,
			character: Some(character),
			repeat: false,
		}
	}

	/// Check if Shift is pressed
	pub fn is_shift(&self) -> bool {
		self.modifiers.contains(KeyModifiers::SHIFT)
	}

	/// Check if Ctrl is pressed
	pub fn is_ctrl(&self) -> bool {
		self.modifiers.contains(KeyModifiers::CTRL)
	}

	/// Check if Alt is pressed
	pub fn is_alt(&self) -> bool {
		self.modifiers.contains(KeyModifiers::ALT)
	}

	/// Check if Meta (Windows/Command) is pressed
	pub fn is_meta(&self) -> bool {
		self.modifiers.contains(KeyModifiers::META)
	}
}

/// Keyboard state tracker
#[derive(Debug)]
pub struct KeyboardState {
	/// Currently pressed keys
	pressed_keys: std::collections::HashSet<KeyCode>,
	/// Current modifiers
	modifiers: KeyModifiers,
}

impl Default for KeyboardState {
	fn default() -> Self {
		Self {
			pressed_keys: std::collections::HashSet::new(),
			modifiers: KeyModifiers::empty(),
		}
	}
}

impl KeyboardState {
	/// Create a new keyboard state
	pub fn new() -> Self {
		Self::default()
	}

	/// Update state with an event
	pub fn update(&mut self, event: &KeyboardEvent) {
		match event.event_type {
			KeyEventType::Down => {
				self.pressed_keys.insert(event.key_code);
				self.update_modifiers(event.key_code, true);
			}
			KeyEventType::Up => {
				self.pressed_keys.remove(&event.key_code);
				self.update_modifiers(event.key_code, false);
			}
			KeyEventType::Char => {
				// Character events don't affect state
			}
		}
	}

	fn update_modifiers(&mut self, key: KeyCode, pressed: bool) {
		match key {
			KeyCode::Shift => {
				self.modifiers.set(KeyModifiers::SHIFT, pressed);
			}
			KeyCode::Control => {
				self.modifiers.set(KeyModifiers::CTRL, pressed);
			}
			KeyCode::Alt => {
				self.modifiers.set(KeyModifiers::ALT, pressed);
			}
			KeyCode::Meta => {
				self.modifiers.set(KeyModifiers::META, pressed);
			}
			_ => {}
		}
	}

	/// Check if a key is currently pressed
	pub fn is_key_pressed(&self, key: KeyCode) -> bool {
		self.pressed_keys.contains(&key)
	}

	/// Get current modifiers
	pub fn modifiers(&self) -> KeyModifiers {
		self.modifiers
	}

	/// Clear all pressed keys
	pub fn clear(&mut self) {
		self.pressed_keys.clear();
		self.modifiers = KeyModifiers::empty();
	}
}

/// Logical key representation for text input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
	/// The key code
	pub code: KeyCode,
	/// The character representation
	pub character: Option<char>,
}

impl fmt::Display for KeyCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			KeyCode::Enter => write!(f, "Enter"),
			KeyCode::Tab => write!(f, "Tab"),
			KeyCode::Escape => write!(f, "Escape"),
			KeyCode::Space => write!(f, "Space"),
			KeyCode::Backspace => write!(f, "Backspace"),
			KeyCode::Delete => write!(f, "Delete"),
			KeyCode::ArrowUp => write!(f, "ArrowUp"),
			KeyCode::ArrowDown => write!(f, "ArrowDown"),
			KeyCode::ArrowLeft => write!(f, "ArrowLeft"),
			KeyCode::ArrowRight => write!(f, "ArrowRight"),
			KeyCode::Home => write!(f, "Home"),
			KeyCode::End => write!(f, "End"),
			KeyCode::PageUp => write!(f, "PageUp"),
			KeyCode::PageDown => write!(f, "PageDown"),
			KeyCode::Key(c) => write!(f, "{}", c),
			KeyCode::F1 => write!(f, "F1"),
			KeyCode::F2 => write!(f, "F2"),
			KeyCode::F3 => write!(f, "F3"),
			KeyCode::F4 => write!(f, "F4"),
			KeyCode::F5 => write!(f, "F5"),
			KeyCode::F6 => write!(f, "F6"),
			KeyCode::F7 => write!(f, "F7"),
			KeyCode::F8 => write!(f, "F8"),
			KeyCode::F9 => write!(f, "F9"),
			KeyCode::F10 => write!(f, "F10"),
			KeyCode::F11 => write!(f, "F11"),
			KeyCode::F12 => write!(f, "F12"),
			KeyCode::Shift => write!(f, "Shift"),
			KeyCode::Control => write!(f, "Control"),
			KeyCode::Alt => write!(f, "Alt"),
			KeyCode::Meta => write!(f, "Meta"),
			KeyCode::Unknown => write!(f, "Unknown"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_key_modifiers() {
		let mut mods = KeyModifiers::empty();
		assert!(!mods.contains(KeyModifiers::SHIFT));

		mods.insert(KeyModifiers::SHIFT);
		assert!(mods.contains(KeyModifiers::SHIFT));

		mods.insert(KeyModifiers::CTRL);
		assert!(mods.contains(KeyModifiers::SHIFT | KeyModifiers::CTRL));
	}

	#[test]
	fn test_keyboard_event_creation() {
		let event = KeyboardEvent::key_down(KeyCode::Enter, KeyModifiers::CTRL);
		assert_eq!(event.event_type, KeyEventType::Down);
		assert_eq!(event.key_code, KeyCode::Enter);
		assert!(event.is_ctrl());
		assert!(!event.is_shift());
	}

	#[test]
	fn test_keyboard_state() {
		let mut state = KeyboardState::new();
		
		let down = KeyboardEvent::key_down(KeyCode::Enter, KeyModifiers::empty());
		state.update(&down);
		assert!(state.is_key_pressed(KeyCode::Enter));
		
		let up = KeyboardEvent::key_up(KeyCode::Enter, KeyModifiers::empty());
		state.update(&up);
		assert!(!state.is_key_pressed(KeyCode::Enter));
	}

	#[test]
	fn test_keyboard_state_modifiers() {
		let mut state = KeyboardState::new();
		
		let shift_down = KeyboardEvent::key_down(KeyCode::Shift, KeyModifiers::SHIFT);
		state.update(&shift_down);
		assert!(state.modifiers().contains(KeyModifiers::SHIFT));
		
		let shift_up = KeyboardEvent::key_up(KeyCode::Shift, KeyModifiers::empty());
		state.update(&shift_up);
		assert!(!state.modifiers().contains(KeyModifiers::SHIFT));
	}

	#[test]
	fn test_char_input_event() {
		let event = KeyboardEvent::char_input('a', KeyModifiers::empty());
		assert_eq!(event.event_type, KeyEventType::Char);
		assert_eq!(event.character, Some('a'));
	}
}
