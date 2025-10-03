//! Engage UX Core - Foundation for cross-platform UI toolkit
//!
//! This crate provides the core functionality for Engage UX including:
//! - Color system (RGB and HSL)
//! - Event handling with Tokio
//! - Component traits
//! - Thread-safe primitives
//! - Input handling (keyboard, mouse, touch)
//! - Accessibility support
//! - SVG rendering (without script execution)
//! - Media support (fonts and images)

pub mod accessibility;
pub mod color;
pub mod component;
pub mod events;
pub mod input;
pub mod media;
pub mod rendering;

pub use color::{Color, ColorSpace};
pub use component::Component;
pub use events::{Event, EventHandler};
pub use input::{InputEvent, InputHandler};
pub use rendering::{RenderError, SvgDocument, SvgParser};

#[cfg(test)]
mod tests {
	#[test]
	fn core_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
