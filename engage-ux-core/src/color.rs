//! Top-level compatibility color module
//!
//! This file provides a stable, top-level `engage_ux::color` path for
//! consumers while the canonical implementations live under
//! `crate::modules::colors`.
//!
//! Prefer the canonical `crate::modules::colors` paths internally, but
//! keep these re-exports to avoid breaking downstream users.

// Re-export the palette types that `modules::colors` publishes at the crate root.
// `modules.rs` already does `pub use colors::*;`, so these items are available
// as `crate::Ansi` / `crate::Web` etc. Re-export them from `engage_ux::color`.
pub use crate::{Ansi, Common, Engage, Web};
