//! Engage UX Core - Foundation for cross-platform UI toolkit
//!
//! This crate provides the core functionality for Engage UX including:
//! - Color system (RGB and HSL)
//! - Event handling with Tokio
//! - Component traits
//! - Thread-safe primitives

pub mod color;
pub mod component;
pub mod events;

pub use color::{Color, ColorSpace};
pub use component::Component;
pub use events::{Event, EventHandler};

#[cfg(test)]
mod tests {
	#[test]
	fn core_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
