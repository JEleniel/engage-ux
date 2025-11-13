//! Public OAL module parent. This file is the module root for the `oal` module
//! and declares submodules stored in the `src/oal/` folder. Do NOT use `mod.rs`.

pub mod backend;
pub mod backends;
pub mod canvas;
pub mod device_metrics;
pub mod oal_core;
pub mod platform;
pub mod renderer;
pub mod transform;
pub mod unit;
pub mod view;
pub mod window;
pub mod window_desc;
pub mod winit_backend;

// Re-export the main types for consumers under `crate::oal::...`.
pub use backend::{Backend, HeadlessBackend, PlatformEvent, SurfaceHandle, SurfaceParams};
pub use canvas::Canvas;
pub use device_metrics::DeviceMetrics;
pub use oal_core::Oal;
pub use platform::Platform;
#[cfg(feature = "native-winit")]
pub use renderer::WgpuRenderer;
pub use renderer::{NoopRenderer, Renderer};
#[allow(unused_imports)]
pub use transform::Transform;
pub use unit::Unit;
pub use view::View;
pub use window::Window;
pub use window_desc::WindowDesc;
pub use winit_backend::run_event_loop;
#[cfg(feature = "native-winit")]
pub mod winit_adapter;
#[cfg(feature = "native-winit")]
pub use winit_adapter::WinitBackend;
