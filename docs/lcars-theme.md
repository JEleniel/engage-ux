# LCARS Theme - "Engage!"

The LCARS (Library Computer Access/Retrieval System) theme is the default theme for Engage UX, inspired by the iconic user interface from Star Trek: The Next Generation. Named after Captain Jean-Luc Picard's famous command "Engage," this theme embodies a sleek, futuristic aesthetic that honors the project's name and spirit.

## Overview

LCARS themes feature:
- **Vibrant Orange/Peach Colors**: Distinctive #FF9966 (LCARS Orange) and #FFCC99 (LCARS Peach)
- **Curved Borders**: Signature 20px radius for that iconic LCARS look
- **Bold Elements**: 3px border width for prominent UI elements
- **Rich Color Palette**: 11+ custom colors including violet, blue, skyblue, lilac, rose, and tan
- **Glowing Effects**: Subtle shadow effects for depth and the "glowing" LCARS appearance
- **Clean Typography**: Helvetica Neue for crisp, readable text
- **Compact Spacing**: Efficient use of space for information-dense interfaces

## Available Variants

### LCARS Light
The default light theme with a black background and vibrant accent colors.

```rust
use engage_ux_themes::Theme;

let theme = Theme::light(); // Returns LCARS Light by default
// or explicitly
let theme = Theme::lcars_light();
```

### LCARS Dark
A darker variant with slightly adjusted colors for better contrast.

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark(); // Returns LCARS Dark by default
// or explicitly
let theme = Theme::lcars_dark();
```

## Color Palette

### Primary Colors
- **Primary**: `#FF9966` - LCARS Orange (signature color)
- **Secondary**: Light - `#CC6699` (LCARS Violet), Dark - `#9999FF` (LCARS Blue)
- **Background**: `#000000` - Pure black
- **Surface**: Light - `#1A1A1A`, Dark - `#0D0D0D` (very dark gray)

### Semantic Colors
- **Error**: Light - `#CC6666`, Dark - `#FF6666` (soft red)
- **Warning**: `#FFCC99` (LCARS Peach)
- **Success**: `#99CCFF` (LCARS Skyblue)
- **Info**: Light - `#9999FF`, Dark - `#CC99CC` (LCARS Lilac)

### Text Colors
- **Primary Text**: `#FFCC99` (LCARS Peach) - high contrast on black
- **Secondary Text**: Light - `#CC99CC`, Dark - `#FF9966` (varying accents)
- **Disabled Text**: Light - `#666666`, Dark - `#555555` (muted gray)

### Custom Colors
Both themes include these custom colors for extended palette usage:

- `lcars_orange`: `#FF9966`
- `lcars_peach`: `#FFCC99`
- `lcars_violet`: `#CC6699`
- `lcars_blue`: `#9999FF`
- `lcars_skyblue`: `#99CCFF`
- `lcars_lilac`: `#CC99CC`
- `lcars_rose`: `#FF9999`
- `lcars_tan`: `#FFCC66`
- `panel_dark`: `#000000`
- `panel_medium`: Light - `#1A1A1A`, Dark - `#0D0D0D`
- `accent_glow`: Light - `#FF9966CC`, Dark - `#FF9966AA` (with alpha)

Dark theme also includes:
- `panel_light`: `#1A1A1A`
- `button_inactive`: `#333333`
- `separator`: `#FF996666`

## Typography

```rust
Typography {
    font_family: "Helvetica Neue, Arial, sans-serif",
    font_size_base: 16.0,
    font_size_small: 13.0,
    font_size_large: 22.0,
    line_height: 1.4,
}
```

## Spacing

Compact spacing for information density:

```rust
Spacing {
    unit: 8.0,
    small: 6.0,
    medium: 12.0,
    large: 20.0,
}
```

## Borders

Distinctive LCARS curved borders:

```rust
BorderStyle {
    width: 3.0,        // Bold borders
    radius: 20.0,      // Signature curves
    color: #FF9966,    // LCARS Orange
}
```

## Shadows

Glowing shadow effects:

```rust
// Light theme
ShadowStyle {
    enabled: true,
    blur_radius: 8.0,
    offset_x: 0.0,
    offset_y: 4.0,
    color: #FF996633,  // Translucent orange
}

// Dark theme
ShadowStyle {
    enabled: true,
    blur_radius: 10.0,
    offset_x: 0.0,
    offset_y: 4.0,
    color: #FF996644,  // Translucent orange
}
```

## Usage Examples

### Basic Usage

```rust
use engage_ux_themes::Theme;

// Get default LCARS Light theme
let theme = Theme::light();

// Use theme colors
let primary_color = theme.colors.primary;
let background = theme.colors.background;

// Access custom LCARS colors
if let Some(lcars_orange) = theme.colors.custom.get("lcars_orange") {
    // Use the signature LCARS orange color
}
```

### Switching Themes

```rust
use engage_ux_themes::Theme;

// Start with LCARS Light
let mut current_theme = Theme::lcars_light();

// Switch to LCARS Dark
current_theme = Theme::lcars_dark();

// Access classic themes if needed
let classic = Theme::classic_light();
```

### JSON Export

```rust
use engage_ux_themes::Theme;

let theme = Theme::light();
let json = theme.to_json().unwrap();

// Save to file or send over network
std::fs::write("themes/my-lcars-theme.json", json).unwrap();
```

## Classic Themes

For users who prefer the original design, classic themes are still available:

```rust
use engage_ux_themes::Theme;

let classic_light = Theme::classic_light();
let classic_dark = Theme::classic_dark();
```

Classic themes use the original Material Design-inspired color palette with standard border radii.

## Design Philosophy

The LCARS theme represents the core identity of Engage UX:

1. **Futuristic**: Drawing from science fiction's vision of advanced interfaces
2. **Bold**: Strong visual presence with vibrant colors and distinct shapes
3. **Functional**: Information-dense design suitable for complex applications
4. **Iconic**: Recognizable aesthetic that pays homage to Star Trek
5. **Professional**: Despite the sci-fi inspiration, suitable for serious applications

## Demo

Run the LCARS theme demo to see all features:

```bash
cargo run --example lcars_theme_demo -p engage-ux-components
```

This displays:
- Complete color palettes for both variants
- Typography specifications
- Spacing and border details
- Shadow effects
- All custom LCARS colors

## "Make it so."

The LCARS theme embodies the spirit of "Engage" - bold, forward-looking, and ready to embark on new missions. Whether you're building a starship control interface or a modern business application, LCARS provides a unique and memorable user experience.

**Engage!**
