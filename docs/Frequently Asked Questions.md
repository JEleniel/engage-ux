# Frequently Asked Questions (FAQ)

Common questions about Engage UX.

## General Questions

### What is Engage UX?

Engage UX is a fully cross-platform Rust UI toolkit that provides a themable component library without depending on a browser engine. It's designed to create native-feeling applications that work consistently across Windows, macOS, Linux, Android, and iOS.

### Why "Engage"?

Named after Captain Jean-Luc Picard's famous "Engage!" command from Star Trek, reflecting the toolkit's flagship LCARS-inspired theme and its mission to empower developers to confidently build cross-platform UIs.

### Is Engage UX production-ready?

Engage UX is currently in Phase 2 (v0.2.0) with all core components and systems implemented. Platform-specific rendering backends are planned for Phase 3. It's suitable for development and prototyping, with production readiness targeting v1.0.0 in Q3 2025.

### What makes Engage UX different?

- **No browser engine**: Unlike Electron or Tauri, doesn't bundle Chromium
- **True cross-platform**: Single codebase for desktop and mobile
- **100% Rust**: Type-safe, memory-safe, no `unsafe` code
- **Fully themable**: JSON-based themes with complete customization
- **Async-first**: Built on Tokio for responsive UIs
- **Accessibility-ready**: WCAG AAA infrastructure built-in

## Installation and Setup

### What are the system requirements?

- **Rust**: 1.70 or later
- **Operating System**:
  - Windows 10 or later
  - macOS 10.15 (Catalina) or later
  - Linux with X11/Wayland
  - Android 8.0 (API 26) or later
  - iOS 13 or later

### How do I install Engage UX?

Add to your `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-components = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-themes = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-oal = { git = "https://github.com/JEleniel/engage-ux" }
```

See the [Getting Started Guide](getting-started.md) for details.

### Can I use Engage UX with existing projects?

Yes! Engage UX is designed to integrate with existing Rust projects. You can use individual crates as needed.

### Is Engage UX available on crates.io?

Not yet. Initial releases are available via Git while we work toward v1.0.0 and crates.io publication.

## Components

### How many components are available?

Engage UX includes all 50 components from the specification:
- 11 Informational components
- 14 Interactive components
- 4 Layout components
- 3 Notification components
- 4 Menu components
- 5 Dialog components
- 3 Window controls
- 2 Pane groups
- 2 Graphic/Display components

See the [Component Reference](components/index.md) for the complete list.

### Can I create custom components?

Yes! See the [Component Development Guide](guides/component-development.md) for instructions on creating custom components.

### Are components accessible?

Yes, all components are designed with accessibility in mind and include WCAG AAA infrastructure. See the [Accessibility Guide](guides/accessibility.md).

### Can I style components?

Absolutely! Use the theme system to customize colors, typography, spacing, and more. See the [Theming Guide](guides/theming.md).

## Themes

### What themes are included?

Engage UX includes:
- **Dark LCARS Theme** - Futuristic sci-fi aesthetic
- **Light LCARS Theme** - Light variant of LCARS
- **Classic Themes** - Traditional light and dark themes

### Can I create custom themes?

Yes! Themes are defined in JSON files. See the [Theming Guide](guides/theming.md) for details.

### Can I switch themes at runtime?

Yes, themes can be changed dynamically. See the [Dynamic Theme Switching](guides/theming.md#dynamic-theme-switching) section.

### What color formats are supported?

Themes support multiple formats:
- Hex: `#FF5733`, `#FF5733CC`
- RGB: `[255, 87, 51]`, `[255, 87, 51, 0.8]`
- HSL: `[9, 100, 60]`, `[9, 100, 60, 0.8]`

See [Color Formats](color-formats.md) for details.

## Platform Support

### Which platforms are supported?

- ✅ Windows 10+ (x86_64, ARM64)
- ✅ macOS 10.15+ (x86_64, ARM64)
- ✅ Linux (x86_64, ARM64)
- 🚧 Android 8+ (ARM64) - Phase 3
- 🚧 iOS 13+ (ARM64) - Phase 3

✅ = Architecture ready | 🚧 = Platform backends in progress

### Does it work on Raspberry Pi?

Yes! Linux ARM64 builds work on Raspberry Pi 3 and later.

### Can I target WASM/web?

Web Assembly support is planned for Phase 5 (2026).

### Does it work with React Native or Electron?

Direct integration isn't available yet, but bridges are planned for Phase 5.

## Performance

### How fast is Engage UX?

Engage UX is built in Rust with performance as a core goal:
- Zero-cost abstractions
- No garbage collection
- Async-first for responsiveness
- GPU acceleration planned (Phase 4)

### What's the memory footprint?

Memory usage depends on your application, but Engage UX itself is lightweight:
- Minimal runtime overhead
- No embedded browser engine
- Efficient component pooling (planned)

### Can it handle large datasets?

Yes, with features like:
- Virtual scrolling (planned Phase 4)
- Lazy loading
- Efficient rendering
- Incremental updates

## Development

### How do I debug my application?

Use standard Rust debugging tools:
- `println!` / `dbg!` macros
- `cargo test` for unit tests
- Rust analyzer in VS Code
- `rust-lldb` or `rust-gdb`

See the [Testing Guide](guides/testing.md).

### Are there examples?

Yes! See the [Examples](examples/index.md) section. Run with:

```bash
cargo run --example basic_components -p engage-ux-components
```

### Where's the API documentation?

Generate locally with:

```bash
cargo doc --all --no-deps --open
```

Or see the [API Reference](api/index.md).

### How do I report bugs?

[Open an issue](https://github.com/JEleniel/engage-ux/issues/new?template=bug_report.md) on GitHub with:
- Description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Rust version and platform

## Features

### Does it support animations?

Yes! Engage UX includes a comprehensive animation system with:
- Fade, slide, scale, rotate animations
- Color transitions
- Easing functions
- Animation sequencing

See [examples/animation_demo](examples/index.md).

### Can I use drag and drop?

Yes! Full drag and drop support is included with:
- Draggable sources
- Drop targets
- Multiple drop effects
- Custom drag data

See [examples/drag_drop_demo](examples/index.md).

### Does it support touch input?

Yes! Multi-touch support with:
- Touch events (down, move, up, cancel)
- Gesture recognition (pinch, pan, swipe)
- Hover simulation
- Pressure sensitivity

### What about keyboard navigation?

Comprehensive keyboard support:
- Tab navigation
- Arrow key navigation
- Keyboard shortcuts
- Focus management
- Screen reader support

### Does it work with gamepads?

Yes! Custom input device support includes gamepads, styluses, sensors, and more.

## Licensing

### What's the license?

Engage UX is dual-licensed under:
- Apache License 2.0
- MIT License

Choose whichever license suits your needs.

### Can I use it in commercial projects?

Yes! Both licenses permit commercial use.

### Do I need to attribute Engage UX?

Check the specific license you choose. Both Apache 2.0 and MIT require attribution.

### Can I modify the source?

Yes! Both licenses permit modification.

## Contributing

### How can I contribute?

See the [Contributing Guide](../CONTRIBUTING.md) for:
- Code contributions
- Bug reports
- Feature requests
- Documentation improvements
- Examples and tutorials

### What are the coding standards?

- Follow Rust style guide
- Use `rustfmt` and `clippy`
- Write tests for new features
- Document public APIs
- Tab indentation (enforced by `rustfmt.toml`)

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

### Is there a roadmap?

Yes! See the [Roadmap](roadmap.md) for planned features and timeline.

### How do I get help?

- [GitHub Discussions](https://github.com/JEleniel/engage-ux/discussions) - Ask questions
- [GitHub Issues](https://github.com/JEleniel/engage-ux/issues) - Report bugs
- Documentation - Browse the docs
- Examples - Learn from examples

## Troubleshooting

### Build errors with linker

**Solution**: Ensure platform-specific build tools are installed:

**Windows**: Visual Studio Build Tools
**macOS**: Xcode Command Line Tools
**Linux**: GCC/Clang and X11 development headers

### Components not rendering

**Explanation**: Phase 2 implements component logic and architecture. Full rendering requires platform-specific backends (Phase 3).

### Theme fails to load

**Solution**: Validate JSON against the [theme schema](../schemas/theme.schema.json).

### Tests failing

**Solution**:
1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Run specific test: `cargo test <test_name>`

### Import errors

**Solution**: Ensure all crates are included in `Cargo.toml`:

```toml
engage-ux-core = { git = "..." }
engage-ux-components = { git = "..." }
engage-ux-themes = { git = "..." }
engage-ux-oal = { git = "..." }
```

## Future Plans

### When will v1.0.0 be released?

Target: Q3 2025 (see [Roadmap](roadmap.md))

### What's next after Phase 2?

Phase 3 focuses on platform-specific backends:
- Windows, macOS, Linux rendering
- Native window management
- Mobile platform support

### Will there be a GUI designer?

Visual component editor is planned for Phase 4.

### Can I sponsor development?

Sponsorship options coming soon. Follow the project for updates.

## Still Have Questions?

- [Start a discussion](https://github.com/JEleniel/engage-ux/discussions)
- [Read the documentation](index.md)
- [Check the examples](examples/index.md)
- [Browse the API docs](api/index.md)

---

[← Back to Documentation](index.md)
