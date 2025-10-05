# Quick Reference

Quick reference guide for common Engage UX tasks and patterns.

## Installation

```toml
[dependencies]
engage-ux-core = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-components = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-themes = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-oal = { git = "https://github.com/JEleniel/engage-ux" }
tokio = { version = "1.0", features = ["full"] }
```

## Basic Setup

```rust
use engage_ux_components::{Button, Container, Label};
use engage_ux_core::component::{Component, Rect};
use engage_ux_themes::Theme;

#[tokio::main]
async fn main() {
    let theme = Theme::dark();
    let mut container = Container::new(1);
    // Add components...
}
```

## Common Components

### Button

```rust
let mut button = Button::new(1, "Click Me");
button.set_bounds(Rect::new(x, y, width, height));
button.set_on_click(|_| println!("Clicked!"));
```

### Label

```rust
let mut label = Label::new(1, "Text");
label.set_font_size(18.0);
label.set_color(Color::from_hex("#333333")?);
```

### Text Input

```rust
let mut input = TextInput::new(1);
input.set_placeholder("Enter text...");
input.set_on_change(|text| println!("Text: {}", text));
```

### Checkbox

```rust
let mut checkbox = Checkbox::new(1, "Accept");
checkbox.set_on_change(|checked| println!("Checked: {}", checked));
```

## Colors

```rust
use engage_ux_core::color::Color;

// Hex
let color = Color::from_hex("#FF5733")?;

// RGB
let color = Color::from_rgb(255, 87, 51);

// HSL
let color = Color::from_hsl(9.0, 100.0, 60.0);

// With alpha
let color = Color::from_rgba(255, 87, 51, 128);
```

## Themes

### Load Theme

```rust
use engage_ux_themes::Theme;

let dark = Theme::dark();
let light = Theme::light();
let custom = Theme::from_json_file("theme.json")?;
```

### Apply Theme

```rust
button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
label.set_color(theme.colors.text_primary);
```

### Theme Colors

```rust
theme.colors.primary
theme.colors.secondary
theme.colors.background
theme.colors.surface
theme.colors.error
theme.colors.success
theme.colors.warning
theme.colors.text_primary
theme.colors.text_secondary
```

## Events

### Button Click

```rust
button.set_on_click(|event| {
    println!("Clicked!");
});
```

### Input Change

```rust
input.set_on_change(|text| {
    println!("Text: {}", text);
});
```

### Checkbox Toggle

```rust
checkbox.set_on_change(|checked| {
    println!("Checked: {}", checked);
});
```

## Layout

### Absolute Positioning

```rust
component.set_bounds(Rect::new(x, y, width, height));
```

### Using Layout System

```rust
use engage_ux_core::layout::{Layout, Unit, Size};

let layout = Layout::new()
    .with_left(Unit::pixels(10.0))
    .with_top(Unit::pixels(20.0))
    .with_width(Size::Fixed(Unit::percent(50.0)))
    .with_height(Size::Fill);

let bounds = layout.calculate_bounds(parent_w, parent_h, base, inherited);
component.set_bounds(bounds);
```

## Container

```rust
let mut container = Container::new(1);
container.add_child(Box::new(button));
container.add_child(Box::new(label));
```

## Keyboard Input

```rust
use engage_ux_core::input::{InputHandler, KeyCode};

impl InputHandler for MyComponent {
    fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
        match event.key_code {
            KeyCode::Enter => {
                // Handle Enter
                true
            }
            _ => false
        }
    }
}
```

## Accessibility

```rust
use engage_ux_core::accessibility::{AccessibilityProps, AriaRole};

let props = AccessibilityProps::new(id)
    .with_role(AriaRole::Button)
    .with_label("Submit form")
    .with_description("Submits the form data");
```

## Animation

```rust
use engage_ux_core::animation::{Animation, AnimationType, EasingFunction};

let fade = Animation::new(
    AnimationType::Fade { from: 0.0, to: 1.0 },
    Duration::from_millis(300),
    EasingFunction::EaseOut
);
```

## Testing

```rust
#[test]
fn test_component() {
    let component = MyComponent::new(1);
    assert_eq!(component.id(), 1);
}

#[tokio::test]
async fn test_async() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

## Common Patterns

### Form Field

```rust
let mut label = Label::new(1, "Email:");
label.set_bounds(Rect::new(20.0, 100.0, 150.0, 25.0));

let mut input = TextInput::new(2);
input.set_bounds(Rect::new(20.0, 130.0, 300.0, 40.0));
input.set_aria_labelled_by(label.id());
```

### Button Group

```rust
let mut primary = Button::new(1, "Save");
primary.set_variant(ButtonVariant::Primary);

let mut secondary = Button::new(2, "Cancel");
secondary.set_variant(ButtonVariant::Secondary);
```

### Dialog

```rust
let mut modal = Modal::new(1, "Confirm");
modal.set_content("Are you sure?");
modal.add_button("Yes", |_| { /* confirm */ });
modal.add_button("No", |_| { /* cancel */ });
```

## Cargo Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test
cargo test --all

# Run example
cargo run --example basic_components -p engage-ux-components

# Documentation
cargo doc --all --no-deps --open

# Lint
cargo clippy --all -- -D warnings

# Format
cargo fmt --all
```

## Component IDs

```rust
// Use unique IDs for each component
let button = Button::new(1, "Button");
let label = Label::new(2, "Label");
let input = TextInput::new(3);
```

## Error Handling

```rust
// Result types
match Color::from_hex("#FF5733") {
    Ok(color) => { /* use color */ }
    Err(e) => { /* handle error */ }
}

// Or with ?
let color = Color::from_hex("#FF5733")?;
```

## Thread Safety

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

let component = Arc::new(RwLock::new(component));
let clone = component.clone();

tokio::spawn(async move {
    let mut c = clone.write().await;
    c.set_text("Updated");
});
```

## Resources

- [Full Documentation](index.md)
- [Getting Started Guide](getting-started.md)
- [Component Reference](components/index.md)
- [API Reference](api/index.md)
- [Examples](examples/index.md)

---

[← Back to Documentation](index.md)
