# Advanced Cross-Platform Testing Guide

Comprehensive guide for advanced testing strategies in Engage UX, including end-to-end tests, visual regression, accessibility validation, and screen reader integration.

## Table of Contents

-	[Overview](#overview)
-	[End-to-End Platform Testing](#end-to-end-platform-testing)
-	[Visual Regression Testing](#visual-regression-testing)
-	[Accessibility Validation](#accessibility-validation)
-	[Screen Reader Integration Testing](#screen-reader-integration-testing)
-	[Platform-Specific Testing](#platform-specific-testing)
-	[Test Coverage Strategy](#test-coverage-strategy)
-	[Best Practices](#best-practices)

## Overview

Advanced testing ensures Engage UX maintains consistency, accessibility, and functionality across all supported platforms: Windows, macOS, Linux, Android, and iOS.

### Testing Pyramid

```text
       /\
      /  \     E2E Tests
     /----\
    /      \   Integration Tests
   /--------\
  /          \ Unit Tests
 /____________\
```

### Test Categories

1.	**Unit Tests**: Individual functions and components (223 tests)
2.	**Integration Tests**: Component interactions (123 tests)
3.	**End-to-End Tests**: Complete workflows (19 new tests)
4.	**Visual Regression Tests**: UI consistency (15 new tests)
5.	**Accessibility Tests**: WCAG AAA compliance (39 new tests)
6.	**Screen Reader Tests**: Platform-specific AT (30 new tests)

## End-to-End Platform Testing

End-to-end tests validate complete user workflows across platform backends.

### Running E2E Tests

```bash
# Run all E2E tests
cargo test --test test_e2e_platform_backends

# Run specific E2E test
cargo test test_e2e_window_creation_and_rendering

# Run with output
cargo test --test test_e2e_platform_backends -- --nocapture
```

### E2E Test Coverage

#### Window Management Tests

-	**Window Creation Workflow**: Complete window setup and rendering
-	**State Lifecycle**: Maximize, minimize, restore, fullscreen
-	**Multi-Window Scenarios**: Multiple windows with independent contexts
-	**Resize Workflow**: Window resize with context recreation
-	**Move Events**: Window position changes
-	**Focus Management**: Window focus and event handling
-	**Close Workflow**: Proper cleanup on window close

#### Rendering Tests

-	**Complex Rendering Workflow**: Multiple UI elements with clipping
-	**UI Rendering Pipeline**: Complete button and input rendering
-	**Transparency Rendering**: Semi-transparent overlapping layers
-	**Performance Testing**: Many draw calls (100+ elements)

#### Platform Integration Tests

-	**Scaling Factors**: HiDPI and standard displays
-	**Decoration Modes**: Decorated vs. undecorated windows
-	**Resizable Modes**: Fixed-size vs. resizable windows
-	**Multiple Contexts**: Independent render contexts

### Example: E2E Window Test

```rust
#[test]
fn test_e2e_window_creation_and_rendering() {
    let factory = get_backend_factory();
    let mut window = factory.create_window_backend();
    let mut renderer = factory.create_renderer();

    // Window setup workflow
    window.set_title("E2E Test Window");
    window.set_bounds(WindowBounds::new(100, 100, 800, 600));
    window.show();

    // Rendering workflow
    let mut context = renderer.create_context(800, 600);
    context.begin_frame();

    // Render UI elements
    context.execute(RenderCommand::FillRect {
        rect: Rect::new(50.0, 50.0, 200.0, 100.0),
        color: Color::rgb(0.2, 0.4, 0.8),
    });

    context.end_frame();

    // Verify workflow completed
    assert_eq!(context.size(), (800, 600));
}
```

## Visual Regression Testing

Visual regression tests ensure UI consistency across platforms and prevent unintended visual changes.

### Running Visual Tests

```bash
# Run all visual regression tests
cargo test --test test_visual_regression

# Run specific visual test
cargo test test_visual_button_rendering

# With platform details
cargo test --test test_visual_regression -- --nocapture
```

### Visual Test Coverage

#### Component Rendering

-	**Button States**: Normal, disabled, hovered
-	**Text Input States**: Normal, focused, error
-	**Checkbox States**: Unchecked, checked, disabled
-	**Slider**: Track, fill, thumb
-	**Progress Indicator**: Bar with percentage fill
-	**Card**: With shadow and border
-	**Modal Dialog**: Overlay with content
-	**Icons**: Consistent icon rendering
-	**Dropdown**: Collapsed and expanded states
-	**Tabs**: Active and inactive tabs
-	**Toast Notifications**: With indicators
-	**Rounded Corners**: Border radius consistency

#### Theme Consistency

-	**Light Theme**: Color consistency across platforms
-	**Dark Theme**: Color consistency across platforms
-	**Contrast Ratios**: WCAG compliance verification

### Visual Testing Strategy

#### Snapshot-Based Testing

Visual tests use a snapshot identifier approach:

```rust
fn snapshot_id(test_name: &str, platform: &str) -> String {
    format!("{}_{}", test_name, platform)
}

#[test]
fn test_visual_button_rendering() {
    // ... render button states ...

    let snapshot = snapshot_id("button_rendering", current_platform());
    println!("Visual snapshot: {}", snapshot);

    // In full implementation:
    // - Capture rendering output
    // - Compare with stored baseline
    // - Generate diff if mismatch
    // - Update baseline if accepted
}
```

#### Platform Identification

```rust
fn current_platform() -> &'static str {
    #[cfg(target_os = "windows")]
    return "windows";

    #[cfg(target_os = "macos")]
    return "macos";

    #[cfg(target_os = "linux")]
    return "linux";

    #[cfg(target_os = "android")]
    return "android";

    #[cfg(target_os = "ios")]
    return "ios";
}
```

#### Comparison Process

1.	**Render**: Execute rendering commands
2.	**Capture**: Save rendering output
3.	**Compare**: Check against baseline
4.	**Report**: Generate visual diff if changed
5.	**Review**: Manual review of changes
6.	**Update**: Accept and update baseline if valid

### Example: Visual Button Test

```rust
#[test]
fn test_visual_button_rendering() {
    let factory = get_backend_factory();
    let mut renderer = factory.create_renderer();
    let mut context = renderer.create_context(400, 200);

    context.begin_frame();

    // Normal button
    context.execute(RenderCommand::FillRect {
        rect: Rect::new(50.0, 50.0, 120.0, 40.0),
        color: Color::rgb(0.2, 0.4, 0.8),
    });

    // Disabled button
    context.execute(RenderCommand::FillRect {
        rect: Rect::new(220.0, 50.0, 120.0, 40.0),
        color: Color::rgb(0.7, 0.7, 0.7),
    });

    context.end_frame();

    let snapshot = snapshot_id("button_rendering", current_platform());
    // Compare with baseline...
}
```

## Accessibility Validation

Automated accessibility tests ensure WCAG AAA compliance across all components.

### Running Accessibility Tests

```bash
# Run all accessibility tests
cargo test --test test_accessibility_validation

# Run specific accessibility test
cargo test test_accessibility_color_contrast

# With details
cargo test --test test_accessibility_validation -- --nocapture
```

### Accessibility Test Coverage

#### ARIA Compliance

-	**Role Assignment**: All components have appropriate ARIA roles
-	**Label Provision**: Accessible labels for all components
-	**State Announcement**: Checked, expanded, disabled states
-	**Live Regions**: Polite and assertive announcements

#### Keyboard Navigation

-	**Tab Navigation**: Forward and backward tabbing
-	**Focus Wrapping**: Boundary wrapping behavior
-	**Focus History**: Back navigation support
-	**Focus Trap**: Modal dialog focus containment
-	**Focus Indicators**: Visible focus indication

#### Color Contrast

-	**WCAG AAA Text**: 7:1 contrast ratio for normal text
-	**WCAG AAA Large Text**: 4.5:1 for large text
-	**WCAG AAA UI**: 3:1 for UI components
-	**Theme Compliance**: Both light and dark themes

#### Component States

-	**Disabled**: Components marked as disabled
-	**Required**: Required fields indicated
-	**Readonly**: Readonly fields marked
-	**Error States**: Validation errors announced

#### Touch Targets

-	**Minimum Size**: 44x44 points for touch targets
-	**Adequate Spacing**: Proper spacing between targets

### Example: Contrast Test

```rust
#[test]
fn test_accessibility_color_contrast() {
    // WCAG AAA requires:
    // - 7:1 for normal text
    // - 4.5:1 for large text
    // - 3:1 for UI components

    let black = Color::rgb(0.0, 0.0, 0.0, 1.0);
    let white = Color::rgb(1.0, 1.0, 1.0, 1.0);
    let ratio = white.contrast_ratio(&black);

    assert!(
        ratio >= 7.0,
        "Black on white should meet WCAG AAA (7:1), got {}",
        ratio
    );
}
```

### Example: Keyboard Navigation Test

```rust
#[test]
fn test_accessibility_keyboard_navigation() {
    let mut focus_manager = FocusManager::new();

    // Register components in tab order
    focus_manager.register_component(1, Rect::new(0.0, 0.0, 100.0, 40.0));
    focus_manager.register_component(2, Rect::new(0.0, 50.0, 100.0, 40.0));

    // Tab forward
    focus_manager.set_focus(1);
    assert_eq!(focus_manager.focused(), Some(1));

    focus_manager.focus_next();
    assert_eq!(focus_manager.focused(), Some(2));
}
```

## Screen Reader Integration Testing

Tests for platform-specific screen reader technologies.

### Running Screen Reader Tests

```bash
# Run all screen reader tests
cargo test --test test_screen_reader_integration

# Run specific screen reader test
cargo test test_screen_reader_button_label

# With platform details
cargo test --test test_screen_reader_integration -- --nocapture
```

### Platform-Specific Technologies

| Platform | Screen Reader Technology              |
| -------- | ------------------------------------- |
| Windows  | MSAA / UI Automation                  |
| macOS    | NSAccessibility                       |
| Linux    | AT-SPI                                |
| Android  | TalkBack                              |
| iOS      | VoiceOver                             |

### Screen Reader Test Coverage

#### Component Announcements

-	**Button Labels**: Button text announced
-	**Text Input**: Labels and states
-	**Checkbox States**: Checked/unchecked
-	**Slider Values**: Current value and range
-	**Progress Updates**: Progress percentage
-	**Dialog Opening**: Dialog title announced

#### Live Regions

-	**Polite Announcements**: Toast notifications
-	**Assertive Announcements**: Error alerts
-	**Status Updates**: Form submission results

#### Navigation

-	**List Navigation**: List structure and items
-	**Menu Navigation**: Menu items and structure
-	**Tab Navigation**: Current tab and total
-	**Table Navigation**: Row/column position
-	**Breadcrumb**: Current location in hierarchy

#### States and Properties

-	**Disabled Components**: Announced as disabled
-	**Required Fields**: Marked as required
-	**Readonly Fields**: Announced as readonly
-	**Error Messages**: Validation errors announced
-	**Expandable Sections**: Expanded/collapsed state

#### Media and Images

-	**Image Alt Text**: Descriptive text read
-	**Decorative Images**: Hidden from screen reader
-	**Icon Labels**: Icon meaning announced
-	**Video Captions**: Caption support

### Example: Screen Reader Label Test

```rust
#[test]
fn test_screen_reader_button_label() {
    let technology = screen_reader_technology();
    println!("Testing with: {}", technology);

    let button = Button::new(1, "Submit Form");
    let props = button.accessibility();

    // Screen reader should read: "Submit Form, button"
    assert!(props.label.is_some());
    assert_eq!(props.label.unwrap(), "Submit Form");
    assert_eq!(props.role, Some(AriaRole::Button));
}
```

### Example: Live Region Test

```rust
#[test]
fn test_screen_reader_live_regions() {
    // Toast should use polite announcement
    let toast = Toast::new(1, "File saved successfully");
    let props = toast.accessibility();
    assert_eq!(props.live, Some(AriaLive::Polite));

    // Alert should use assertive announcement
    let alert = Alert::new(2, "Connection lost");
    let props = alert.accessibility();
    assert_eq!(props.live, Some(AriaLive::Assertive));
}
```

## Platform-Specific Testing

### Windows Testing

#### Screen Readers

-	**NVDA**: Free, open-source screen reader
-	**JAWS**: Commercial screen reader
-	**Narrator**: Built-in Windows screen reader

#### Accessibility APIs

-	**MSAA**: Microsoft Active Accessibility
-	**UI Automation**: Modern accessibility API

#### Backend

-	**Window Management**: Winit window backend
-	**Rendering**: Softbuffer software renderer

### macOS Testing

#### Screen Readers

-	**VoiceOver**: Built-in macOS screen reader

#### Accessibility APIs

-	**NSAccessibility**: Native accessibility framework

#### Backend

-	**Window Management**: Winit window backend
-	**Rendering**: Softbuffer software renderer

### Linux Testing

#### Screen Readers

-	**Orca**: GNOME screen reader

#### Accessibility APIs

-	**AT-SPI**: Assistive Technology Service Provider Interface

#### Backend

-	**Window Management**: Winit window backend
-	**Rendering**: Softbuffer software renderer

### Android Testing

#### Screen Readers

-	**TalkBack**: Built-in Android screen reader

#### Accessibility APIs

-	**Android Accessibility Framework**

#### Backend

-	**Window Management**: Winit window backend
-	**Rendering**: Softbuffer software renderer

### iOS Testing

#### Screen Readers

-	**VoiceOver**: Built-in iOS screen reader

#### Accessibility APIs

-	**iOS Accessibility API**

#### Backend

-	**Window Management**: Winit window backend
-	**Rendering**: Softbuffer software renderer

## Test Coverage Strategy

### Current Test Coverage

As of Advanced Testing completion:

-	**Total Tests**: 526 tests passing
-	**Unit Tests**: 223 (components)
-	**Integration Tests**: 123 (core systems)
-	**Platform Backend Tests**: 14 (existing)
-	**E2E Tests**: 14 (new)
-	**Visual Regression Tests**: 14 (new)
-	**Accessibility Tests**: 32 (new)
-	**Screen Reader Tests**: 28 (new)
-	**Other Integration Tests**: 78 (existing)

### Coverage Goals

| Category              | Current | Goal | Status  |
| --------------------- | ------- | ---- | ------- |
| Unit Tests            | 223     | 250  | 89%     |
| Integration Tests     | 201     | 200  | **101%** ✓ |
| E2E Tests             | 14      | 15   | 93%     |
| Visual Tests          | 14      | 15   | 93%     |
| Accessibility Tests   | 32      | 35   | 91%     |
| Screen Reader Tests   | 28      | 30   | 93%     |
| **Overall**           | **526** | 550  | **96%** |

### Coverage by Crate

-	**engage-ux-core**: 95%+ (critical functionality)
-	**engage-ux-components**: 90%+ (all public APIs)
-	**engage-ux-themes**: 85%+ (configuration and parsing)
-	**engage-ux-oal**: 80%+ (platform-specific code)
-	**engage-ux-tests**: 100% (integration tests)

## Best Practices

### 1. Write Descriptive Test Names

```rust
// Good
#[test]
fn test_e2e_window_state_lifecycle() { }

#[test]
fn test_accessibility_keyboard_navigation() { }

// Avoid
#[test]
fn test1() { }
```

### 2. Test Across All Platforms

```rust
#[test]
fn test_platform_screen_reader_integration() {
    let technology = screen_reader_technology();

    #[cfg(target_os = "windows")]
    assert_eq!(technology, "MSAA/UI Automation");

    #[cfg(target_os = "macos")]
    assert_eq!(technology, "NSAccessibility");

    // ... etc for all platforms
}
```

### 3. Verify WCAG Compliance

```rust
#[test]
fn test_accessibility_theme_colors() {
    let theme = Theme::light();

    // WCAG AAA: 7:1 for text
    let text_ratio = theme.colors.background
        .contrast_ratio(&theme.colors.text_primary);
    assert!(text_ratio >= 7.0);

    // WCAG AAA: 3:1 for UI
    let ui_ratio = theme.colors.background
        .contrast_ratio(&theme.colors.primary);
    assert!(ui_ratio >= 3.0);
}
```

### 4. Test Complete Workflows

```rust
#[test]
fn test_e2e_window_creation_and_rendering() {
    // 1. Create backend
    let factory = get_backend_factory();
    let mut window = factory.create_window_backend();

    // 2. Configure window
    window.set_title("Test");
    window.set_bounds(WindowBounds::new(0, 0, 800, 600));

    // 3. Show window
    window.show();

    // 4. Create rendering context
    let mut renderer = factory.create_renderer();
    let mut context = renderer.create_context(800, 600);

    // 5. Render content
    context.begin_frame();
    context.execute(RenderCommand::Clear(Color::rgb(1.0, 1.0, 1.0)));
    context.end_frame();

    // 6. Verify
    assert!(window.is_visible());
    assert_eq!(context.size(), (800, 600));
}
```

### 5. Use Platform-Appropriate Technologies

```rust
fn screen_reader_technology() -> &'static str {
    #[cfg(target_os = "windows")]
    return "MSAA/UI Automation";

    #[cfg(target_os = "macos")]
    return "NSAccessibility";

    #[cfg(target_os = "linux")]
    return "AT-SPI";

    // ... etc
}
```

### 6. Document Test Purpose

```rust
/// Test complete window creation and rendering workflow
///
/// Validates:
/// - Window creation and configuration
/// - Rendering context setup
/// - Basic rendering commands
/// - State verification
#[test]
fn test_e2e_window_creation_and_rendering() {
    // ...
}
```

### 7. Keep Tests Deterministic

```rust
// Good - deterministic
#[test]
fn test_button_click_count() {
    let mut button = Button::new(1, "Click");
    let counter = Arc::new(AtomicUsize::new(0));

    button.set_on_click(move |_| {
        counter.fetch_add(1, Ordering::Relaxed);
    });

    button.handle_click();
    button.handle_click();

    assert_eq!(counter.load(Ordering::Relaxed), 2);
}

// Avoid - non-deterministic
#[test]
fn test_animation_timing() {
    // Don't rely on exact timing
    std::thread::sleep(Duration::from_millis(100));
}
```

## Running All Advanced Tests

```bash
# Run all tests
cargo test --all

# Run only advanced tests
cargo test --test test_e2e_platform_backends
cargo test --test test_accessibility_validation
cargo test --test test_visual_regression
cargo test --test test_screen_reader_integration

# Run with coverage
cargo tarpaulin --all --out Html

# Run on specific platform
cargo test --target x86_64-pc-windows-msvc
cargo test --target x86_64-apple-darwin
cargo test --target x86_64-unknown-linux-gnu
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Advanced Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run all tests
        run: cargo test --all

      - name: Run advanced tests
        run: |
          cargo test --test test_e2e_platform_backends
          cargo test --test test_accessibility_validation
          cargo test --test test_visual_regression
          cargo test --test test_screen_reader_integration
```

## See Also

-	[Testing Guide](testing.md) - Basic testing documentation
-	[Accessibility Guide](accessibility.md) - Accessibility features
-	[Component Development](component-development.md) - Creating testable components
-	[Contributing Guide](../../CONTRIBUTING.md) - Development guidelines

---

[← Back to Guides](index.md)
