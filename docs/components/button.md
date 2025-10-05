# Button Component

A clickable button component supporting multiple variants, icons, and loading states.

## Overview

The Button component is one of the most fundamental interactive elements in Engage UX. It provides a consistent way to trigger actions in your application.

## Basic Usage

```rust
use engage_ux_components::Button;
use engage_ux_core::component::{Component, Rect};

// Create a simple button
let mut button = Button::new(1, "Click Me");
button.set_bounds(Rect::new(100.0, 100.0, 120.0, 40.0));

// Add a click handler
button.set_on_click(|event| {
    println!("Button clicked!");
});
```

## Variants

Buttons come in several visual variants:

### Primary Button

The main call-to-action button, typically used for the primary action in a view.

```rust
use engage_ux_components::ButtonVariant;

let mut button = Button::new(1, "Save");
button.set_variant(ButtonVariant::Primary);
```

### Secondary Button

For secondary actions.

```rust
button.set_variant(ButtonVariant::Secondary);
```

### Outlined Button

Button with just an outline, no fill.

```rust
button.set_variant(ButtonVariant::Outlined);
```

### Text Button

Minimal button with just text, no background or border.

```rust
button.set_variant(ButtonVariant::Text);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `text` | `String` | Button label text | Required |
| `variant` | `ButtonVariant` | Visual style | `Primary` |
| `background_color` | `Color` | Background color | Theme dependent |
| `text_color` | `Color` | Text color | Theme dependent |
| `border_color` | `Color` | Border color | Theme dependent |
| `border_width` | `f32` | Border thickness | `1.0` |
| `border_radius` | `f32` | Corner rounding | `4.0` |
| `padding` | `(f32, f32)` | Horizontal, vertical padding | `(16.0, 8.0)` |
| `disabled` | `bool` | Disabled state | `false` |
| `loading` | `bool` | Loading state | `false` |

## Methods

### Text

```rust
// Get button text
let text = button.text();

// Set button text
button.set_text("New Text");
```

### Variant

```rust
// Get button variant
let variant = button.variant();

// Set button variant
button.set_variant(ButtonVariant::Secondary);
```

### Colors

```rust
use engage_ux_core::color::Color;

// Set background color
button.set_background_color(Color::from_hex("#007AFF").unwrap());

// Set text color
button.set_text_color(Color::from_hex("#FFFFFF").unwrap());

// Set border color
button.set_border_color(Color::from_hex("#007AFF").unwrap());
```

### Styling

```rust
// Set border width
button.set_border_width(2.0);

// Set border radius
button.set_border_radius(8.0);

// Set padding
button.set_padding(20.0, 10.0);
```

### State

```rust
// Disable button
button.set_enabled(false);

// Check if enabled
if button.is_enabled() {
    // Button can be clicked
}

// Set loading state
button.set_loading(true);

// Check loading state
if button.is_loading() {
    // Show loading indicator
}
```

### Events

```rust
use engage_ux_core::events::Event;

// Set click handler
button.set_on_click(|event: &Event| {
    println!("Button {} clicked!", event.target_id);
});

// Set hover handler
button.set_on_hover(|is_hovering: bool| {
    if is_hovering {
        println!("Mouse entered button");
    } else {
        println!("Mouse left button");
    }
});
```

## Examples

### Themed Button

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut button = Button::new(1, "Themed Button");

// Apply theme colors
button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
button.set_border_radius(theme.borders.radius);
```

### Icon Button

```rust
// Button with icon (icon implementation may vary)
let mut button = Button::new(1, "Save");
// Set icon property when available
```

### Loading Button

```rust
let mut submit_button = Button::new(1, "Submit");

submit_button.set_on_click(|_| {
    // Set loading state
    submit_button.set_loading(true);
    submit_button.set_enabled(false);
    
    // Perform async operation
    // ... 
    
    // Reset state when done
    submit_button.set_loading(false);
    submit_button.set_enabled(true);
});
```

### Disabled Button

```rust
let mut button = Button::new(1, "Disabled");
button.set_enabled(false);

// Visually indicates disabled state
// Click events won't fire
```

## Accessibility

Buttons automatically include accessibility features:

- **Role**: `button`
- **Label**: Button text content
- **Keyboard**: Activates on `Enter` or `Space`
- **Focus**: Can receive keyboard focus
- **States**: Disabled state is announced

```rust
// Accessibility is built-in, but you can enhance it
button.set_aria_label("Submit form");
button.set_aria_describedby("form-help-text");
```

## Best Practices

### Clear Labels

Use clear, action-oriented labels:

```rust
// Good
let save_button = Button::new(1, "Save Changes");
let cancel_button = Button::new(2, "Cancel");

// Avoid
let ok_button = Button::new(3, "OK");
let button = Button::new(4, "Click");
```

### Visual Hierarchy

Use variants to establish hierarchy:

```rust
// Primary action - stands out
let save = Button::new(1, "Save");
save.set_variant(ButtonVariant::Primary);

// Secondary action - less prominent
let cancel = Button::new(2, "Cancel");
cancel.set_variant(ButtonVariant::Secondary);

// Tertiary action - minimal
let skip = Button::new(3, "Skip");
skip.set_variant(ButtonVariant::Text);
```

### Loading States

Show feedback for async operations:

```rust
submit_button.set_on_click(|_| {
    submit_button.set_loading(true);
    
    // Perform operation
    async_operation().await;
    
    submit_button.set_loading(false);
});
```

### Sizing

Size buttons appropriately for touch targets (minimum 44x44 points):

```rust
button.set_bounds(Rect::new(0.0, 0.0, 120.0, 44.0));
```

## Related Components

- [Link](link.md) - For navigation actions
- [Toggle](toggle.md) - For on/off states
- [Checkbox](checkbox.md) - For multi-select options
- [Radio Button](radio.md) - For single-select options

## See Also

- [Event System](../guides/events.md) - Understanding events
- [Theming Guide](../guides/theming.md) - Customizing appearance
- [Accessibility](../guides/accessibility.md) - WCAG compliance

---

[← Back to Components](index.md)
