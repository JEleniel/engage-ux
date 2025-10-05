# Text Area Component

A multi-line text input component for longer text entry.

## Overview

The Text Area component allows users to enter multiple lines of text. It's ideal for comments, descriptions, messages, and other longer text content.

## Basic Usage

```rust
use engage_ux_components::TextArea;
use engage_ux_core::component::{Component, Rect};

// Create a text area
let mut textarea = TextArea::new(1);
textarea.set_placeholder("Enter your message...");
textarea.set_bounds(Rect::new(20.0, 20.0, 400.0, 150.0));

// Set dimensions in rows/cols
textarea.set_rows(5);
textarea.set_cols(50);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `value` | `String` | Current text content | `""` |
| `placeholder` | `String` | Placeholder text | `""` |
| `rows` | `usize` | Number of visible rows | `4` |
| `cols` | `usize` | Number of visible columns | `40` |
| `max_length` | `Option<usize>` | Maximum character count | `None` |
| `read_only` | `bool` | Read-only state | `false` |
| `wrap` | `bool` | Enable text wrapping | `true` |
| `resize` | `bool` | Allow manual resize | `true` |

## Methods

### Value Management

```rust
// Get value
let text = textarea.value();

// Set value
textarea.set_value("Hello\nWorld");

// Append text
textarea.append("More text");

// Clear
textarea.set_value("");
```

### Configuration

```rust
// Set dimensions
textarea.set_rows(10);
textarea.set_cols(80);

// Set max length
textarea.set_max_length(Some(500));

// Set read-only
textarea.set_read_only(true);

// Enable/disable wrapping
textarea.set_wrap(true);

// Enable/disable resize
textarea.set_resize(true);
```

### Styling

```rust
use engage_ux_core::color::Color;

textarea.set_color(Color::from_hex("#000000").unwrap());
textarea.set_background_color(Color::from_hex("#FFFFFF").unwrap());
textarea.set_border_color(Color::from_hex("#CCCCCC").unwrap());
textarea.set_font_size(14.0);
```

### Events

```rust
textarea.set_on_change(|event| {
    println!("Text changed");
});

textarea.set_on_focus(|event| {
    println!("Focused");
});

textarea.set_on_blur(|event| {
    println!("Lost focus");
});
```

## Examples

### Comment Box

```rust
let mut comment = TextArea::new(1);
comment.set_placeholder("Write a comment...");
comment.set_rows(4);
comment.set_cols(60);
comment.set_max_length(Some(500));

// Show character count
let remaining = 500 - comment.value().len();
println!("{} characters remaining", remaining);
```

### Message Composer

```rust
struct MessageComposer {
    textarea: TextArea,
    max_length: usize,
}

impl MessageComposer {
    fn new(id: u64) -> Self {
        let mut textarea = TextArea::new(id);
        textarea.set_placeholder("Type your message...");
        textarea.set_rows(6);
        textarea.set_max_length(Some(1000));
        
        Self {
            textarea,
            max_length: 1000,
        }
    }
    
    fn get_character_count(&self) -> usize {
        self.textarea.value().len()
    }
    
    fn get_remaining_characters(&self) -> usize {
        self.max_length - self.get_character_count()
    }
    
    fn is_valid(&self) -> bool {
        let text = self.textarea.value();
        !text.trim().is_empty()
    }
}
```

### Read-Only Display

```rust
let mut display = TextArea::new(1);
display.set_value("This is read-only content\nthat spans multiple lines.");
display.set_read_only(true);
display.set_rows(10);
display.set_resize(false);
```

### Form Field

```rust
struct TextAreaField {
    label: Label,
    textarea: TextArea,
    help_text: Option<String>,
}

impl TextAreaField {
    fn new(id: u64, label: &str, placeholder: &str) -> Self {
        let label_comp = Label::new(id, label);
        
        let mut textarea = TextArea::new(id + 1);
        textarea.set_placeholder(placeholder);
        textarea.set_aria_labelled_by(label_comp.id());
        
        Self {
            label: label_comp,
            textarea,
            help_text: None,
        }
    }
    
    fn set_help_text(&mut self, text: &str) {
        self.help_text = Some(text.to_string());
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.textarea.value().trim().is_empty() {
            Err("This field is required".to_string())
        } else {
            Ok(())
        }
    }
}
```

### Auto-Expanding Textarea

```rust
struct AutoExpandingTextArea {
    textarea: TextArea,
    min_rows: usize,
    max_rows: usize,
}

impl AutoExpandingTextArea {
    fn new(id: u64, min_rows: usize, max_rows: usize) -> Self {
        let mut textarea = TextArea::new(id);
        textarea.set_rows(min_rows);
        
        Self {
            textarea,
            min_rows,
            max_rows,
        }
    }
    
    fn update_height(&mut self) {
        let line_count = self.textarea.value().lines().count();
        let rows = line_count.max(self.min_rows).min(self.max_rows);
        self.textarea.set_rows(rows);
    }
}
```

## Validation

### Required Field

```rust
fn validate_required(textarea: &TextArea) -> Result<(), String> {
    if textarea.value().trim().is_empty() {
        Err("This field is required".to_string())
    } else {
        Ok(())
    }
}
```

### Length Validation

```rust
fn validate_length(textarea: &TextArea, min: usize, max: usize) -> Result<(), String> {
    let len = textarea.value().len();
    
    if len < min {
        Err(format!("Must be at least {} characters", min))
    } else if len > max {
        Err(format!("Must be no more than {} characters", max))
    } else {
        Ok(())
    }
}
```

### Word Count

```rust
fn count_words(textarea: &TextArea) -> usize {
    textarea.value()
        .split_whitespace()
        .count()
}

fn validate_word_count(textarea: &TextArea, max_words: usize) -> Result<(), String> {
    let words = count_words(textarea);
    
    if words > max_words {
        Err(format!("Must be no more than {} words", max_words))
    } else {
        Ok(())
    }
}
```

## Accessibility

Text areas are automatically accessible:

- **Role**: `textbox` with `multiline=true`
- **Label**: Via aria-labelledby or aria-label
- **Keyboard**: Standard text editing behavior
- **States**: Disabled, required states announced

```rust
// Associate with label
textarea.set_aria_labelled_by(label_id);

// Or provide label
textarea.set_aria_label("Message content");

// Provide description
textarea.set_aria_describedby(help_text_id);

// Set required
textarea.set_aria_required(true);
```

## Best Practices

### Appropriate Sizing

```rust
// Small textarea (comments)
textarea.set_rows(3);

// Medium textarea (messages)
textarea.set_rows(6);

// Large textarea (documents)
textarea.set_rows(15);
```

### Character Limits

```rust
// Set reasonable limits
textarea.set_max_length(Some(1000));

// Show character count
let count = textarea.value().len();
let max = 1000;
let label = format!("{}/{} characters", count, max);
```

### Placeholder Text

```rust
// Good: Helpful placeholder
textarea.set_placeholder("Describe the issue you're experiencing...");

// Avoid: Generic placeholder
textarea.set_placeholder("Enter text here");
```

### Validation Feedback

```rust
match validate_required(&textarea) {
    Ok(_) => {
        textarea.set_border_color(theme.colors.outline);
    }
    Err(msg) => {
        textarea.set_border_color(theme.colors.error);
        show_error(&msg);
    }
}
```

## Related Components

- [Text Input](text-input.md) - Single-line text input
- [Text Editor](text-editor.md) - Rich text editing
- [Label](label.md) - Text labels
- [Console](console.md) - Terminal-like output

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Validation Guide](../guides/validation.md) - Input validation
- [Accessibility](../guides/accessibility.md) - Accessible text areas

---

[← Back to Components](index.md)
