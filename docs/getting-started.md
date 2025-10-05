# Getting Started with Engage UX

This guide will help you install Engage UX and build your first application.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Your First Application](#your-first-application)
- [Understanding the Architecture](#understanding-the-architecture)
- [Next Steps](#next-steps)

## Prerequisites

Before you begin, ensure you have:

- **Rust 1.70 or later** - Install from [rustup.rs](https://rustup.rs/)
- **Cargo** - Comes with Rust
- **Git** - For cloning the repository

### Platform-Specific Requirements

#### Windows

- Visual Studio Build Tools or Visual Studio with C++ development tools
- Windows 10 or later

#### macOS

- Xcode Command Line Tools: `xcode-select --install`
- macOS 10.15 (Catalina) or later

#### Linux

- GCC or Clang
- Development headers for X11 (most distributions)

```bash
# Ubuntu/Debian
sudo apt-get install build-essential libx11-dev

# Fedora
sudo dnf install gcc gcc-c++ libX11-devel

# Arch
sudo pacman -S base-devel libx11
```

#### Android

- Android NDK r25 or later
- Android Studio (recommended)

#### iOS

- Xcode 14 or later
- iOS SDK
- macOS host system required

## Installation

### Option 1: Add as a Git Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-components = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-themes = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-oal = { git = "https://github.com/JEleniel/engage-ux" }
tokio = { version = "1.0", features = ["full"] }
```

### Option 2: Clone and Use Locally

```bash
# Clone the repository
git clone https://github.com/JEleniel/engage-ux.git
cd engage-ux

# Build the project
cargo build --all

# Run tests
cargo test --all

# Run examples
cargo run --example basic_components -p engage-ux-components
```

Then reference in your `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { path = "../engage-ux/engage-ux-core" }
engage-ux-components = { path = "../engage-ux/engage-ux-components" }
engage-ux-themes = { path = "../engage-ux/engage-ux-themes" }
engage-ux-oal = { path = "../engage-ux/engage-ux-oal" }
tokio = { version = "1.0", features = ["full"] }
```

## Your First Application

Let's create a simple "Hello World" application with a button.

### Step 1: Create a New Project

```bash
cargo new hello-engage-ux
cd hello-engage-ux
```

### Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "hello-engage-ux"
version = "0.1.0"
edition = "2021"

[dependencies]
engage-ux-core = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-components = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-themes = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-oal = { git = "https://github.com/JEleniel/engage-ux" }
tokio = { version = "1.0", features = ["full"] }
```

### Step 3: Create Your Application

Edit `src/main.rs`:

```rust
use engage_ux_components::{Button, Container, Label};
use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, Rect};
use engage_ux_themes::Theme;

#[tokio::main]
async fn main() {
    println!("🚀 Engage UX - Hello World!");
    
    // Load a theme
    let theme = Theme::dark();
    println!("✓ Theme loaded: {}", theme.name);
    
    // Create a container
    let mut container = Container::new(1);
    container.set_bounds(Rect::new(0.0, 0.0, 800.0, 600.0));
    println!("✓ Container created");
    
    // Create a label
    let mut label = Label::new(2, "Welcome to Engage UX!");
    label.set_bounds(Rect::new(250.0, 200.0, 300.0, 40.0));
    label.set_font_size(24.0);
    label.set_color(theme.colors.primary);
    println!("✓ Label created: {}", label.text());
    
    // Create a button
    let mut button = Button::new(3, "Click Me!");
    button.set_bounds(Rect::new(325.0, 280.0, 150.0, 40.0));
    button.set_background_color(theme.colors.primary);
    button.set_text_color(theme.colors.on_primary);
    
    // Add click handler
    button.set_on_click(|_| {
        println!("🎉 Button clicked!");
    });
    println!("✓ Button created with click handler");
    
    // Add components to container
    container.add_child(Box::new(label));
    container.add_child(Box::new(button));
    println!("✓ Components added to container");
    
    // In a real application, you would now:
    // 1. Initialize the window system
    // 2. Create a window
    // 3. Attach the container to the window
    // 4. Start the event loop
    
    println!("\n✅ Application setup complete!");
    println!("📝 Note: This is a demonstration. Full rendering requires");
    println!("   platform-specific window and rendering backends.");
}
```

### Step 4: Build and Run

```bash
cargo build
cargo run
```

You should see output like:

```
🚀 Engage UX - Hello World!
✓ Theme loaded: Dark Theme
✓ Container created
✓ Label created: Welcome to Engage UX!
✓ Button created with click handler
✓ Components added to container

✅ Application setup complete!
📝 Note: This is a demonstration. Full rendering requires
   platform-specific window and rendering backends.
```

## Understanding the Architecture

Engage UX is organized into four main crates:

### engage-ux-core

The foundation layer providing:

- **Color System**: RGB/HSL colors with conversions
- **Component Trait**: Base interface for all UI elements
- **Event System**: Async event handling with Tokio
- **Input System**: Keyboard, mouse, touch, and custom devices
- **Animation System**: Built-in animations with easing
- **Accessibility**: WCAG AAA support infrastructure

```rust
use engage_ux_core::color::Color;
use engage_ux_core::component::Component;
use engage_ux_core::input::{KeyCode, MouseButton};
```

### engage-ux-components

The UI component library with all 50 components:

- Informational: Labels, Text, Icons, Images, Lists, Progress, etc.
- Interactive: Buttons, Inputs, Checkboxes, Sliders, etc.
- Layout: Containers, Cards, Tables, Windows
- Notification: Badges, Toasts, Banners
- Menus & Dialogs: Dropdowns, Modals, File Dialogs

```rust
use engage_ux_components::{Button, Label, TextInput, Container};
```

### engage-ux-themes

JSON-based theme system:

- Default themes (LCARS, Classic Light, Classic Dark)
- Color palettes
- Typography settings
- Spacing and layout
- Borders and shadows

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let theme = Theme::light();
let theme = Theme::from_json(json_string)?;
```

### engage-ux-oal

OS Abstraction Layer:

- Platform detection
- Window management
- Graphics rendering backends
- Input device management

```rust
use engage_ux_oal::{Platform, WindowBackend};

let platform = Platform::detect();
println!("Running on: {}", platform.name());
```

## Basic Concepts

### Components

All UI elements implement the `Component` trait:

```rust
use engage_ux_core::component::Component;

// Every component has:
let id = component.id();                    // Unique identifier
let visible = component.is_visible();       // Visibility state
let enabled = component.is_enabled();       // Enabled state
let bounds = component.bounds();            // Position and size
```

### Events

Components handle events asynchronously:

```rust
button.set_on_click(|event| {
    println!("Button clicked!");
});

checkbox.set_on_change(|checked| {
    println!("Checkbox is now: {}", checked);
});

input.set_on_change(|text| {
    println!("Input value: {}", text);
});
```

### Colors

Engage UX supports multiple color formats:

```rust
use engage_ux_core::color::Color;

// RGB
let red = Color::from_rgb(255, 0, 0);

// RGB with alpha
let semi_red = Color::from_rgba(255, 0, 0, 128);

// Hex
let blue = Color::from_hex("#0000FF").unwrap();

// HSL
let green = Color::from_hsl(120.0, 1.0, 0.5);
```

### Theming

Apply themes to components:

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();

button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
label.set_color(theme.colors.text_primary);
```

## Next Steps

Now that you have the basics, explore:

1. **[Component Documentation](components/)** - Learn about all 50 components
2. **[Theming Guide](guides/theming.md)** - Customize your application's appearance
3. **[Examples](examples/)** - See working code examples
4. **[Component Development](guides/component-development.md)** - Create custom components
5. **[Architecture](design/architecture/)** - Understand the system design

### Run the Examples

Engage UX includes several examples:

```bash
# Basic component usage
cargo run --example basic_components -p engage-ux-components

# Theme demonstration
cargo run --example theme_demo -p engage-ux-components

# LCARS theme showcase
cargo run --example lcars_theme_demo -p engage-ux-components

# Animation system
cargo run --example animation_demo -p engage-ux-components

# Drag and drop
cargo run --example drag_drop_demo -p engage-ux-components

# Custom input devices
cargo run --example custom_input_demo -p engage-ux-components

# Layout system
cargo run --example layout_demo -p engage-ux-components

# Color formats
cargo run --example color_formats -p engage-ux-themes
```

## Common Patterns

### Creating a Form

```rust
use engage_ux_components::{Container, Label, TextInput, Button};

let mut form = Container::new(1);

let name_label = Label::new(2, "Name:");
let name_input = TextInput::new(3);

let email_label = Label::new(4, "Email:");
let email_input = TextInput::new(5);

let mut submit_button = Button::new(6, "Submit");
submit_button.set_on_click(|_| {
    println!("Form submitted!");
});

form.add_child(Box::new(name_label));
form.add_child(Box::new(name_input));
form.add_child(Box::new(email_label));
form.add_child(Box::new(email_input));
form.add_child(Box::new(submit_button));
```

### Using Layout System

```rust
use engage_ux_core::layout::{Layout, Size, Unit};

let layout = Layout::new()
    .with_left(Unit::pixels(10.0))
    .with_top(Unit::pixels(20.0))
    .with_width(Size::Fixed(Unit::percent(80.0)))
    .with_height(Size::Fill)
    .with_min_width(Unit::pixels(200.0))
    .with_max_width(Unit::pixels(800.0));

let bounds = layout.calculate_bounds(1024.0, 768.0, 16.0, 16.0);
component.set_bounds(bounds);
```

### Handling Input

```rust
use engage_ux_core::input::{InputHandler, KeyCode, MouseButton};

impl InputHandler for MyComponent {
    fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
        match event.key_code {
            KeyCode::Enter => {
                self.submit();
                true
            }
            _ => false
        }
    }

    fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        // Handle mouse events
        false
    }
}
```

## Troubleshooting

### Build Errors

**Error**: `cargo build` fails with linker errors

**Solution**: Ensure you have the platform-specific build tools installed (see [Prerequisites](#prerequisites))

### Component Not Rendering

**Issue**: Components created but nothing appears

**Explanation**: The current version implements the component architecture and logic. Full rendering requires platform-specific backends that integrate with native window systems.

### Theme Not Loading

**Issue**: `Theme::from_json()` returns an error

**Solution**: Validate your JSON against the [theme schema](../schemas/theme.schema.json)

## Getting Help

- **Documentation**: [Browse the docs](index.md)
- **Examples**: Check the [examples directory](examples/)
- **Issues**: [Open an issue](https://github.com/JEleniel/engage-ux/issues)
- **Discussions**: [Start a discussion](https://github.com/JEleniel/engage-ux/discussions)

---

[← Back to Documentation Index](index.md) | [Next: Component Documentation →](components/)
