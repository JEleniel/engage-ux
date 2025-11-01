# API Reference

Complete API documentation for all Engage UX crates.

## Quick Links

- [Core API](#core-api) - Foundation types and traits
- [Components API](#components-api) - All UI components
- [Themes API](#themes-api) - Theme system
- [OAL API](#oal-api) - OS Abstraction Layer

## Core API

The `engage-ux-core` crate provides the foundation for the entire toolkit.

### Modules

#### Color

RGB and HSL color system with conversions.

```rust
use engage_ux_core::Color;
```

**Key Types**:

- `Color` - RGBA color representation
- Color parsing and conversion methods

[Full Color API Documentation →](core/color.md)

#### Component

Base component trait and properties. Note: small primitives (for example `Rect` and `ComponentId`) are centralized in `engage_ux_core::types` for consistency across the crate.

```rust
use engage_ux_core::component::Component;
use engage_ux_core::component_properties::ComponentProperties;
use engage_ux_core::types::{Rect, ComponentId};
// or, for convenience:
use engage_ux_core::prelude::*;
```

**Key Types**:

- `Component` trait - Base interface for all UI elements (defined in `component.rs`)
- `ComponentProperties` - Common component attributes (defined in `component_properties.rs`)
- `Rect` - Position and size (re-exported from `types`)
- `ComponentId` - Unique component identifier (re-exported from `types`)

[Full Component API Documentation →](core/component.md)

#### Events

Async event system using Tokio.

```rust
use engage_ux_core::events::{Event, EventType, EventHandler};
```

**Key Types**:

- `Event` - Event data structure
- `EventType` - Event type enumeration
- `EventHandler` - Async event handler
- `EventCallback` - Event callback function

[Full Events API Documentation →](core/events.md)

#### Input

Keyboard, mouse, touch, and custom input handling.

```rust
use engage_ux_core::input::{
    KeyboardEvent, KeyCode, KeyModifiers,
    MouseEvent, MouseButton,
    TouchEvent, TouchPhase,
    InputHandler
};
```

**Key Types**:

- `KeyboardEvent`, `KeyCode`, `KeyModifiers` - Keyboard input
- `MouseEvent`, `MouseButton` - Mouse input
- `TouchEvent`, `TouchPhase` - Touch input
- `InputHandler` trait - Component input handling

[Full Input API Documentation →](core/input.md)

#### Animation

Built-in animation system with easing functions. Note: zero-duration animations are treated as a no-op and are immediately marked `Completed` when started; `update()` will return `None` for already-completed animations.

```rust
use engage_ux_core::animation::{Animation, AnimationType, Easing, AnimationController};
```

**Key Types**:

- `Animation` - Animation definition; call `start()` to begin, `update(duration)` to advance; zero-duration animations are completed immediately.
- `AnimationType` - Fade, slide, scale, rotate, color
- `Easing` - Linear, ease-in, ease-out, etc.
- `AnimationController` - Animation playback control

[Full Animation API Documentation →](core/animation.md)

#### Accessibility

WCAG AAA accessibility infrastructure.

```rust
use engage_ux_core::accessibility::{
    AccessibilityProps, AriaRole, FocusManager, ScreenReader
};
```

**Key Types**:

- `AccessibilityProps` - Component accessibility properties
- `AriaRole` - ARIA role enumeration
- `FocusManager` - Keyboard focus management
- `ScreenReader` - Screen reader announcements

[Full Accessibility API Documentation →](core/accessibility.md)

#### Layout

Layout and positioning system.

```rust
use engage_ux_core::layout::{Layout, Unit, Size, PositionMode};
```

**Key Types**:

- `Layout` - Layout specification
- `Unit` - Pixels, rb, rp, percentage
- `Size` - Fixed, Fill, FitContent
- `PositionMode` - Relative, Absolute

[Full Layout API Documentation →](core/layout.md)

#### Drag and Drop

Comprehensive drag and drop system.

```rust
use engage_ux_core::drag_drop::{
    DragSource, DropTarget, DragData, DropEffect
};
```

**Key Types**:

- `DragSource` trait - Draggable components
- `DropTarget` trait - Drop zones
- `DragData` - Data being dragged
- `DropEffect` - Copy, move, link, none

[Full Drag & Drop API Documentation →](core/drag-drop.md)

## Components API

The `engage-ux-components` crate provides all 50 UI components.

### Component Categories

#### Informational

Display information to users.

- [Label](../components/label.md) - Text display
- [Text](../components/text.md) - Rich text
- [Icon](../components/icon.md) - Icons
- [Image](../components/image.md) - Images
- [Avatar](../components/avatar.md) - User avatars
- [List](../components/list.md) - Lists
- [Progress](../components/progress.md) - Progress indicators
- [Tooltip](../components/tooltip.md) - Tooltips and popovers
- [Breadcrumb](../components/breadcrumb.md) - Breadcrumb navigation
- [Line Numbers](../components/line-numbers.md) - Code line numbers
- [Ruler](../components/ruler.md) - Measurement rulers

#### Interactive

Allow user interaction.

- [Button](../components/button.md) - Buttons
- [Checkbox](../components/checkbox.md) - Checkboxes
- [Radio](../components/radio.md) - Radio buttons
- [Toggle](../components/toggle.md) - Toggle switches
- [Slider](../components/slider.md) - Sliders
- [Text Input](../components/text-input.md) - Text inputs
- [Text Area](../components/text-area.md) - Multi-line text
- [Select](../components/select.md) - Dropdown selection
- [Link](../components/link.md) - Navigation links
- [Pagination](../components/pagination.md) - Page navigation
- [Carousel](../components/carousel.md) - Carousels
- [Date Picker](../components/date-picker.md) - Date selection
- [Text Editor](../components/text-editor.md) - Rich text editing
- [Console](../components/console.md) - Terminal display

[View All Components →](../components/index.md)

## Themes API

The `engage-ux-themes` crate provides the theming system.

```rust
use engage_ux_themes::Theme;
```

**Key Types**:

- `Theme` - Theme definition
- `ColorPalette` - Theme colors
- `Typography` - Font settings
- `Spacing` - Spacing scale
- `Borders` - Border styles
- `Shadows` - Shadow effects

**Methods**:

```rust
// Built-in themes
let theme = Theme::dark();
let theme = Theme::light();

// Load from JSON
let theme = Theme::from_json(json_string)?;
let theme = Theme::from_json_file("theme.json")?;

// Export to JSON
let json = theme.to_json()?;
theme.save_to_file("theme.json")?;
```

[Full Themes API Documentation →](themes/index.md)

## OAL API

The `engage-ux-oal` crate provides OS abstraction.

```rust
use engage_ux_oal::{
    Platform,
    WindowBackend, WindowState,
    RenderBackend, RenderContext,
    Monitor, MonitorConfiguration
};
```

**Key Types**:

- `Platform` - Platform detection
- `WindowBackend` trait - Window management
- `RenderBackend` trait - Graphics rendering
- `Monitor` - Monitor information
- `MonitorConfiguration` - Multi-monitor setup

[Full OAL API Documentation →](oal/index.md)

## Generating API Documentation

Generate complete API documentation with Cargo:

```bash
# Generate and open documentation
cargo doc --all --no-deps --open

# Generate without opening
cargo doc --all --no-deps

# Include private items
cargo doc --all --no-deps --document-private-items
```

## API Stability

Engage UX follows semantic versioning:

- **Major version** (1.0.0) - Breaking changes
- **Minor version** (0.1.0) - New features, backward compatible
- **Patch version** (0.0.1) - Bug fixes, backward compatible

Current version: **0.2.0** (Phase 2 Complete)

## Deprecation Policy

Deprecated APIs will:

1. Be marked with `#[deprecated]` attribute
2. Include migration instructions
3. Remain available for at least one minor version
4. Be documented in release notes

## Platform Support Matrix

| Feature | Windows | macOS | Linux | Android | iOS |
|---------|---------|-------|-------|---------|-----|
| Core API | ✅ | ✅ | ✅ | ✅ | ✅ |
| Components | ✅ | ✅ | ✅ | ✅ | ✅ |
| Themes | ✅ | ✅ | ✅ | ✅ | ✅ |
| Window Backend | 🚧 | 🚧 | 🚧 | 🚧 | 🚧 |
| Render Backend | 🚧 | 🚧 | 🚧 | 🚧 | 🚧 |

✅ Fully supported | 🚧 Architecture ready, platform implementation pending

## Common Patterns

### Component Creation

```rust
use engage_ux_components::Button;
use engage_ux_core::component::Component;

let button = Button::new(1, "Click Me");
```

### Event Handling

```rust
button.set_on_click(|event| {
    println!("Clicked!");
});
```

### Theming

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
button.set_background_color(theme.colors.primary);
```

### Async Operations

```rust
#[tokio::main]
async fn main() {
    // All event handlers run in async context
}
```

## Type Aliases

Common type aliases used throughout the API:

```rust
// Component reference (thread-safe)
pub type ComponentRef<T> = Arc<RwLock<T>>;

// Component ID
pub type ComponentId = u64;

// Event callback
pub type EventCallback = Arc<dyn Fn(&Event) + Send + Sync>;

// Color channel (0-255)
pub type ColorChannel = u8;
```

## Error Handling

Most fallible operations return `Result<T, E>`:

```rust
use engage_ux_themes::Theme;

match Theme::from_json_file("theme.json") {
    Ok(theme) => println!("Loaded theme: {}", theme.name),
    Err(e) => eprintln!("Failed to load theme: {}", e),
}
```

## See Also

- [Getting Started Guide](../getting-started.md) - Installation and setup
- [Component Documentation](../components/index.md) - Component usage
- [Examples](../examples/) - Working code examples
- [Architecture](../design/architecture/) - System design

---

[← Back to Documentation](../index.md)
