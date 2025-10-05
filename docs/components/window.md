# Window Component

An application window container with title bar and optional controls.

## Overview

The Window component provides a top-level container for applications, with a title bar, optional window controls (minimize, maximize, close), and content area.

## Basic Usage

```rust
use engage_ux_components::Window;
use engage_ux_core::component::{Component, Rect};

// Create a window
let mut window = Window::new(1);
window.set_title("My Application");
window.set_bounds(Rect::new(100.0, 100.0, 800.0, 600.0));

// Add content
window.set_content(Box::new(main_container));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `title` | `String` | Window title | `""` |
| `resizable` | `bool` | User can resize | `true` |
| `minimizable` | `bool` | Has minimize button | `true` |
| `maximizable` | `bool` | Has maximize button | `true` |
| `closeable` | `bool` | Has close button | `true` |
| `modal` | `bool` | Modal window | `false` |

## Methods

```rust
// Set title
window.set_title("Document Editor");

// Window features
window.set_resizable(true);
window.set_minimizable(true);
window.set_maximizable(true);
window.set_closeable(true);

// Window state
window.minimize();
window.maximize();
window.restore();
window.close();

// Modal
window.set_modal(true);

// Events
window.set_on_close(|_| {
    // Handle close
});

window.set_on_resize(|width, height| {
    // Handle resize
});
```

## Examples

### Main Application Window

```rust
let mut main_window = Window::new(1);
main_window.set_title("My App");
main_window.set_bounds(Rect::new(0.0, 0.0, 1024.0, 768.0));

// Add menu bar
let menu = create_menu_bar();
main_window.set_menu(menu);

// Add content
main_window.set_content(Box::new(app_container));
```

### Dialog Window

```rust
let mut dialog = Window::new(1);
dialog.set_title("Settings");
dialog.set_modal(true);
dialog.set_resizable(false);
dialog.set_minimizable(false);
dialog.set_maximizable(false);
dialog.set_bounds(Rect::new(200.0, 200.0, 400.0, 300.0));
```

## Accessibility

Windows should have proper titles:

```rust
window.set_aria_label("Main application window");
```

## Related Components

- [Window Controls](window_controls.md) - Window buttons
- [Title Menu](title_menu.md) - Menu bar
- [Dialog](dialog.md) - Modal dialogs

---

[← Back to Components](index.md)
