# Platforms API Design

This document describes the cross-platform windowing API for Engage UX, updated to use [`winit`](https://github.com/rust-windowing/winit) for window and event management. The API surface is platform-agnostic and supports multiple backends, including Wayland, X11, Windows, macOS, iOS, and Android. For drawing on window surfaces, [`wgpu`](https://github.com/gfx-rs/wgpu) is recommended for GPU-accelerated, cross-platform rendering.

## Overview

- The `Platform` type is the main entry point for windowing and surface management, built on top of `winit` for cross-platform event loop and window handling.
- The `Platform` is constructed with a `Backend` parameter, which provides the implementation for a specific OS or windowing system.
- The API surface matches ergonomic, builder-based, event-driven patterns, leveraging `winit` for event delivery and window management.
- Backends implement a trait (e.g., `Backend`) that provides the necessary primitives for window/surface creation, event delivery, and state management. Surface drawing should use `wgpu` for rendering to `winit` windows.

## Key Types (all under `platforms::` module)

- `Platform<B: Backend>` — main entry point for windowing, parameterized by a backend implementation.
- `Backend` — trait that must be implemented by each supported backend (Wayland, X11, Win32, Cocoa, Android, etc.).
- `SurfaceHandle` — cross-platform abstraction for a drawable/renderable surface.
- `Toplevel` — toplevel window role (maximize/restore/fullscreen/minimize, title, app_id).
- `Popup` — transient popup role; anchored coordinates and dismissal events.
- `Configure` — typed configure event: `{ serial: Serial, size: Option<Size>, states: WindowStates }`.
- `Serial` — event serial (u32), used for event ordering and acknowledgement.
- `WindowState` — bitflags or enum set (e.g., Maximized, Fullscreen, Activated, Resizable).
- `ShellError` — enum of recoverable/terminal errors (Protocol, MissingAck, Destroyed, BackendError).

## Example Rust API Signatures

```rust
mod platforms {
    pub trait Backend {
        fn create_surface(&self, params: SurfaceParams) -> SurfaceHandle;
        fn poll_events(&self) -> Vec<PlatformEvent>;
        // ... other backend-specific methods
    }

    pub struct Platform<B: Backend> {
        backend: B,
    }

    impl<B: Backend> Platform<B> {
        pub fn new(backend: B) -> Self;
        pub fn create_surface(&self, params: SurfaceParams) -> SurfaceHandle;
        pub fn poll_events(&self) -> Vec<PlatformEvent>;
        // ... other platform-agnostic methods
    }

    pub struct SurfaceHandle { /* ... */ }
    pub struct Toplevel { /* ... */ }
    pub struct Popup { /* ... */ }
    pub struct Configure { /* ... */ }
    pub struct Serial(pub u32);
    pub struct WindowState { /* ... */ }
    pub enum ShellError { /* ... */ }
    // ... other types
}
```

## Backend Example

- The Wayland backend implements the `Backend` trait and provides all required primitives for Wayland windowing, using `winit` for event loop and window management.
- Other backends (X11, Win32, Cocoa, Android) implement the same trait, using `winit` for window/event plumbing and `wgpu` for surface drawing, allowing Engage UX to use the same API surface across platforms.

## Event-Driven and Builder Patterns

- All window/surface creation uses builder patterns for configuration, leveraging `winit`'s window builder APIs.
- Event-driven flows: surfaces and windows emit events (e.g., `Configure`, `Close`, `PopupCreated`) and require explicit acknowledgement (`ack_configure`). Events are delivered via `winit`'s event loop.
- Errors are surfaced as `ShellError`.

- All window/surface creation uses builder patterns for configuration.
- Event-driven flows: surfaces and windows emit events (e.g., `Configure`, `Close`, `PopupCreated`) and require explicit acknowledgement (`ack_configure`).
- Errors are surfaced as `ShellError`.

## Backend Trait Requirements

- The `Backend` trait defines the contract for platform-specific windowing implementations. All backends should use `winit` for window/event management and `wgpu` for drawing on surfaces. Any backend (Wayland, X11, Win32, Cocoa, Android, etc.) must implement this trait to be compatible with the Engage UX Platforms API.

The `Backend` trait defines the contract for platform-specific windowing implementations. Any backend (Wayland, X11, Win32, Cocoa, Android, etc.) must implement this trait to be compatible with the Engage UX Platforms API.

### Trait Definition

```rust
pub trait Backend {
    /// Create a new surface with the given parameters.
    fn create_surface(&self, params: SurfaceParams) -> SurfaceHandle;

    /// Poll for platform events (input, window state changes, etc.).
    fn poll_events(&self) -> Vec<PlatformEvent>;

    /// Present a frame to the surface, optionally specifying dirty regions.
    fn present_frame(&self, surface: &SurfaceHandle, frame: Frame, dirty: &[DeviceRect]) -> Result<(), ShellError>;

    /// Invalidate a region of the surface (for partial redraws).
    fn invalidate_region(&self, surface: &SurfaceHandle, rects: &[DeviceRect]) -> Result<(), ShellError>;

    /// Assign a toplevel role to a surface (window semantics).
    fn assign_toplevel(&self, surface: &SurfaceHandle, params: ToplevelParams) -> Result<Toplevel, ShellError>;

    /// Assign a popup role to a surface (transient semantics).
    fn assign_popup(&self, surface: &SurfaceHandle, parent: &SurfaceHandle, positioner: Positioner) -> Result<Popup, ShellError>;

    /// Acknowledge a configure event (for event-driven backends).
    fn ack_configure(&self, surface: &SurfaceHandle, serial: Serial) -> Result<(), ShellError>;

    /// Destroy a surface and release resources.
    fn destroy_surface(&self, surface: SurfaceHandle);

    /// Additional backend-specific methods as needed (e.g., window state, input, accessibility).
}
```

### Contract and Notes

- All methods must be thread-safe unless documented otherwise.
- Surface and window creation must use builder/configuration patterns for ergonomic setup.
- Event-driven backends (e.g., Wayland) must support explicit event acknowledgement (`ack_configure`).
- Errors must be surfaced as `ShellError`.
- Backends are responsible for mapping platform-specific details to the general API surface; no platform-specific types should leak into the public API.
- The trait can be extended with additional methods for platform-specific features (e.g., accessibility, input, window state management) as needed.

This trait enables Engage UX to support new platforms by implementing the required primitives for windowing, surfaces, and event delivery.

## OAL Integration Notes

All platform backends must:

- Accept and use the EventBus for input event delivery and cross-thread communication.
- Present layout in terms of Units (U), with default 1U=1cm, and support runtime scale changes (Metric, Imperial, Point, Pixel presets).
- Perform Unit-to-pixel conversion inside the backend as part of the rendering pipeline, using device metrics (DPCM/DPI).
- Expose a logical Canvas (f32 x f32), View, and Window as described in the OAL public API.
- Ensure rendering occurs on the main/UI thread, with all other logic off-main-thread via EventBus.
- Surface platform-specific errors and configuration via Result types and EventBus events.

See `../OAL.md` for the full API contract and Mermaid diagram.

## Notes

- The API is designed to be portable, ergonomic, and suitable for Engage UX on any supported OS. All window/event management should use `winit`, and all surface drawing should use `wgpu` unless a specific alternative is required.
- No platform-specific details are exposed in the public API; all platform logic is encapsulated in the `Backend` trait implementations.
- This design enables Engage UX to support new platforms by simply implementing the `Backend` trait for each new windowing system, using `winit` and `wgpu` as the foundation for cross-platform support.
