# Label Component

A simple text display component for showing static text or labels.

## Overview

The Label component is one of the most basic components in Engage UX. It displays text with customizable styling including font, size, color, and alignment.

## Basic Usage

```rust
use engage_ux_components::Label;
use engage_ux_core::component::{Component, Rect};

// Create a simple label
let mut label = Label::new(1, "Hello, World!");
label.set_bounds(Rect::new(100.0, 100.0, 200.0, 30.0));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `text` | `String` | Label text content | Required |
| `color` | `Color` | Text color | Theme dependent |
| `font_size` | `f32` | Font size in points | `16.0` |
| `font_weight` | `FontWeight` | Text weight | `Normal` |
| `alignment` | `TextAlign` | Text alignment | `Left` |
| `line_height` | `f32` | Line height multiplier | `1.5` |
| `wrap` | `bool` | Enable text wrapping | `true` |
| `max_lines` | `Option<usize>` | Maximum lines to display | `None` |

## Methods

### Text Content

```rust
// Get text
let text = label.text();

// Set text
label.set_text("New Text");
label.set_text(format!("Count: {}", count));
```

### Styling

```rust
use engage_ux_core::color::Color;

// Set color
label.set_color(Color::from_hex("#333333")?);

// Set font size
label.set_font_size(18.0);

// Set font weight
use engage_ux_core::typography::FontWeight;
label.set_font_weight(FontWeight::Bold);
```

### Alignment

```rust
use engage_ux_core::typography::TextAlign;

// Left align (default)
label.set_alignment(TextAlign::Left);

// Center align
label.set_alignment(TextAlign::Center);

// Right align
label.set_alignment(TextAlign::Right);

// Justify
label.set_alignment(TextAlign::Justify);
```

### Text Wrapping

```rust
// Enable wrapping (default)
label.set_wrap(true);

// Disable wrapping (truncate with ellipsis)
label.set_wrap(false);

// Limit lines
label.set_max_lines(Some(3)); // Show max 3 lines
```

### Line Height

```rust
// Set line height (multiplier of font size)
label.set_line_height(1.5); // 1.5x font size
label.set_line_height(2.0);  // 2x font size (more spacious)
```

## Examples

### Simple Label

```rust
let mut label = Label::new(1, "Welcome!");
label.set_bounds(Rect::new(10.0, 10.0, 200.0, 30.0));
label.set_font_size(20.0);
```

### Themed Label

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut label = Label::new(1, "Themed Label");
label.set_color(theme.colors.text_primary);
label.set_font_size(theme.typography.base_size);
```

### Form Label

```rust
// Label for an input field
let mut email_label = Label::new(1, "Email Address:");
email_label.set_bounds(Rect::new(20.0, 100.0, 150.0, 25.0));
email_label.set_font_weight(FontWeight::SemiBold);
email_label.set_color(theme.colors.text_primary);

// Associated input
let mut email_input = TextInput::new(2);
email_input.set_bounds(Rect::new(20.0, 130.0, 300.0, 40.0));
email_input.set_aria_labelled_by(email_label.id());
```

### Multi-line Label

```rust
let mut description = Label::new(1, 
    "This is a longer description that will wrap across multiple lines. \
     The label will automatically wrap text to fit within its bounds."
);
description.set_bounds(Rect::new(20.0, 20.0, 300.0, 100.0));
description.set_wrap(true);
description.set_alignment(TextAlign::Left);
description.set_line_height(1.6);
```

### Centered Title

```rust
let mut title = Label::new(1, "Application Title");
title.set_bounds(Rect::new(0.0, 20.0, 800.0, 50.0));
title.set_font_size(28.0);
title.set_font_weight(FontWeight::Bold);
title.set_alignment(TextAlign::Center);
title.set_color(theme.colors.primary);
```

### Status Label

```rust
let mut status = Label::new(1, "Connected");
status.set_bounds(Rect::new(700.0, 10.0, 90.0, 25.0));
status.set_font_size(12.0);
status.set_color(theme.colors.success);
status.set_alignment(TextAlign::Right);
```

### Truncated Label

```rust
let mut path = Label::new(1, "/very/long/file/path/that/should/truncate.txt");
path.set_bounds(Rect::new(20.0, 20.0, 200.0, 25.0));
path.set_wrap(false); // Don't wrap
path.set_truncate_with_ellipsis(true); // Show "..." at end
```

### Dynamic Label

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

let label = Arc::new(RwLock::new(Label::new(1, "Count: 0")));

// Update label text from async context
let label_clone = label.clone();
tokio::spawn(async move {
    for i in 1..=10 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let mut label = label_clone.write().await;
        label.set_text(format!("Count: {}", i));
    }
});
```

## Font Weights

Available font weights:

```rust
pub enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    Normal,     // 400 (default)
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}
```

Example:

```rust
heading.set_font_weight(FontWeight::Bold);
body.set_font_weight(FontWeight::Normal);
caption.set_font_weight(FontWeight::Light);
```

## Text Alignment

Available alignment options:

```rust
pub enum TextAlign {
    Left,    // Align to left edge
    Center,  // Center horizontally
    Right,   // Align to right edge
    Justify, // Justify text
}
```

## Typography Scale

Use the theme's typography scale for consistent sizing:

```rust
let theme = Theme::dark();

// Small text
label.set_font_size(theme.typography.sizes["sm"]); // 14px

// Normal text
label.set_font_size(theme.typography.base_size); // 16px

// Large text
label.set_font_size(theme.typography.sizes["lg"]); // 20px

// Heading
label.set_font_size(theme.typography.sizes["2xl"]); // 30px
```

## Accessibility

Labels automatically include accessibility features:

- **Role**: `text` or `label`
- **Label**: Text content is the accessible name
- **Focus**: Not focusable by default (static content)

### Form Labels

For form labels, associate with input:

```rust
let label = Label::new(1, "Username:");
let mut input = TextInput::new(2);
input.set_aria_labelled_by(label.id());
```

### Screen Reader Announcements

For dynamic labels that convey important information:

```rust
use engage_ux_core::accessibility::{ScreenReader, AnnouncementPriority};

// Update label
status_label.set_text("Processing complete");

// Announce to screen reader
ScreenReader::announce(
    "Processing complete",
    AnnouncementPriority::Polite
);
```

## Best Practices

### Clear and Concise

```rust
// Good
let label = Label::new(1, "Email:");

// Avoid
let label = Label::new(1, "Please enter your email address below:");
```

### Appropriate Sizing

```rust
// Body text
body.set_font_size(16.0);

// Captions and notes
caption.set_font_size(12.0);

// Headings
heading.set_font_size(24.0);
```

### Color Contrast

Ensure sufficient contrast (WCAG AAA: 7:1):

```rust
// Good contrast
label.set_color(Color::from_hex("#000000")?); // Black on white
label.set_color(Color::from_hex("#FFFFFF")?); // White on black

// Check contrast
let background = Color::from_hex("#FFFFFF")?;
let text = Color::from_hex("#333333")?;
assert!(text.contrast_ratio(&background) >= 7.0);
```

### Consistent Spacing

Use the theme's spacing scale:

```rust
// Gap between label and input
let gap = theme.spacing.sm; // 8px

label.set_bounds(Rect::new(x, y, width, height));
input.set_bounds(Rect::new(x, y + height + gap, width, input_height));
```

## Common Patterns

### Label-Input Pair

```rust
fn create_form_field(
    id_base: u64,
    label_text: &str,
    y: f32,
    theme: &Theme,
) -> (Label, TextInput) {
    let mut label = Label::new(id_base, label_text);
    label.set_bounds(Rect::new(20.0, y, 150.0, 25.0));
    label.set_font_weight(FontWeight::Medium);
    label.set_color(theme.colors.text_primary);
    
    let mut input = TextInput::new(id_base + 1);
    input.set_bounds(Rect::new(20.0, y + 30.0, 300.0, 40.0));
    input.set_aria_labelled_by(label.id());
    
    (label, input)
}

// Usage
let (email_label, email_input) = create_form_field(1, "Email:", 100.0, &theme);
let (password_label, password_input) = create_form_field(3, "Password:", 180.0, &theme);
```

### Status Indicator

```rust
struct StatusIndicator {
    label: Label,
}

impl StatusIndicator {
    fn new(id: ComponentId, theme: &Theme) -> Self {
        let mut label = Label::new(id, "Ready");
        label.set_font_size(12.0);
        label.set_alignment(TextAlign::Center);
        Self { label }
    }
    
    fn set_status(&mut self, status: &str, color: Color) {
        self.label.set_text(status);
        self.label.set_color(color);
    }
}

// Usage
let mut status = StatusIndicator::new(1, &theme);
status.set_status("Connected", theme.colors.success);
status.set_status("Connecting...", theme.colors.warning);
status.set_status("Disconnected", theme.colors.error);
```

## Related Components

- [Text](text.md) - Rich text with more formatting options
- [Button](button.md) - Clickable text button
- [Link](link.md) - Clickable navigation link
- [Text Input](text-input.md) - User text entry

## See Also

- [Typography Guide](../guides/typography.md) - Typography system
- [Theming Guide](../guides/theming.md) - Customizing appearance
- [Accessibility](../guides/accessibility.md) - Making labels accessible

---

[← Back to Components](index.md)
