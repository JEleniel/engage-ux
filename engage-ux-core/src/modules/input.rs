//! Input handling system for Engage UX
//!
//! Modern modular layout: small files for each major element.

pub mod focus_manager;
pub mod input_event;
pub mod input_handler;
pub mod keyboard;
pub mod mouse;
pub mod touch;

pub use focus_manager::FocusManager;
pub use input_event::InputEvent;
pub use input_handler::InputHandler;
pub use mouse::{MouseButton, MouseEvent, MouseState};
pub use touch::{Touch, TouchEvent, TouchPhase, TouchState};

#[cfg(test)]
mod tests {}
