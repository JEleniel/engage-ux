//! UI Components for Engage UX
//!
//! Provides all UI components including:
//! - Basic components (Label, Text, Icon, Image)
//! - Interactive components (Button, Input, Checkbox, etc.)
//! - Layout components (Container, Card, Table)
//! - Complex components (Menu, Dialog, etc.)

// Informational components
pub mod label;
pub mod text;
pub mod icon;
pub mod image;

// Interactive components
pub mod button;
pub mod text_input;
pub mod checkbox;
pub mod radio;
pub mod toggle;
pub mod slider;

// Layout components
pub mod container;
pub mod card;

// Re-exports
pub use label::Label;
pub use text::Text;
pub use button::Button;
pub use text_input::TextInput;
pub use checkbox::Checkbox;
pub use container::Container;

#[cfg(test)]
mod tests {
	#[test]
	fn components_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
