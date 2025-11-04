// Compatibility shim for the legacy `modules::events` module name.
//
// Re-export the new `modules::event` module so existing code that refers
// to `crate::events`/`engage_ux_core::events` continues to work.

pub use crate::modules::event::*;
