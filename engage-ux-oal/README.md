# engage-ux-oal (OS Abstraction Layer) — Native backend notes

This crate implements the OS Abstraction Layer (OAL) for Engage UX and contains platform
backend adapters (Wayland/X11/macOS/Windows/iOS/Android) and helpers used by the
`engage-ux` workspace to create windows, surfaces, and manage device metrics.

## Minimal unsafe usage

Due to the nature of native windowing and GPU surface creation, a very small and
well-scoped `unsafe` operation is required when creating a `wgpu::Surface` from a
platform window (for example via `winit::window::Window`). The `wgpu` API marks
`Instance::create_surface` as `unsafe` because the caller must uphold platform- and
backend-specific invariants regarding lifetimes and thread usage of the returned
surface.

Where this appears in the codebase

- The unsafe call is encapsulated in `engage-ux-oal/src/oal/winit_adapter.rs` inside
  a single helper function called `create_surface_from_window`. The implementation
  intentionally centralizes the `unsafe` call so the invariants and justification are
  easy to review and audit.

Safety invariants (callers must ensure):

1. The `Window`'s underlying raw window handle must remain valid for the lifetime of the
   `wgpu::Surface` returned by `create_surface`.
2. The surface must be created and used on the same thread that owns the native window
   event loop (this crate creates and manages surfaces on the event-loop thread).
3. The `Window` must not be dropped while the surface is still in use.

Recommended practices

- Keep the `unsafe` call restricted to the helper and do not spread `unsafe` into
  other modules or call sites.
- Store the `Window` and `Surface` together (as `GpuSurface` does) so lifetimes are
  tied together and the window outlives the surface.
- Add unit/integration tests around surface creation and destruction to exercise
  lifecycle transitions and to make sure the code behaves correctly on the supported
  platforms.
- When modifying surface creation logic, update the comment block in
  `create_surface_from_window` and reference any platform-specific constraints learned
  during testing.

References

- wgpu `Instance::create_surface` docs: <https://docs.rs/wgpu>
- winit `Window` and raw window handle docs: <https://docs.rs/winit>

If you want, I can open a small follow-up PR that adds an integration test to exercise
surface creation and destruction on the native-winit backend (Linux headless or a
platform runner), and I can add CI jobs to validate the behavior across supported
platforms.

# engage-ux-oal

Small notes for developers working with the OAL (OS Abstraction Layer).

Enabling native windowing (Wayland/X11)

- To build the native winit/wgpu based backend enable the `native-winit` feature:

  cargo build -p engage-ux-oal --features native-winit

- When `native-winit` is enabled the crate will include the winit-based
  backend which supports Wayland and X11 via winit. Without the feature the
  crate falls back to a headless backend for tests and CI.

Current Work-in-Progress

- Device-lost handling and GPU cache invalidation need additional work.
- GPU text rendering (glyph atlas and shaping) is not implemented yet.

If you plan to run Wayland/X11 integration tests locally, ensure your
development environment has a compositor (Wayland compositor or X server)
available and that system libraries required by winit/wgpu are installed.
