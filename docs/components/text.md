# Text Component

A component for displaying formatted text with various styling options.

## Overview

The Text component provides more formatting capabilities than Label, including font weight variations, italic, and underline. It's ideal for displaying rich text content with emphasis.

## Basic Usage

```rust
use engage_ux_components::Text;
use engage_ux_core::component::{Component, Rect};

// Create a text component
let mut text = Text::new(1, "Hello, World!");
text.set_bounds(Rect::new(100.0, 100.0, 200.0, 30.0));
```

## Font Weights

Text supports six font weight options:

```rust
use engage_ux_components::FontWeight;

// Thin (lightest)
text.set_font_weight(FontWeight::Thin);

// Light
text.set_font_weight(FontWeight::Light);

// Regular (default)
text.set_font_weight(FontWeight::Regular);

// Medium
text.set_font_weight(FontWeight::Medium);

// Bold
text.set_font_weight(FontWeight::Bold);

// Black (heaviest)
text.set_font_weight(FontWeight::Black);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `content` | `String` | Text content | Required |
| `color` | `Color` | Text color | `#000000` |
| `font_size` | `f32` | Font size in points | `16.0` |
| `font_weight` | `FontWeight` | Font weight | `Regular` |
| `italic` | `bool` | Italic style | `false` |
| `underline` | `bool` | Underline style | `false` |

## Methods

### Content

```rust
// Get content
let content = text.content();

// Set content
text.set_content("New content");
text.set_content(format!("Count: {}", count));
```

### Styling

```rust
use engage_ux_core::color::Color;

// Set color
text.set_color(Color::from_hex("#333333").unwrap());

// Set font size
text.set_font_size(18.0);

// Set font weight
text.set_font_weight(FontWeight::Bold);

// Set italic
text.set_italic(true);

// Set underline
text.set_underline(true);
```

### Getters

```rust
// Get properties
let color = text.color();
let size = text.font_size();
let weight = text.font_weight();
let is_italic = text.is_italic();
let is_underline = text.is_underline();
```

## Examples

### Simple Text

```rust
let mut text = Text::new(1, "Welcome to Engage UX");
text.set_bounds(Rect::new(20.0, 20.0, 300.0, 40.0));
text.set_font_size(24.0);
text.set_font_weight(FontWeight::Bold);
```

### Emphasized Text

```rust
let mut emphasis = Text::new(1, "Important notice");
emphasis.set_font_weight(FontWeight::Bold);
emphasis.set_italic(true);
emphasis.set_color(Color::from_hex("#D32F2F").unwrap());
```

### Link-Style Text

```rust
let mut link_text = Text::new(1, "Click here for more information");
link_text.set_color(Color::from_hex("#1976D2").unwrap());
link_text.set_underline(true);
```

### Themed Text

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut text = Text::new(1, "Themed text");

// Apply theme
text.set_color(theme.colors.text_primary);
text.set_font_size(theme.typography.base_size);
```

### Paragraph Text

```rust
let mut paragraph = Text::new(1, 
    "This is a longer paragraph of text that demonstrates the text component. \
     It can contain multiple sentences and will wrap according to its bounds."
);
paragraph.set_bounds(Rect::new(20.0, 20.0, 400.0, 100.0));
paragraph.set_font_size(16.0);
paragraph.set_font_weight(FontWeight::Regular);
paragraph.set_color(Color::from_hex("#333333").unwrap());
```

### Heading Text

```rust
fn create_heading(id: u64, text: &str, level: u8, theme: &Theme) -> Text {
    let mut heading = Text::new(id, text);
    
    match level {
        1 => {
            heading.set_font_size(32.0);
            heading.set_font_weight(FontWeight::Bold);
        }
        2 => {
            heading.set_font_size(28.0);
            heading.set_font_weight(FontWeight::Bold);
        }
        3 => {
            heading.set_font_size(24.0);
            heading.set_font_weight(FontWeight::Medium);
        }
        _ => {
            heading.set_font_size(20.0);
            heading.set_font_weight(FontWeight::Medium);
        }
    }
    
    heading.set_color(theme.colors.text_primary);
    heading
}

// Usage
let h1 = create_heading(1, "Main Title", 1, &theme);
let h2 = create_heading(2, "Section Title", 2, &theme);
let h3 = create_heading(3, "Subsection Title", 3, &theme);
```

### Dynamic Text

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

let text = Arc::new(RwLock::new(Text::new(1, "Loading...")));

// Update from async context
let text_clone = text.clone();
tokio::spawn(async move {
    // Fetch data
    let data = fetch_data().await;
    
    // Update text
    let mut t = text_clone.write().await;
    t.set_content(data);
    t.set_font_weight(FontWeight::Regular);
});
```

### Status Text

```rust
enum Status {
    Success,
    Warning,
    Error,
    Info,
}

fn create_status_text(id: u64, message: &str, status: Status) -> Text {
    let mut text = Text::new(id, message);
    text.set_font_weight(FontWeight::Medium);
    
    let color = match status {
        Status::Success => Color::from_hex("#4CAF50").unwrap(),
        Status::Warning => Color::from_hex("#FF9800").unwrap(),
        Status::Error => Color::from_hex("#F44336").unwrap(),
        Status::Info => Color::from_hex("#2196F3").unwrap(),
    };
    
    text.set_color(color);
    text
}

// Usage
let success = create_status_text(1, "Operation completed", Status::Success);
let error = create_status_text(2, "Operation failed", Status::Error);
```

## Typography Hierarchy

Use consistent typography for better readability:

```rust
use engage_ux_themes::Theme;

let theme = Theme::light();

// Display text (largest)
let mut display = Text::new(1, "Display Text");
display.set_font_size(48.0);
display.set_font_weight(FontWeight::Bold);

// Heading 1
let mut h1 = Text::new(2, "Heading 1");
h1.set_font_size(32.0);
h1.set_font_weight(FontWeight::Bold);

// Heading 2
let mut h2 = Text::new(3, "Heading 2");
h2.set_font_size(24.0);
h2.set_font_weight(FontWeight::Bold);

// Body text
let mut body = Text::new(4, "Body text");
body.set_font_size(16.0);
body.set_font_weight(FontWeight::Regular);

// Caption (smallest)
let mut caption = Text::new(5, "Caption text");
caption.set_font_size(12.0);
caption.set_font_weight(FontWeight::Regular);
```

## Accessibility

Text components automatically include accessibility features:

- **Role**: `text`
- **Content**: Text content is the accessible name
- **Focus**: Not focusable by default (static content)

```rust
// For important text, ensure good contrast
let background = Color::from_hex("#FFFFFF").unwrap();
let text_color = Color::from_hex("#212121").unwrap();

// WCAG AAA requires 7:1 contrast for normal text
assert!(text_color.contrast_ratio(&background) >= 7.0);
```

## Best Practices

### Font Size Guidelines

```rust
// Minimum readable size
text.set_font_size(12.0); // Minimum for body text

// Standard sizes
text.set_font_size(14.0); // Small body text
text.set_font_size(16.0); // Standard body text
text.set_font_size(20.0); // Large body text

// Headings
text.set_font_size(24.0); // H3
text.set_font_size(28.0); // H2
text.set_font_size(32.0); // H1
```

### Weight Selection

```rust
// Headings: Bold or Black
heading.set_font_weight(FontWeight::Bold);

// Emphasis: Medium or Bold
emphasis.set_font_weight(FontWeight::Medium);

// Body text: Regular
body.set_font_weight(FontWeight::Regular);

// De-emphasized: Light
subtitle.set_font_weight(FontWeight::Light);
```

### Color Contrast

Ensure sufficient contrast for readability:

```rust
// Light backgrounds
text.set_color(Color::from_hex("#212121").unwrap()); // Very good contrast

// Dark backgrounds
text.set_color(Color::from_hex("#FFFFFF").unwrap()); // White text

// Check contrast
fn has_sufficient_contrast(fg: &Color, bg: &Color) -> bool {
    fg.contrast_ratio(bg) >= 4.5 // WCAG AA for normal text
}
```

### Combining Styles

```rust
// Good: Use weight for emphasis
text.set_font_weight(FontWeight::Bold);

// Good: Use italic for foreign words or titles
text.set_italic(true);

// Avoid: Too many styles at once
// Don't use bold + italic + underline together
```

## Common Patterns

### Rich Text Builder

```rust
struct RichTextBuilder {
    text: Text,
}

impl RichTextBuilder {
    fn new(id: u64, content: &str) -> Self {
        Self {
            text: Text::new(id, content),
        }
    }
    
    fn bold(mut self) -> Self {
        self.text.set_font_weight(FontWeight::Bold);
        self
    }
    
    fn italic(mut self) -> Self {
        self.text.set_italic(true);
        self
    }
    
    fn underline(mut self) -> Self {
        self.text.set_underline(true);
        self
    }
    
    fn size(mut self, size: f32) -> Self {
        self.text.set_font_size(size);
        self
    }
    
    fn color(mut self, color: Color) -> Self {
        self.text.set_color(color);
        self
    }
    
    fn build(self) -> Text {
        self.text
    }
}

// Usage
let text = RichTextBuilder::new(1, "Important!")
    .bold()
    .italic()
    .color(Color::from_hex("#D32F2F").unwrap())
    .build();
```

## Related Components

- [Label](label.md) - Simple text display
- [Button](button.md) - Clickable button with text
- [Link](link.md) - Clickable text link
- [Text Input](text-input.md) - User text entry

## See Also

- [Typography Guide](../guides/typography.md) - Typography system
- [Theming Guide](../guides/theming.md) - Customizing appearance
- [Accessibility](../guides/accessibility.md) - Making text accessible

---

[← Back to Components](index.md)
