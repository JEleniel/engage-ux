//! Shared primitive types used across the crate
//!
//! This module centralizes small, widely-used primitives so they are defined
//! in one place and reused by other modules.
use flagset::flags;
use serde::{Deserialize, Serialize};

/// Unique identifier for components
pub type ComponentId = usize; // Usize to match the limitations of Vec

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
	/// The left mouse button
	Left,
	/// The right mouse button
	Right,
	/// The middle mouse button
	Middle,
	/// Other mouse buttons
	Other(u8),
}

flags! {
	#[derive(Serialize, Deserialize)]
	/// Keyboard modifiers
	pub enum KeyModifiers: u8 {
		/// The Shift key
		SHIFT = 0b00000001,
		/// The Control key
		 CTRL  = 0b00000010,
		/// The Alt key
		 ALT   = 0b00000100,
		/// The Meta key
		 META  = 0b00001000,
		/// The Windows key
		 WIN   = 0b00010000,
		/// The Command key
		 CMD   = 0b00100000,
		/// The Function key
		 FN    = 0b01000000,
	}
}
