# Implementation Roadmap

## Current Status Summary (January 2025)

**Overall Completion: ~95%**

- ✅ **Phases 1-6**: All major features complete
- ✅ **Components**: All 50 components implemented and tested
- ✅ **Platform Backends**: All 5 platforms supported (Windows, macOS, Linux, Android, iOS)
- ⏳ **Testing**: 541/611 tests (88% complete) - 70 tests remaining
- ⏳ **Documentation**: 95% complete

**Actual Test Breakdown:**

- engage-ux-components: 223 tests
- engage-ux-core: 123 tests
- engage-ux-tests: 132 integration tests
- engage-ux-oal: 54 tests

# Wayland OAL — Agent TODO

This file tracks remaining work for the Wayland OS Abstraction Layer (OAL) implementation maintained by agents and contributors. It focuses on concrete, testable tasks required to reach a robust, production-ready Wayland backend.

## Summary

The repository already contains a working baseline: a `WaylandRuntime` with a main-thread processing loop, a `WaylandHandle` that implements `Oal` for queuing tasks, a `WaylandWindow` implementation, and a thread-safe `WaylandSurface` wrapper that marshals presents from background threads into the runtime. Unit tests exist that exercise anonymous-file creation, the runtime queue, and basic surface enqueue/present roundtrips without requiring a compositor.

The remaining work falls into several focused areas:

1. API contract and acceptance criteria
2. Main-thread runtime improvements (bounded processing, shutdown)
3. Buffer pooling and lifecycle management
4. Input event mapping
5. Accessibility bridge (main-thread-only)
6. Clipboard and IME baseline
7. Timers and main-thread task API completion
8. Dirty-region / partial redraw semantics
9. Unit tests and mocks expansion
10. Integration example enhancements
11. CI, build automation, docs and changelog
12. Optional GPU/Skia path (deferred, gated)

For each area below there are concrete steps, expected deliverables, and acceptance criteria.

## Details and actionable tasks

### 1) Define API contract and acceptance criteria

- Deliverable: short contract document (inputs, outputs, error modes, thread-affinity) and a list of Rust types/traits to implement/refine.
- Location: doc-comments in `engage-ux-oal/src/traits.rs` + this file (TODO entry).
- Acceptance: contract exists and unit tests reference/validate key invariants (e.g., runtime main-thread affinity, OAL error mappings).

### 2) Main-thread runtime improvements

- Deliverable: ensure `WaylandRuntime` supports bounded task processing, graceful shutdown, and a public `process_main_thread_tasks`/`run_once` API.
- Add a `shutdown()`/`Stop` runtime message that clears `GLOBAL_WAYLAND_HANDLE` and tears down registered surfaces.
- Acceptance: runtime can be cleanly shut down from another thread; unit tests exercise budgeting and shutdown behavior.

### 3) Buffer pooling and lifecycle management (high priority)

- Deliverable: implement robust per-`WaylandWindow` buffer pooling using `wl_shm` buffers, correct `wl_buffer` release handling, and resize semantics that drop incompatible buffers.
- Tests: buffer selection logic, release handler marking busy/free, resize replacing incompatible buffers, and FD lifetime (memfd/tempfile mocks).
- Acceptance: resizes replace incompatible buffers; no FD leaks; tests cover lifecycle.

### 4) Input event mapping

- Deliverable: map `wl_pointer`, `wl_keyboard`, `wl_touch` into `engage-ux-core` input events and publish via the Window event callback or an event bus.
- Acceptance: sample/mocks show Wayland events transformed into core event types and delivered to the embedder.

### 5) Accessibility bridge implementation

- Deliverable: `WaylandAccessibilityBridge` implementing `AccessibilityBridge` (main-thread-only). No-op on platforms without accessibility.
- Acceptance: bridge compiles, can be swapped in tests, and emits expected OS mapping under a mock.

### 6) Clipboard and IME baseline

- Deliverable: minimal clipboard `get`/`set` functions and IME input stubs integrated into input flow.
- Acceptance: clipboard set/get roundtrip works in tests or gracefully falls back on headless CI.

### 7) Timers and main-thread task API completion

- Deliverable: robust `WaylandHandle::queue_main_thread` and `queue_main_thread_with_handle` implementations (oneshot semantics, blocking and non-blocking variants).
- Acceptance: tests verify blocking/non-blocking variants, oneshot responder behavior, timeouts, and error modes.

### 8) Dirty-region / partial redraw semantics

- Deliverable: ensure `wl_surface.damage_buffer` is called with provided device rects; software fallback honors same dirty semantics.
- Acceptance: unit tests (mocking) demonstrate partial updates and parity between GPU and software paths.

### 9) Unit tests and mocks expansion

- Deliverable: tests for anonymous-file creation, buffer selection, queue-roundtrips, `Surface` wrapper behavior, input mapping, buffer lifecycle, and FD/handle cleanup. Provide mock types for Wayland objects where feasible so tests do not require a compositor.
- Acceptance: `cargo test -p engage-ux-oal --features=wayland` passes on CI for tests that don't need a compositor. Integration tests requiring a compositor are documented, optional, and marked/ignored in CI.

### 10) Integration example enhancements

- Deliverable: improve `examples/wayland_example.rs` to demonstrate `SurfaceBuilder::build()` usage, presenting from a worker thread, and main-thread processing loop with a budget and graceful shutdown.
- Acceptance: example builds and runs on a Wayland compositor and demonstrates intended behavior; usage documented.

### 11) CI, build, and automation

- Deliverable: CI steps to build with `--features=wayland` (where supported), run unit tests, and run lint/format checks. Add matrix entries for optional features (GPU/Skia) later.
- Acceptance: CI reports build/test status; where Wayland is not available tests are skipped or use mocks.

### 12) Optional GPU/Skia design (deferred, gated)

- Deliverable: design a feature-flagged Skia GPU path with deterministic software fallback and feature gating.
- Acceptance: gated behind a feature (e.g., `skia_gpu`), compile-tested in CI for gated configuration, and smoke-tested where GPU resources exist.

## Quality gates and verification

- Always run unit tests after making changes to `engage-ux-oal`.
- Prefer to run `cargo test -p engage-ux-oal --features=wayland` locally and in CI.
- Add unit tests first for any new behavior (happy path + 1-2 edge cases).
- Run `cargo build` across workspace, `rustfmt`, and `clippy` (CI will run these where available).

## Files & places to edit

- Primary implementation: `engage-ux-oal/src/platform/wayland.rs`
- API/traits: `engage-ux-oal/src/traits.rs` (add doc comments/contract)
- Tests: keep runtime/unit tests next to `wayland.rs` (cfg test) and add integration tests under `engage-ux-oal/tests` for higher-level scenarios.
- Docs: `docs/OAL.md`, `docs/design/agents/TODO.md` (this file), example updates in `engage-ux-oal/examples/wayland_example.rs`.
- Cargo features: `engage-ux-oal/Cargo.toml` (feature gating for `wayland`, optional deps like `once_cell`, `memfd`, `wayland-client`).

## Acceptance criteria recap (short)

- Main runtime supports bounded processing and graceful shutdown.
- `Surface::present_frame()` is callable from background threads and reliably schedules presents to the main thread.
- Buffer pooling and release handling prevent fd leaks and replace incompatible buffers on resize.
- Input events are mapped to `engage-ux-core` types and delivered to embedder.
- Accessibility, clipboard, and IME basics present and documented with graceful fallbacks.
- Extensive unit tests and CI coverage for all non-compositor-dependent tests.

## Next immediate steps (recommended)

1. ✅ Finalized the API contract in `engage-ux-oal/src/traits.rs` (doc-comments added) and added minimal unit tests validating small contract invariants. (2025-11-08)
2. Implement runtime shutdown messaging (`RuntimeMessage::Shutdown`/`Stop`) and add tests verifying clean teardown and `GLOBAL_WAYLAND_HANDLE` clearing.
3. Implement buffer pooling/cleanup tests (mock `wl_buffer` release handler) and ensure no fd leaks.

If you'd like, I can start implementing step (1) now (contract + short tests) or proceed with step (2) (runtime shutdown). Choose one and I will make the edits + run tests.

- ✅ **Linux**: LinuxScreenReader using AT-SPI D-Bus protocol (70 lines)
- ✅ **Android**: AndroidScreenReader using TalkBack APIs (69 lines)
- ✅ **iOS**: IOSScreenReader using VoiceOver APIs (67 lines)

#### Linux AT-SPI Integration ✅

- ✅ **COMPLETED** - AT-SPI accessibility bridge (323 lines of code)
   	+ Full D-Bus protocol implementation for AT-SPI
   	+ Component state management (Focusable, Focused, Enabled, Visible, etc.)
   	+ Role and property mapping
   	+ Event notification system
   	+ Focus tracking
   	+ 14 Linux-specific accessibility tests

**Implementation Notes**:

- All screen reader backends provide consistent APIs across platforms
- Stub implementation available for testing and unsupported platforms
- Platform-specific integration follows native accessibility guidelines
- Full WCAG AAA compliance support through accessibility infrastructure

---

## Work Remaining to 100%

### Testing Gap (70 tests needed)

#### Additional Integration Tests (~40 tests)

- [ ] End-to-end workflow tests for complex component interactions
    + Form submission with validation
    + Navigation flows with state persistence
    + Data table with sorting, filtering, and pagination
    + Modal dialog chains and state management
- [ ] Cross-component event propagation tests
    + Event bubbling through component hierarchy
    + Focus management across nested components
    + Keyboard shortcuts with multiple listeners
- [ ] Multi-window application scenarios
    + Window-to-window communication
    + State synchronization across windows
    + Multi-monitor window placement
- [ ] Complex layout constraint resolution
    + Deeply nested layouts with mixed units
    + Circular constraint detection
    + Layout invalidation and recomputation
- [ ] Platform-specific backend edge cases
    + Window resize with minimum/maximum bounds
    + DPI scaling transitions
    + Multi-touch gesture edge cases
    + Platform-specific key bindings
- [ ] More comprehensive accessibility validation scenarios
    + Complete keyboard navigation flows
    + Screen reader announcement sequences
    + ARIA state synchronization
    + Focus trap scenarios
- [ ] Theme switching with component state preservation
    + Hot reload theme without losing state
    + Transition animations between themes
    + Custom theme color palette validation

#### Visual Regression Tests (~20 tests)

- [ ] Automated screenshot comparison tests for all components
    + Baseline screenshots for each component in default state
    + Diff detection algorithm with tolerance thresholds
    + Automated test failure reporting with visual diffs
- [ ] Cross-platform rendering consistency validation
    + Windows vs macOS vs Linux rendering comparison
    + Font rendering consistency (anti-aliasing, hinting)
    + Color accuracy across platforms
- [ ] Theme application verification (light/dark/custom themes)
    + All components rendered in light theme
    + All components rendered in dark theme
    + LCARS theme visual accuracy
    + Custom theme color application
- [ ] High DPI rendering accuracy tests
    + 1x, 1.5x, 2x, 3x scaling factors
    + Text clarity at different scales
    + Icon sharpness verification
- [ ] Component rendering with various states
    + Hover state visual feedback
    + Focus state indicators
    + Disabled state appearance
    + Loading state animations
    + Error state styling

#### Performance Tests (~10 tests)

- [ ] Rendering performance benchmarks (target: 60 FPS)
    + Frame time measurements for complex scenes
    + Rendering pipeline profiling
    + CPU usage during continuous rendering
- [ ] Memory usage validation under load
    + Memory consumption with 1000+ components
    + Memory leak detection tests
    + Memory usage growth over time
- [ ] Component responsiveness tests with large datasets
    + Table with 10,000+ rows performance
    + List with 5,000+ items scrolling performance
    + Tree view with deep nesting
- [ ] Animation smoothness benchmarks
    + Concurrent animation performance
    + Easing function CPU overhead
    + Animation frame consistency
- [ ] Startup time measurements
    + Cold start performance
    + Component initialization overhead
    + Theme loading time

### Documentation Enhancements

- [ ] Add more inline code examples in API documentation
- [ ] Create comprehensive tutorial series
- [ ] Expand troubleshooting guides with common issues
- [ ] Add migration guide from other UI frameworks
- [ ] Create platform-specific deployment guides

---

## Future Enhancements (Post Phase 6)

#### Advanced Testing ⏳

- End-to-end functional tests (basic platform backend tests implemented)
- Platform-specific visual testing (partially implemented)
- Performance benchmarking suite (planned)
- Automated visual regression testing (planned)

#### Native Integration ⏳

- Native file dialogs
- System tray integration
- Native notifications
- Clipboard integration
- System color scheme detection
- Native window decorations

#### Performance Optimization ⏳

- Hardware-accelerated GPU rendering (wgpu)
- Virtual scrolling for large lists
- Component pooling and reuse
- SIMD optimizations
- Incremental rendering optimizations

---

## Phase 7 (Future)

Future work for client/server rendering:

- Support for client/server rendering.
    + Mode 1 (default): The server renders the image, using the monitor layout of the client, and sends the compressed, rendered view to the client. The client then displays the view, and returns any input to the server. This is meant for use cases where the server has the rendering horsepower.
    + Mode 2: The server sends all information required to render the UX to the client who then renders the view. The client sends any input events to the server. This is meant for the use case where the client has rendering horsepower.
    + The entire connection must be encrypted with a minimum equivalent to TLS 1.3 (you may use HTTPS and TLS 1.3 if it will be performant enough). Both client and server must support using the OS Certificate Authorities as well as configurable additional CAs. The server must support both encrypted and unencrypted key files.
    + All connections must be fully authenticated. Support for built in Windows authentication (including Active Directory), Linux PAM, LDAP, and OAuth are required. Support for basic user/password authentication will not be supported.
    + Minimally, the system must be able to render, send, and display 24fps video without noticable stuttering or any degradation. Ideally, it should be able to support 4k 120fps, given sufficient bandwidth.

## Implementation Guidelines For Machine Agents

When implementing features, follow these guidelines:

### Review Architecture First

- Read the relevant architecture documents before coding
- Understand the requirements and acceptance criteria
- Review the NFRs to ensure compliance
- Study the diagrams to understand system interactions

### Follow Design Patterns

- Use trait-based architecture (Component, RenderBackend, WindowBackend)
- Apply builder pattern for platform-specific implementations
- Use observer pattern for events (broadcast channels)
- Follow strategy pattern for platform-specific behavior
- Use adapter pattern for OS API integration

### Component Implementation Checklist

When implementing a new component:

- [ ] Implement `Component` trait
- [ ] Add `ComponentProperties` field
- [ ] Implement builder pattern for construction
- [ ] Add event callbacks (on_click, on_change, etc.)
- [ ] Add accessibility properties (ARIA role, label)
- [ ] Support theme colors and styles
- [ ] Implement rendering logic
- [ ] Add comprehensive unit tests (creation, properties, events, state)
- [ ] Add documentation comments with examples
- [ ] Test with both light and dark themes
- [ ] Verify keyboard navigation works
- [ ] Verify screen reader announcements
- [ ] Profile performance if interactive

### Backend Implementation Checklist

When implementing a platform backend:

- [ ] Implement platform-specific trait (WindowBackend or RenderBackend)
- [ ] Handle all required operations
- [ ] Convert between Engage types and platform types
- [ ] Handle platform-specific events
- [ ] Ensure thread safety
- [ ] Handle errors gracefully
- [ ] Add platform-specific tests
- [ ] Document platform-specific requirements

- [ ]
- [ ] Ensure performance meets targetsVerify memory is released properly
