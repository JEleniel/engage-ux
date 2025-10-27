# Theming Guide

Learn how to customize the appearance of your Engage UX applications using the powerful theming system.

## Table of Contents

- [Overview](#overview)
- [Built-in Themes](#built-in-themes)
- [Theme Structure](#theme-structure)
- [Using Themes](#using-themes)
- [Creating Custom Themes](#creating-custom-themes)
- [Color System](#color-system)
- [Typography](#typography)
- [Spacing and Layout](#spacing-and-layout)
- [Component Theming](#component-theming)
- [Dynamic Theme Switching](#dynamic-theme-switching)

## Overview

Engage UX uses a JSON-based theming system that allows you to fully customize the appearance of your application. Themes control:

- **Colors**: Primary, secondary, background, text, and semantic colors
- **Typography**: Font families, sizes, weights, and line heights
- **Spacing**: Consistent spacing scale for layout
- **Borders**: Border widths, radii, and styles
- **Shadows**: Elevation and depth effects

## Built-in Themes

Engage UX includes three built-in themes:

### Dark Theme (LCARS-inspired)

A futuristic theme inspired by Star Trek's LCARS interface.

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
```

**Features**:

- Deep space backgrounds
- Bright accent colors
- High contrast for readability
- Sci-fi aesthetic

### Light Theme (LCARS-inspired)

A lighter variant of the LCARS theme.

```rust
let theme = Theme::light();
```

**Features**:

- Light backgrounds
- Softer accent colors
- Clean and modern
- Better for bright environments

### Classic Themes

Traditional light and dark themes for conventional applications.

```rust
// Load from JSON
let classic_light = Theme::from_json_file("themes/classic-light.json")?;
let classic_dark = Theme::from_json_file("themes/classic-dark.json")?;
```

## Theme Structure

A theme is defined in JSON format:

```json
{
  "name": "My Custom Theme",
  "colors": {
    "primary": "#007AFF",
    "secondary": "#5856D6",
    "background": "#FFFFFF",
    "surface": "#F2F2F7",
    "error": "#FF3B30",
    "warning": "#FF9500",
    "success": "#34C759",
    "info": "#5AC8FA",
    "text_primary": "#000000",
    "text_secondary": "#3C3C43",
    "text_disabled": "#3C3C4399",
    "on_primary": "#FFFFFF",
    "on_secondary": "#FFFFFF",
    "on_background": "#000000",
    "on_surface": "#000000",
    "on_error": "#FFFFFF"
  },
  "typography": {
    "font_family": "Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
    "font_family_mono": "'SF Mono', 'Monaco', 'Consolas', monospace",
    "base_size": 16.0,
    "scale": 1.25,
    "line_height": 1.5,
    "sizes": {
      "xs": 12.0,
      "sm": 14.0,
      "base": 16.0,
      "lg": 20.0,
      "xl": 24.0,
      "2xl": 30.0,
      "3xl": 36.0,
      "4xl": 48.0
    }
  },
  "spacing": {
    "base": 8.0,
    "xs": 4.0,
    "sm": 8.0,
    "md": 16.0,
    "lg": 24.0,
    "xl": 32.0,
    "2xl": 48.0,
    "3xl": 64.0
  },
  "borders": {
    "width": 1.0,
    "radius": 8.0,
    "radius_sm": 4.0,
    "radius_lg": 12.0,
    "radius_full": 9999.0
  },
  "shadows": {
    "sm": "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
    "base": "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)",
    "md": "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
    "lg": "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
    "xl": "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)"
  }
}
```

## Using Themes

### Loading a Theme

```rust
use engage_ux_themes::Theme;

// Use built-in themes
let theme = Theme::dark();
let theme = Theme::light();

// Load from JSON string
let json = r#"{"name": "My Theme", ...}"#;
let theme = Theme::from_json(json)?;

// Load from file
let theme = Theme::from_json_file("path/to/theme.json")?;
```

### Applying Themes to Components

```rust
use engage_ux_components::Button;
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut button = Button::new(1, "Themed Button");

// Apply theme colors
button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
button.set_border_radius(theme.borders.radius);
```

### Accessing Theme Properties

```rust
// Colors
let primary = theme.colors.primary;
let background = theme.colors.background;
let text = theme.colors.text_primary;

// Typography
let font_family = &theme.typography.font_family;
let base_size = theme.typography.base_size;
let heading_size = theme.typography.sizes.get("2xl");

// Spacing
let small_spacing = theme.spacing.sm;
let medium_spacing = theme.spacing.md;

// Borders
let border_radius = theme.borders.radius;
let border_width = theme.borders.width;
```

## Creating Custom Themes

### Step 1: Define Your Color Palette

Start with your primary and secondary colors:

```json
{
  "name": "My Brand Theme",
  "colors": {
    "primary": "#6366F1",
    "secondary": "#8B5CF6",
    "background": "#FFFFFF",
    "surface": "#F9FAFB"
  }
}
```

### Step 2: Add Semantic Colors

Add colors for different states and contexts:

```json
{
  "colors": {
    "error": "#EF4444",
    "warning": "#F59E0B",
    "success": "#10B981",
    "info": "#3B82F6",
    "text_primary": "#111827",
    "text_secondary": "#6B7280"
  }
}
```

### Step 3: Define "On" Colors

Colors for text/icons on colored backgrounds:

```json
{
  "colors": {
    "on_primary": "#FFFFFF",
    "on_secondary": "#FFFFFF",
    "on_error": "#FFFFFF",
    "on_background": "#111827",
    "on_surface": "#111827"
  }
}
```

### Step 4: Configure Typography

```json
{
  "typography": {
    "font_family": "Your-Font, system-ui, sans-serif",
    "font_family_mono": "'Your-Mono', monospace",
    "base_size": 16.0,
    "scale": 1.2,
    "line_height": 1.6
  }
}
```

### Step 5: Set Spacing and Borders

```json
{
  "spacing": {
    "base": 8.0,
    "xs": 4.0,
    "sm": 8.0,
    "md": 16.0,
    "lg": 24.0,
    "xl": 32.0
  },
  "borders": {
    "width": 1.0,
    "radius": 6.0,
    "radius_sm": 3.0,
    "radius_lg": 12.0
  }
}
```

### Complete Custom Theme Example

See [example-theme.json](../assets/example-theme.json) for a complete example.

## Color System

Engage UX supports multiple color formats. See [Color Formats](../color-formats.md) for details.

### Color Formats

```json
{
  "colors": {
    "hex": "#FF5733",
    "hex_alpha": "#FF5733CC",
    "rgb": [255, 87, 51],
    "rgba": [255, 87, 51, 0.8],
    "hsl": [9, 100, 60],
    "hsla": [9, 100, 60, 0.8]
  }
}
```

### Using Colors in Rust

```rust
use engage_ux_core::color::Color;

// From theme
let primary = theme.colors.primary;

// Create programmatically
let red = Color::from_rgb(255, 0, 0);
let blue = Color::from_hex("#0000FF")?;
let green = Color::from_hsl(120.0, 1.0, 0.5);

// With alpha
let semi_transparent = Color::from_rgba(255, 0, 0, 128);

// Color transformations
let lighter = primary.lighten(0.1);
let darker = primary.darken(0.1);
let desaturated = primary.desaturate(0.2);
```

## Typography

### Font Families

```json
{
  "typography": {
    "font_family": "Inter, system-ui, sans-serif",
    "font_family_mono": "'Fira Code', 'Courier New', monospace"
  }
}
```

### Font Sizes

Use the type scale for consistent sizing:

```rust
// Access predefined sizes
let small_text = theme.typography.sizes.get("sm");
let heading = theme.typography.sizes.get("2xl");

// Or use base size with scale
let custom_size = theme.typography.base_size * theme.typography.scale.powi(2);
```

### Type Scale

The default scale is 1.25 (major third):

- xs: 12px
- sm: 14px
- base: 16px
- lg: 20px
- xl: 24px
- 2xl: 30px
- 3xl: 36px
- 4xl: 48px

## Spacing and Layout

### Spacing Scale

Use consistent spacing:

```rust
let button_padding = theme.spacing.md; // 16px
let section_margin = theme.spacing.xl; // 32px
let gap = theme.spacing.sm; // 8px
```

### Layout with Spacing

```rust
use engage_ux_core::layout::{Layout, Unit};

let layout = Layout::new()
    .with_left(Unit::pixels(theme.spacing.md))
    .with_top(Unit::pixels(theme.spacing.md));
```

## Component Theming

### Applying Themes Globally

```rust
// Create a theme context
let theme = Theme::dark();

// Apply to all components in a container
fn apply_theme_to_container(container: &mut Container, theme: &Theme) {
    for child in container.children_mut() {
        if let Some(button) = child.as_any_mut().downcast_mut::<Button>() {
            button.set_background_color(theme.colors.primary);
            button.set_text_color(theme.colors.on_primary);
        }
        // Handle other component types...
    }
}
```

### Component-Specific Theming

Some components may need specific theme customization:

```rust
// Button theming
button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
button.set_border_radius(theme.borders.radius);

// Input theming
input.set_background_color(theme.colors.surface);
input.set_text_color(theme.colors.text_primary);
input.set_border_color(theme.colors.primary);
input.set_border_width(theme.borders.width);

// Card theming
card.set_background_color(theme.colors.surface);
card.set_border_radius(theme.borders.radius_lg);
card.set_shadow(theme.shadows.md);
```

## Dynamic Theme Switching

### Switching Themes at Runtime

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

struct App {
    current_theme: Arc<RwLock<Theme>>,
    // ... other fields
}

impl App {
    async fn switch_theme(&self, new_theme: Theme) {
        let mut theme = self.current_theme.write().await;
        *theme = new_theme;
        
        // Notify components to re-render
        self.update_all_components().await;
    }
    
    async fn toggle_dark_mode(&self) {
        let current = self.current_theme.read().await;
        let new_theme = if current.name.contains("Dark") {
            Theme::light()
        } else {
            Theme::dark()
        };
        drop(current);
        
        self.switch_theme(new_theme).await;
    }
}
```

### Persisting Theme Selection

```rust
use std::fs;

// Save theme preference
fn save_theme_preference(theme_name: &str) -> std::io::Result<()> {
    fs::write("theme.txt", theme_name)
}

// Load theme preference
fn load_theme_preference() -> std::io::Result<String> {
    fs::read_to_string("theme.txt")
}

// Use with themes
fn load_preferred_theme() -> Theme {
    if let Ok(name) = load_theme_preference() {
        match name.as_str() {
            "dark" => Theme::dark(),
            "light" => Theme::light(),
            _ => Theme::dark(),
        }
    } else {
        Theme::dark()
    }
}
```

## Best Practices

### 1. Use Semantic Colors

Use semantic color names that describe their purpose:

```rust
// Good
let error_color = theme.colors.error;
let success_color = theme.colors.success;

// Avoid
let red = theme.colors.red;
let green = theme.colors.green;
```

### 2. Maintain Contrast

Ensure sufficient contrast for accessibility (WCAG AAA):

```rust
// Light text on dark background
button.set_background_color(theme.colors.primary);  // Dark
button.set_text_color(theme.colors.on_primary);     // Light

// Dark text on light background
card.set_background_color(theme.colors.surface);    // Light
card.set_text_color(theme.colors.on_surface);       // Dark
```

### 3. Use the Spacing Scale

Stick to the spacing scale for consistency:

```rust
// Good
margin: theme.spacing.md

// Avoid
margin: 15.0  // Not in scale
```

### 4. Test Both Light and Dark

Always test your app in both light and dark themes:

```rust
#[test]
fn test_component_in_light_theme() {
    let theme = Theme::light();
    // Test component...
}

#[test]
fn test_component_in_dark_theme() {
    let theme = Theme::dark();
    // Test component...
}
```

## Theme Validation

Validate your theme against the schema:

```bash
# Using a JSON schema validator
jsonschema -i your-theme.json schemas/theme.schema.json
```

## Examples

- [Theme Demo Example](../examples/theme-demo.md)
- [LCARS Theme Demo](../examples/lcars-theme-demo.md)
- [Dynamic Theme Switching](../examples/theme-switching.md)

## See Also

- [Color Formats Guide](../color-formats.md)
- [Component Styling](styling.md)
- [Accessibility Guide](accessibility.md)
- [Theme Schema Reference](../../schemas/theme.schema.json)

---

[← Back to Guides](index.md) | [Next: Accessibility →](accessibility.md)
