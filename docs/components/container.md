# Container Component

A generic container component for organizing and grouping other components.

## Overview

The Container component provides a flexible way to group and layout other components. It supports padding, margins, borders, and can be used to create complex layouts.

## Basic Usage

```rust
use engage_ux_components::Container;
use engage_ux_core::component::{Component, Rect};

// Create a container
let mut container = Container::new(1);
container.set_bounds(Rect::new(20.0, 20.0, 400.0, 300.0));

// Add children
container.add_child(Box::new(button));
container.add_child(Box::new(label));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `children` | `Vec<Box<dyn Component>>` | Child components | `[]` |
| `padding` | `f32` | Internal padding | `0.0` |
| `gap` | `f32` | Spacing between children | `0.0` |
| `background_color` | `Color` | Background color | Transparent |
| `border_width` | `f32` | Border width | `0.0` |
| `border_radius` | `f32` | Corner radius | `0.0` |

## Methods

```rust
// Add child
container.add_child(Box::new(component));

// Remove child
container.remove_child(child_id);

// Get children
let children = container.children();

// Clear all children
container.clear();

// Styling
container.set_padding(16.0);
container.set_gap(8.0);
container.set_background_color(Color::from_hex("#F5F5F5").unwrap());
container.set_border_width(1.0);
container.set_border_radius(8.0);
```

## Examples

### Form Container

```rust
let mut form = Container::new(1);
form.set_padding(20.0);
form.set_gap(12.0);
form.set_background_color(Color::from_hex("#FFFFFF").unwrap());
form.set_border_radius(8.0);

form.add_child(Box::new(username_input));
form.add_child(Box::new(password_input));
form.add_child(Box::new(submit_button));
```

### Card Layout

```rust
let mut card = Container::new(1);
card.set_padding(16.0);
card.set_background_color(Color::from_hex("#FFFFFF").unwrap());
card.set_border_width(1.0);
card.set_border_radius(4.0);
card.set_border_color(Color::from_hex("#E0E0E0").unwrap());
```

## Related Components

- [Card](card.md) - Specialized container
- [Group](group.md) - Component grouping
- [Window](window.md) - Application window

---

[← Back to Components](index.md)
