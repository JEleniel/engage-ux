# Android Backend Quick Start

## Overview

The Android backend is implemented using cross-platform libraries (winit + softbuffer) that provide native Android integration while maintaining 100% safe Rust code.

## Usage

### Basic Setup

```rust
use engage_ux_oal::backends::get_backend_factory;

// Get platform-specific backend (auto-detects Android)
let factory = get_backend_factory();
let mut renderer = factory.create_renderer();
let mut window = factory.create_window_backend();
```

### Rendering

```rust
// Create render context
let mut context = renderer.create_context(1080, 1920);

// Render frame
context.begin_frame();

// Clear background
context.execute(RenderCommand::Clear(Color::rgb(1.0, 1.0, 1.0)));

// Draw UI elements
context.execute(RenderCommand::FillRect {
    rect: Rect::new(0.0, 0.0, 1080.0, 56.0),
    color: Color::rgb(0.2, 0.4, 0.8),
});

context.end_frame();
```

### Window Management

```rust
// Set window properties
window.set_title("My Android App");
window.set_bounds(WindowBounds::new(0, 0, 1080, 1920));

// Handle events
while let Some(event) = window.poll_event() {
    match event {
        WindowBackendEvent::Resized { width, height } => {
            // Handle orientation change
        }
        WindowBackendEvent::CloseRequested => {
            // Handle back button or app close
        }
        _ => {}
    }
}
```

## Key Features

### Canvas API Integration

All render commands are mapped to Android Canvas API calls:

-	`Clear(color)` → `Canvas.drawColor()`
-	`FillRect` → `Canvas.drawRect()` with Paint fill
-	`Circle` → `Canvas.drawCircle()`
-	`Line` → `Canvas.drawLine()`
-	`SetClip/RestoreClip` → `Canvas.save()/restore()`

### Activity Lifecycle

Window backend integrates with Android Activity lifecycle:

```rust
// onPause (app goes to background)
window.set_state(WindowState::Minimized);

// onResume (app comes to foreground)
window.set_state(WindowState::Normal);

// onDestroy (app closes)
window.close();
```

### Touch Events

Touch events are handled through focus management:

```rust
// Handle touch/tap
window.request_focus();
assert!(window.is_focused());
```

### Accessibility (TalkBack)

TalkBack integration is automatic through Android's native accessibility APIs:

```rust
// Set accessible title
window.set_title("My Accessible App");

// TalkBack will announce:
// - Window titles
// - Focus changes
// - State transitions
```

### DPI Scaling

Support for various Android device densities:

```rust
let scale = window.scale_factor();
let button_height = 48.0 * scale; // 48dp button
```

## Testing

### Run All Tests

```bash
cargo test --all
```

### Run Android-Specific Tests

```bash
# On Android device/emulator
cargo test --target aarch64-linux-android test_android

# On other platforms (tests will be ignored)
cargo test test_android
```

## Building for Android

### Prerequisites

1.	Install Android NDK
2.	Add Rust Android targets:

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
```

### Build

```bash
# Build for 64-bit ARM
cargo build --target aarch64-linux-android --release

# Build for 32-bit ARM
cargo build --target armv7-linux-androideabi --release
```

## Common Patterns

### Material Design UI

```rust
context.begin_frame();

// App bar (56dp)
context.execute(RenderCommand::FillRect {
    rect: Rect::new(0.0, 0.0, 1080.0, 56.0),
    color: Color::rgb(0.2, 0.4, 0.8),
});

// Floating Action Button
context.execute(RenderCommand::Circle {
    x: 980.0,
    y: 1820.0,
    radius: 28.0,
    color: Color::rgb(1.0, 0.3, 0.3),
    filled: true,
});

context.end_frame();
```

### Scrollable Content

```rust
// Clip to content area
context.execute(RenderCommand::SetClip(
    Rect::new(0.0, 56.0, 1080.0, 1864.0)
));

// Draw scrollable content
context.execute(RenderCommand::FillRect {
    rect: Rect::new(0.0, 56.0, 1080.0, 3000.0),
    color: Color::rgb(0.9, 0.9, 0.9),
});

context.execute(RenderCommand::RestoreClip);
```

### Handle Orientation Change

```rust
match window.poll_event() {
    Some(WindowBackendEvent::Resized { width, height }) => {
        // Recreate render context with new dimensions
        context = renderer.create_context(width, height);
        
        // Adjust UI layout for new size
        if width > height {
            // Landscape
        } else {
            // Portrait
        }
    }
    _ => {}
}
```

## Performance Tips

1.	**Batch Commands**: Execute multiple render commands in one frame
2.	**Use Clipping**: Limit overdraw with clip regions
3.	**Minimize Allocations**: Reuse buffers and contexts when possible
4.	**DPI-Aware Sizing**: Use scale factor for proper sizing

## Troubleshooting

### Build Errors

**Issue**: Missing Android NDK

```
Error: linker `aarch64-linux-android-clang` not found
```

**Solution**: Install Android NDK and set environment variables:

```bash
export ANDROID_NDK_HOME=/path/to/ndk
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
```

### Runtime Issues

**Issue**: Window not showing

**Solution**: Ensure Activity is properly initialized in AndroidManifest.xml

**Issue**: Touch events not working

**Solution**: Verify focus management in window backend

## Further Reading

-	[Android Backend Implementation](./Android_Backend_Implementation.md) - Comprehensive documentation
-	[Android Canvas API](https://developer.android.com/reference/android/graphics/Canvas)
-	[Android Activity Lifecycle](https://developer.android.com/guide/components/activities/activity-lifecycle)
-	[winit Android Support](https://docs.rs/winit/latest/winit/platform/android/)

## API Reference

### Backend Factory

```rust
pub fn get_backend_factory() -> Box<dyn BackendFactory>
```

Returns the Android backend factory on Android platform.

### Render Backend

```rust
pub trait RenderBackend {
    fn create_context(&mut self, width: u32, height: u32) -> Box<dyn RenderContext>;
    fn name(&self) -> &str;
    fn is_hardware_accelerated(&self) -> bool;
}
```

### Window Backend

```rust
pub trait WindowBackend {
    fn bounds(&self) -> WindowBounds;
    fn set_bounds(&mut self, bounds: WindowBounds);
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn state(&self) -> WindowState;
    fn set_state(&mut self, state: WindowState);
    fn poll_event(&mut self) -> Option<WindowBackendEvent>;
    fn scale_factor(&self) -> f32;
    // ... more methods
}
```

## Examples

See `engage-ux-tests/test_android_backend.rs` for comprehensive examples of:

-	Canvas API rendering
-	Activity lifecycle handling
-	Touch event processing
-	Accessibility integration
-	DPI scaling
-	Material Design patterns

## Support

For issues or questions about the Android backend:

1.	Check existing tests in `engage-ux-tests/test_android_backend.rs`
2.	Review documentation in `Android_Backend_Implementation.md`
3.	Open an issue on GitHub with Android-specific details
