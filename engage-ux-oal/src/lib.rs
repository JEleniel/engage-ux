//! OS Abstraction Layer (OAL) crate - public traits and types.
#![allow(missing_docs)]
//!
//! This crate defines the minimal, stable API surface for platform services used by
//! `engage-ux-core`. It intentionally provides traits and light-weight types only;
//! platform-specific crates implement these traits.

mod engage_ux_oal;

pub use engage_ux_oal::*;
