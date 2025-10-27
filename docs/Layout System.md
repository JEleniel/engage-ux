# Layout System

The Engage UX layout system provides flexible positioning and sizing with support for relative units, constraints, and multi-monitor configurations.

## Relative Units

The layout system supports four types of units:

### 1. Pixels (px)
Absolute pixel values.

```rust
use engage_ux_core::layout::Unit;

let unit = Unit::pixels(100.0);
// Always resolves to 100 pixels
```

### 2. Relative to Base (rb)
Scales relative to the theme's base size (similar to `em` in CSS).

```rust
let unit = Unit::rb(2.0);
// With base_size = 16px, resolves to 32px
```

### 3. Relative to Parent (rp)
Scales relative to the inherited size (similar to `rem` in CSS).

```rust
let unit = Unit::rp(1.5);
// With inherited_size = 20px, resolves to 30px
```

### 4. Percentage (%)
Percentage of the parent dimension.

```rust
let unit = Unit::percent(50.0);
// With parent_width = 800px, resolves to 400px
```

## Parsing Units

Units can be parsed from strings:

```rust
use engage_ux_core::layout::Unit;

let px = Unit::parse("100px").unwrap();    // or "100"
let rb = Unit::parse("2rb").unwrap();
let rp = Unit::parse("1.5rp").unwrap();
let percent = Unit::parse("50%").unwrap();
```

## Layout System

The `Layout` struct provides complete layout specification:

```rust
use engage_ux_core::layout::{Layout, Size, Unit};

let layout = Layout::new()
    .with_left(Unit::pixels(10.0))
    .with_top(Unit::pixels(20.0))
    .with_width(Size::Fixed(Unit::pixels(200.0)))
    .with_height(Size::Fixed(Unit::pixels(100.0)));
```

### Size Modes

Three sizing modes are supported:

1. **Fixed**: Specific size in any unit
   ```rust
   Size::Fixed(Unit::pixels(200.0))
   Size::Fixed(Unit::percent(50.0))
   ```

2. **Fill**: Fill available space in parent
   ```rust
   Size::Fill
   ```

3. **FitContent**: Size to content (calculated later)
   ```rust
   Size::FitContent
   ```

### Position Modes

Two positioning modes are available:

1. **Relative**: Position relative to parent's content box
   ```rust
   layout.with_position_mode(PositionMode::Relative)
   ```

2. **Absolute**: Position absolute within parent
   ```rust
   layout.with_position_mode(PositionMode::Absolute)
   ```

### Constraints

Add minimum and maximum size constraints:

```rust
let layout = Layout::new()
    .with_width(Size::Fill)
    .with_height(Size::Fill)
    .with_min_width(Unit::pixels(200.0))
    .with_max_width(Unit::pixels(600.0))
    .with_min_height(Unit::pixels(150.0))
    .with_max_height(Unit::pixels(400.0));
```

### Edge-Based Sizing

Define size implicitly using edges:

```rust
let layout = Layout::new()
    .with_left(Unit::pixels(20.0))
    .with_right(Unit::pixels(20.0))
    .with_top(Unit::pixels(30.0))
    .with_bottom(Unit::pixels(30.0));
// Width = parent_width - left - right
// Height = parent_height - top - bottom
```

## Calculating Bounds

Convert layout specification to absolute pixels:

```rust
let bounds = layout.calculate_bounds(
    800.0,  // parent width
    600.0,  // parent height
    16.0,   // base size (for rb units)
    20.0,   // inherited size (for rp units)
);

println!("x={}, y={}, w={}, h={}", 
    bounds.x, bounds.y, bounds.width, bounds.height);
```

## Theme Integration

Layouts can be defined in theme files for specific components:

```json
{
  "name": "My Theme",
  "colors": { ... },
  "typography": { ... },
  "component_layouts": {
    "main-container": {
      "left": { "type": "rb", "value": 1.0 },
      "top": { "type": "rb", "value": 1.0 },
      "width": { "Fixed": { "type": "%", "value": 80.0 } },
      "height": { "Fill": null }
    },
    "sidebar": {
      "left": { "Pixels": 0.0 },
      "width": { "Fixed": { "Pixels": 250.0 } },
      "height": { "Fill": null }
    }
  }
}
```

## Multi-Monitor Support

The layout system integrates with multi-monitor configurations:

```rust
use engage_ux_oal::{Monitor, MonitorConfiguration, MonitorLayoutMode};

let mut config = MonitorConfiguration::new(MonitorLayoutMode::Unified);

// Add monitors
config.add_monitor(
    Monitor::new(1, "Primary".to_string(), (2560, 1440))
        .with_position(0, 0)
        .as_primary()
);

config.add_monitor(
    Monitor::new(2, "Secondary".to_string(), (1920, 1080))
        .with_position(2560, 0)
);

// Get virtual bounds for unified layout
let virtual_bounds = config.virtual_bounds().unwrap();

// Find monitor at point
let monitor = config.monitor_at_point(3000, 500);
```

### Monitor Layout Modes

Three layout modes are supported:

1. **Unified**: Treat all monitors as one virtual surface
   ```rust
   MonitorLayoutMode::Unified
   ```

2. **Separate**: Each monitor is a separate surface
   ```rust
   MonitorLayoutMode::Separate
   ```

3. **Mixed**: Custom groupings of monitors
   ```rust
   let mut config = MonitorConfiguration::new(MonitorLayoutMode::Mixed);
   config.add_monitor_group(vec![1, 2]);  // Group monitors 1 and 2
   // Monitor 3 remains separate
   ```

## Example

See the complete working example:

```bash
cargo run --example layout_demo -p engage-ux-components
```

This demonstrates:
- All unit types and conversions
- Layout calculations
- Constraints and fill modes
- Multi-monitor configuration
- Point-to-monitor mapping
