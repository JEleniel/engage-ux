# Changelog

## 2025-11-02 — v1.0.0-pre (pre-release changes)

- Remove core screen reader API and announcement trait from `engage-ux-core`.
    + The `screen_reader` module has been removed from the accessibility module. Backends / OALs should implement any platform-specific screen reader integration as needed.
- Move `FocusManager` from `engage-ux-core/src/modules/accessibility` into the input subsystem at `engage-ux-core/src/modules/input/focus_manager.rs` and re-export it from `crate::modules::accessibility` for backward compatibility.
- Update compatibility re-exports in `modules/props.rs` and `modules/accessibility.rs` to reflect removal of `ScreenReader`.

Notes:

- These changes simplify the core API surface for the pre-release: accessibility remains declarative via `AccessibilityProps` and core focus state is handled by the central input/focus subsystem.
- Backends may continue to provide screen reader integrations; the core no longer provides or calls a generic screen reader trait.
