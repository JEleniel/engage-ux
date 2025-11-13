# Changelog

## Unreleased Changes

- OAL: Added graceful shutdown support for event loops. Introduced `Oal::stop_event_loop()` and a run-loop flag to allow headless and native loops to exit cleanly.
- OAL: Added headless example demonstrating `NoopRenderer` and graceful shutdown (`engage-ux-oal/examples/headless_oal.rs`).
- OAL: Added integration test for headless event loop shutdown (`engage-ux-oal/tests/stop_event_loop.rs`).
- OAL: Audited library code for panics/unwrapped mutex usage and ensured poisoned mutex locks return `OalError::PoisonedLock` where appropriate.
- OAL: Introduced a `Backend` trait and a simple `HeadlessBackend` implementation to provide a platform-agnostic backend contract.
- OAL: Added `Platform` wrapper (thin Arc<dyn Backend>) to simplify backend usage.
- OAL: Added lightweight backend shims for Wayland and X11 that delegate to the headless backend as placeholders for future full implementations.
- OAL: Migrated `oal` backend modules to the modern module layout (removed `mod.rs` usage and added `backends.rs`).
