# Engage UX Documentation

This directory contains detailed documentation for Engage UX.

## User Documentation

### Color System

-	[Color Formats](color-formats.md) - Comprehensive guide to supported color formats (hex, RGB, HSL)

## Design Documentation

-	[Implementation Summary](design/IMPLEMENTATION_SUMMARY.md) - Current implementation status and statistics
-	[TODO](design/TODO.md) - Planned features and tasks for future phases

## Quick Links

-	[Main README](../README.md) - Project overview and getting started
-	[Contributing Guidelines](../CONTRIBUTING.md) - How to contribute to the project
-	[Code of Conduct](../CODE_OF_CONDUCT.md) - Community guidelines

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
