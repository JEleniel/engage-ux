//! Core module registry for the `engage-ux-core` crate.
//!
//! This file declares the major functional modules that make up the core
//! crate (animation, layout, eventing, media, rendering, etc.) and re-exports
//! commonly used items. Keep module-level documentation here so that
//! `engage_ux_core::modules` is discoverable in generated docs.

/// Animation utilities and timeline helpers
pub mod animation;
mod colors;
/// Drag and drop system
pub mod drag_drop;
/// Event types and the event bus
pub mod event;
/// Geometry primitives (Point, Rectangle, Offset, etc.)
pub mod geometry;
/// Layout system and units
pub mod layout;
/// Media types (fonts, images)
pub mod media;
/// Rendering primitives (SVG, render errors)
pub mod rendering;

/// Re-export the color palette types at the crate level for convenience.
pub use colors::*;
