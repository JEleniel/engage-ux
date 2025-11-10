//! OS Abstraction Layer (OAL) traits split across submodules for maintainability.
//!
//! This file re-exports the smaller trait modules so callers can access them as
//! `engage_ux_oal::traits::Frame`, `...::Surface`, `...::Window`, etc.

/// Accessibility bridge trait.
pub mod accessibility;
/// Frame-related types.
pub mod frame;
/// OAL top-level trait.
pub mod oal;
/// Surface/back-buffer related traits and builders.
pub mod surface;
/// Top-level window traits and builders.
pub mod window;

// Re-exports for convenient access from `engage_ux_oal::traits`.
#[allow(unused_imports)]
pub use accessibility::AccessibilityBridge;
pub use frame::Frame;
pub use oal::PlatformTrait;
pub use surface::{Surface, SurfaceBuilder};
pub use window::{Window, WindowBuilder};
