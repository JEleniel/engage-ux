# Icon Component

A component for displaying icons with rotation, flipping, and sizing options.

## Overview

The Icon component provides a flexible way to display icons in your interface. Icons can be rotated, flipped, resized, and colored to fit your design needs.

## Basic Usage

```rust
use engage_ux_components::Icon;
use engage_ux_core::component::{Component, Rect};

// Create an icon
let mut icon = Icon::new(1, "home");
icon.set_bounds(Rect::new(20.0, 20.0, 24.0, 24.0));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `name` | `String` | Icon name/identifier | Required |
| `size` | `f32` | Icon size in pixels | `24.0` |
| `color` | `Color` | Icon color | Theme dependent |
| `rotation` | `f32` | Rotation in degrees (0-360) | `0.0` |
| `flip_horizontal` | `bool` | Horizontal flip | `false` |
| `flip_vertical` | `bool` | Vertical flip | `false` |

## Methods

### Icon Name

```rust
// Get icon name
let name = icon.name();

// Set icon name
icon.set_name("settings");
icon.set_name("user");
```

### Sizing

```rust
// Get size
let size = icon.size();

// Set size
icon.set_size(32.0);  // Larger icon
icon.set_size(16.0);  // Smaller icon
```

### Coloring

```rust
use engage_ux_core::color::Color;

// Set icon color
icon.set_color(Color::from_hex("#1976D2").unwrap());

// Get color
let color = icon.color();
```

### Rotation

```rust
// Rotate icon
icon.set_rotation(45.0);   // 45 degrees clockwise
icon.set_rotation(90.0);   // 90 degrees clockwise
icon.set_rotation(180.0);  // Upside down
icon.set_rotation(270.0);  // 90 degrees counter-clockwise

// Get rotation
let degrees = icon.rotation();
```

### Flipping

```rust
// Flip horizontally (mirror)
icon.set_flip_horizontal(true);
if icon.is_flipped_horizontal() {
    println!("Icon is mirrored");
}

// Flip vertically
icon.set_flip_vertical(true);
if icon.is_flipped_vertical() {
    println!("Icon is upside down");
}

// Both flips
icon.set_flip_horizontal(true);
icon.set_flip_vertical(true);
```

## Examples

### Simple Icon

```rust
let mut home_icon = Icon::new(1, "home");
home_icon.set_bounds(Rect::new(10.0, 10.0, 24.0, 24.0));
home_icon.set_color(Color::from_hex("#333333").unwrap());
```

### Themed Icon

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut icon = Icon::new(1, "settings");
icon.set_color(theme.colors.primary);
icon.set_size(28.0);
```

### Rotated Icon

```rust
// Chevron pointing down
let mut chevron_down = Icon::new(1, "chevron-right");
chevron_down.set_rotation(90.0);

// Chevron pointing up
let mut chevron_up = Icon::new(2, "chevron-right");
chevron_up.set_rotation(270.0);

// Chevron pointing left
let mut chevron_left = Icon::new(3, "chevron-right");
chevron_left.set_rotation(180.0);
```

### Loading Spinner

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

async fn create_spinner(icon: Arc<RwLock<Icon>>) {
    let mut rotation = 0.0;
    
    loop {
        {
            let mut i = icon.write().await;
            i.set_rotation(rotation);
        }
        
        rotation = (rotation + 10.0) % 360.0;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

// Usage
let spinner = Arc::new(RwLock::new(Icon::new(1, "spinner")));
tokio::spawn(create_spinner(spinner.clone()));
```

### Icon Button

```rust
struct IconButton {
    icon: Icon,
    label: String,
}

impl IconButton {
    fn new(id: u64, icon_name: &str, label: &str) -> Self {
        let mut icon = Icon::new(id, icon_name);
        icon.set_size(20.0);
        
        Self {
            icon,
            label: label.to_string(),
        }
    }
}

// Usage
let save_btn = IconButton::new(1, "save", "Save");
let delete_btn = IconButton::new(2, "trash", "Delete");
```

### Icon Sizes

```rust
// Define standard icon sizes
pub enum IconSize {
    Small,   // 16px
    Medium,  // 24px
    Large,   // 32px
    XLarge,  // 48px
}

impl IconSize {
    fn pixels(&self) -> f32 {
        match self {
            IconSize::Small => 16.0,
            IconSize::Medium => 24.0,
            IconSize::Large => 32.0,
            IconSize::XLarge => 48.0,
        }
    }
}

fn create_icon(id: u64, name: &str, size: IconSize) -> Icon {
    let mut icon = Icon::new(id, name);
    icon.set_size(size.pixels());
    icon
}

// Usage
let small_icon = create_icon(1, "info", IconSize::Small);
let large_icon = create_icon(2, "warning", IconSize::Large);
```

### Status Icons

```rust
enum Status {
    Success,
    Warning,
    Error,
    Info,
}

fn create_status_icon(id: u64, status: Status) -> Icon {
    let (icon_name, color) = match status {
        Status::Success => ("check-circle", "#4CAF50"),
        Status::Warning => ("alert-triangle", "#FF9800"),
        Status::Error => ("x-circle", "#F44336"),
        Status::Info => ("info-circle", "#2196F3"),
    };
    
    let mut icon = Icon::new(id, icon_name);
    icon.set_color(Color::from_hex(color).unwrap());
    icon.set_size(20.0);
    icon
}

// Usage
let success = create_status_icon(1, Status::Success);
let error = create_status_icon(2, Status::Error);
```

### Navigation Icons

```rust
struct NavigationIcon {
    icon: Icon,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl NavigationIcon {
    fn new(id: u64, direction: Direction) -> Self {
        let mut icon = Icon::new(id, "arrow-right");
        
        // Rotate based on direction
        let rotation = match direction {
            Direction::Right => 0.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Up => 270.0,
        };
        
        icon.set_rotation(rotation);
        icon.set_size(24.0);
        
        Self { icon, direction }
    }
}

// Usage
let up_arrow = NavigationIcon::new(1, Direction::Up);
let down_arrow = NavigationIcon::new(2, Direction::Down);
```

### Animated Icon

```rust
struct AnimatedIcon {
    icon: Icon,
    start_rotation: f32,
    end_rotation: f32,
    duration_ms: u64,
}

impl AnimatedIcon {
    async fn animate(&mut self) {
        let steps = 60; // 60 FPS
        let step_duration = self.duration_ms / steps;
        let rotation_step = (self.end_rotation - self.start_rotation) / steps as f32;
        
        for i in 0..steps {
            let current_rotation = self.start_rotation + (rotation_step * i as f32);
            self.icon.set_rotation(current_rotation);
            tokio::time::sleep(Duration::from_millis(step_duration)).await;
        }
    }
}

// Usage: Rotate icon 360 degrees
let mut animated = AnimatedIcon {
    icon: Icon::new(1, "refresh"),
    start_rotation: 0.0,
    end_rotation: 360.0,
    duration_ms: 500,
};
animated.animate().await;
```

## Icon Naming Conventions

Use consistent, descriptive icon names:

```rust
// Good: Clear, descriptive names
Icon::new(1, "home");
Icon::new(2, "settings");
Icon::new(3, "user-profile");
Icon::new(4, "chevron-right");

// Avoid: Ambiguous names
Icon::new(5, "icon1");
Icon::new(6, "img");
```

## Accessibility

Icons should have proper accessibility support:

```rust
// For decorative icons
icon.set_aria_hidden(true);

// For functional icons
icon.set_aria_label("Settings");
icon.set_role("img");

// Icon with adjacent text
// No additional aria needed
```

### Accessible Icon Button

```rust
struct AccessibleIconButton {
    icon: Icon,
    label: String,
}

impl AccessibleIconButton {
    fn new(id: u64, icon_name: &str, label: &str) -> Self {
        let mut icon = Icon::new(id, icon_name);
        icon.set_aria_label(label);
        icon.set_role("button");
        
        Self {
            icon,
            label: label.to_string(),
        }
    }
}
```

## Best Practices

### Consistent Sizing

Use a limited set of icon sizes:

```rust
// Standard sizes
const ICON_SMALL: f32 = 16.0;
const ICON_MEDIUM: f32 = 24.0;
const ICON_LARGE: f32 = 32.0;

// Usage
icon.set_size(ICON_MEDIUM);
```

### Color Usage

Match icon colors to your theme:

```rust
use engage_ux_themes::Theme;

let theme = Theme::light();

// Primary icons
primary_icon.set_color(theme.colors.primary);

// Secondary icons
secondary_icon.set_color(theme.colors.text_secondary);

// Disabled icons
disabled_icon.set_color(theme.colors.disabled);
```

### Appropriate Context

Use icons that match user expectations:

```rust
// Good: Standard icons
let save = Icon::new(1, "save");      // Disk icon
let delete = Icon::new(2, "trash");   // Trash can icon
let search = Icon::new(3, "search");  // Magnifying glass

// Avoid: Unconventional icons
// Don't use a heart icon for delete
```

### Icon + Text

Always provide text alongside icons when possible:

```rust
struct IconTextPair {
    icon: Icon,
    label: Label,
}

// Better for accessibility and clarity
```

## Related Components

- [Button](button.md) - Buttons with icons
- [Image](image.md) - Raster images
- [Label](label.md) - Text labels

## See Also

- [Theming Guide](../guides/theming.md) - Customizing icon colors
- [Accessibility](../guides/accessibility.md) - Accessible icons
- [Animation Guide](../guides/animation.md) - Animating icons

---

[← Back to Components](index.md)
