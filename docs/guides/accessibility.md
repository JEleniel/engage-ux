# Accessibility Guide

Creating accessible applications with Engage UX that meet WCAG AAA standards.

## Table of Contents

- [Overview](#overview)
- [Accessibility Infrastructure](#accessibility-infrastructure)
- [ARIA Roles and Attributes](#aria-roles-and-attributes)
- [Keyboard Navigation](#keyboard-navigation)
- [Screen Reader Support](#screen-reader-support)
- [Focus Management](#focus-management)
- [Color and Contrast](#color-and-contrast)
- [Component Accessibility](#component-accessibility)
- [Testing Accessibility](#testing-accessibility)
- [Best Practices](#best-practices)

## Overview

Engage UX is designed with accessibility as a core feature, not an afterthought. All components include built-in accessibility support aligned with [WCAG AAA guidelines](https://www.w3.org/WAI/WCAG21/quickref/).

### WCAG Conformance Levels

- **Level A**: Basic accessibility (minimum)
- **Level AA**: Addresses major barriers (recommended)
- **Level AAA**: Highest level (Engage UX target)

### Key Principles

1. **Perceivable** - Information must be presentable to users in ways they can perceive
2. **Operable** - Interface components must be operable
3. **Understandable** - Information and operation must be understandable
4. **Robust** - Content must be robust enough for assistive technologies

## Accessibility Infrastructure

Engage UX provides comprehensive accessibility infrastructure:

```rust
use engage_ux_core::accessibility::{
    AccessibilityProps,
    AriaRole,
    FocusManager,
    ScreenReader,
};
```

### AccessibilityProps

Every component can define accessibility properties:

```rust
let accessibility = AccessibilityProps::new(component_id)
    .with_role(AriaRole::Button)
    .with_label("Submit form")
    .with_description("Submits the form data to the server");
```

## ARIA Roles and Attributes

### Common ARIA Roles

Engage UX supports all standard ARIA roles:

```rust
pub enum AriaRole {
    // Document Structure
    Article,
    Section,
    Navigation,
    Main,
    Complementary,
    
    // Landmark Roles
    Banner,
    Contentinfo,
    Form,
    Search,
    
    // Widget Roles
    Button,
    Checkbox,
    Radio,
    Textbox,
    Slider,
    Progressbar,
    Tab,
    Tabpanel,
    Dialog,
    Alertdialog,
    Alert,
    
    // And many more...
}
```

### Setting Roles

```rust
use engage_ux_components::Button;
use engage_ux_core::accessibility::{AccessibilityProps, AriaRole};

let mut button = Button::new(1, "Submit");

// Role is automatically set for standard components
// But can be customized if needed
let props = AccessibilityProps::new(button.id())
    .with_role(AriaRole::Button);
```

### ARIA Attributes

```rust
let props = AccessibilityProps::new(id)
    .with_label("User name")              // aria-label
    .with_labelled_by("label-id")         // aria-labelledby
    .with_described_by("help-text-id")    // aria-describedby
    .with_hidden(false)                   // aria-hidden
    .with_disabled(false)                 // aria-disabled
    .with_required(true)                  // aria-required
    .with_invalid(false)                  // aria-invalid
    .with_expanded(false)                 // aria-expanded
    .with_selected(false)                 // aria-selected
    .with_checked(false)                  // aria-checked
    .with_value_now(50.0)                 // aria-valuenow
    .with_value_min(0.0)                  // aria-valuemin
    .with_value_max(100.0);               // aria-valuemax
```

## Keyboard Navigation

### Tab Navigation

Components support tab navigation by default:

```rust
use engage_ux_core::accessibility::FocusManager;

let mut focus_manager = FocusManager::new();

// Register components in tab order
focus_manager.register_component(button.id(), button.bounds());
focus_manager.register_component(input.id(), input.bounds());
focus_manager.register_component(checkbox.id(), checkbox.bounds());

// Navigate
focus_manager.focus_next();  // Tab
focus_manager.focus_prev();  // Shift+Tab
```

### Keyboard Shortcuts

Components handle standard keyboard interactions:

```rust
use engage_ux_core::input::{InputHandler, KeyCode, KeyModifiers};

impl InputHandler for MyComponent {
    fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
        match event.key_code {
            KeyCode::Enter | KeyCode::Space => {
                // Activate component
                self.activate();
                true
            }
            KeyCode::Escape => {
                // Cancel or close
                self.cancel();
                true
            }
            KeyCode::ArrowUp if event.modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+Up: Navigate to parent
                self.navigate_to_parent();
                true
            }
            _ => false
        }
    }
}
```

### Focus Indicators

Ensure visible focus indicators:

```rust
let mut button = Button::new(1, "Click Me");

// Focus styling
button.set_focus_color(Color::from_hex("#007AFF")?);
button.set_focus_width(2.0);
button.set_focus_style(FocusStyle::Outline);
```

## Screen Reader Support

Engage UX provides native screen reader integration for all supported platforms through the OS Abstraction Layer (OAL).

### Platform Support

- **Windows**: MSAA/UI Automation integration
- **macOS**: NSAccessibility API integration
- **Linux**: AT-SPI (Assistive Technology Service Provider Interface)
- **Android**: TalkBack integration
- **iOS**: VoiceOver integration

### Screen Reader Backend

The screen reader backend is automatically selected based on your platform:

```rust
use engage_ux_oal::{get_backend_factory, ScreenReaderBackend};

// Get the platform-appropriate screen reader backend
let factory = get_backend_factory();
let mut screen_reader = factory.create_screen_reader();

// Check if a screen reader is enabled
if screen_reader.is_enabled() {
    println!("Screen reader detected: {}", screen_reader.backend_name());
}
```

### Screen Reader Announcements

```rust
use engage_ux_core::accessibility::{Announcement, AnnouncementPriority};

// Create announcements with different priorities
let mut screen_reader = factory.create_screen_reader();

// Polite - waits for pause in speech
screen_reader.announce(Announcement::low("Form submitted successfully"));

// Medium priority
screen_reader.announce(Announcement::medium("New message received"));

// Assertive - interrupts current speech
screen_reader.announce(Announcement::high("Error: Invalid email address"));

// Stop current announcement
screen_reader.stop();
```

### Component Accessibility Tree

Components are automatically added to the accessibility tree:

```rust
use engage_ux_core::accessibility::{AccessibilityProps, AriaRole};

// Create accessibility properties
let button_props = AccessibilityProps::new()
    .with_role(AriaRole::Button)
    .with_label("Submit Form")
    .with_description("Submits the form data to the server")
    .with_focusable(true);

// Update component in accessibility tree
screen_reader.update_component(button_id, button_props);

// Remove component when destroyed
screen_reader.remove_component(button_id);
```

### Focus Management

Screen readers track focus changes:

```rust
// Set focus to a component
screen_reader.set_focus(component_id);

// Clear focus
screen_reader.clear_focus();
```

### Live Regions

```rust
let props = AccessibilityProps::new()
    .with_role(AriaRole::Status)
    .with_label("Loading Status");

screen_reader.update_component(status_id, props);

// Announce updates to live regions
screen_reader.announce(Announcement::medium("Loading complete"));
```

### Component Labels

```rust
// Explicit label
let mut input = TextInput::new(1);
input.set_aria_label("Email address");

// Label association
let label = Label::new(2, "Password:");
let mut input = TextInput::new(3);
input.set_aria_labelled_by(label.id());

// Description
let help_text = Text::new(4, "Must be at least 8 characters");
input.set_aria_described_by(help_text.id());
```

## Focus Management

### Focus Order

Define logical focus order:

```rust
use engage_ux_core::accessibility::FocusManager;

let mut manager = FocusManager::new();

// Add components in tab order
manager.register_component(username_input.id(), username_input.bounds());
manager.register_component(password_input.id(), password_input.bounds());
manager.register_component(remember_checkbox.id(), remember_checkbox.bounds());
manager.register_component(submit_button.id(), submit_button.bounds());
```

### Focus Trapping

Trap focus within modal dialogs:

```rust
let mut modal = Modal::new(1, "Confirm Action");

// Enable focus trap
modal.set_focus_trap(true);

// Focus will cycle within modal
// Escape key closes and returns focus
```

### Initial Focus

Set initial focus on important elements:

```rust
// Auto-focus first input
username_input.set_auto_focus(true);

// Or programmatically
focus_manager.set_focus(username_input.id());
```

## Color and Contrast

### Contrast Requirements

WCAG AAA requires:

- **Normal text**: 7:1 contrast ratio
- **Large text**: 4.5:1 contrast ratio
- **UI components**: 3:1 contrast ratio

### Checking Contrast

```rust
use engage_ux_core::Color;

let background = Color::from_hex("#FFFFFF")?;
let text = Color::from_hex("#000000")?;

// Calculate contrast ratio
let ratio = background.contrast_ratio(&text);
assert!(ratio >= 7.0); // WCAG AAA for normal text
```

### Theme Color Accessibility

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();

// Built-in themes ensure proper contrast
// Primary on background
assert!(theme.colors.primary.contrast_ratio(&theme.colors.background) >= 3.0);

// Text on background
assert!(theme.colors.text_primary.contrast_ratio(&theme.colors.background) >= 7.0);
```

### Color Independence

Don't rely solely on color to convey information:

```rust
// Bad: Only color indicates error
input.set_border_color(Color::from_hex("#FF0000")?);

// Good: Color + icon + text
input.set_border_color(Color::from_hex("#FF0000")?);
input.set_error_icon(true);
input.set_error_message("Invalid email format");
```

## Component Accessibility

### Button

```rust
let mut button = Button::new(1, "Delete Item");

// Accessibility is built-in
// - Role: button
// - Label: Button text
// - Keyboard: Enter/Space
// - Focus: Can receive focus
```

### Text Input

```rust
let mut input = TextInput::new(1);
input.set_aria_label("Search query");
input.set_placeholder("Enter search terms...");
input.set_required(true);

// Announce validation errors
input.set_on_validation_error(|error| {
    ScreenReader::announce(error, AnnouncementPriority::Assertive);
});
```

### Checkbox

```rust
let mut checkbox = Checkbox::new(1, "I agree to terms");

// Built-in accessibility:
// - Role: checkbox
// - State: aria-checked
// - Keyboard: Space to toggle
```

### Slider

```rust
let mut slider = Slider::new(1);
slider.set_range(0.0, 100.0);
slider.set_aria_label("Volume");
slider.set_value_text_formatter(|value| format!("{}%", value));

// Built-in:
// - Role: slider
// - aria-valuenow, aria-valuemin, aria-valuemax
// - Keyboard: Arrow keys
```

### Modal Dialog

```rust
let mut modal = Modal::new(1, "Confirm Delete");

// Accessibility features:
// - Role: dialog
// - Focus trap
// - Escape to close
// - Returns focus on close
// - aria-modal="true"
```

## Testing Accessibility

### Automated Testing

```rust
#[test]
fn test_button_accessibility() {
    let button = Button::new(1, "Submit");
    let props = button.accessibility();
    
    assert_eq!(props.role, AriaRole::Button);
    assert_eq!(props.label.as_ref().unwrap(), "Submit");
    assert!(props.focusable);
}
```

### Keyboard Testing

```rust
#[test]
fn test_keyboard_navigation() {
    let mut manager = FocusManager::new();
    manager.register_component(1, Rect::new(0.0, 0.0, 100.0, 40.0));
    manager.register_component(2, Rect::new(0.0, 50.0, 100.0, 40.0));
    
    manager.set_focus(1);
    assert_eq!(manager.focused_component(), Some(1));
    
    manager.focus_next();
    assert_eq!(manager.focused_component(), Some(2));
}
```

### Screen Reader Testing

Test with actual screen readers:

- **Windows**: NVDA (free), JAWS
- **macOS**: VoiceOver (built-in)
- **Linux**: Orca
- **Mobile**: TalkBack (Android), VoiceOver (iOS)

### Contrast Testing

```rust
#[test]
fn test_theme_contrast() {
    let theme = Theme::dark();
    
    // Text on background
    let contrast = theme.colors.text_primary.contrast_ratio(&theme.colors.background);
    assert!(contrast >= 7.0, "WCAG AAA requires 7:1 for normal text");
    
    // Primary on background
    let contrast = theme.colors.primary.contrast_ratio(&theme.colors.background);
    assert!(contrast >= 3.0, "WCAG AAA requires 3:1 for UI components");
}
```

## Best Practices

### 1. Use Semantic HTML-like Structure

```rust
// Use appropriate component types
let header = Container::new(1);
header.set_role(AriaRole::Banner);

let nav = Container::new(2);
nav.set_role(AriaRole::Navigation);

let main = Container::new(3);
main.set_role(AriaRole::Main);
```

### 2. Provide Text Alternatives

```rust
// Images
let mut image = Image::new(1, "photo.jpg");
image.set_alt_text("Team photo at conference");

// Icons
let mut icon = Icon::new(1, "warning");
icon.set_aria_label("Warning");

// Buttons with only icons
let mut button = Button::new(1, "");
button.set_icon("trash");
button.set_aria_label("Delete item");
```

### 3. Ensure Keyboard Access

```rust
// All interactive components should be keyboard accessible
button.set_keyboard_handler(|event| {
    match event.key_code {
        KeyCode::Enter | KeyCode::Space => {
            button.handle_click();
            true
        }
        _ => false
    }
});
```

### 4. Provide Clear Error Messages

```rust
let mut input = TextInput::new(1);
input.set_validator(|text| {
    if text.is_empty() {
        Err("Email is required".to_string())
    } else if !text.contains('@') {
        Err("Please enter a valid email address".to_string())
    } else {
        Ok(())
    }
});

input.set_on_validation_error(|error| {
    // Announce to screen readers
    ScreenReader::announce(error, AnnouncementPriority::Assertive);
});
```

### 5. Manage Focus Appropriately

```rust
// Move focus to error
if validation_failed {
    focus_manager.set_focus(first_error_input.id());
    ScreenReader::announce(
        "Form has errors. Please correct them and try again.",
        AnnouncementPriority::Assertive
    );
}
```

### 6. Use Sufficient Timeouts

```rust
// Give users enough time
let toast = Toast::new(1, "Item saved successfully");
toast.set_duration(Duration::from_secs(5)); // Not too short

// Or no timeout for important messages
toast.set_dismissible(true);  // User controlled
toast.set_auto_dismiss(false);
```

### 7. Support Text Scaling

```rust
// Use relative units
let layout = Layout::new()
    .with_width(Size::Fixed(Unit::rb(20.0)))  // 20x base size
    .with_height(Size::Fixed(Unit::rb(2.5))); // 2.5x base size

// Text scales with theme
label.set_font_size(theme.typography.base_size * 1.5);
```

## Accessibility Checklist

Use this checklist for each component:

- [ ] Appropriate ARIA role assigned
- [ ] Accessible name provided (label, aria-label, or aria-labelledby)
- [ ] Keyboard accessible (Tab, Enter, Space, Arrows, Escape)
- [ ] Visible focus indicator
- [ ] Sufficient color contrast (7:1 for text, 3:1 for UI)
- [ ] Not relying on color alone
- [ ] Screen reader announces state changes
- [ ] Error messages are clear and announced
- [ ] Supports text scaling
- [ ] Touch targets minimum 44x44 points
- [ ] Works with screen readers
- [ ] Supports reduced motion preferences

## Resources

### WCAG Guidelines

- [WCAG 2.1 Quick Reference](https://www.w3.org/WAI/WCAG21/quickref/)
- [Understanding WCAG](https://www.w3.org/WAI/WCAG21/Understanding/)
- [How to Meet WCAG](https://www.w3.org/WAI/WCAG21/quickref/)

### Tools

- [axe DevTools](https://www.deque.com/axe/) - Accessibility testing
- [WAVE](https://wave.webaim.org/) - Web accessibility evaluation
- [Color Contrast Analyzer](https://www.tpgi.com/color-contrast-checker/) - Check contrast ratios

### Screen Readers

- [NVDA](https://www.nvaccess.org/) - Free Windows screen reader
- [VoiceOver](https://www.apple.com/accessibility/voiceover/) - Built-in macOS/iOS
- [Orca](https://help.gnome.org/users/orca/stable/) - Linux screen reader
- [TalkBack](https://support.google.com/accessibility/android/answer/6283677) - Android

## See Also

- [Component Development](component-development.md) - Creating accessible components
- [Theming Guide](theming.md) - Ensuring theme accessibility
- [Testing Guide](testing.md) - Testing accessibility
- [WCAG AAA Compliance](../design/architecture/NFRs.md) - Requirements

---

[← Back to Guides](index.md)
