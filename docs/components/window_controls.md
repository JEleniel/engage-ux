# Window Controls Component

Window control buttons for minimize, maximize/restore, and close operations.

## Overview

The Window Controls component provides standard window control buttons typically found in application title bars.

## Basic Usage

```rust
use engage_ux_components::WindowControls;

// Create window controls
let mut controls = WindowControls::new(1);
controls.set_bounds(Rect::new(700.0, 0.0, 100.0, 32.0));

// Configure which buttons to show
controls.set_show_minimize(true);
controls.set_show_maximize(true);
controls.set_show_close(true);
```

## Individual Buttons

### Close Button

```rust
use engage_ux_components::WindowControlButton;

let mut close_btn = WindowControlButton::close(1);
close_btn.set_on_click(|_| {
    window.close();
});
```

### Minimize Button

```rust
let mut min_btn = WindowControlButton::minimize(1);
min_btn.set_on_click(|_| {
    window.minimize();
});
```

### Maximize/Restore Button

```rust
let mut max_btn = WindowControlButton::maximize(1);
max_btn.set_on_click(|_| {
    if window.is_maximized() {
        window.restore();
    } else {
        window.maximize();
    }
});
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `show_minimize` | `bool` | Show minimize button | `true` |
| `show_maximize` | `bool` | Show maximize button | `true` |
| `show_close` | `bool` | Show close button | `true` |

## Examples

### Standard Controls

```rust
let mut controls = WindowControls::new(1);

controls.set_on_minimize(|_| {
    window.minimize();
});

controls.set_on_maximize(|_| {
    if window.is_maximized() {
        window.restore();
    } else {
        window.maximize();
    }
});

controls.set_on_close(|_| {
    if confirm_close() {
        window.close();
    }
});
```

### Themed Controls

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut controls = WindowControls::new(1);

controls.set_button_color(theme.colors.on_surface);
controls.set_hover_color(theme.colors.primary);
```

## Accessibility

Window controls should be keyboard accessible:

```rust
controls.set_aria_label("Window controls");
```

## Related Components

- [Window](window.md) - Window container
- [Button](button.md) - Generic buttons

---

[← Back to Components](index.md)
