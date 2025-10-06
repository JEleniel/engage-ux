# Android Backend Implementation

## Overview

The Android backend for Engage UX uses cross-platform libraries (winit + softbuffer) that provide safe, 100% Rust implementations while integrating seamlessly with Android's native graphics and window management systems.

## Architecture

### Renderer: Softbuffer

The Android renderer uses `softbuffer`, a safe software rendering library that:

-	Integrates with Android's Canvas API through winit's Android support layer
-	Provides all required rendering primitives (rectangles, circles, lines, text)
-	Supports clipping regions for nested view hierarchies
-	Implements color blending and transparency
-	Works without unsafe Rust code

**Canvas API Mapping**:

| Engage UX Command      | Android Canvas API          |
| ---------------------- | --------------------------- |
| `Clear(color)`         | `Canvas.drawColor()`        |
| `FillRect`             | `Canvas.drawRect()` (fill)  |
| `StrokeRect`           | `Canvas.drawRect()` (stroke)|
| `Circle`               | `Canvas.drawCircle()`       |
| `Line`                 | `Canvas.drawLine()`         |
| `Text`                 | `Canvas.drawText()`         |
| `SetClip` / `RestoreClip` | `Canvas.save()` / `Canvas.restore()` |

### Window Backend: Winit

The Android window backend uses `winit`, which provides:

-	Integration with Android Activity lifecycle
-	Native touch event handling
-	Window state management (minimized, maximized, fullscreen)
-	DPI scaling support for various Android device densities
-	Accessibility API integration for TalkBack support

## Android-Specific Features

### Activity Lifecycle Support

The window backend integrates with Android's Activity lifecycle:

-	**onCreate/onStart** → Window creation and initialization
-	**onPause** → Window minimized state
-	**onResume** → Window restored state
-	**onDestroy** → Window close event

```rust
// Example: Handle Activity lifecycle
let factory = get_backend_factory();
let mut window = factory.create_window_backend();

// onPause
window.set_state(WindowState::Minimized);

// onResume
window.set_state(WindowState::Normal);

// onDestroy
window.close();
```

### Touch Event Handling

The window backend supports Android touch events through winit's event system:

-	Single touch (tap)
-	Multi-touch gestures
-	Touch focus events
-	Touch coordinate mapping

Focus management is provided for tap events:

```rust
// Handle tap (focus request)
window.request_focus();
assert!(window.is_focused());
```

### Accessibility (TalkBack)

The Android backend is TalkBack-ready through winit's integration with Android's accessibility APIs:

-	Window titles are exposed to TalkBack
-	Focus management works with accessibility services
-	DPI scaling supports accessibility features (text size, touch targets)
-	Native Android accessibility layer provides screen reader integration

```rust
// Ensure window has accessible title
window.set_title("My App");

// Accessibility services can query:
// - Window title for announcements
// - Focus state for navigation
// - Scale factor for text sizing
```

### DPI Scaling

Android devices have various screen densities (mdpi, hdpi, xhdpi, xxhdpi, xxxhdpi). The backend supports DPI scaling:

```rust
let scale_factor = window.scale_factor();
// Use scale_factor to size UI elements appropriately
let button_size = 48.0 * scale_factor; // 48dp button
```

### Orientation Changes

Window resize events are generated when device orientation changes:

```rust
// Portrait to landscape transition generates Resized event
window.set_bounds(WindowBounds::new(0, 0, 1920, 1080));
let event = window.poll_event();
// Receives WindowBackendEvent::Resized { width: 1920, height: 1080 }
```

### Immersive Mode (Fullscreen)

Android's immersive fullscreen mode is supported:

```rust
// Enable immersive mode (hide system bars)
window.set_state(WindowState::Fullscreen);
```

## UI Component Rendering

### Material Design Components

The renderer supports common Android UI patterns:

```rust
let factory = get_backend_factory();
let mut renderer = factory.create_renderer();
let mut context = renderer.create_context(1080, 1920);

context.begin_frame();

// App Bar (56dp height)
context.execute(RenderCommand::FillRect {
    rect: Rect::new(0.0, 0.0, 1080.0, 56.0),
    color: Color::rgb(0.2, 0.4, 0.8),
});

// Content Cards
context.execute(RenderCommand::FillRect {
    rect: Rect::new(16.0, 72.0, 1048.0, 100.0),
    color: Color::rgb(1.0, 1.0, 1.0),
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

Clipping regions support scrollable views:

```rust
// Clip content area (below app bar)
context.execute(RenderCommand::SetClip(Rect::new(0.0, 56.0, 1080.0, 1864.0)));

// Draw scrollable content (may extend beyond visible area)
context.execute(RenderCommand::FillRect {
    rect: Rect::new(0.0, 56.0, 1080.0, 3000.0),
    color: Color::rgb(0.9, 0.9, 0.9),
});

context.execute(RenderCommand::RestoreClip);
```

## Performance Considerations

### Software Rendering

Softbuffer is a software renderer, which means:

-	**Advantages**: 100% safe Rust, no GPU driver dependencies, works on all devices
-	**Considerations**: CPU-based rendering, suitable for UI components but not complex games

For most UI applications, software rendering provides acceptable performance. The renderer is optimized for:

-	Efficient pixel-level operations
-	Clipping to reduce overdraw
-	Minimal allocations during rendering

### Hardware Acceleration

The current implementation uses software rendering. For applications requiring hardware acceleration, future versions may support:

-	Vulkan backend (via wgpu)
-	OpenGL ES backend
-	Android Graphics API (NDK)

The architecture supports adding hardware-accelerated backends without changing the application code.

## Testing

### Platform-Specific Tests

Android-specific tests are provided in `engage-ux-tests/test_android_backend.rs`:

-	`test_android_backend_factory` - Verify correct backend creation
-	`test_android_canvas_rendering` - Test Canvas API rendering
-	`test_android_activity_window_management` - Test Activity integration
-	`test_android_lifecycle_support` - Test lifecycle events
-	`test_android_touch_event_handling` - Test touch events
-	`test_android_accessibility_talkback_readiness` - Test TalkBack support
-	`test_android_dpi_scaling` - Test DPI handling
-	`test_android_fullscreen` - Test immersive mode
-	`test_android_ui_rendering_pipeline` - Test Material Design rendering
-	`test_android_clipping_for_nested_views` - Test nested view clipping
-	`test_android_multi_context_rendering` - Test multiple view contexts

### Running Tests on Android

Tests can be run on Android devices or emulators:

```bash
# Build for Android
cargo build --target aarch64-linux-android --package engage-ux-oal

# Run tests on connected device
cargo test --target aarch64-linux-android --package engage-ux-tests test_android
```

Note: Android tests require Android NDK and appropriate Rust targets installed.

## Limitations and Future Work

### Current Limitations

1.	**Text Rendering**: Basic text rendering is implemented but full font support (custom fonts, advanced text layout) is not yet complete
2.	**Image Support**: Image rendering commands are defined but require integration with Android's Bitmap APIs
3.	**Hardware Acceleration**: Currently software-only, no GPU acceleration

### Future Enhancements

1.	**Native Canvas Integration**: Direct JNI bindings to Android Canvas API for better performance
2.	**Vulkan Backend**: Hardware-accelerated rendering via Vulkan
3.	**Advanced Text**: Full text rendering with system fonts, emoji support
4.	**Image Loading**: Integration with Android ImageDecoder
5.	**Custom Views**: Support for embedding native Android Views
6.	**Jetpack Compose Interop**: Integration with Compose UI

## Best Practices

### Window Management

1.	Always handle lifecycle events properly
2.	Save and restore state during pause/resume
3.	Release resources on destroy

### Rendering

1.	Use clipping to limit overdraw
2.	Batch render commands when possible
3.	Size UI elements using DPI scaling
4.	Follow Material Design guidelines

### Accessibility

1.	Provide meaningful window titles
2.	Ensure touch targets are at least 48dp
3.	Support high contrast themes
4.	Test with TalkBack enabled

## References

-	[Android Canvas API Documentation](https://developer.android.com/reference/android/graphics/Canvas)
-	[Android Activity Lifecycle](https://developer.android.com/guide/components/activities/activity-lifecycle)
-	[Android Accessibility](https://developer.android.com/guide/topics/ui/accessibility)
-	[Material Design Guidelines](https://material.io/design)
-	[winit Android Support](https://docs.rs/winit/latest/winit/platform/android/index.html)
-	[softbuffer Documentation](https://docs.rs/softbuffer/latest/softbuffer/)
