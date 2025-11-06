use keyboard_types::Modifiers;
use serde::{Deserialize, Serialize};

/// A keyboard event emitted when a key is pressed, released, or held down.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardEvent {
	/// The raw key code of the key event.
	pub key: KeyCode,
	/// The modifier keys active during the key event.
	pub modifiers: Modifiers,
	/// Whether the key event is a repeat (key held down).
	pub repeat: bool,
}

/// Specific keys that can be part of a keyboard event's `key` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyCode {
	/// Arrow Down key.
	ArrowDown,
	/// Arrow Left key.
	ArrowLeft,
	/// Arrow Right key.
	ArrowRight,
	/// Arrow Up key.
	ArrowUp,
	/// Backspace key.
	Backspace,
	/// Break key.
	Break,
	/// A character key.
	Character(char),
	/// Delete key.
	Delete,
	/// End key.
	End,
	/// Enter key.
	Enter,
	/// Escape key.
	Escape,
	/// F1 to F12 function keys.
	Function(u8),
	/// Home key.
	Home,
	/// Insert key.
	Insert,
	/// Num Lock key.
	NumLock,
	/// Numpad keys.
	Numpad(u8),
	/// Numpad Arrow Down key.
	NumpadArrowDown,
	/// Numpad Arrow Left key.
	NumpadArrowLeft,
	/// Numpad Arrow Right key.
	NumpadArrowRight,
	/// Numpad Arrow Up key.
	NumpadArrowUp,
	/// Numpad Asterisk key.
	NumpadAsterisk,
	/// Numpad Decimal key.
	NumpadDecimal,
	/// Numpad Delete key.
	NumpadDelete,
	/// Numpad End key.
	NumpadEnd,
	/// Numpad Enter key.
	NumpadEnter,
	/// Numpad Home key.
	NumpadHome,
	/// Numpad Insert key.
	NumpadInsert,
	/// Numpad Minus key.
	NumpadMinus,
	/// Numpad Page Down key.
	NumpadPageDown,
	/// Numpad Page Up key.
	NumpadPageUp,
	/// Numpad Plus key.
	NumpadPlus,
	/// Numpad Solidus key.
	NumpadSolidus,
	/// Page Down key.
	PageDown,
	/// Page Up key.
	PageUp,
	/// Pause key.
	Pause,
	/// Print Screen key.
	PrintScreen,
	/// Scroll Lock key.
	ScrollLock,
	/// Space key.
	Space,
	/// SysRq key.
	SysReq,
	/// Tab key.
	Tab,
	/// Windows key.
	Windows,
}

/// Specific modifier keys that can be part of a keyboard event's `modifiers` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyboardModifierKeys {
	/// The left Alt key.
	LeftAlt,
	/// The right Alt key.
	RightAlt,
	/// The Control key.
	Control,
	/// The left Shift key.
	LeftShift,
	/// The right Shift key.
	RightShift,
	/// The left Option key.
	LeftOption,
	/// The right Option key.
	RightOption,
	/// The Meta key.
	Meta,
	/// The Command key.
	Command,
	/// Caps Lock key.
	CapsLock,
}
