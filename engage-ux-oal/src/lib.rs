//! OS Abstraction Layer for Engage UX
//!
//! This crate provides a minimal OAL implementation: unit conversions, device metrics,
//! window handles, and an EventBus integration. Rendering backends are pluggable via
//! the `Renderer` trait.

mod errors;
mod oal;

pub use errors::*;
pub use oal::{
	Canvas, DeviceMetrics, NoopRenderer, Oal, Renderer, Unit, View, Window, WindowDesc,
	run_event_loop,
};
#[cfg(test)]
#[path = "units_tests.rs"]
mod units_tests;
