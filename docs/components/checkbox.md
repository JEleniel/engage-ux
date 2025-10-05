# Checkbox Component

A checkbox component for selecting one or more options from a set.

## Overview

The Checkbox component allows users to select or deselect options. It supports three states: unchecked, checked, and indeterminate (useful for "select all" functionality).

## Basic Usage

```rust
use engage_ux_components::Checkbox;
use engage_ux_core::component::{Component, Rect};

// Create a simple checkbox
let mut checkbox = Checkbox::new(1, "Accept terms and conditions");
checkbox.set_bounds(Rect::new(20.0, 20.0, 200.0, 24.0));

// Add a change handler
checkbox.set_on_change(|event| {
    println!("Checkbox state changed!");
});
```

## States

Checkboxes have three possible states:

### Unchecked

The default state, indicating the option is not selected.

```rust
use engage_ux_components::CheckboxState;

let mut checkbox = Checkbox::new(1, "Option");
assert_eq!(checkbox.state(), CheckboxState::Unchecked);
```

### Checked

Indicates the option is selected.

```rust
checkbox.set_checked(true);
assert!(checkbox.is_checked());
assert_eq!(checkbox.state(), CheckboxState::Checked);
```

### Indeterminate

A special state indicating partial selection, commonly used in hierarchical lists.

```rust
checkbox.set_indeterminate(true);
assert!(checkbox.is_indeterminate());
assert_eq!(checkbox.state(), CheckboxState::Indeterminate);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `state` | `CheckboxState` | Current state | `Unchecked` |
| `label` | `String` | Checkbox label text | Required |
| `disabled` | `bool` | Disabled state | `false` |
| `color` | `Color` | Checkbox color when checked | `#1976D2` |
| `check_color` | `Color` | Check mark color | `#FFFFFF` |
| `label_color` | `Color` | Label text color | `#000000` |
| `size` | `f32` | Checkbox size in pixels | `20.0` |

## Methods

### State Management

```rust
// Get current state
let state = checkbox.state();

// Set state directly
checkbox.set_state(CheckboxState::Checked);

// Check if checked
if checkbox.is_checked() {
    println!("Checkbox is checked");
}

// Set checked state
checkbox.set_checked(true);
checkbox.set_checked(false);

// Toggle between checked and unchecked
checkbox.toggle();

// Set indeterminate state
checkbox.set_indeterminate(true);
```

### Label

```rust
// Get label
let label = checkbox.label();

// Set label
checkbox.set_label("New label text");
```

### Styling

```rust
use engage_ux_core::color::Color;

// Set checkbox color (when checked)
checkbox.set_color(Color::from_hex("#4CAF50").unwrap());

// Set check mark color
checkbox.set_check_color(Color::from_hex("#FFFFFF").unwrap());

// Set label color
checkbox.set_label_color(Color::from_hex("#333333").unwrap());

// Set size
checkbox.set_size(24.0); // Larger checkbox
```

### State Control

```rust
// Disable checkbox
checkbox.set_disabled(true);

// Check if disabled
if checkbox.is_disabled() {
    println!("Checkbox is disabled");
}
```

### Events

```rust
use engage_ux_core::events::Event;

// Set change handler
checkbox.set_on_change(|event: &Event| {
    println!("Checkbox {} changed", event.target_id);
});
```

## Examples

### Simple Checkbox

```rust
let mut agree = Checkbox::new(1, "I agree to the terms");
agree.set_bounds(Rect::new(20.0, 100.0, 200.0, 24.0));

agree.set_on_change(|_| {
    println!("User agreed to terms");
});
```

### Styled Checkbox

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut checkbox = Checkbox::new(1, "Subscribe to newsletter");

// Apply theme colors
checkbox.set_color(theme.colors.primary);
checkbox.set_check_color(theme.colors.on_primary);
checkbox.set_label_color(theme.colors.text_primary);
checkbox.set_size(22.0);
```

### Checkbox Group

```rust
struct CheckboxGroup {
    checkboxes: Vec<Checkbox>,
}

impl CheckboxGroup {
    fn new(options: &[&str]) -> Self {
        let checkboxes = options
            .iter()
            .enumerate()
            .map(|(i, label)| Checkbox::new(i as u64, *label))
            .collect();
        
        Self { checkboxes }
    }
    
    fn get_selected(&self) -> Vec<usize> {
        self.checkboxes
            .iter()
            .enumerate()
            .filter(|(_, cb)| cb.is_checked())
            .map(|(i, _)| i)
            .collect()
    }
}

// Usage
let mut group = CheckboxGroup::new(&[
    "Option 1",
    "Option 2",
    "Option 3",
]);

// Later, get selected indices
let selected = group.get_selected();
println!("Selected: {:?}", selected);
```

### Select All with Indeterminate

```rust
struct SelectAllCheckbox {
    parent: Checkbox,
    children: Vec<Checkbox>,
}

impl SelectAllCheckbox {
    fn update_parent_state(&mut self) {
        let checked_count = self.children.iter().filter(|c| c.is_checked()).count();
        
        if checked_count == 0 {
            self.parent.set_checked(false);
        } else if checked_count == self.children.len() {
            self.parent.set_checked(true);
        } else {
            self.parent.set_indeterminate(true);
        }
    }
    
    fn toggle_all(&mut self) {
        let new_state = !self.parent.is_checked();
        for child in &mut self.children {
            child.set_checked(new_state);
        }
        self.parent.set_checked(new_state);
    }
}

// Usage
let mut select_all = SelectAllCheckbox {
    parent: Checkbox::new(0, "Select All"),
    children: vec![
        Checkbox::new(1, "Item 1"),
        Checkbox::new(2, "Item 2"),
        Checkbox::new(3, "Item 3"),
    ],
};

// After user interaction with children
select_all.update_parent_state();
```

### Form Integration

```rust
struct FormData {
    subscribe_newsletter: bool,
    accept_terms: bool,
    enable_notifications: bool,
}

impl FormData {
    fn from_checkboxes(
        newsletter: &Checkbox,
        terms: &Checkbox,
        notifications: &Checkbox,
    ) -> Self {
        Self {
            subscribe_newsletter: newsletter.is_checked(),
            accept_terms: terms.is_checked(),
            enable_notifications: notifications.is_checked(),
        }
    }
    
    fn is_valid(&self) -> bool {
        // Must accept terms
        self.accept_terms
    }
}
```

### Disabled Checkbox

```rust
let mut checkbox = Checkbox::new(1, "Currently unavailable");
checkbox.set_disabled(true);

// Style to indicate disabled state
checkbox.set_color(Color::from_hex("#CCCCCC").unwrap());
checkbox.set_label_color(Color::from_hex("#999999").unwrap());
```

## Accessibility

Checkboxes automatically include accessibility features:

- **Role**: `checkbox`
- **Label**: Checkbox label text
- **Keyboard**: Activates on `Space`
- **Focus**: Can receive keyboard focus
- **States**: Checked/unchecked/indeterminate states are announced

```rust
// Accessibility is built-in
checkbox.set_aria_label("Accept terms and conditions");
checkbox.set_aria_describedby("terms-description");
```

### Screen Reader Support

```rust
// Provide additional context for screen readers
checkbox.set_aria_label("Subscribe to monthly newsletter (optional)");

// Link to description
checkbox.set_aria_describedby("newsletter-info");
```

## Best Practices

### Clear Labels

Use descriptive, action-oriented labels:

```rust
// Good
let checkbox = Checkbox::new(1, "Send me promotional emails");

// Avoid
let checkbox = Checkbox::new(1, "Emails");
```

### Appropriate Sizing

Ensure checkboxes are large enough for easy interaction (minimum 20x20 pixels, 24x24 recommended):

```rust
checkbox.set_size(24.0); // Good for touch interfaces
```

### Logical Grouping

Group related checkboxes together:

```rust
// Notification preferences
let email = Checkbox::new(1, "Email notifications");
let sms = Checkbox::new(2, "SMS notifications");
let push = Checkbox::new(3, "Push notifications");
```

### Validation Feedback

Provide clear feedback for required checkboxes:

```rust
fn validate_form(terms_checkbox: &Checkbox) -> Result<(), String> {
    if !terms_checkbox.is_checked() {
        Err("You must accept the terms and conditions".to_string())
    } else {
        Ok(())
    }
}
```

### Indeterminate Usage

Use indeterminate state only for "select all" scenarios:

```rust
// Good: Select all with mixed children
parent_checkbox.set_indeterminate(true);

// Avoid: Using indeterminate for loading states
// Use a different component for loading indicators
```

## Common Patterns

### Checkbox List

```rust
fn create_checkbox_list(
    items: &[&str],
    start_y: f32,
    theme: &Theme,
) -> Vec<Checkbox> {
    items
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let mut checkbox = Checkbox::new(i as u64, *label);
            checkbox.set_bounds(Rect::new(
                20.0,
                start_y + (i as f32 * 35.0),
                200.0,
                24.0,
            ));
            checkbox.set_color(theme.colors.primary);
            checkbox.set_label_color(theme.colors.text_primary);
            checkbox
        })
        .collect()
}

// Usage
let items = vec!["Red", "Green", "Blue"];
let checkboxes = create_checkbox_list(&items, 100.0, &theme);
```

### Conditional Checkboxes

```rust
struct ConditionalCheckbox {
    parent: Checkbox,
    children: Vec<Checkbox>,
}

impl ConditionalCheckbox {
    fn update(&mut self) {
        // Enable children only if parent is checked
        let enabled = self.parent.is_checked();
        for child in &mut self.children {
            child.set_disabled(!enabled);
            if !enabled {
                child.set_checked(false);
            }
        }
    }
}
```

## Related Components

- [Radio Button](radio.md) - For mutually exclusive selections
- [Toggle](toggle.md) - For on/off states
- [Button](button.md) - For action triggers
- [Select](select.md) - For dropdown selections

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Accessibility](../guides/accessibility.md) - WCAG compliance
- [Theming Guide](../guides/theming.md) - Customizing appearance

---

[← Back to Components](index.md)
