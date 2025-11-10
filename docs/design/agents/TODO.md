
# Agent TODO

## Summary

This file is a compact, actionable TODO for agent tasks. It is not meant to contain progress notes of details. Use this as the primary source of remaining work.

## Maintenance

- This file is the primary, concise agent TODO remaining work.
- Keep it checklist-style (short actionable lines).
- Do NOT include completion checkmarks, dates, or long status paragraphs in this file; keep historical or release notes in `CHANGELOG.md` instead.
- When work starts, update this file with the minimal checkbox state. When work completes, remove the item from this file and append an entry to the `CHANGELOG.md` as required by the project guidance.

## OAL: Wayland

### API & Contracts

- [ ] Add/clarify API doc-comments in `engage-ux-oal/src/traits.rs` describing inputs, outputs, thread affinity, and error mappings.
- [ ] Create short contract document/tests that validate runtime affinity and OAL invariants.

### Runtime & Shutdown

- [ ] Add bounded processing APIs (e.g., `process_main_thread_tasks` / `run_once`).
- [ ] Implement runtime shutdown (`RuntimeMessage::Shutdown` / `stop`) that clears global handles and tears down surfaces.
- [ ] Add unit tests for budgeting and clean shutdown from other threads.

### Buffer Pooling & Lifecycle

- [ ] Implement per-window buffer pool for wl_shm path; handle wl_buffer release and FD lifetimes.
- [ ] Ensure resizes replace incompatible buffers and no FD leaks occur; add lifecycle tests using memfd/tempfile mocks.

### Input Mapping

- [ ] Map wl_pointer, wl_keyboard, wl_touch to `engage-ux-core` input events and publish via window event callbacks or event bus.
- [ ] Add tests/mocks that demonstrate event translation and delivery.

### Accessibility, Clipboard & IME

- [ ] Implement `WaylandAccessibilityBridge` (main-thread) with a mockable interface for tests.
- [ ] Add minimal clipboard get/set and IME stubs with graceful fallbacks for headless CI.

### Timers & Main-thread Task API

- [ ] Finalize `WaylandHandle::queue_main_thread` variants (blocking, non-blocking, oneshot) and add tests for timeouts and responder behavior.

### Dirty-region / Partial Redraw

- [ ] Ensure damage_buffer semantics are honored for software and GPU paths; add unit tests for partial updates.

### Tests & Mocks

- [ ] Expand unit tests for anonymous-file creation, buffer selection, queue roundtrips, Surface wrapper behavior, and FD cleanup.
- [ ] Provide mock Wayland types where feasible to avoid requiring a compositor for unit tests.

### Examples

- [ ] Improve `engage-ux-oal/examples/wayland_example.rs` to show building a Window/Surface, presenting from worker threads, and graceful shutdown.

### CI, Build & Automation

- [ ] Add CI steps/matrix entries to build with `--features=wayland` where available; skip or mock where Wayland isn't present.
- [ ] Add `rustfmt`/`clippy` checks to the CI flow for the OAL crate.

### Optional GPU/Skia (deferred)

- [ ] Design and gate a Skia/EGL GPU path behind a feature flag (e.g., `skia_gpu`) with deterministic software fallback.

### Recent changes

- 2025-11-09 — JEleniel: Removed feature gating from the OAL so platform backends (Wayland) are compiled and available by default. Files touched: `engage-ux-oal/src/engage_ux_oal/oal.rs`, `platform.rs`, `platform/wayland.rs`, `oal_error.rs`, `platform/wayland_error.rs`, `traits/surface.rs`, `Cargo.toml`.

	Summary: The OAL no longer relies on Cargo feature flags to enable the Wayland backend. The runtime is autodetected at startup and the crate's error types were unified so platform implementations can convert errors into the crate-level `OalError`. See the CHANGELOG entry for a short description.

- 2025-11-09 — JEleniel: Implement minimal `xdg-protocols` wrappers and tests.

	Summary: Added thin ergonomic wrappers in `xdg-protocols/src/shell.rs` and an integration test `xdg-protocols/tests/shell_tests.rs`.

- 2025-11-09 — JEleniel: Convert `xdg-protocols` to wayland-scanner v0.31 proc-macro generation.

	Summary: Switched the crate to use `wayland-scanner` v0.31 proc-macro generation for client bindings (via `wayland_scanner::generate_client_code!(...)`). Removed the checked-in fallback bindings; bindings are now generated at compile time from `protocols/xdg-shell-compat.xml`.

- 2025-11-09 — JEleniel: Prototype runtime protocol parser for xdg-protocols.

	Summary: Added `xdg-protocols/src/dynamic.rs` which parses vendored protocol XML into a runtime `Protocol` description. This enables experimentation with dynamic/adaptive handling of protocol versions without requiring compile-time code generation. Tests added in `xdg-protocols/tests/dynamic_tests.rs`.

## Work items for broader project testing & docs (condensed)

- [ ] Add integration tests for cross-component flows (forms, navigation, dialogs, multi-window scenarios).
- [ ] Add visual regression baseline/screenshot tests with diff reporting and cross-platform comparisons.
- [ ] Add performance benchmarks (rendering fps, memory under load, large datasets responsiveness).
- [ ] Expand documentation: inline API examples, tutorials, troubleshooting, and migration guides.
