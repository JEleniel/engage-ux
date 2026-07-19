# OAL (OS Abstraction Layer) Status

This document summarizes the current implementation status of the `engage-ux-oal` crate (as of branch `v1.0.0`). It's intended as a short, living status note for implementers and maintainers.

## Implemented

- Core types: `Oal`, `Window`, `Canvas`, `View`, `WindowDesc`, `DeviceMetrics`, `Unit`.
- Renderer trait and `NoopRenderer` for headless/test use.
- Headless event loop: emits `FrameRequested` periodically when native winit is not enabled.
- `winit` backend (feature-gated) mapping native events to core `Event` variants.
- Unit tests for `Unit` conversion and event emission test `tests/event_emission.rs`.

## Remaining work (high priority)

- Replace `expect`/`unwrap` panics with proper error handling and `OalError` propagation.
- Add `Oal::destroy_window` API to remove windows and clean up native backend resources.
- Add graceful shutdown/quit API for `run_event_loop` and provide a way to stop the loop.
- Improve mutex poison handling and avoid blind `unwrap()` on locks.

## Remaining work (medium / polish)

- Clarify and possibly extend `DeviceMetrics` semantics (e.g., DPI vs pixels-per-unit).
- Add integration tests for native `winit` backend where feasible.
- Wire a real renderer implementation (e.g., a simple software renderer) or enable `choose_renderer` to pick a backend.

## Documentation

- Add crate-level documentation and examples (an example `examples/headless_oal.rs` is included).
- Address missing-docs warnings for public API items.

## Notes

- Current implementation is intentionally simple for early development and prototyping. The above remaining items should be prioritized by safety (panic removal) and API completeness (destroy window, shutdown).

***

Updated: 2025-11-12
