# Toggle Component

An on/off switch component for binary state changes.

## Overview

The Toggle component (also known as a switch) provides an intuitive way to turn settings on or off. It's visually distinct from checkboxes and is best used for immediate state changes rather than form submissions.

## Basic Usage

```rust
use engage_ux_components::Toggle;
use engage_ux_core::component::{Component, Rect};

// Create a simple toggle
let mut toggle = Toggle::new(1);
toggle.set_bounds(Rect::new(20.0, 20.0, 44.0, 24.0));

// Create a toggle with label
let mut toggle_with_label = Toggle::with_label(2, "Enable notifications");

// Add a change handler
toggle.set_on_change(|event| {
    println!("Toggle state changed!");
});
```

## Sizes

Toggles come in three predefined sizes:

### Small

Compact toggle for dense interfaces.

```rust
use engage_ux_components::ToggleSize;

let mut toggle = Toggle::new(1);
toggle.set_size(ToggleSize::Small);
// Size: 32x18 pixels
```

### Medium (Default)

Standard size for most use cases.

```rust
toggle.set_size(ToggleSize::Medium);
// Size: 44x24 pixels
```

### Large

Larger toggle for touch interfaces or emphasis.

```rust
toggle.set_size(ToggleSize::Large);
// Size: 56x30 pixels
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `active` | `bool` | Toggle state (on/off) | `false` |
| `disabled` | `bool` | Disabled state | `false` |
| `label` | `String` | Optional label text | `""` |
| `size` | `ToggleSize` | Toggle size | `Medium` |
| `active_color` | `Color` | Background color when active | `#1976D2` |
| `inactive_color` | `Color` | Background color when inactive | `#BDBDBD` |
| `thumb_color` | `Color` | Sliding thumb color | `#FFFFFF` |
| `label_color` | `Color` | Label text color | `#000000` |

## Methods

### State Management

```rust
// Check if active
if toggle.is_active() {
    println!("Toggle is ON");
}

// Set active state
toggle.set_active(true);  // Turn ON
toggle.set_active(false); // Turn OFF

// Toggle state
toggle.toggle(); // Switch between ON and OFF
```

### Label

```rust
// Get label
let label = toggle.label();

// Set label
toggle.set_label("Dark mode");
```

### Sizing

```rust
use engage_ux_components::ToggleSize;

// Set size
toggle.set_size(ToggleSize::Large);

// Get size
let size = toggle.size();
let width = size.width();
let height = size.height();
```

### Styling

```rust
use engage_ux_core::color::Color;

// Set active (ON) color
toggle.set_active_color(Color::from_hex("#4CAF50").unwrap());

// Set inactive (OFF) color
toggle.set_inactive_color(Color::from_hex("#E0E0E0").unwrap());

// Set thumb (slider) color
toggle.set_thumb_color(Color::from_hex("#FFFFFF").unwrap());

// Set label color
toggle.set_label_color(Color::from_hex("#333333").unwrap());
```

### State Control

```rust
// Disable toggle
toggle.set_disabled(true);

// Check if disabled
if toggle.is_disabled() {
    println!("Toggle is disabled");
}
```

### Events

```rust
use engage_ux_core::events::Event;

// Set change handler
toggle.set_on_change(|event: &Event| {
    println!("Toggle {} changed state", event.target_id);
});
```

## Examples

### Simple Toggle

```rust
let mut dark_mode = Toggle::with_label(1, "Dark mode");
dark_mode.set_bounds(Rect::new(20.0, 20.0, 200.0, 24.0));

dark_mode.set_on_change(|_| {
    println!("Dark mode toggled");
});
```

### Themed Toggle

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut toggle = Toggle::with_label(1, "Enable feature");

// Apply theme colors
toggle.set_active_color(theme.colors.primary);
toggle.set_inactive_color(theme.colors.surface_variant);
toggle.set_thumb_color(theme.colors.on_primary);
toggle.set_label_color(theme.colors.text_primary);
```

### Settings Panel

```rust
struct SettingsPanel {
    notifications: Toggle,
    auto_save: Toggle,
    dark_mode: Toggle,
}

impl SettingsPanel {
    fn new() -> Self {
        Self {
            notifications: Toggle::with_label(1, "Notifications"),
            auto_save: Toggle::with_label(2, "Auto-save"),
            dark_mode: Toggle::with_label(3, "Dark mode"),
        }
    }
    
    fn apply_settings(&self) {
        if self.notifications.is_active() {
            // Enable notifications
        }
        if self.auto_save.is_active() {
            // Enable auto-save
        }
        if self.dark_mode.is_active() {
            // Switch to dark theme
        }
    }
}

// Usage
let settings = SettingsPanel::new();
settings.apply_settings();
```

### Responsive Toggle Sizes

```rust
fn create_toggle_for_screen_size(
    id: u64,
    label: &str,
    screen_width: f32,
) -> Toggle {
    let mut toggle = Toggle::with_label(id, label);
    
    let size = if screen_width < 768.0 {
        ToggleSize::Large  // Larger for mobile
    } else {
        ToggleSize::Medium // Standard for desktop
    };
    
    toggle.set_size(size);
    toggle
}

// Usage
let toggle = create_toggle_for_screen_size(1, "Notifications", 375.0);
```

### Conditional Toggles

```rust
struct ConditionalToggle {
    master: Toggle,
    dependent: Vec<Toggle>,
}

impl ConditionalToggle {
    fn update(&mut self) {
        let enabled = self.master.is_active();
        for toggle in &mut self.dependent {
            toggle.set_disabled(!enabled);
            if !enabled {
                toggle.set_active(false);
            }
        }
    }
}

// Usage
let mut system = ConditionalToggle {
    master: Toggle::with_label(1, "Enable notifications"),
    dependent: vec![
        Toggle::with_label(2, "Email notifications"),
        Toggle::with_label(3, "Push notifications"),
        Toggle::with_label(4, "SMS notifications"),
    ],
};

// When master changes
system.update();
```

### Async State Update

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

async fn toggle_feature(
    toggle: Arc<RwLock<Toggle>>,
    feature_name: String,
) {
    // Disable during operation
    {
        let mut t = toggle.write().await;
        t.set_disabled(true);
    }
    
    // Simulate async operation
    let result = enable_feature(&feature_name).await;
    
    // Update state based on result
    {
        let mut t = toggle.write().await;
        t.set_active(result.is_ok());
        t.set_disabled(false);
    }
}
```

### Toggle with Confirmation

```rust
struct ConfirmToggle {
    toggle: Toggle,
    requires_confirmation: bool,
}

impl ConfirmToggle {
    fn handle_change(&mut self) -> bool {
        if self.requires_confirmation && self.toggle.is_active() {
            // Show confirmation dialog
            let confirmed = show_confirmation_dialog(
                "Are you sure you want to enable this feature?"
            );
            
            if !confirmed {
                self.toggle.set_active(false);
                return false;
            }
        }
        true
    }
}
```

## Accessibility

Toggles automatically include accessibility features:

- **Role**: `switch`
- **Label**: Label text or aria-label
- **Keyboard**: Activates on `Space` or `Enter`
- **Focus**: Can receive keyboard focus
- **States**: On/off state is announced

```rust
// Accessibility is built-in
toggle.set_aria_label("Enable dark mode");
toggle.set_aria_describedby("dark-mode-description");
```

### Screen Reader Support

```rust
// Provide additional context
toggle.set_aria_label("Enable notifications (immediate effect)");

// Link to description
toggle.set_aria_describedby("notifications-info");
```

## Best Practices

### Use for Immediate Changes

Toggles are best for settings that take effect immediately:

```rust
// Good: Immediate effect
let dark_mode = Toggle::with_label(1, "Dark mode");

// Avoid: Use checkbox for form submission
// Use checkboxes when changes apply on form submit
```

### Clear Labels

Use descriptive labels that indicate the enabled state:

```rust
// Good
let toggle = Toggle::with_label(1, "Enable notifications");
let toggle = Toggle::with_label(2, "Auto-save documents");

// Avoid ambiguous labels
let toggle = Toggle::with_label(3, "Notifications"); // On or off?
```

### Visual Feedback

Provide clear visual distinction between states:

```rust
// Strong contrast between states
toggle.set_active_color(Color::from_hex("#4CAF50").unwrap()); // Green
toggle.set_inactive_color(Color::from_hex("#E0E0E0").unwrap()); // Gray
```

### Appropriate Sizing

Choose size based on interface density:

```rust
// Dense desktop interface
toggle.set_size(ToggleSize::Small);

// Touch-friendly mobile interface
toggle.set_size(ToggleSize::Large);
```

### Disabled State

Style disabled toggles appropriately:

```rust
let mut toggle = Toggle::new(1);
toggle.set_disabled(true);

// Reduce opacity or use muted colors
toggle.set_active_color(Color::from_hex("#CCCCCC").unwrap());
toggle.set_label_color(Color::from_hex("#999999").unwrap());
```

## When to Use Toggle vs Checkbox

### Use Toggle When:

- Setting takes effect immediately
- Representing binary on/off state
- Controlling application settings
- Switching between modes (e.g., dark/light theme)

### Use Checkbox When:

- Multiple selections possible
- Changes apply on form submission
- Part of a form
- Collecting user preferences for later processing

## Common Patterns

### Toggle Group

```rust
fn create_toggle_group(
    settings: &[(&str, bool)],
    start_y: f32,
    theme: &Theme,
) -> Vec<Toggle> {
    settings
        .iter()
        .enumerate()
        .map(|(i, (label, default_state))| {
            let mut toggle = Toggle::with_label(i as u64, *label);
            toggle.set_active(*default_state);
            toggle.set_bounds(Rect::new(
                20.0,
                start_y + (i as f32 * 40.0),
                200.0,
                24.0,
            ));
            toggle.set_active_color(theme.colors.primary);
            toggle
        })
        .collect()
}

// Usage
let settings = vec![
    ("Notifications", true),
    ("Auto-save", false),
    ("Dark mode", true),
];
let toggles = create_toggle_group(&settings, 100.0, &theme);
```

### Persisted Settings

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct AppSettings {
    dark_mode: bool,
    notifications: bool,
    auto_save: bool,
}

impl AppSettings {
    fn from_toggles(
        dark_mode: &Toggle,
        notifications: &Toggle,
        auto_save: &Toggle,
    ) -> Self {
        Self {
            dark_mode: dark_mode.is_active(),
            notifications: notifications.is_active(),
            auto_save: auto_save.is_active(),
        }
    }
    
    fn save_to_disk(&self) -> std::io::Result<()> {
        let json = serde_json::to_string(self)?;
        std::fs::write("settings.json", json)?;
        Ok(())
    }
}
```

## Related Components

- [Checkbox](checkbox.md) - For selection in forms
- [Radio Button](radio.md) - For mutually exclusive options
- [Button](button.md) - For action triggers
- [Slider](slider.md) - For range values

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Accessibility](../guides/accessibility.md) - WCAG compliance
- [Theming Guide](../guides/theming.md) - Customizing appearance

---

[← Back to Components](index.md)
