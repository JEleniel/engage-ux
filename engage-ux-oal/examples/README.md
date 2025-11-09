# Wayland example

This example demonstrates a minimal use of the `engage-ux-oal` Wayland runtime.

- Build with the `wayland` feature enabled.

To run the example locally against a Wayland compositor (e.g., Sway, Weston):

```bash
cargo run -p engage-ux-oal --features wayland --example wayland_example
```

Notes:

- The `WaylandRuntime` must be created and driven on the application's main thread.
- The handle returned by `WaylandRuntime::new()` is `Send + Sync` and can be used from other threads to queue main-thread tasks.
- The example is intentionally tiny and intended for manual runs on a Wayland session.
