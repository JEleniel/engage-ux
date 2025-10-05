# Tooltip Component

A contextual help component that displays additional information on hover or focus.

## Overview

The Tooltip component (also called Popover) displays additional information when users hover over or focus on an element. It's useful for providing help text, descriptions, or additional context.

## Basic Usage

```rust
use engage_ux_components::Tooltip;
use engage_ux_core::component::{Component, Rect};

// Create a tooltip
let mut tooltip = Tooltip::new(1, "This is a helpful tooltip");
tooltip.set_target_id(button_id);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `content` | `String` | Tooltip text | Required |
| `target_id` | `ComponentId` | Target component ID | Required |
| `position` | `TooltipPosition` | Display position | `Top` |
| `delay` | `u32` | Show delay (ms) | `500` |
| `visible` | `bool` | Current visibility | `false` |

## Positioning

```rust
use engage_ux_components::TooltipPosition;

// Above the target
tooltip.set_position(TooltipPosition::Top);

// Below the target
tooltip.set_position(TooltipPosition::Bottom);

// To the left
tooltip.set_position(TooltipPosition::Left);

// To the right
tooltip.set_position(TooltipPosition::Right);
```

## Methods

```rust
// Set content
tooltip.set_content("Updated tooltip text");

// Set target
tooltip.set_target_id(component_id);

// Set position
tooltip.set_position(TooltipPosition::Bottom);

// Set delay
tooltip.set_delay(300); // Show after 300ms

// Show/hide
tooltip.show();
tooltip.hide();
```

## Examples

### Button Tooltip

```rust
let mut button = Button::new(1, "Save");
let mut tooltip = Tooltip::new(2, "Save your changes");
tooltip.set_target_id(button.id());
tooltip.set_position(TooltipPosition::Top);
```

### Icon Tooltip

```rust
let mut icon = Icon::new(1, "info");
let mut tooltip = Tooltip::new(2, "Click for more information");
tooltip.set_target_id(icon.id());
tooltip.set_position(TooltipPosition::Right);
tooltip.set_delay(200);
```

### Multi-line Tooltip

```rust
let content = "Line 1\nLine 2\nLine 3";
let mut tooltip = Tooltip::new(1, content);
tooltip.set_max_width(200.0);
```

## Accessibility

Tooltips should be accessible:

```rust
tooltip.set_role("tooltip");
target.set_aria_describedby(tooltip.id());
```

## Related Components

- [Popover](popover.md) - For interactive content
- [Dialog](dialog.md) - For modal content

---

[← Back to Components](index.md)
