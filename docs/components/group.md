# Group Component

A component for grouping related elements together.

## Overview

The Group component provides a way to logically and visually group related components, sharing events and styling.

## Basic Usage

```rust
use engage_ux_components::Group;

let mut group = Group::new(1);
group.set_bounds(Rect::new(20.0, 20.0, 300.0, 200.0));

// Add components
group.add_child(Box::new(button1));
group.add_child(Box::new(button2));
group.add_child(Box::new(button3));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `children` | `Vec<Box<dyn Component>>` | Grouped components | `[]` |
| `layout` | `GroupLayout` | Layout direction | `Vertical` |
| `gap` | `f32` | Spacing between items | `8.0` |

## Methods

```rust
// Add children
group.add_child(Box::new(component));

// Set layout
group.set_layout(GroupLayout::Horizontal);
group.set_layout(GroupLayout::Vertical);

// Set gap
group.set_gap(12.0);
```

## Examples

### Button Group

```rust
let mut button_group = Group::new(1);
button_group.set_layout(GroupLayout::Horizontal);
button_group.set_gap(8.0);

button_group.add_child(Box::new(Button::new(2, "Yes")));
button_group.add_child(Box::new(Button::new(3, "No")));
button_group.add_child(Box::new(Button::new(4, "Cancel")));
```

### Form Field Group

```rust
let mut field_group = Group::new(1);
field_group.set_layout(GroupLayout::Vertical);
field_group.set_gap(16.0);

field_group.add_child(Box::new(name_field));
field_group.add_child(Box::new(email_field));
field_group.add_child(Box::new(phone_field));
```

## Related Components

- [Container](container.md) - Generic container
- [Card](card.md) - Content card

---

[← Back to Components](index.md)
