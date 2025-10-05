# Badge Component

A small visual indicator for displaying counts, status, or labels.

## Overview

The Badge component displays numerical counts or status indicators, commonly used for notifications, counts, or labels on other components.

## Basic Usage

```rust
use engage_ux_components::Badge;
use engage_ux_core::component::{Component, Rect};

// Create a badge with count
let mut badge = Badge::new(1, "5");
badge.set_bounds(Rect::new(50.0, 10.0, 24.0, 24.0));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `content` | `String` | Badge content (text/number) | Required |
| `variant` | `BadgeVariant` | Visual style | `Default` |
| `color` | `Color` | Badge color | Theme dependent |
| `text_color` | `Color` | Text color | `#FFFFFF` |
| `size` | `BadgeSize` | Badge size | `Medium` |

## Variants

```rust
use engage_ux_components::BadgeVariant;

// Default badge
badge.set_variant(BadgeVariant::Default);

// Success (green)
badge.set_variant(BadgeVariant::Success);

// Warning (yellow)
badge.set_variant(BadgeVariant::Warning);

// Error (red)
badge.set_variant(BadgeVariant::Error);

// Info (blue)
badge.set_variant(BadgeVariant::Info);
```

## Methods

```rust
// Set content
badge.set_content("10");
badge.set_content("New");

// Set variant
badge.set_variant(BadgeVariant::Success);

// Set colors
badge.set_color(Color::from_hex("#1976D2").unwrap());
badge.set_text_color(Color::from_hex("#FFFFFF").unwrap());

// Set size
badge.set_size(BadgeSize::Small);
```

## Examples

### Notification Badge

```rust
let mut notifications = Badge::new(1, "3");
notifications.set_variant(BadgeVariant::Error);
notifications.set_size(BadgeSize::Small);

// Position on top-right of icon
// badge.set_position(BadgePosition::TopRight);
```

### Status Badge

```rust
let mut status = Badge::new(1, "Active");
status.set_variant(BadgeVariant::Success);
status.set_content("Active");
```

### Count Badge

```rust
fn create_count_badge(id: u64, count: usize) -> Badge {
    let content = if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    };
    
    let mut badge = Badge::new(id, &content);
    badge.set_variant(BadgeVariant::Error);
    badge.set_size(BadgeSize::Small);
    badge
}
```

## Accessibility

Badges should have appropriate accessible labels:

```rust
badge.set_aria_label("3 unread notifications");
```

## Related Components

- [Toast](toast.md) - For messages
- [Banner](banner.md) - For announcements
- [Avatar](avatar.md) - Often combined with badges

---

[← Back to Components](index.md)
