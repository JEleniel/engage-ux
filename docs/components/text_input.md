# Text Input Component

A single-line text input field for user text entry.

## Overview

The Text Input component allows users to enter a single line of text. It supports various input types (text, password, email, etc.), validation, and styling options.

## Basic Usage

```rust
use engage_ux_components::TextInput;
use engage_ux_core::component::{Component, Rect};

// Create a text input
let mut input = TextInput::new(1);
input.set_placeholder("Enter your name");
input.set_bounds(Rect::new(20.0, 20.0, 300.0, 40.0));
```

## Input Types

```rust
use engage_ux_components::InputType;

// Plain text (default)
input.set_input_type(InputType::Text);

// Password (hidden characters)
input.set_input_type(InputType::Password);

// Email (email validation)
input.set_input_type(InputType::Email);

// Number (numeric input)
input.set_input_type(InputType::Number);

// Telephone
input.set_input_type(InputType::Tel);

// URL
input.set_input_type(InputType::Url);

// Search
input.set_input_type(InputType::Search);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `value` | `String` | Current input value | `""` |
| `placeholder` | `String` | Placeholder text | `""` |
| `input_type` | `InputType` | Type of input | `Text` |
| `max_length` | `Option<usize>` | Maximum character count | `None` |
| `read_only` | `bool` | Read-only state | `false` |
| `disabled` | `bool` | Disabled state | `false` |
| `required` | `bool` | Required field | `false` |
| `autocomplete` | `bool` | Enable autocomplete | `true` |

## Methods

### Value Management

```rust
// Get value
let text = input.value();

// Set value
input.set_value("Hello");

// Clear value
input.set_value("");
```

### Configuration

```rust
// Set placeholder
input.set_placeholder("Enter email");

// Set input type
input.set_input_type(InputType::Email);

// Set max length
input.set_max_length(Some(100));

// Set read-only
input.set_read_only(true);

// Set disabled
input.set_disabled(true);

// Set required
input.set_required(true);

// Set autocomplete
input.set_autocomplete(false);
```

### Styling

```rust
use engage_ux_core::color::Color;

// Set colors
input.set_color(Color::from_hex("#000000").unwrap());
input.set_background_color(Color::from_hex("#FFFFFF").unwrap());
input.set_border_color(Color::from_hex("#CCCCCC").unwrap());
input.set_focus_border_color(Color::from_hex("#1976D2").unwrap());

// Set font size
input.set_font_size(16.0);
```

### Events

```rust
// Handle changes
input.set_on_change(|event| {
    println!("Input changed");
});

// Handle focus
input.set_on_focus(|event| {
    println!("Input focused");
});

// Handle blur
input.set_on_blur(|event| {
    println!("Input lost focus");
});
```

## Examples

### Email Input

```rust
let mut email = TextInput::new(1);
email.set_placeholder("name@example.com");
email.set_input_type(InputType::Email);
email.set_required(true);
email.set_autocomplete(true);
```

### Password Input

```rust
let mut password = TextInput::new(1);
password.set_placeholder("Enter password");
password.set_input_type(InputType::Password);
password.set_required(true);
password.set_min_length(Some(8));
password.set_autocomplete(false);
```

### Search Input

```rust
let mut search = TextInput::new(1);
search.set_placeholder("Search...");
search.set_input_type(InputType::Search);
search.set_autocomplete(true);

search.set_on_change(|_| {
    // Perform search
});
```

### Character Limit

```rust
let mut bio = TextInput::new(1);
bio.set_placeholder("Bio (max 150 characters)");
bio.set_max_length(Some(150));

// Show character count
let remaining = 150 - bio.value().len();
println!("{} characters remaining", remaining);
```

### Form Field

```rust
struct FormField {
    label: Label,
    input: TextInput,
}

impl FormField {
    fn new(id: u64, label_text: &str, placeholder: &str) -> Self {
        let label = Label::new(id, label_text);
        
        let mut input = TextInput::new(id + 1);
        input.set_placeholder(placeholder);
        input.set_aria_labelled_by(label.id());
        
        Self { label, input }
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.input.is_required() && self.input.value().is_empty() {
            Err("This field is required".to_string())
        } else {
            Ok(())
        }
    }
}
```

### Validation

```rust
fn validate_email(input: &TextInput) -> Result<(), String> {
    let email = input.value();
    
    if email.is_empty() {
        return Err("Email is required".to_string());
    }
    
    if !email.contains('@') {
        return Err("Invalid email address".to_string());
    }
    
    Ok(())
}
```

### Themed Input

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut input = TextInput::new(1);

input.set_color(theme.colors.text_primary);
input.set_background_color(theme.colors.surface);
input.set_border_color(theme.colors.outline);
input.set_focus_border_color(theme.colors.primary);
input.set_font_size(theme.typography.base_size);
```

## Validation Patterns

### Required Field

```rust
input.set_required(true);

fn validate_required(input: &TextInput) -> Result<(), String> {
    if input.is_required() && input.value().is_empty() {
        Err("This field is required".to_string())
    } else {
        Ok(())
    }
}
```

### Pattern Matching

```rust
fn validate_pattern(input: &TextInput, pattern: &str) -> Result<(), String> {
    // Implement regex validation
    Ok(())
}
```

### Custom Validation

```rust
fn validate_username(input: &TextInput) -> Result<(), String> {
    let username = input.value();
    
    if username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username can only contain letters, numbers, and underscores".to_string());
    }
    
    Ok(())
}
```

## Accessibility

Text inputs are automatically accessible:

- **Role**: `textbox`
- **Label**: Via aria-labelledby or aria-label
- **Keyboard**: Standard text input behavior
- **States**: Disabled, required, invalid states announced

```rust
// Associate with label
input.set_aria_labelled_by(label_id);

// Or provide direct label
input.set_aria_label("Email address");

// Provide description
input.set_aria_describedby(description_id);

// Set invalid state
input.set_aria_invalid(true);
```

## Best Practices

### Clear Placeholders

```rust
// Good: Descriptive placeholder
input.set_placeholder("name@example.com");

// Avoid: Redundant with label
input.set_placeholder("Email");
```

### Appropriate Input Types

```rust
// Use specific types for better UX
email_input.set_input_type(InputType::Email);
phone_input.set_input_type(InputType::Tel);
url_input.set_input_type(InputType::Url);
```

### Validation Feedback

```rust
// Provide clear error messages
match validate_email(&input) {
    Ok(_) => {
        input.set_border_color(theme.colors.success);
    }
    Err(msg) => {
        input.set_border_color(theme.colors.error);
        show_error_message(&msg);
    }
}
```

### Character Limits

```rust
// Set reasonable limits
input.set_max_length(Some(100));

// Show remaining characters
let remaining = max_length - input.value().len();
```

## Related Components

- [Text Area](text-area.md) - Multi-line text input
- [Select](select.md) - Dropdown selection
- [Checkbox](checkbox.md) - Boolean input
- [Label](label.md) - Input labels

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Validation Guide](../guides/validation.md) - Input validation
- [Accessibility](../guides/accessibility.md) - Accessible inputs

---

[← Back to Components](index.md)
