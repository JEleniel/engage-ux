# Engage UX

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A fully cross-platform Rust UI toolkit that provides a themable component library without depending on a browser engine. Engage UX uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing a single set of components to work across Windows, macOS, Linux, Android, and iOS.

## Features

- **Cross-Platform Support**: Windows, MacOS, Linux, Android, and iOS
- **Feature Parity**: Consistent behavior and appearance across all platforms
- **Native Integration**: Uses native controls and styles whenever possible
- **100% Rust**: All source code is written in Rust with `unsafe_code = "forbid"`
- **JSON Configuration**: Theme and configuration files use JSON with full schemas
- **Rich Media Support**: Support for common font and image formats (including SVG)
- **Advanced Color System**: Full RGB and HSL color model support with conversions
- **Runtime Customization**: Components have minimal fixed properties for runtime modification
- **100% Themable**: Full theming support including transparency, colors, rounding, layout, etc.
- **No Browser Engine**: Does not use Chromium or any other browser engine
- **Async by Default**: Uses Tokio async runtime with signal-based event handling
- **Thread-Safe**: 100% thread-safe and non-blocking design

## Architecture

Engage UX is organized as a Cargo workspace with the following crates:

### Core Crates

- **engage-ux-core**: Foundation layer providing:
	- Color system (RGB/HSL support)
	- Component trait and base structures
	- Event system using Tokio signals
	- Thread-safe primitives

- **engage-ux-oal**: OS Abstraction Layer providing:
	- Platform detection and initialization
	- Window management abstractions
	- Platform-specific implementations

- **engage-ux-themes**: Theme system providing:
	- JSON-based theme configuration
	- Default light and dark themes
	- Color palettes, typography, spacing, borders, and shadows

- **engage-ux-components**: UI components including:
	- Informational (Label, Text, Icon, Image)
	- Interactive (Button, TextInput, Checkbox, Radio, Toggle, Slider)
	- Layout (Container, Card)

## Components

### Informational
- **Breadcrumb** ✓
- Line Numbers
- **List** ✓
- **Progress Indicator** ✓
- Ruler
- **Tooltip / Popover** ✓
- **Label** ✓
- **Text** ✓
- **Icon** ✓
- **Image** ✓
- **Avatar** ✓

### Interaction
- **Button** ✓
- Carousel
- **Checkbox** ✓
- Date Picker
- Formatted Text Editor
- **Link** ✓
- **Pagination** ✓
- **Radio Button** ✓
- **Select / Dropdown Input** ✓
- **Slider / Range Selector** ✓
- **Text Area** ✓
- **Text Input** ✓
- **Toggle** ✓
- Console View (with ANSI escape code support)

### Graphic and Display
- Group
- Video

### Notification
- **Badge** ✓
- **Banner** ✓
- **Toast** ✓

### Menus
- **Drawer** ✓
- **Dropdown** ✓
- **Hamburger Menu** ✓
- Title Menu

### Window Controls
- **Close** ✓
- **Maximize / Restore** ✓
- **Minimize / Restore** ✓

### Panes Groups
- **Accordion** ✓
- **Tabbed** ✓

### Dialogs
- **Alert** ✓
- **Confirm Dialog** ✓
- **Custom Modal** ✓
- **Open Dialog** ✓
- **Save As Dialog** ✓

### Grouping / Layout
- **Card** ✓
- **Container** ✓
- **Table** ✓
- Window

✓ = Implemented (30+ components)

## Usage

Add Engage UX to your `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { path = "path/to/engage-ux/engage-ux-core" }
engage-ux-components = { path = "path/to/engage-ux/engage-ux-components" }
engage-ux-themes = { path = "path/to/engage-ux/engage-ux-themes" }
engage-ux-oal = { path = "path/to/engage-ux/engage-ux-oal" }
```

### Example: Creating a Button

```rust
use engage_ux_components::Button;
use engage_ux_core::events::{Event, EventType};

let mut button = Button::new(1, "Click me!");
button.set_on_click(|event| {
    println!("Button clicked!");
});
```

### Example: Using Themes

```rust
use engage_ux_themes::Theme;

// Use default light theme
let light_theme = Theme::light();

// Use default dark theme
let dark_theme = Theme::dark();

// Load theme from JSON
let json = r#"{
    "name": "Custom Theme",
    "colors": { ... },
    ...
}"#;
let custom_theme = Theme::from_json(json).unwrap();
```

### Example: Working with Colors

```rust
use engage_ux_core::color::Color;

// Create RGB color
let red = Color::rgb(1.0, 0.0, 0.0, 1.0);

// Create HSL color
let blue = Color::hsl(240.0, 1.0, 0.5, 1.0);

// From hex
let green = Color::from_hex("#00FF00").unwrap();

// Convert between color spaces
let hsl_red = red.to_hsl();
let rgb_blue = blue.to_rgb();

// Adjust alpha
let transparent_red = red.with_alpha(0.5);
```

## Design Philosophy

### What Engage UX Is

- **Device Independent**: Works across all supported platforms with feature parity
- **Pure Rust**: 100% Rust code with `unsafe_code = "forbid"`
- **UX Focused**: Entirely focused on the user experience layer
- **Themable**: Every visual aspect can be customized through themes
- **Thread-Safe**: Built on Tokio for async, non-blocking operations

### What Engage UX Is Not

- **Not Reactive**: Engage UX is not reactive and is decoupled from data/logic handling
- **Not Hybrid**: No web technologies or JavaScript - pure native code
- **Not a Framework**: Does not provide state management, routing, or business logic
- **Not an SVG Renderer**: SVG graphics are supported but scripts are not executed (security feature)

## Development

### Building

```bash
cargo build --all
```

### Testing

```bash
cargo test --all
```

### Running Tests with Coverage

```bash
cargo test --all -- --test-threads=1
```

## Non-Functional Requirements

- **WCAG AAA Compliance**: Full accessibility support (in progress)
- **90% Unit Test Coverage**: Comprehensive test coverage (in progress)
- **No Unsafe Code**: `unsafe_code = "forbid"` in all crates ✓
- **Active Dependencies**: Only use crates updated within the past six months ✓
- **Cargo Features**: Platform-specific features are gated appropriately
- **Code Style**: Uses tabs for indentation, follows Rust style guide ✓

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass
2. Code follows the Rust style guide (tabs for indentation)
3. No `unsafe` code is introduced
4. Dependencies are actively maintained
5. Platform-specific code is properly gated

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Complete all component implementations
- [ ] Add comprehensive examples
- [ ] Implement platform-specific OAL backends
- [ ] Add WCAG AAA accessibility features
- [ ] Reach 90% test coverage
- [ ] Add interactive/functional tests
- [ ] Support for additional image and font formats
- [ ] SVG rendering engine (without script execution)
- [ ] Documentation site

## Support

For bugs, feature requests, or questions, please open an issue on GitHub.
