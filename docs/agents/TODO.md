# Agent TODO

## Summary

This file is a compact, actionable TODO for agent tasks. It is not meant to contain progress notes of details. Use this as the primary source of remaining work.

## Maintenance

- This file is the primary, concise agent TODO remaining work.
- Keep it checklist-style (short actionable lines).
- Do NOT include completion checkmarks, dates, or long status paragraphs in this file; keep historical or release notes in `CHANGELOG.md` instead.
- When work starts, update this file with the minimal checkbox state. When work completes, remove the item from this file and append an entry to the `CHANGELOG.md` as required by the project guidance.

## Agent Work Plan (short)

- [ ] Review and align agent TODO (`docs/agents/TODO.md`)
- [ ] Prepare implementation plan
- [ ] Update `docs/agents/TODO.md` to reflect the plan
- [ ] Report back and propose next steps (tests, commits, further work)

## OAL Implementation (all backends)

- [ ] Implement Backend trait for Wayland, X11, Windows, MacOS, iOS, Android in `engage-ux-oal`

- [ ] Integrate EventBus for platform input delivery (map platform events -> `engage_ux_core` events)

- [ ] Align Unit system and DeviceMetrics to production model
   	+ Replace simplified `pixels_per_unit` with `dpcm`, `dpi`, `dpr` or document the supported simplification

- [ ] Implement U→px conversion using device metrics (DPCM/DPI) across backends

- [ ] Expose and wire logical Canvas, View, and Window APIs in the runtime
   	+ `Canvas` and `Window` exist; add `Window::view` and `Window::set_view` if movable views are required

- [ ] Ensure rendering runs on main/UI thread; keep non-render logic on EventBus threads

- [ ] Implement consistent error handling and surface error propagation via `OalError`/EventBus

- [ ] Implement full window/surface lifecycle per Backend trait (create, reconfigure, present, destroy)

- [ ] Implement input mapping (pointer, keyboard, touch) to core input events for each platform backend

- [ ] Add accessibility, clipboard, and IME stubs/bridges (platform-specific implementations later)

- [ ] Implement buffer lifecycle / swapchain semantics and partial redraw support (buffer pooling optional)

- [ ] Add targeted unit and integration tests:
   	+ Unit conversion tests for `Unit::to_px`/`px_to_units`
   	+ Renderer GPU submission path tests (`try_gpu_render` + `submit_render`)
   	+ Backend trait compliance tests (headless + native-winit where available)

- [ ] Add CI steps to build and test with/without `native-winit` features and run unit/integration tests

- [ ] Document backend-specific notes, edge cases, and platform integration details in code and docs

### Current engage-ux-oal actionable work (focused)

## Work items for broader project testing & docs (condensed)

- [ ] Add integration tests for cross-component flows (forms, navigation, dialogs, multi-window scenarios).
- [ ] Add visual regression baseline/screenshot tests with diff reporting and cross-platform comparisons.
- [ ] Add performance benchmarks (rendering fps, memory under load, large datasets responsiveness).
- [ ] Expand documentation: inline API examples, tutorials, troubleshooting, and migration guides.

## Recent agent actions

- Core modules API review: harmonized component identifier usage to the `ComponentId` alias and introduced a `Timestamp` alias in the event module. Applied minimal edits to `engage-ux-core/src/component_properties.rs` and `engage-ux-core/src/modules/event/event.rs`. Verified with `cargo check -p engage-ux-core` (warnings only).
