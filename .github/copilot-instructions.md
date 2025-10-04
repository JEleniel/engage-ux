# GitHub Copilot Instructions for Engage UX

This file provides guidelines and requirements for GitHub Copilot when working with the Engage UX repository.

## Project Overview

Engage UX is a fully cross-platform Rust UI toolkit that provides a themable component library without depending on a browser engine. It uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing a single set of components to work across Windows, macOS, Linux, Android, and iOS.

## Code Style Requirements

### Indentation and Formatting
- **Use tabs for indentation, not spaces** - This is enforced by `rustfmt.toml`
- Follow the Rust style guide for all other formatting
- Keep lines under 100 characters when practical
- Use `//!` for module/crate documentation
- Use `///` for item documentation

### Safety
- **NEVER introduce `unsafe` code** - All crates have `#![forbid(unsafe_code)]`
- Never remove the `unsafe_code = "forbid"` lint
- All code must be 100% safe Rust

### Dependencies
- Only use crates that have been updated within the past **6 months**
- Avoid adding new dependencies unless absolutely necessary
- Prefer crates that are well-maintained and widely used
- The project currently uses minimal dependencies (tokio, serde, serde_json)

## Project Structure

```
engage-ux/
├── engage-ux-core/         # Core functionality (colors, events, components)
├── engage-ux-oal/          # OS Abstraction Layer
├── engage-ux-components/   # UI components
└── engage-ux-themes/       # Theme system
```

## Testing Requirements

- All new features **must** include unit tests
- Aim for **90% code coverage**
- Tests should be comprehensive and test edge cases
- Use descriptive test names that explain what is being tested
- Follow the existing test patterns in the codebase

### Test Example Pattern
```rust
#[test]
fn test_component_handles_event() {
    // Setup
    let mut component = Component::new(1, "Test");
    
    // Action
    component.handle_event(&event);
    
    // Assert
    assert!(expected_result);
}
```

## Documentation

- All public APIs **must** have documentation comments
- Include examples in documentation when helpful
- Update README.md if adding major features
- Keep documentation clear and concise

## Platform-Specific Code

When adding platform-specific code:

1. Use conditional compilation (`#[cfg(target_os = "...")]`)
2. Place platform-specific code in the `engage-ux-oal` crate
3. Ensure the code works on all supported platforms or use feature gates
4. Add tests for each platform when possible

### Example
```rust
#[cfg(target_os = "windows")]
fn platform_specific_function() {
    // Windows implementation
}

#[cfg(target_os = "macos")]
fn platform_specific_function() {
    // macOS implementation
}
```

## Accessibility

- Follow WCAG AAA guidelines
- Ensure all components are keyboard navigable
- Provide appropriate ARIA labels and roles
- Test with screen readers when possible

## Build and Test Commands

### Build
```bash
cargo build --all
```

### Test
```bash
cargo test --all
```

### Format Check
```bash
cargo fmt --all -- --check
```

### Linter
```bash
cargo clippy --all -- -D warnings
```

### Run Examples
```bash
cargo run --example basic_components -p engage-ux-components
cargo run --example theme_demo -p engage-ux-components
```

## Key Design Principles

- **Device Independent**: Works across all supported platforms with feature parity
- **Pure Rust**: 100% Rust code with no unsafe code
- **UX Focused**: Entirely focused on the user experience layer
- **Themable**: Every visual aspect can be customized through themes
- **Thread-Safe**: Built on Tokio for async, non-blocking operations
- **Not Reactive**: Decoupled from data/logic handling
- **Not Hybrid**: No web technologies or JavaScript

## What NOT to Do

- Don't use spaces for indentation (use tabs)
- Don't introduce unsafe code
- Don't add dependencies without careful consideration
- Don't remove existing tests
- Don't break cross-platform compatibility
- Don't use web technologies or browser engines
- Don't add reactive patterns or state management

## License

All contributions are licensed under GPL-3.0. Ensure any code suggestions respect this license.
