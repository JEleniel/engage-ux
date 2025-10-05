# Engage UX Documentation

> A fully cross-platform Rust UI toolkit that provides a themable component library without depending on a browser engine.

Named after Captain Jean-Luc Picard's famous "Engage!" command, this toolkit features a sleek LCARS-inspired theme by default. Engage UX uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing a single set of components to work across Windows, macOS, Linux, Android, and iOS.

## 🚀 Quick Links

- [Getting Started](getting-started.md) - Installation and your first app
- [Component Gallery](components/) - Browse all 50 components
- [Theme Guide](guides/theming.md) - Customize appearance
- [API Reference](api/) - Detailed API documentation
- [Examples](examples/) - Working code examples
- [Architecture](design/architecture/) - System design and architecture

## ✨ Key Features

- **Cross-Platform**: Windows, macOS, Linux, Android, and iOS
- **50+ Components**: Complete UI toolkit with all essential components
- **100% Rust**: Written entirely in Rust with `unsafe_code = "forbid"`
- **Fully Themable**: JSON-based themes with LCARS and classic designs
- **No Browser Engine**: Native performance without Chromium or WebView
- **Async by Default**: Built on Tokio for responsive UIs
- **WCAG AAA Ready**: Comprehensive accessibility support
- **Thread-Safe**: 100% thread-safe and non-blocking design

## 📦 Installation

Add Engage UX to your `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-components = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-themes = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-oal = { git = "https://github.com/JEleniel/engage-ux" }
```

## 📚 Documentation Sections

### For Users

- **[Getting Started Guide](getting-started.md)** - Install, configure, and create your first app
- **[Component Documentation](components/)** - Detailed docs for all 50 components
- **[Theming Guide](guides/theming.md)** - Customize colors, fonts, and styles
- **[Color Formats](color-formats.md)** - Using hex, RGB, HSL in themes
- **[Layout System](layout-system.md)** - Positioning and sizing components
- **[Examples](examples/)** - Working code examples and demos

### For Developers

- **[Component Development](guides/component-development.md)** - Create custom components
- **[Architecture Overview](design/architecture/)** - System design and patterns
- **[API Reference](api/)** - Complete API documentation
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute
- **[Testing Guide](guides/testing.md)** - Writing and running tests

### Reference

- **[All Components List](components/)** - Complete component catalog
- **[Theme Schema](../schemas/theme.schema.json)** - JSON schema for themes
- **[Changelog](../CHANGELOG.md)** - Version history and changes
- **[Roadmap](roadmap.md)** - Planned features

## 🎯 Component Categories

### Informational (11)
Labels, Text, Icons, Images, Lists, Progress, Tooltips, Breadcrumbs, Avatars, Line Numbers, Rulers

### Interactive (14)
Buttons, Checkboxes, Radio Buttons, Sliders, Text Inputs, Text Areas, Toggles, Links, Selects, Pagination, Carousels, Date Pickers, Text Editors, Consoles

### Layout (4)
Containers, Cards, Tables, Windows

### Notification (3)
Badges, Toasts, Banners

### Menus (4)
Drawers, Dropdowns, Hamburger Menus, Title Menus

### Dialogs (5)
Alerts, Confirm Dialogs, Custom Modals, Open Dialogs, Save-As Dialogs

### Window Controls (3)
Close, Maximize/Restore, Minimize/Restore

### Pane Groups (2)
Accordions, Tabs

### Graphic/Display (2)
Groups, Videos

[View Complete Component List →](components/)

## 🎨 Example

Create a button with a click handler:

```rust
use engage_ux_components::Button;
use engage_ux_core::component::Component;

#[tokio::main]
async fn main() {
    let mut button = Button::new(1, "Click Me!");
    
    button.set_on_click(|_| {
        println!("Button clicked!");
    });
    
    // Add to your UI...
}
```

[View More Examples →](examples/)

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](../CONTRIBUTING.md) for details on:

- Code of conduct
- Development setup
- Coding standards
- Pull request process
- Testing requirements

## 📄 License

Engage UX is dual-licensed under:

- Apache License 2.0 ([LICENSE-Apache](../LICENSE-Apache.md))
- MIT License ([LICENSE-MIT](../LICENSE-MIT.md))

Choose the license that best suits your needs.

## ❓ Help and Support

- **[FAQ](faq.md)** - Frequently asked questions
- **[Troubleshooting Guide](troubleshooting.md)** - Common problems and solutions
- **[Accessibility Guide](guides/accessibility.md)** - WCAG AAA compliance

## 🔗 Resources

- [GitHub Repository](https://github.com/JEleniel/engage-ux)
- [Issue Tracker](https://github.com/JEleniel/engage-ux/issues)
- [Discussions](https://github.com/JEleniel/engage-ux/discussions)
- [Release Notes](https://github.com/JEleniel/engage-ux/releases)

## 📞 Support

- **Bug Reports**: [Open an issue](https://github.com/JEleniel/engage-ux/issues/new?template=bug_report.md)
- **Feature Requests**: [Open an issue](https://github.com/JEleniel/engage-ux/issues/new?template=feature_request.md)
- **Questions**: [Start a discussion](https://github.com/JEleniel/engage-ux/discussions)
- **Troubleshooting**: [Read the troubleshooting guide](troubleshooting.md)

---

**Status**: ✅ All 50 components implemented | 🧪 321+ tests passing | 📦 Phase 2 Complete
