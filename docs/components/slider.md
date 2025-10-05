# Slider Component

A component for selecting a numeric value from a range using a sliding control.

## Overview

The Slider component (also called range selector) allows users to select a numeric value by dragging a handle along a track. It's ideal for settings where users need to choose from a continuous range of values.

## Basic Usage

```rust
use engage_ux_components::Slider;
use engage_ux_core::component::{Component, Rect};

// Create a slider with min and max values
let mut slider = Slider::new(1, 0.0, 100.0);
slider.set_bounds(Rect::new(20.0, 20.0, 300.0, 40.0));

// Set initial value
slider.set_value(50.0);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `value` | `f32` | Current value | `min` |
| `min` | `f32` | Minimum value | Required |
| `max` | `f32` | Maximum value | Required |
| `step` | `f32` | Value increment step | `1.0` |

## Methods

### Value Management

```rust
// Get current value
let value = slider.value();

// Set value (clamped to min/max)
slider.set_value(75.0);

// Get min/max
let min = slider.min();
let max = slider.max();

// Set step size
slider.set_step(5.0); // Increment by 5
```

## Examples

### Volume Control

```rust
let mut volume = Slider::new(1, 0.0, 100.0);
volume.set_value(70.0);
volume.set_step(1.0);

// Later, get the volume level
let level = volume.value();
apply_volume(level);
```

### Opacity Slider

```rust
let mut opacity = Slider::new(1, 0.0, 1.0);
opacity.set_value(1.0);  // Fully opaque
opacity.set_step(0.1);   // Increment by 0.1
```

### Price Range

```rust
let mut min_price = Slider::new(1, 0.0, 1000.0);
let mut max_price = Slider::new(2, 0.0, 1000.0);

min_price.set_value(100.0);
max_price.set_value(500.0);
min_price.set_step(10.0);
max_price.set_step(10.0);

// Ensure min < max
fn update_price_range(min_slider: &mut Slider, max_slider: &mut Slider) {
    if min_slider.value() > max_slider.value() {
        max_slider.set_value(min_slider.value());
    }
}
```

### Themed Slider

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut slider = Slider::new(1, 0.0, 100.0);
slider.set_value(50.0);

// Apply theme colors (when available)
// slider.set_track_color(theme.colors.surface_variant);
// slider.set_thumb_color(theme.colors.primary);
```

## Accessibility

Sliders should be keyboard accessible:

- **Role**: `slider`
- **Keyboard**: Arrow keys to adjust value
- **Focus**: Can receive keyboard focus
- **Labels**: Should have accessible label

```rust
slider.set_aria_label("Volume");
slider.set_aria_valuemin(0.0);
slider.set_aria_valuemax(100.0);
slider.set_aria_valuenow(70.0);
```

## Best Practices

### Appropriate Ranges

Choose meaningful min/max values:

```rust
// Good: Reasonable range
let brightness = Slider::new(1, 0.0, 100.0);

// Avoid: Unnecessarily large range
let count = Slider::new(2, 0.0, 1000000.0); // Too many steps
```

### Step Size

Set step size based on precision needed:

```rust
// Integers
slider.set_step(1.0);

// Decimals
slider.set_step(0.1);

// Larger increments
slider.set_step(5.0);
```

### Value Display

Show the current value to users:

```rust
let label = format!("Volume: {:.0}%", slider.value());
```

### Touch-Friendly Sizing

Make sliders large enough for touch interaction:

```rust
// Minimum height for touch
slider.set_bounds(Rect::new(20.0, 20.0, 300.0, 44.0));
```

## Related Components

- [Text Input](text-input.md) - For precise numeric entry
- [Toggle](toggle.md) - For on/off values
- [Radio Button](radio.md) - For discrete choices
- [Progress](progress.md) - For showing progress

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Accessibility](../guides/accessibility.md) - WCAG compliance
- [Theming Guide](../guides/theming.md) - Customizing appearance

---

[← Back to Components](index.md)
