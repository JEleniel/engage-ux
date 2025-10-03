# Engage UX Documentation

This directory contains comprehensive documentation for the Engage UX toolkit.

## Getting Started

- [Main README](../README.md) - Project overview, features, and quick start guide
- [Contributing Guidelines](../CONTRIBUTING.md) - Development setup and contribution process
- [Code of Conduct](../CODE_OF_CONDUCT.md) - Community guidelines and standards

## User Guides

### Theme System

- [Color Formats](color-formats.md) - Complete guide to color format specification
	- Hex format (`#RRGGBB`, `#RRGGBBAA`)
	- RGB array format (`[r, g, b]`, `[r, g, b, a]`)
	- HSL array format (`[h, s, l]`, `[h, s, l, a]`)
	- Legacy format (backward compatible)
	- Format mixing and examples

## Developer Documentation

### Architecture & Design

- [Implementation Summary](design/IMPLEMENTATION_SUMMARY.md) - Current implementation status, statistics, and Phase 2 completion
- [TODO](design/TODO.md) - Planned features and tasks for Phase 3, Phase 4, and beyond

### Phase Completion Status

**Phase 1 - Complete ✅**
- All 50 components implemented
- 223 component tests passing
- Core systems (colors, events, themes)

**Phase 2 - Complete ✅**
- User-friendly color formats
- Complete input system (keyboard, mouse, touch)
- Accessibility infrastructure
- Rendering and window backends
- SVG, font, and image support
- 321 total tests passing

## Examples

The project includes several examples demonstrating various features:

### Themes

-	`color_formats` - Demonstrates the various color formats supported in theme files

Run examples with:

```bash
cargo run --example <example_name> -p <package_name>
```

For example:

```bash
cargo run --example color_formats -p engage-ux-themes
```

## API Documentation

Generate API documentation with:

```bash
cargo doc --all --no-deps --open
```

This will generate and open the full API documentation in your browser.
