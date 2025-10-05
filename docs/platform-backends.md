# Platform-Specific Backends

This document describes the platform-specific backend implementations for Engage UX, providing window management and graphics rendering across Windows, macOS, Linux, Android, and iOS.

## Overview

Engage UX uses a backend abstraction layer that allows platform-independent UI code to work across all supported platforms. The backend implementations are built using safe Rust abstractions to comply with the project's `#![forbid(unsafe_code)]` constraint.

## Architecture

### Backend Factory Pattern

The system uses a factory pattern to create platform-specific backends:

```rust
pub trait BackendFactory {
    fn create_renderer(&self) -> Box<dyn RenderBackend>;
    fn create_window_backend(&self) -> Box<dyn WindowBackend>;
}
```

At runtime, `get_backend_factory()` returns the appropriate factory for the current platform:

```rust
let factory = get_backend_factory();
let renderer = factory.create_renderer();
let window = factory.create_window_backend();
```

### Supported Platforms

- **Windows** - WindowsBackendFactory
- **macOS** - MacOSBackendFactory
- **Linux** - LinuxBackendFactory (supports both X11 and Wayland)
- **Android** - AndroidBackendFactory
- **iOS** - IOSBackendFactory

## Window Management Backend

### WinitWindowBackend

The window backend is implemented using the [winit](https://github.com/rust-windowing/winit) crate (v0.30), which provides safe, cross-platform window management.

#### Features

- **Window Creation**: Create and manage application windows
- **State Management**: Normal, Minimized, Maximized, Fullscreen
- **Bounds Control**: Position and size management
- **Event Generation**: Resized, Moved, CloseRequested, FocusGained, FocusLost
- **Properties**: Title, visibility, focus, resizable, decorated
- **DPI Support**: Automatic scaling factor tracking

#### Example Usage

```rust
use engage_ux_oal::backends::{get_backend_factory, WindowState, WindowBounds};

let factory = get_backend_factory();
let mut window = factory.create_window_backend();

// Set window properties
window.set_title("My Application");
window.set_bounds(WindowBounds::new(100, 100, 1280, 720));

// Manage window state
window.set_state(WindowState::Maximized);
window.show();

// Handle events
while let Some(event) = window.poll_event() {
    match event {
        WindowBackendEvent::CloseRequested => break,
        WindowBackendEvent::Resized { width, height } => {
            println!("Window resized to {}x{}", width, height);
        }
        _ => {}
    }
}
```

#### Window States

- `Normal` - Standard windowed mode
- `Minimized` - Window is minimized to taskbar/dock
- `Maximized` - Window fills the screen (with decorations)
- `Fullscreen` - Window occupies entire screen (no decorations)

#### Window Events

- `Resized { width, height }` - Window dimensions changed
- `Moved { x, y }` - Window position changed
- `CloseRequested` - User requested to close the window
- `FocusGained` - Window received focus
- `FocusLost` - Window lost focus
- `Minimized` - Window was minimized
- `Restored` - Window was restored from minimized state
- `Maximized` - Window was maximized
- `DpiChanged { scale }` - DPI scale factor changed

## Graphics Rendering Backend

### SoftbufferRenderer

The rendering backend is implemented using the [softbuffer](https://github.com/rust-windowing/softbuffer) crate (v0.4), which provides safe, CPU-based software rendering.

#### Features

- **Shape Rendering**: Rectangles (filled and stroked), circles, lines
- **Clipping**: Clip regions for constrained drawing
- **Color Support**: RGBA colors with alpha blending
- **Drawing Algorithms**: Bresenham's line algorithm, midpoint circle algorithm
- **Multi-Context**: Support for multiple render contexts

#### Example Usage

```rust
use engage_ux_oal::backends::{get_backend_factory, RenderCommand};
use engage_ux_oal::backends::renderer::{Color, Rect};

let factory = get_backend_factory();
let mut renderer = factory.create_renderer();

// Create a render context
let mut context = renderer.create_context(800, 600);

// Render a frame
context.begin_frame();

// Clear the screen
context.execute(RenderCommand::Clear(Color::rgb(0.0, 0.0, 0.0)));

// Draw a filled rectangle
context.execute(RenderCommand::FillRect {
    rect: Rect::new(50.0, 50.0, 200.0, 100.0),
    color: Color::rgba(1.0, 0.0, 0.0, 0.8),
});

// Draw a circle
context.execute(RenderCommand::Circle {
    x: 400.0,
    y: 300.0,
    radius: 50.0,
    color: Color::rgb(0.0, 1.0, 0.0),
    filled: true,
});

// Draw a line
context.execute(RenderCommand::Line {
    x1: 100.0,
    y1: 400.0,
    x2: 700.0,
    y2: 400.0,
    color: Color::rgb(1.0, 1.0, 0.0),
    width: 2.0,
});

context.end_frame();
```

#### Render Commands

- `Clear(color)` - Clear the entire surface with a color
- `FillRect { rect, color }` - Draw a filled rectangle
- `StrokeRect { rect, color, width }` - Draw a rectangle outline
- `Circle { x, y, radius, color, filled }` - Draw a circle
- `Line { x1, y1, x2, y2, color, width }` - Draw a line
- `Text { text, x, y, font_size, color, align }` - Draw text (requires font rasterization)
- `SetClip(rect)` - Push a clip region
- `RestoreClip` - Pop the last clip region

## Implementation Details

### Safe Rust Compliance

All platform backends are implemented using 100% safe Rust code:

- **No `unsafe` blocks** - All code complies with `#![forbid(unsafe_code)]`
- **Safe abstractions** - winit and softbuffer provide safe wrappers around native APIs
- **Memory safety** - Rust's ownership system prevents memory issues
- **Thread safety** - Types properly implement Send/Sync where appropriate

### Performance Characteristics

- **Rendering**: CPU-based software rendering suitable for UI elements
- **Memory**: Efficient buffer management with minimal allocations
- **Latency**: Low-latency event handling through winit's event loop
- **Throughput**: Suitable for typical UI workloads (60 FPS)

### Platform-Specific Notes

#### Windows
- Uses winit's Win32 backend
- Supports Windows 7 and later
- DPI awareness enabled by default
- Native taskbar integration

#### macOS
- Uses winit's Cocoa backend
- Supports macOS 10.11 and later
- Retina display support
- Native menu bar integration

#### Linux
- Supports both X11 and Wayland
- Runtime selection based on environment
- Full multi-monitor support
- Desktop environment integration

#### Android
- Uses winit's Android backend
- Supports Android 5.0 (API 21) and later
- Lifecycle management
- Touch input support

#### iOS
- Uses winit's UIKit backend
- Supports iOS 11 and later
- Lifecycle management
- Touch input support

## Testing

The platform backends include comprehensive test coverage:

### Unit Tests (11 tests)

- Window backend creation and properties (5 tests)
- Renderer creation and operations (6 tests)

### Integration Tests (14 tests)

- Backend factory creation
- Renderer operations (shapes, clipping, complex scenes)
- Window backend properties and state management
- Event generation and handling
- Multiple render contexts
- Hardware acceleration queries

Run the tests with:

```bash
cargo test --test test_platform_backends
```

## Future Enhancements

While the current implementation provides solid cross-platform support using safe Rust, future enhancements could include:

### Hardware-Accelerated Rendering

- **wgpu** integration for GPU-accelerated rendering
- Compute shader support for advanced effects
- Metal on Apple platforms, DirectX on Windows, Vulkan on Linux

### Advanced Window Features

- Multi-window support
- Custom window shapes (non-rectangular windows)
- Window thumbnails and previews
- Platform-specific window extensions

### Text Rendering

- Full text layout and shaping (using HarfBuzz or similar)
- Complex script support (Arabic, Hebrew, CJK)
- Font fallback chains
- Subpixel antialiasing

### Input Handling

- Raw input support (high-precision mouse, pen/stylus)
- Gamepad and joystick integration
- IME (Input Method Editor) support for international text input

## Dependencies

The platform backends rely on these external crates:

- **winit** (v0.30) - Cross-platform window management
- **softbuffer** (v0.4) - Cross-platform software rendering
- **raw-window-handle** (v0.6) - Window handle abstraction

All dependencies are actively maintained and widely used in the Rust ecosystem.

## Contributing

When contributing to the platform backends:

1. **Maintain safe Rust** - Never add `unsafe` code
2. **Test on multiple platforms** - Verify changes work on all supported platforms
3. **Follow existing patterns** - Use the established backend factory pattern
4. **Add tests** - Include unit and integration tests for new features
5. **Document platform differences** - Note any platform-specific behavior

## License

The platform backend implementations are licensed under the same terms as the main Engage UX project (MIT OR Apache-2.0).
