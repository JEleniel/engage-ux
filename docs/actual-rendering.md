# Actual Window Rendering

## Current Status

The Engage UX project has two types of backends:

### 1. Stub Backends (Default - Phase 4 Complete)

The current default backends (`WinitWindowBackend`, `SoftbufferRenderer`, `TinySkiaRenderer`) are **simulation/stub implementations** used for testing:

- ✅ Track window state (bounds, title, visibility, etc.)
- ✅ Generate events (resize, focus, close, etc.)
- ✅ Maintain pixel buffers for rendering commands
- ✅ Support all rendering operations (shapes, lines, text, clipping)
- ❌ **Do NOT create actual OS windows**
- ❌ **Do NOT display visual output on screen**

These are intentionally designed this way to allow:

- Unit testing without requiring a display
- CI/CD pipeline testing in headless environments
- Architecture validation and API design
- Integration testing of components

### 2. Actual Visual Rendering (Example Available)

For **actual visual window rendering**, see the working example:

```bash
cargo run --example visual_window_demo -p engage-ux-oal
```

This example demonstrates:

- ✅ **Real window creation** using winit
- ✅ **Visual rendering** using softbuffer
- ✅ **Actual display** on your screen
- ✅ Drawing shapes, text, and graphics
- ✅ Event loop handling
- ✅ Window management

**Code:** `engage-ux-oal/examples/visual_window_demo.rs`

## Why Two Types?

### Stub Backends (Current Default)

- **Purpose**: Testing, CI/CD, architecture validation
- **Use case**: Unit tests, integration tests, examples
- **Benefit**: Works in headless environments
- **Limitation**: No visual output

### Visual Rendering Example

- **Purpose**: Actual application development
- **Use case**: Real applications with GUI
- **Benefit**: Actual windows and rendering
- **Limitation**: Requires display server (X11/Wayland/Windows/macOS)

## Running the Visual Example

### On Linux (with X11 or Wayland)

```bash
cargo run --example visual_window_demo -p engage-ux-oal
```

### On Windows

```bash
cargo run --example visual_window_demo -p engage-ux-oal
```

### On macOS

```bash
cargo run --example visual_window_demo -p engage-ux-oal
```

### Expected Output

A window will appear showing:

- Dark blue-gray background
- Three colored rectangles (red, green, blue)
- Outlined rectangle (yellow)
- Filled and outlined circles
- A diagonal line
- Text message at the bottom

## Integrating Visual Rendering into Your Application

To create a real application with visual rendering, follow the pattern in `visual_window_demo.rs`:

1. Create a winit `EventLoop`
2. Create a `Window` using winit
3. Create a softbuffer `Surface` for the window
4. Implement the `ApplicationHandler` trait
5. Draw to the buffer in your render loop
6. Present the buffer to display

## Future Work (Phase 7)

The roadmap includes:

- [ ] Integrate actual rendering into the default backends
- [ ] Hardware-accelerated GPU rendering (wgpu)
- [ ] Component-to-renderer integration
- [ ] Theme-aware rendering
- [ ] Font and image rendering integration

## Summary

**Current examples** (basic_components, theme_demo, etc.):

- Use stub backends
- Print console output
- No visual window displayed
- **This is intentional** for testing

**Visual rendering example** (visual_window_demo):

- Uses actual winit + softbuffer
- Creates real window
- Displays graphics on screen
- **Requires display server**

Both approaches are valid depending on your use case!
