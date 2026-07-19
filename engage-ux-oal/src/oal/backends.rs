// Platform-specific backend shims. These are only compiled when the
// `native-winit` feature is enabled; otherwise the headless backend is
// the default and the platform-specific shims are omitted to avoid
// unused-code warnings.
#[cfg(feature = "native-winit")]
pub mod wayland;

#[cfg(feature = "native-winit")]
pub mod x11;

// Re-exports of platform-specific backends were previously exposed here.
// They are intentionally left out of the public prelude to avoid unused
// import warnings when a particular backend isn't referenced by the
// current build. Consumers may access the backend modules directly, e.g.
// `engage_ux_oal::oal::backends::wayland::WaylandBackend` when needed.
