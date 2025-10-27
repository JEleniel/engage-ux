# Engage UX Documentation

> **Note**: This documentation is now organized for GitHub Pages. Visit the [main documentation site](index.md) for the complete, navigable documentation.

This directory contains comprehensive documentation for the Engage UX toolkit, organized into the following sections:

## 📚 Documentation Structure

### Getting Started

- **[Documentation Home](index.md)** - Main documentation hub
- **[Getting Started Guide](getting-started.md)** - Installation, setup, and first app
- **[Roadmap](roadmap.md)** - Future plans and version timeline

### Component Documentation

- **[Component Reference](components/index.md)** - All 50 components
- **[Button Component](components/button.md)** - Example component documentation

### Guides

- **[Theming Guide](guides/theming.md)** - Customizing appearance
- **[Component Development](guides/component-development.md)** - Creating custom components
- **[Color Formats](color-formats.md)** - Color format specification
- **[Layout System](layout-system.md)** - Layout and positioning
- **[LCARS Theme](lcars-theme.md)** - LCARS design guide

### API Reference

- **[API Documentation](api/index.md)** - Complete API reference
- Generate detailed docs: `cargo doc --all --no-deps --open`

### Examples

- **[Examples Index](examples/index.md)** - Working code examples
- Run examples: `cargo run --example <name> -p <package>`

### Architecture & Design

- **[System Architecture](design/architecture/System_Architecture.md)** - High-level design
- **[Component Architecture](design/architecture/Component_Architecture.md)** - Component patterns
- **[Data Flow](design/architecture/Data_Flow.md)** - Event and data flow
- **[Requirements](design/architecture/)** - Detailed specifications

### Project Information

- **[Main README](../README.md)** - Project overview and quick start
- **[Contributing](../CONTRIBUTING.md)** - Development and contribution guidelines
- **[Code of Conduct](../CODEOFCONDUCT.md)** - Community standards
- **[Security](../SECURITY.md)** - Security policy

## 🎯 Phase Completion Status

### Phase 1 - Complete ✅

- All 50 components implemented
- 223 component tests passing
- Core systems (colors, events, themes)

### Phase 2 - Complete ✅

- User-friendly color formats
- Complete input system (keyboard, mouse, touch)
- Accessibility infrastructure (WCAG AAA ready)
- Rendering and window backend architecture
- Animation system with easing
- Drag and drop support
- 321 total tests passing

### Phase 3 - Planned 🚧

- Platform-specific window backends
- Platform-specific rendering backends
- Native file dialogs and system integration

[View Full Roadmap →](roadmap.md)

## Engage UX Documentation

```bash


cargo doc --all --no-deps --open



- Responsive design

- Code syntax highlighting
- Cross-referenced content
- API integration

Configure in repository settings: Settings → Pages → Source: `/docs` folder

---




