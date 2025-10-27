# Examples

Learn by example with these working code samples demonstrating Engage UX features.

## Running Examples

All examples can be run using Cargo:

```bash
cargo run --example <example_name> -p <package>
```

For example:

```bash
cargo run --example basic_components -p engage-ux-components
```

## Available Examples

### Component Examples

#### Basic Components

Demonstrates basic component creation and usage.

```bash
cargo run --example basic_components -p engage-ux-components
```

**What it shows**:

- Creating buttons, labels, and inputs
- Setting component properties
- Handling events
- Basic layout

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/basic_components.rs)

#### Theme Demo

Shows how to use and customize themes.

```bash
cargo run --example theme_demo -p engage-ux-components
```

**What it shows**:

- Loading built-in themes
- Applying themes to components
- Color conversions
- Typography settings

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/theme_demo.rs)

#### LCARS Theme Demo

Demonstrates the futuristic LCARS-inspired theme.

```bash
cargo run --example lcars_theme_demo -p engage-ux-components
```

**What it shows**:

- LCARS design aesthetic
- High contrast colors
- Unique component styling
- Sci-fi interface patterns

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/lcars_theme_demo.rs)

### Animation Examples

#### Animation Demo

Shows the animation system in action.

```bash
cargo run --example animation_demo -p engage-ux-components
```

**What it shows**:

- Fade animations
- Slide animations
- Scale and rotate animations
- Color transitions
- Easing functions
- Animation sequencing

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/animation_demo.rs)

### Interaction Examples

#### Drag and Drop Demo

Demonstrates drag and drop functionality.

```bash
cargo run --example drag_drop_demo -p engage-ux-components
```

**What it shows**:

- Creating draggable components
- Defining drop zones
- Handling drag events
- Drop effects (copy, move, link)
- Visual feedback during drag

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/drag_drop_demo.rs)

#### Custom Input Demo

Shows custom input device handling.

```bash
cargo run --example custom_input_demo -p engage-ux-components
```

**What it shows**:

- Gamepad input
- Stylus/pen input
- Sensor input
- Custom device types
- Multi-modal input handling

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/custom_input_demo.rs)

### Layout Examples

#### Layout Demo

Demonstrates the layout system.

```bash
cargo run --example layout_demo -p engage-ux-components
```

**What it shows**:

- Unit types (pixels, rb, rp, percentage)
- Layout calculations
- Constraints (min/max)
- Fill and fit-content modes
- Multi-monitor configuration

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-components/examples/layout_demo.rs)

### Theme Examples

#### Color Formats

Shows different color format support in themes.

```bash
cargo run --example color_formats -p engage-ux-themes
```

**What it shows**:

- Hex color format
- RGB array format
- HSL array format
- Format mixing in themes
- Color conversions

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-themes/examples/color_formats.rs)

#### Export Themes

Utility to export built-in themes to JSON files.

```bash
cargo run --example export_themes -p engage-ux-themes
```

**What it does**:

- Exports light theme to `themes/light.json`
- Exports dark theme to `themes/dark.json`
- Creates theme directory if needed

[View Source Code →](https://github.com/JEleniel/engage-ux/blob/main/engage-ux-themes/examples/export_themes.rs)

## Example Code Patterns

### Creating a Simple UI

```rust
use engage_ux_components::{Button, Container, Label};
use engage_ux_core::component::{Component, Rect};
use engage_ux_themes::Theme;

#[tokio::main]
async fn main() {
    let theme = Theme::dark();
    
    let mut container = Container::new(1);
    container.set_bounds(Rect::new(0.0, 0.0, 800.0, 600.0));
    
    let mut label = Label::new(2, "Welcome!");
    label.set_bounds(Rect::new(300.0, 200.0, 200.0, 40.0));
    label.set_color(theme.colors.primary);
    
    let mut button = Button::new(3, "Click Me");
    button.set_bounds(Rect::new(325.0, 260.0, 150.0, 40.0));
    button.set_on_click(|_| println!("Clicked!"));
    
    container.add_child(Box::new(label));
    container.add_child(Box::new(button));
}
```

### Handling Multiple Events

```rust
use engage_ux_components::TextInput;

let mut input = TextInput::new(1);

// Handle text changes
input.set_on_change(|text| {
    println!("Text changed: {}", text);
});

// Handle focus
input.set_on_focus(|gained| {
    if gained {
        println!("Input focused");
    } else {
        println!("Input blurred");
    }
});

// Handle validation
input.set_validator(|text| {
    !text.is_empty() && text.len() <= 50
});
```

### Using Animations

```rust
use engage_ux_core::animation::{Animation, AnimationType, EasingFunction};
use std::time::Duration;

let fade_in = Animation::new(
    AnimationType::Fade { from: 0.0, to: 1.0 },
    Duration::from_millis(300),
    EasingFunction::EaseOut
);

let slide_in = Animation::new(
    AnimationType::Slide { 
        from_x: -100.0, from_y: 0.0,
        to_x: 0.0, to_y: 0.0 
    },
    Duration::from_millis(400),
    EasingFunction::EaseInOut
);
```

### Custom Component Example

```rust
use engage_ux_core::component::{Component, ComponentId, ComponentProperties, Rect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Counter {
    properties: ComponentProperties,
    count: i32,
}

impl Counter {
    pub fn new(id: ComponentId) -> Self {
        Self {
            properties: ComponentProperties::new(id),
            count: 0,
        }
    }
    
    pub fn increment(&mut self) {
        self.count += 1;
    }
    
    pub fn count(&self) -> i32 {
        self.count
    }
}

impl Component for Counter {
    fn id(&self) -> ComponentId {
        self.properties.id
    }
    
    fn is_visible(&self) -> bool {
        self.properties.visible
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.properties.visible = visible;
    }
    
    fn is_enabled(&self) -> bool {
        self.properties.enabled
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.properties.enabled = enabled;
    }
    
    fn bounds(&self) -> Rect {
        self.properties.bounds
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.properties.bounds = bounds;
    }
}
```

## Integration Examples

### Web Assembly (Future)

```rust
// Future support for WASM
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn create_button() -> Button {
    Button::new(1, "WASM Button")
}
```

### Mobile (Future)

```rust
// Future mobile-specific features
#[cfg(target_os = "android")]
fn setup_mobile_ui() {
    // Android-specific setup
}

#[cfg(target_os = "ios")]
fn setup_mobile_ui() {
    // iOS-specific setup
}
```

## Testing Examples

### Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_click() {
        let mut button = Button::new(1, "Test");
        let mut clicked = false;
        
        button.set_on_click(|_| {
            clicked = true;
        });
        
        // Simulate click
        button.handle_click();
        assert!(clicked);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_form_submission() {
    let mut form = create_form();
    
    // Fill form
    form.set_field("name", "John Doe");
    form.set_field("email", "john@example.com");
    
    // Submit
    let result = form.submit().await;
    assert!(result.is_ok());
}
```

## Example Projects

Complete example projects demonstrating real-world usage:

### Todo App (Coming Soon)

A complete todo list application showing:

- CRUD operations
- Data persistence
- Theme customization
- Responsive layout

Note: These example projects are tracked in the agent roadmap. Machine agents and contributors can find tasks and acceptance criteria in `docs/design/agents/TODO.md`.

### Dashboard (Coming Soon)

A data dashboard demonstrating:

- Charts and graphs
- Real-time updates
- Complex layouts
- Multiple views

Note: See `docs/design/agents/TODO.md` for tasks related to creating example projects.

### Text Editor (Coming Soon)

A simple text editor showing:

- File operations
- Syntax highlighting
- Find and replace
- Multiple tabs

Note: Example project tasks are tracked under `docs/design/agents/TODO.md`.

## Community Examples

Have you created something with Engage UX? Share it with the community!

- [Submit your example](https://github.com/JEleniel/engage-ux/discussions)
- [Browse community examples](https://github.com/JEleniel/engage-ux/discussions/categories/show-and-tell)

## Learning Resources

- [Getting Started Guide](../getting-started.md) - Installation and basics
- [Component Documentation](../components/index.md) - Component reference
- [API Reference](../api/index.md) - Complete API docs
- [Theming Guide](../guides/theming.md) - Customization

## Next Steps

1. **Pick an example** - Choose one that interests you
2. **Run it** - See how it works
3. **Modify it** - Experiment with changes
4. **Build something** - Create your own application

---

[← Back to Documentation](../index.md)
