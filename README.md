# Engage UX

A fully cross-platform Rust UI toolkit that provides a themable component library without depending on a browser engine. Named after Captain Jean-Luc Picard's famous "Engage!" command from "Star Trek: The Next Generation" by Gene Rodenberry, this toolkit features a sleek futuristic theme by default. Engage UX uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing a single set of components to work across Windows, macOS, Linux (Wayland and X11), Android, and iOS.

## Features

- **Cross-Platform Support**: Windows, MacOS, Linux, Android, and iOS (no web, this is meant for native apps)
- **Feature Parity**: Consistent behavior and appearance across all platforms
- **Native Integration**: Platform-specific renderers and input handling via an OS Abstraction Layer (OAL)
- **100% Rust**: All source code is written in Rust with `unsafe_code = "forbid"`
- **JSON Configuration**: Theme and configuration files use JSON with full schemas
- **Rich Media Support**: Support for common font and image formats (including SVG)
- **Advanced Color System**: Full RGB and HSL color model support with conversions
- **Runtime Customization**: Components have minimal fixed properties for runtime modification
- **100% Themable**: Full theming support including transparency, colors, rounding, layout, etc.
- **No Browser Engine**: Does not use Chromium or any other browser engine
- **Async by Default**: Provides functions to start the logic loop in an async context and runs the windowing/input/rendering in the main thread, as required by most platforms.
- **Thread-Safe**: 100% thread-safe and non-blocking design, including a built in cross thread, thread safe event bus.
- **Accessibility Support**: Built-in support for screen readers, keyboard navigation, and ARIA roles (WCAG AAA compliance in progress)
- **Animation System**: Built-in animation framework with easing functions
- **Drag and Drop**: Comprehensive drag and drop API
- **Custom Input Devices**: Supports keyboard, mouse, and general touch (+gesture) input out of the box. Extensible input system for gamepad, stylus, sensors, and more

## Architecture

Engage UX is organized into multiple crates to separate concerns and allow for modular usage:

### Core Crates

- **engage-ux-core**: Foundation layer providing:
   	+ Geometric primitives (Point, Rectangle, etc.)
   	+ Color system (RGB/HSL support with user-friendly formats)
   	+ Component trait and base structures
   	+ Cross thread, thread safe EventBus
   	+ Input system (keyboard, mouse, touch with gesture recognition, custom devices)
   	+ Animation system (fade, slide, scale, rotate, color transitions with easing)
   	+ Drag and drop system (DragSource, DropTarget, event management)
   	+ Accessibility infrastructure (ARIA, focus management, screen readers)
   	+ Rendering abstractions (SVG parsing, font and image loading)
   	+ Thread-safe primitives

- **engage-ux-oal**: OS Abstraction Layer providing:
   	+ Platform detection and initialization
   	+ Window management abstractions
   	+ Graphics rendering backend interfaces
   	+ Platform-specific factory patterns
   	+ All OS specific code is isolated to this crate, making it easy to maintain and extend.

- **engage-ux**: The main entry point crate that ties everything together and provides the API for application developers.
   	+ Re-exports necessary items from core.
   	+ Handles all coordination between the UX and OAL layers.
   	+ Includes a default theme and basic application scaffolding.
   	+ Includes common controls, layouts, and utilities for building applications, including:
       	* Informational components (Label, Text, Icon, Image, Progress Indicator, etc.)
       	* Interactive components (Button, Input, Dropdown, Checkbox, Radio, Toggle, Slider, etc.)
       	* Layout components (Container, Card, Table, Window)
       	* Notification components (Badge, Banner, Toast)
       	* Menu components (Drawer, Dropdown, Hamburger Menu)
       	* Dialog components (Alert, Confirm, Custom Modal, File dialogs)
       	* System components (Tray Icon, System Notifications, Clipboard access)

- **engage-ux-tests**: Integration test suite providing:
   	+ Input system integration tests
   	+ Rendering pipeline tests
   	+ Theme integration tests
   	+ Animation system tests
   	+ Drag and drop tests
   	+ Custom input device tests

### Additional Crates

- **engage-ux-themes**: Additional themes for Engage UX.
   	+ Each theme includes light and dark variants.
   	+ Tons of free and freely distributable icons in mutliple styles (outlined, filled, rounded, two-tone).
   	+ Pre-built themes include:
      	*A "Material 3" styled theme based on Google's Material You design system.
       	* A "Cupertino" styled theme based on Apple's iOS design language.
       	* A "Fluent" styled theme based on Microsoft's Fluent Design System.

- **engage-ux-components**: UI components library providing:
   	+ 50+ additional, reusable, fully themable UI components, including:
       	* Accordion
       	* Avatar
       	* Banner
       	* Bento Menu
       	* Breadcrumb
       	* Carousel
       	* Console View (with ANSI escape code support)
       	* Date Picker
       	* Formatted Text Editor
       	* Group
       	* Pagination
       	* Ruler
       	* Tabbed Pane
       	* Video Player
       	* And many more!

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
- **Not an SVG Script Executor**: SVG graphics are supported but scripts are stripped before rendering

## Non-Functional Requirements

- **WCAG AAA Compliance**: Full accessibility support (in progress)
- **No Unsafe Code**: `unsafe_code = "forbid"` in all crates ✓

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass
2. Code follows the Rust style guide (tabs for indentation)
3. No `unsafe` code is introduced
4. Dependencies are actively maintained
5. Platform-specific code is properly gated

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Support

For bugs, feature requests, or questions, please open an issue on GitHub.
