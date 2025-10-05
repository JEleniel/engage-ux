# Toast Component

A temporary notification message that appears and automatically disappears.

## Overview

The Toast component displays brief, non-intrusive messages to users. Toasts automatically dismiss after a set duration and can include actions.

## Basic Usage

```rust
use engage_ux_components::{Toast, ToastType};
use engage_ux_core::component::{Component, Rect};

// Create a toast
let mut toast = Toast::new(1, "Operation completed successfully");
toast.set_type(ToastType::Success);
toast.set_duration(3000); // 3 seconds

// Show the toast
toast.show();
```

## Types

```rust
use engage_ux_components::ToastType;

// Success message (green)
toast.set_type(ToastType::Success);

// Error message (red)
toast.set_type(ToastType::Error);

// Warning message (yellow)
toast.set_type(ToastType::Warning);

// Info message (blue)
toast.set_type(ToastType::Info);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `message` | `String` | Toast message | Required |
| `type` | `ToastType` | Message type | `Info` |
| `duration` | `u32` | Auto-dismiss time (ms) | `3000` |
| `position` | `ToastPosition` | Screen position | `BottomRight` |
| `action` | `Option<String>` | Action button text | `None` |

## Methods

```rust
// Set message
toast.set_message("Changes saved");

// Set type
toast.set_type(ToastType::Success);

// Set duration
toast.set_duration(5000); // 5 seconds

// Set position
toast.set_position(ToastPosition::TopRight);

// Add action
toast.set_action("Undo");

// Show/hide
toast.show();
toast.hide();
```

## Examples

### Success Toast

```rust
fn show_success(message: &str) -> Toast {
    let mut toast = Toast::new(1, message);
    toast.set_type(ToastType::Success);
    toast.set_duration(3000);
    toast.show();
    toast
}

// Usage
show_success("File saved successfully");
```

### Error Toast

```rust
fn show_error(message: &str) -> Toast {
    let mut toast = Toast::new(1, message);
    toast.set_type(ToastType::Error);
    toast.set_duration(5000); // Show errors longer
    toast.show();
    toast
}
```

### Toast with Action

```rust
let mut toast = Toast::new(1, "Message deleted");
toast.set_type(ToastType::Info);
toast.set_action("Undo");

toast.set_on_action(|_| {
    // Handle undo action
    restore_message();
});
```

## Related Components

- [Banner](banner.md) - For persistent messages
- [Dialog](dialog.md) - For modal messages

---

[← Back to Components](index.md)
