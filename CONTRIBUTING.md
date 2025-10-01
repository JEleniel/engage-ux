# Contributing to Engage UX

Thank you for your interest in contributing to Engage UX! This document provides guidelines and requirements for contributions.

## Code of Conduct

Please be respectful and constructive in all interactions with the project and its community.

## Development Requirements

### Rust Version

Engage UX requires Rust 1.70 or later.

### Code Style

1. **Indentation**: Use **tabs** for indentation (not spaces)
2. **Formatting**: Follow the Rust style guide for all other formatting
3. **Comments**: Use `//!` for module/crate documentation, `///` for item documentation
4. **Line Length**: Try to keep lines under 100 characters when practical

The project includes:
- `rustfmt.toml` - Rust formatting configuration (enforces hard tabs)
- `.editorconfig` - Editor configuration for consistent formatting

### Safety

- **No unsafe code**: All crates have `#![forbid(unsafe_code)]`
- Never introduce `unsafe` blocks or remove the `unsafe_code = "forbid"` lint

### Dependencies

- Only use crates that have been updated within the past **6 months**
- Avoid adding new dependencies unless absolutely necessary
- Prefer crates that are well-maintained and widely used

## Project Structure

```
engage-ux/
├── engage-ux-core/         # Core functionality (colors, events, components)
├── engage-ux-oal/          # OS Abstraction Layer
├── engage-ux-components/   # UI components
└── engage-ux-themes/       # Theme system
```

## Building and Testing

### Build

```bash
cargo build --all
```

### Test

```bash
cargo test --all
```

### Run Examples

```bash
cargo run --example basic_components -p engage-ux-components
cargo run --example theme_demo -p engage-ux-components
```

### Check Formatting

```bash
cargo fmt --all -- --check
```

### Run Linter

```bash
cargo clippy --all -- -D warnings
```

## Testing Requirements

- All new features must include unit tests
- Aim for **90% code coverage**
- Tests should be comprehensive and test edge cases
- Use descriptive test names that explain what is being tested

Example test:

```rust
#[test]
fn test_button_handles_click_event() {
    let mut button = Button::new(1, "Click");
    let clicked = Arc::new(AtomicBool::new(false));
    let clicked_clone = clicked.clone();
    
    button.set_on_click(move |_| {
        clicked_clone.store(true, Ordering::Relaxed);
    });
    
    let event = Event::new(1, EventType::Click);
    button.handle_click(&event);
    
    assert!(clicked.load(Ordering::Relaxed));
}
```

## Documentation

- All public APIs must have documentation comments
- Include examples in documentation when helpful
- Update README.md if adding major features

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following the guidelines above
4. Ensure all tests pass
5. Ensure code is properly formatted
6. Commit your changes with clear, descriptive messages
7. Push to your fork
8. Open a Pull Request with a clear description of the changes

### PR Description Should Include

- What problem does this solve?
- What changes were made?
- Are there any breaking changes?
- Screenshots (if UI changes)
- Test coverage information

## Platform-Specific Code

When adding platform-specific code:

1. Use conditional compilation (`#[cfg(target_os = "...")]`)
2. Place platform-specific code in the `engage-ux-oal` crate
3. Ensure the code works on all supported platforms or use feature gates
4. Add tests for each platform when possible

Example:

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

## License

By contributing to Engage UX, you agree that your contributions will be licensed under the GPL-3.0 license.

## Questions?

If you have questions about contributing, please open an issue for discussion.

## Recognition

Contributors will be recognized in the project's documentation and release notes.
