//! Engage UX Core - Core functionality for Engage UX - a fully themable, modular UX framework for
//! 	Rust, providing core functionality, components, and base themes.
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
//! - Layout system with relative units

mod modules;

pub use modules::*;

#[cfg(test)]
mod tests {
	#[test]
	fn core_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
