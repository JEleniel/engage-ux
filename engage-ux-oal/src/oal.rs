//! Public OAL module parent. This file is the module root for the `oal` module
//! and declares submodules stored in the `src/oal/` folder. Do NOT use `mod.rs`.

/// Platform backend primitives and traits (SurfaceDescriptor, Backend, etc.).
pub mod backend;

/// Platform-specific backend shims (Wayland/X11 adapters).
pub mod backends;

/// Canvas command recording surface used by UI controls.
pub mod canvas;
/// Core adapters that convert `engage-ux-core` primitives into OAL `Canvas`
/// commands. This module contains helpers used by the core→OAL adapter and
/// by examples that want to programmatically emit canvas primitives.
pub mod core_adapter;
/// Device metrics (DPR, pixel conversions) helpers.
pub mod device_metrics;

/// Core OAL implementation: `Oal` type, window registry and high-level APIs.
pub mod oal_core;

/// Platform wrapper around a concrete `Backend` implementation.
pub mod platform;

/// Renderer trait and renderer implementations (software and GPU paths).
pub mod renderer;

/// Device lifecycle helper utilities (device registration & recreation)
pub mod device_lifecycle;
/// Minimal GPU text helpers and glyph atlas.
pub mod text;

/// Transform utilities and helpers.
pub mod transform;

/// Unit scale helpers (Metric/Imperial etc.).
pub mod unit;

/// View abstraction for layout and composition.
pub mod view;

/// Logical `Window` and related helpers.
pub mod window;
/// Window descriptor helpers and builders.
pub mod window_desc;

/// Winit adapter and helpers for native windowing.
pub mod winit_backend;

// Re-export the main types for consumers under `crate::oal::...`.
// Re-export commonly-used types for consumers. To avoid unused-import
// warnings during internal development we intentionally keep the set
// minimal — consumers can access other types via their modules, for
// example `crate::oal::backend::HeadlessBackend`.
pub use canvas::Canvas;
pub use device_metrics::DeviceMetrics;
pub use oal_core::Oal;
#[cfg(feature = "native-winit")]
pub use renderer::WgpuRenderer;
pub use renderer::{NoopRenderer, Renderer};
pub use unit::Unit;
pub use view::View;
pub use window::Window;
pub use window_desc::WindowDesc;
pub use winit_backend::run_event_loop;

// Re-export a few backend helpers for tests so integration tests can
// exercise backend implementations without making these types part of
// the stable public prelude for normal consumers.
#[cfg(test)]
pub use backend::{HeadlessBackend, SurfaceDescriptor};
#[cfg(feature = "native-winit")]
pub mod winit_adapter;
#[cfg(feature = "native-winit")]
pub use winit_adapter::WinitBackend;
