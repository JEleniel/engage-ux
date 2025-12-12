//! OS Abstraction Layer for Engage UX
//!
//! This crate provides a minimal OAL implementation: unit conversions, device metrics,
//! window handles, and an EventBus integration. Rendering backends are pluggable via
//! the `Renderer` trait.
//!
//! # Safety note — limited `unsafe` usage
//!
//! This crate intentionally contains a very small, well-documented `unsafe` usage in
//! `src/oal/winit_adapter.rs` where we call `wgpu::Instance::create_surface(&window)`.
//! That call is marked `unsafe` by `wgpu` because it relies on platform-specific
//! invariants (the raw window handle must remain valid for the surface lifetime, the
//! surface must be used on the same thread that created the window, etc.). To keep the
//! unsafe surface creation auditable and minimal we centralize it in a single helper
//! function (`create_surface_from_window`) and document the required invariants in the
//! same file. The crate-level Cargo linting intentionally allows `unsafe` for this
//! crate only (see `Cargo.toml`) — the rest of the workspace forbids `unsafe`.
//!
//! See `src/oal/winit_adapter.rs` for the exact safety rationale and guidance on
//! auditing/changing that code.

mod errors;
pub mod oal;

pub use errors::*;
pub use oal::{
	Canvas, DeviceMetrics, NoopRenderer, Oal, Renderer, Unit, View, Window, WindowDesc,
	canvas::Primitive, core_adapter, run_event_loop,
};
#[cfg(test)]
#[path = "units_tests.rs"]
mod units_tests;
