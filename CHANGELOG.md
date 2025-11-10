# Changelog

## 2025-11-09 — JEleniel: Convert xdg-protocols to wayland-scanner v0.31 proc-macro generation

- Convert `xdg-protocols` to use `wayland-scanner` v0.31 proc-macro generation for client bindings.
- Remove checked-in fallback bindings in `xdg-protocols/src/xdg_client.rs` and rely on `wayland_scanner::generate_client_code!("protocols/xdg-shell-compat.xml")` to produce bindings at compile time.

## 2025-11-09 — JEleniel: Implement xdg-protocols wrappers and tests

- Add ergonomic, thin wrappers in `xdg-protocols/src/shell.rs` that forward to generated (or stubbed) xdg client bindings. Include a small integration test (`xdg-protocols/tests/shell_tests.rs`) that exercises the forwarding surface. The crate continues to emit a minimal stub from `build.rs` when `wayland-scanner` isn't available.

## 2025-11-09 — JEleniel: Add runtime protocol parser prototype

- Add a lightweight runtime XML parser prototype in `xdg-protocols/src/dynamic.rs` which can parse vendored protocol XML files into a small, dynamic description (`Protocol`, `Interface`, `Message`). Added `roxmltree` and `anyhow` as dependencies and an integration test at `xdg-protocols/tests/dynamic_tests.rs`.

## 2025-11-02 — v1.0.0-pre (pre-release changes)

- Remove core screen reader API and announcement trait from `engage-ux-core`.
    + The `screen_reader` module has been removed from the accessibility module. Backends / OALs should implement any platform-specific screen reader integration as needed.
- Move `FocusManager` from `engage-ux-core/src/modules/accessibility` into the input subsystem at `engage-ux-core/src/modules/input/focus_manager.rs` and re-export it from `crate::modules::accessibility` for backward compatibility.
- Update compatibility re-exports in `modules/props.rs` and `modules/accessibility.rs` to reflect removal of `ScreenReader`.

Notes:

- These changes simplify the core API surface for the pre-release: accessibility remains declarative via `AccessibilityProps` and core focus state is handled by the central input/focus subsystem.
- Backends may continue to provide screen reader integrations; the core no longer provides or calls a generic screen reader trait.

## 2025-11-09 — JEleniel: Remove OAL feature gating

- Removed feature gating from the OAL crate so platform backends (Wayland) are compiled by default. This simplifies autodetection and embedding: the runtime is instantiated at `Oal::new()` when available. See `docs/design/agents/TODO.md` for follow-ups and implementation notes.
