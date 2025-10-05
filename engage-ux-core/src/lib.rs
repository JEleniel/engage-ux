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
//! - Layout system with relative units

pub mod accessibility;
pub mod animation;
pub mod color;
pub mod component;
pub mod drag_drop;
pub mod events;
pub mod input;
pub mod layout;
pub mod media;
pub mod rendering;

pub use animation::{Animation, AnimationController, AnimationState, Easing};
pub use color::{Color, ColorSpace};
pub use component::Component;
pub use drag_drop::{DragData, DragEvent, DragManager, DragOperation, DragSource, DropTarget};
pub use events::{Event, EventHandler};
pub use input::{InputEvent, InputHandler};
pub use layout::{
	CalculatedBounds, Constraints, Layout, Position, PositionMode, RelativeUnit, Size, SizeMode,
	Unit,
};
pub use rendering::{RenderError, SvgDocument, SvgParser};

#[cfg(test)]
mod tests {
	#[test]
	fn core_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
