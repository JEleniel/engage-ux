# Card Component

A content container with optional header, body, and footer sections.

## Overview

The Card component provides a styled container for grouping related content. It typically includes a header, body, and optional footer, with elevation and hover effects.

## Basic Usage

```rust
use engage_ux_components::Card;
use engage_ux_core::component::{Component, Rect};

// Create a card
let mut card = Card::new(1);
card.set_bounds(Rect::new(20.0, 20.0, 300.0, 400.0));

// Set content
card.set_title("Card Title");
card.set_body("Card body content goes here...");
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `title` | `String` | Card title/header | `""` |
| `body` | `String` | Card body content | `""` |
| `footer` | `String` | Card footer content | `""` |
| `elevation` | `f32` | Shadow depth (0-24) | `2.0` |
| `hoverable` | `bool` | Elevation on hover | `false` |
| `border_radius` | `f32` | Corner rounding | `8.0` |

## Methods

```rust
// Set content
card.set_title("Product Name");
card.set_body("Product description and details...");
card.set_footer("$29.99");

// Styling
card.set_elevation(4.0);
card.set_hoverable(true);
card.set_border_radius(8.0);
card.set_background_color(Color::from_hex("#FFFFFF").unwrap());
```

## Examples

### Product Card

```rust
let mut product = Card::new(1);
product.set_title("Wireless Headphones");
product.set_body("High-quality sound with noise cancellation");
product.set_footer("$149.99");
product.set_elevation(2.0);
product.set_hoverable(true);
```

### Info Card

```rust
let mut info = Card::new(1);
info.set_title("Important Notice");
info.set_body("Please update your payment information.");
info.set_background_color(Color::from_hex("#E3F2FD").unwrap());
info.set_border_radius(4.0);
```

### Clickable Card

```rust
let mut card = Card::new(1);
card.set_title("Learn More");
card.set_body("Click to explore our features");
card.set_hoverable(true);

card.set_on_click(|_| {
    navigate_to_features();
});
```

## Related Components

- [Container](container.md) - Generic container
- [Badge](badge.md) - For status indicators
- [Button](button.md) - For actions

---

[← Back to Components](index.md)
