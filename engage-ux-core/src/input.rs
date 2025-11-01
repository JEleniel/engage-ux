//! Input module surface: keep this file minimal and forward to `input::inner`.

pub mod keyboard;
pub mod mouse;
pub mod touch;

pub mod events;

// Allow unused_imports here: these `pub use` are intentional public re-exports
#[allow(unused_imports)]
pub use keyboard::{Key, KeyCode, KeyModifiers, KeyboardEvent, KeyboardState};
#[allow(unused_imports)]
pub use mouse::{MouseEvent, MouseState};
#[allow(unused_imports)]
pub use touch::{Touch, TouchEvent, TouchPhase, TouchState};
