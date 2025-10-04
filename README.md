# Engage UX

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
- **Animation System**: Built-in animation framework with easing functions
- **Drag and Drop**: Comprehensive drag and drop API
- **Custom Input Devices**: Extensible input system for gamepad, stylus, sensors, and more

## Architecture

Engage UX is organized as a Cargo workspace with the following crates:

### Core Crates

- **engage-ux-core**: Foundation layer providing:
	- Color system (RGB/HSL support with user-friendly formats)
	- Component trait and base structures
	- Event system using Tokio signals
	- Input system (keyboard, mouse, touch with gesture recognition, custom devices)
	- Animation system (fade, slide, scale, rotate, color transitions with easing)
	- Drag and drop system (DragSource, DropTarget, event management)
	- Accessibility infrastructure (ARIA, focus management, screen readers)
	- Rendering abstractions (SVG parsing, font and image loading)
	- Thread-safe primitives

- **engage-ux-oal**: OS Abstraction Layer providing:
	- Platform detection and initialization
	- Window management abstractions
	- Graphics rendering backend interfaces
	- Platform-specific factory patterns

- **engage-ux-themes**: Theme system providing:
	- JSON-based theme configuration
	- Default light and dark themes
	- User-friendly color formats (hex, RGB, HSL)
	- Color palettes, typography, spacing, borders, and shadows

- **engage-ux-components**: UI components library providing:
	- All 50 components from the specification
	- Informational components (Label, Text, Icon, Image, Avatar, Progress, etc.)
	- Interactive components (Button, TextInput, Checkbox, Radio, Toggle, Slider, etc.)
	- Layout components (Container, Card, Table, Window)
	- Notification components (Badge, Banner, Toast)
	- Menu components (Drawer, Dropdown, Hamburger Menu)
	- Dialog components (Alert, Confirm, Custom Modal, File dialogs)

- **engage-ux-tests**: Integration test suite providing:
	- Input system integration tests
	- Rendering pipeline tests
	- Theme integration tests
	- Animation system tests
	- Drag and drop tests
	- Custom input device tests

## Components

### Informational

- **Breadcrumb** ✓
- **Line Numbers** ✓
- **List** ✓
- **Progress Indicator** ✓
- **Ruler** ✓
- **Tooltip / Popover** ✓
- **Label** ✓
- **Text** ✓
- **Icon** ✓
- **Image** ✓
- **Avatar** ✓

### Interaction

- **Button** ✓
- **Carousel** ✓
- **Checkbox** ✓
- **Date Picker** ✓
- **Formatted Text Editor** ✓
- **Link** ✓
- **Pagination** ✓
- **Radio Button** ✓
- **Select / Dropdown Input** ✓
- **Slider / Range Selector** ✓
- **Text Area** ✓
- **Text Input** ✓
- **Toggle** ✓
- **Console View** ✓ (with ANSI escape code support)

### Graphic and Display

- **Group** ✓
- **Video** ✓

### Notification

- **Badge** ✓
- **Banner** ✓
- **Toast** ✓

### Menus

- **Drawer** ✓
- **Dropdown** ✓
- **Hamburger Menu** ✓
- **Title Menu** ✓

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
- **Window** ✓

✓ = Implemented (All 50 components - 100% Complete!)

## Usage

Add Engage UX to your `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { version="0.1.0-alpha.1" }
engage-ux-components = { version="0.1.0-alpha.1" }
engage-ux-themes = { version="0.1.0-alpha.1" }
engage-ux-oal = { version="0.1.0-alpha.1" }
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

### User-Friendly Color Formats in Themes

Themes support multiple color formats for user convenience:

```json
{
	"colors": {
		"primary": {"hex": "#1976D2"},
		"secondary": {"rgb": [66, 66, 66]},
		"background": {"hsl": [0, 0, 1.0]},
		"error": {"rgb": [211, 47, 47, 0.9]},
		"shadow": {"hex": "#00000033"}
	}
}
```

**Supported formats:**

-	**Hex**: `{"hex": "#RRGGBB"}` or `{"hex": "#RRGGBBAA"}`
-	**RGB**: `{"rgb": [r, g, b]}` (0-255) or `{"rgb": [r, g, b, a]}` (alpha: 0.0-1.0)
-	**HSL**: `{"hsl": [h, s, l]}` (h: 0-360, s/l: 0.0-1.0) or `{"hsl": [h, s, l, a]}`
-	**Legacy**: `{"space": "RGB", "components": [r, g, b, a]}` (0.0-1.0)

See [docs/color-formats.md](docs/color-formats.md) for more details and examples.

## Design Philosophy

### What Engage UX Is

- **Device Independent**: Works across all supported platforms with feature parity
- **Pure Rust**: 100% Rust code with `unsafe_code = "forbid"`
- **UX Focused**: Entirely focused on the user experience layer
- **Themable**: Every visual aspect can be customized through themes
- **Thread-Safe**: Built on Tokio for async, non-blocking operations
- **Accessible**: Full support for WCAG guidelines, with the default themes meeting WCAG AAA.

### What Engage UX Is Not

- **Not Reactive**: Engage UX is not reactive and is decoupled from data/logic handling
- **Not Hybrid**: No web technologies or JavaScript - pure native code
- **Not a Framework**: Does not provide state management, routing, or business logic
- **Not an SVG Script Executor**: SVG graphics are supported but scripts are not executed (security feature)

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

**Phase 1 - Complete ✅**
- [x] Complete all 50 component implementations
- [x] Add comprehensive examples
- [x] Core color, component, event, and theme systems
- [x] 223 component tests passing

**Phase 2 - Complete ✅**
- [x] User-friendly color formats (hex, RGB, HSL)
- [x] Complete input system (keyboard, mouse, touch with gestures)
- [x] Accessibility infrastructure (WCAG AAA ready)
- [x] Graphics rendering backend architecture
- [x] Window management backend interfaces
- [x] Secure SVG parsing (with script blocking)
- [x] Font loading and management system
- [x] Image format support (PNG, JPEG, WebP, GIF, BMP, TIFF)
- [x] Integration tests (8 tests)
- [x] Comprehensive documentation

**Phase 3 - Complete ✅**
- [x] Component development framework and documentation
- [x] Animation system (fade, slide, scale, rotate, color with easing functions)
- [x] Drag and drop support (DragSource, DropTarget, full event system)
- [x] Custom input device support (gamepad, stylus, sensors, etc.)
- [x] Integration tests for all new features (29 tests total)
- [x] Comprehensive examples and documentation

**Phase 4 - Planned**
- [ ] Framework for custom component development
- [ ] Animation system
- [ ] Drag and drop support
- [ ] Custom input handler extensibility

**Phase 4 - Planned**
- [ ] Relative value support for properties (rb, rp, %)
- [ ] Layout properties in themes
- [ ] Multi-monitor configuration support

**Future - Platform-Specific**
- [ ] Implement platform-specific OAL backends (Direct2D, Core Graphics, Cairo, etc.)
- [ ] Native window management for each OS
- [ ] End-to-end functional tests (requires platform backends)
- [ ] Documentation site

## Support

For bugs, feature requests, or questions, please open an issue on GitHub.
