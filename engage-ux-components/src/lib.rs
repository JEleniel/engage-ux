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
pub mod list;
pub mod progress;
pub mod tooltip;

// Interactive components
pub mod button;
pub mod text_input;
pub mod text_area;
pub mod checkbox;
pub mod radio;
pub mod toggle;
pub mod slider;
pub mod select;
pub mod link;

// Notification components
pub mod badge;
pub mod toast;

// Layout components
pub mod container;
pub mod card;
pub mod table;

// Window components
pub mod window_controls;

// Re-exports
pub use label::Label;
pub use text::Text;
pub use button::Button;
pub use text_input::TextInput;
pub use text_area::TextArea;
pub use checkbox::Checkbox;
pub use container::Container;
pub use list::{List, ListItem};
pub use select::{Select, SelectOption};
pub use link::Link;
pub use progress::Progress;
pub use tooltip::{Tooltip, Popover};
pub use badge::Badge;
pub use toast::Toast;
pub use table::Table;
pub use window_controls::{WindowControlButton, WindowControls};

#[cfg(test)]
mod tests {
	#[test]
	fn components_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
