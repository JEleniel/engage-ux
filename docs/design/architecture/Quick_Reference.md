# Engage UX Quick Reference

## For Developers

This quick reference provides essential information for working with Engage UX.

## Project Structure

```
engage-ux/
├── engage-ux-core/          # Foundation layer
│   ├── color.rs             # Color system (RGB/HSL)
│   ├── component.rs         # Component trait and properties
│   ├── events.rs            # Event system with Tokio
│   ├── input/               # Keyboard, mouse, touch input
│   ├── accessibility/       # WCAG AAA support
│   ├── media/               # Font, image, SVG support
│   └── rendering/           # Render abstractions
├── engage-ux-oal/           # OS Abstraction Layer
│   ├── platform.rs          # Platform detection
│   ├── window/              # Window management
│   └── rendering/           # Graphics backends
├── engage-ux-components/    # UI Components (50 total)
│   ├── button.rs
│   ├── text_input.rs
│   └── ...
├── engage-ux-themes/        # Theme system
│   └── lib.rs               # JSON theme loading
└── engage-ux-tests/         # Integration tests
```

## Core Concepts

### 1. Components

All UI elements implement the `Component` trait:

```rust
pub trait Component {
    fn id(&self) -> ComponentId;
    fn properties(&self) -> &ComponentProperties;
    fn properties_mut(&mut self) -> &mut ComponentProperties;
}
```

### 2. Events

Events use Tokio broadcast channels:

```rust
let mut handler = EventHandler::new();
handler.subscribe(EventType::Click);
handler.publish(Event::new(component_id, EventType::Click));
```

### 3. Colors

Colors support multiple formats:

```rust
// RGB
let red = Color::rgb(1.0, 0.0, 0.0, 1.0);

// HSL
let blue = Color::hsl(240.0, 1.0, 0.5, 1.0);

// Hex
let green = Color::from_hex("#00FF00").unwrap();

// Conversion
let hsl_red = red.to_hsl();
```

### 4. Themes

Themes are loaded from JSON:

```rust
use engage_ux_themes::Theme;

let theme = Theme::from_file("themes/dark.json").unwrap();
let primary_color = theme.colors.primary;
```

### 5. Input Handling

Components can handle input:

```rust
impl InputHandler for MyComponent {
    fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
        // Handle keyboard input
        true // Event consumed
    }
    
    fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        // Handle mouse input
        true
    }
}
```

## Component Categories

### Informational (11)
Label, Text, Icon, Image, List, Progress, Tooltip, Breadcrumb, Avatar, Line Numbers, Ruler

### Interactive (14)
Button, Checkbox, Radio, Slider, Text Input, Text Area, Toggle, Select, Link, Pagination, Carousel, Date Picker, Text Editor, Console

### Graphic & Display (2)
Group, Video

### Notification (3)
Badge, Banner, Toast

### Menus (4)
Drawer, Dropdown, Hamburger Menu, Title Menu

### Window Controls (3)
Close, Maximize/Restore, Minimize/Restore

### Pane Groups (2)
Accordion, Tabbed

### Dialogs (5)
Alert, Confirm, Custom Modal, Open Dialog, Save As Dialog

### Layout/Grouping (4)
Card, Container, Table, Window

## Creating a Component

Example: Creating a button

```rust
use engage_ux_components::Button;
use engage_ux_core::events::{Event, EventType};

// Create button
let mut button = Button::new(1, "Click me!");

// Set properties
button.set_variant(ButtonVariant::Primary);
button.set_color(Color::from_hex("#1976D2").unwrap());

// Add event handler
button.set_on_click(|event| {
    println!("Button {} clicked!", event.component_id);
});

// Trigger event
let event = Event::new(1, EventType::Click);
// Event is routed to button's handler
```

## Theme JSON Structure

```json
{
    "name": "My Theme",
    "colors": {
        "primary": {"hex": "#1976D2"},
        "secondary": {"rgb": [66, 66, 66]},
        "background": {"hsl": [0, 0, 1.0]},
        "text": {"hex": "#212121"}
    },
    "typography": {
        "fontFamily": "Inter, system-ui, sans-serif",
        "fontSize": {
            "base": 16,
            "sm": 14,
            "lg": 18
        }
    },
    "spacing": {
        "base": 8,
        "xs": 4,
        "sm": 8,
        "md": 16,
        "lg": 24,
        "xl": 32
    },
    "borders": {
        "width": {
            "thin": 1,
            "medium": 2,
            "thick": 4
        },
        "radius": {
            "small": 4,
            "medium": 8,
            "large": 16
        }
    },
    "shadows": {
        "low": {
            "offsetX": 0,
            "offsetY": 2,
            "blur": 4,
            "spread": 0,
            "color": {"hex": "#00000033"}
        }
    }
}
```

## Color Formats in Themes

### Hex Format
```json
{"hex": "#RRGGBB"}
{"hex": "#RRGGBBAA"}
```

### RGB Array (0-255)
```json
{"rgb": [255, 0, 0]}        // Red
{"rgb": [0, 255, 0, 0.5]}   // Green with 50% alpha
```

### HSL Array
```json
{"hsl": [240, 1.0, 0.5]}    // Blue
{"hsl": [0, 1.0, 0.5, 0.8]} // Red with 80% alpha
```

## Non-Functional Requirements

### Security
- ✅ No `unsafe` code (`#![forbid(unsafe_code)]`)
- ✅ All input validated
- ✅ SVG scripts blocked
- ✅ Only maintained dependencies

### Accessibility (WCAG AAA)
- ✅ 7:1 contrast ratio for text
- ✅ ARIA roles and labels
- ✅ Keyboard navigation
- ✅ Screen reader support
- ✅ Focus indicators

### Performance
- ✅ 60 FPS target (16ms per frame)
- ✅ Lazy loading for media
- ✅ Async operations
- ✅ Efficient memory usage

### Testing
- ✅ 90% unit test coverage target
- ✅ Integration tests
- ✅ All tests pass
- ✅ No flaky tests

### Code Quality
- ✅ Zero clippy warnings
- ✅ Formatted with rustfmt
- ✅ Tabs for indentation
- ✅ Documentation comments

## Common Patterns

### Builder Pattern
```rust
let button = Button::new(1, "Click")
    .variant(ButtonVariant::Primary)
    .color(Color::from_hex("#1976D2").unwrap())
    .build();
```

### Event Callbacks
```rust
button.set_on_click(move |event| {
    // Handle click
});
```

### Thread-Safe Components
```rust
use engage_ux_core::component::ComponentRef;
use std::sync::Arc;

let button = Arc::new(RwLock::new(Button::new(1, "Click")));
let button_ref = ComponentRef::new(button);

// Access from multiple threads
let button_clone = button_ref.clone();
thread::spawn(move || {
    let mut btn = button_clone.write().unwrap();
    btn.set_text("Updated");
});
```

### Async Event Handling
```rust
#[tokio::main]
async fn main() {
    let handler = EventHandler::new();
    let mut receiver = handler.subscribe(EventType::Click);
    
    tokio::spawn(async move {
        while let Ok(event) = receiver.recv().await {
            println!("Received event: {:?}", event);
        }
    });
}
```

## Platform Abstraction

### Creating a Window
```rust
use engage_ux_oal::window::WindowBackend;

let backend = PlatformFactory::create_window_backend();
let window = backend.create_window("My App", 800, 600);
window.show();
```

### Rendering
```rust
use engage_ux_oal::rendering::RenderBackend;

let backend = PlatformFactory::create_render_backend();
let context = backend.create_context();

// Generate render commands
context.clear(Color::white());
context.fill_rect(10.0, 10.0, 100.0, 50.0, Color::blue());
context.draw_text("Hello", 10.0, 30.0, font, Color::black());

// Submit to backend
backend.present(context);
```

## Testing

### Unit Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_creation() {
        let button = Button::new(1, "Test");
        assert_eq!(button.id(), 1);
        assert_eq!(button.text(), "Test");
    }
    
    #[test]
    fn test_button_click() {
        let mut button = Button::new(1, "Test");
        let clicked = Arc::new(AtomicBool::new(false));
        let clicked_clone = clicked.clone();
        
        button.set_on_click(move |_| {
            clicked_clone.store(true, Ordering::Relaxed);
        });
        
        let event = Event::new(1, EventType::Click);
        button.handle_event(&event);
        
        assert!(clicked.load(Ordering::Relaxed));
    }
}
```

### Integration Test Example
```rust
#[tokio::test]
async fn test_event_system() {
    let handler = EventHandler::new();
    let mut receiver = handler.subscribe(EventType::Click);
    
    let event = Event::new(1, EventType::Click);
    handler.publish(event).await;
    
    let received = receiver.recv().await.unwrap();
    assert_eq!(received.event_type, EventType::Click);
}
```

## Resources

- [System Architecture](System_Architecture.md) - Overall system design
- [Component Architecture](Component_Architecture.md) - Component patterns
- [Data Flow](Data_Flow.md) - How data flows through the system
- [Requirements](Requirement_1_Core_System.md) - Detailed specifications
- [NFRs](NFRs.md) - Non-functional requirements

## Build and Run

```bash
# Build all crates
cargo build --all

# Run tests
cargo test --all

# Run clippy
cargo clippy --all -- -D warnings

# Format code
cargo fmt --all

# Run examples
cargo run --example basic_components -p engage-ux-components
cargo run --example theme_demo -p engage-ux-components
```

## Common Commands

```bash
# Check for unused dependencies
cargo machete

# Generate documentation
cargo doc --all --open

# Run with release optimizations
cargo build --release --all

# Run benchmarks (when available)
cargo bench --all
```

## Getting Help

- Review architecture documents in `docs/design/architecture/`
- Check existing component implementations
- Read the README.md for project overview
- Review CONTRIBUTING.md for development guidelines
- Open an issue on GitHub for questions
