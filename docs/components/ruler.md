# Ruler Component

A measurement ruler component for design and layout tools.

## Overview

The Ruler component displays a measurement ruler with customizable units (pixels, cm, inches, points) and orientation.

## Basic Usage

```rust
use engage_ux_components::{Ruler, RulerUnit, RulerOrientation};

let mut ruler = Ruler::new(1);
ruler.set_unit(RulerUnit::Pixels);
ruler.set_orientation(RulerOrientation::Horizontal);
ruler.set_bounds(Rect::new(0.0, 0.0, 800.0, 30.0));
```

## Units

```rust
use engage_ux_components::RulerUnit;

// Pixels
ruler.set_unit(RulerUnit::Pixels);

// Centimeters
ruler.set_unit(RulerUnit::Centimeters);

// Inches
ruler.set_unit(RulerUnit::Inches);

// Points (typography)
ruler.set_unit(RulerUnit::Points);
```

## Orientation

```rust
use engage_ux_components::RulerOrientation;

// Horizontal ruler
ruler.set_orientation(RulerOrientation::Horizontal);

// Vertical ruler
ruler.set_orientation(RulerOrientation::Vertical);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `unit` | `RulerUnit` | Measurement unit | `Pixels` |
| `orientation` | `RulerOrientation` | Ruler direction | `Horizontal` |
| `zoom` | `f32` | Zoom level | `1.0` |
| `offset` | `f32` | Start offset | `0.0` |

## Methods

```rust
// Set unit
ruler.set_unit(RulerUnit::Pixels);

// Set orientation
ruler.set_orientation(RulerOrientation::Vertical);

// Set zoom
ruler.set_zoom(2.0); // 200% zoom

// Set offset
ruler.set_offset(100.0); // Start at 100
```

## Examples

### Design Tool Rulers

```rust
// Horizontal ruler at top
let mut h_ruler = Ruler::new(1);
h_ruler.set_orientation(RulerOrientation::Horizontal);
h_ruler.set_unit(RulerUnit::Pixels);
h_ruler.set_bounds(Rect::new(0.0, 0.0, 1000.0, 20.0));

// Vertical ruler at left
let mut v_ruler = Ruler::new(2);
v_ruler.set_orientation(RulerOrientation::Vertical);
v_ruler.set_unit(RulerUnit::Pixels);
v_ruler.set_bounds(Rect::new(0.0, 20.0, 20.0, 800.0));
```

### Zoomed Ruler

```rust
let mut ruler = Ruler::new(1);
ruler.set_zoom(canvas_zoom);

// Update when zoom changes
canvas.set_on_zoom(|zoom| {
    ruler.set_zoom(zoom);
});
```

## Related Components

- [Container](container.md) - For layout
- [Canvas](canvas.md) - For drawing

---

[← Back to Components](index.md)
