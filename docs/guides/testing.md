# Testing Guide

Learn how to test your Engage UX applications and components effectively.

## Table of Contents

- [Overview](#overview)
- [Running Tests](#running-tests)
- [Unit Testing](#unit-testing)
- [Component Testing](#component-testing)
- [Integration Testing](#integration-testing)
- [Test Coverage](#test-coverage)
- [Best Practices](#best-practices)

## Overview

Engage UX uses Rust's built-in testing framework along with Tokio for async testing. The project aims for 90%+ code coverage with comprehensive unit and integration tests.

### Testing Philosophy

- **Test behavior, not implementation** - Focus on what components do
- **Test edge cases** - Handle boundary conditions
- **Use descriptive names** - Tests should document behavior
- **Keep tests focused** - One concept per test
- **Make tests deterministic** - Avoid flaky tests

## Running Tests

### Run All Tests

```bash
# Run all tests in workspace
cargo test --all

# Run with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_button_creation
```

### Run Tests for Specific Crate

```bash
# Core tests
cargo test -p engage-ux-core

# Component tests
cargo test -p engage-ux-components

# Theme tests
cargo test -p engage-ux-themes

# OAL tests
cargo test -p engage-ux-oal
```

### Run Tests in Release Mode

```bash
cargo test --release
```

## Unit Testing

Unit tests validate individual functions and methods.

### Basic Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_rgb() {
        let color = Color::from_rgb(255, 128, 0);
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.alpha(), 255);
    }
}
```

### Testing with Expected Failures

```rust
#[test]
#[should_panic(expected = "Invalid color")]
fn test_invalid_color() {
    Color::from_rgb(256, 0, 0); // Should panic
}
```

### Testing Results

```rust
#[test]
fn test_color_from_hex() -> Result<(), String> {
    let color = Color::from_hex("#FF8000")?;
    assert_eq!(color.red(), 255);
    Ok(())
}
```

## Component Testing

Test component behavior and state management.

### Component Creation

```rust
#[test]
fn test_button_creation() {
    let button = Button::new(1, "Click Me");
    
    assert_eq!(button.id(), 1);
    assert_eq!(button.text(), "Click Me");
    assert!(button.is_visible());
    assert!(button.is_enabled());
}
```

### Component Properties

```rust
#[test]
fn test_button_properties() {
    let mut button = Button::new(1, "Test");
    
    // Test visibility
    button.set_visible(false);
    assert!(!button.is_visible());
    
    // Test enabled state
    button.set_enabled(false);
    assert!(!button.is_enabled());
    
    // Test bounds
    let bounds = Rect::new(10.0, 20.0, 100.0, 50.0);
    button.set_bounds(bounds);
    assert_eq!(button.bounds(), bounds);
}
```

### Event Handling

```rust
#[test]
fn test_button_click_handler() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    let mut button = Button::new(1, "Click");
    let clicked = Arc::new(AtomicBool::new(false));
    let clicked_clone = clicked.clone();
    
    button.set_on_click(move |_| {
        clicked_clone.store(true, Ordering::Relaxed);
    });
    
    // Simulate click
    let event = Event::new(1, EventType::Click);
    button.handle_event(&event);
    
    assert!(clicked.load(Ordering::Relaxed));
}
```

### State Changes

```rust
#[test]
fn test_checkbox_state() {
    let mut checkbox = Checkbox::new(1, "Agree");
    
    assert!(!checkbox.is_checked());
    
    checkbox.set_checked(true);
    assert!(checkbox.is_checked());
    
    checkbox.toggle();
    assert!(!checkbox.is_checked());
}
```

## Integration Testing

Integration tests validate component interactions and system behavior.

### Async Tests

```rust
#[tokio::test]
async fn test_event_system() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    
    // Send event
    tx.send(Event::new(1, EventType::Click)).await.unwrap();
    
    // Receive event
    let event = rx.recv().await.unwrap();
    assert_eq!(event.target_id, 1);
}
```

### Component Composition

```rust
#[test]
fn test_container_with_children() {
    let mut container = Container::new(1);
    let button = Button::new(2, "Child");
    let label = Label::new(3, "Label");
    
    container.add_child(Box::new(button));
    container.add_child(Box::new(label));
    
    assert_eq!(container.child_count(), 2);
}
```

### Theme Integration

```rust
#[test]
fn test_themed_component() {
    let theme = Theme::dark();
    let mut button = Button::new(1, "Themed");
    
    button.set_background_color(theme.colors.primary);
    button.set_text_color(theme.colors.on_primary);
    
    assert_eq!(button.background_color(), theme.colors.primary);
    assert_eq!(button.text_color(), theme.colors.on_primary);
}
```

## Test Coverage

### Measuring Coverage

Use `tarpaulin` for coverage reports:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --all --out Html

# Generate and open report
cargo tarpaulin --all --out Html --output-dir ./coverage && open coverage/index.html
```

### Coverage Goals

- **Overall**: 90%+ code coverage
- **Core**: 95%+ (critical functionality)
- **Components**: 90%+ (all public APIs)
- **Themes**: 85%+ (configuration and parsing)
- **OAL**: 80%+ (platform-specific code)

### Current Coverage

As of Phase 2 completion:

- **Total**: 321 tests passing
- **engage-ux-core**: 14 tests
- **engage-ux-components**: 200 tests
- **engage-ux-themes**: 4 tests
- **engage-ux-oal**: 5 tests
- **engage-ux-tests**: 98 integration tests

## Best Practices

### 1. Use Descriptive Test Names

```rust
// Good
#[test]
fn button_click_handler_is_called_when_clicked() { }

#[test]
fn text_input_validates_email_format_correctly() { }

// Avoid
#[test]
fn test1() { }

#[test]
fn it_works() { }
```

### 2. Arrange-Act-Assert Pattern

```rust
#[test]
fn test_slider_value_update() {
    // Arrange
    let mut slider = Slider::new(1);
    slider.set_range(0.0, 100.0);
    
    // Act
    slider.set_value(50.0);
    
    // Assert
    assert_eq!(slider.value(), 50.0);
}
```

### 3. Test Edge Cases

```rust
#[test]
fn test_slider_clamps_values() {
    let mut slider = Slider::new(1);
    slider.set_range(0.0, 100.0);
    
    // Test below minimum
    slider.set_value(-10.0);
    assert_eq!(slider.value(), 0.0);
    
    // Test above maximum
    slider.set_value(110.0);
    assert_eq!(slider.value(), 100.0);
}
```

### 4. Use Test Fixtures

```rust
fn create_test_button() -> Button {
    let mut button = Button::new(1, "Test");
    button.set_bounds(Rect::new(0.0, 0.0, 100.0, 40.0));
    button
}

#[test]
fn test_with_fixture() {
    let button = create_test_button();
    assert_eq!(button.id(), 1);
}
```

### 5. Test Async Code Properly

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_with_timeout() {
    let timeout = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        async_function()
    );
    
    assert!(timeout.await.is_ok());
}
```

### 6. Mock External Dependencies

```rust
struct MockEventHandler {
    events: Vec<Event>,
}

impl MockEventHandler {
    fn new() -> Self {
        Self { events: Vec::new() }
    }
    
    fn handle(&mut self, event: Event) {
        self.events.push(event);
    }
}

#[test]
fn test_with_mock() {
    let mut handler = MockEventHandler::new();
    let event = Event::new(1, EventType::Click);
    
    handler.handle(event);
    
    assert_eq!(handler.events.len(), 1);
}
```

### 7. Use Property-Based Testing

```rust
// Using proptest (add to dev-dependencies)
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_color_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let color = Color::from_rgb(r, g, b);
        assert_eq!(color.red(), r);
        assert_eq!(color.green(), g);
        assert_eq!(color.blue(), b);
    }
}
```

## Common Testing Patterns

### Testing Callbacks

```rust
#[test]
fn test_callback_execution() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    
    let mut button = Button::new(1, "Click");
    button.set_on_click(move |_| {
        counter_clone.fetch_add(1, Ordering::Relaxed);
    });
    
    // Trigger multiple times
    button.handle_click();
    button.handle_click();
    button.handle_click();
    
    assert_eq!(counter.load(Ordering::Relaxed), 3);
}
```

### Testing Validation

```rust
#[test]
fn test_input_validation() {
    let mut input = TextInput::new(1);
    
    input.set_validator(|text: &str| {
        text.len() >= 3 && text.len() <= 20
    });
    
    assert!(input.validate("hello"));
    assert!(!input.validate("hi"));
    assert!(!input.validate("this is a very long text"));
}
```

### Testing State Machines

```rust
#[test]
fn test_animation_state_transitions() {
    let mut animation = Animation::new();
    
    assert_eq!(animation.state(), AnimationState::Idle);
    
    animation.play();
    assert_eq!(animation.state(), AnimationState::Playing);
    
    animation.pause();
    assert_eq!(animation.state(), AnimationState::Paused);
    
    animation.stop();
    assert_eq!(animation.state(), AnimationState::Idle);
}
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, nightly]
    
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
    
    - name: Run tests
      run: cargo test --all
    
    - name: Run clippy
      run: cargo clippy --all -- -D warnings
```

## Debugging Tests

### Running Specific Tests

```bash
# Run single test
cargo test test_button_creation

# Run tests matching pattern
cargo test button

# Show test output
cargo test -- --nocapture

# Show test names without running
cargo test -- --list
```

### Using println! in Tests

```rust
#[test]
fn test_with_debug_output() {
    let button = Button::new(1, "Test");
    println!("Button ID: {}", button.id());
    println!("Button text: {}", button.text());
    assert_eq!(button.id(), 1);
}
```

Run with `cargo test -- --nocapture` to see output.

## See Also

- [Contributing Guide](../../CONTRIBUTING.md) - Development guidelines
- [Component Development](component-development.md) - Creating components
- [API Reference](../api/index.md) - API documentation

---

[← Back to Guides](index.md)
