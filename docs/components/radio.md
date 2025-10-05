# Radio Button Component

A component for selecting a single option from a set of mutually exclusive choices.

## Overview

Radio buttons allow users to select exactly one option from a group. When one radio button is selected, all others in the same group are automatically deselected.

## Basic Usage

```rust
use engage_ux_components::RadioButton;
use engage_ux_core::component::{Component, Rect};

// Create radio buttons in a group
let mut option1 = RadioButton::new(1, "Option 1", "value1", "mygroup");
let mut option2 = RadioButton::new(2, "Option 2", "value2", "mygroup");
let mut option3 = RadioButton::new(3, "Option 3", "value3", "mygroup");

// Set bounds
option1.set_bounds(Rect::new(20.0, 20.0, 150.0, 24.0));
option2.set_bounds(Rect::new(20.0, 50.0, 150.0, 24.0));
option3.set_bounds(Rect::new(20.0, 80.0, 150.0, 24.0));

// Select one
option1.set_selected(true);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `selected` | `bool` | Whether this button is selected | `false` |
| `label` | `String` | Display label | Required |
| `value` | `String` | Value when selected | Required |
| `group` | `String` | Group name for mutual exclusion | Required |
| `disabled` | `bool` | Disabled state | `false` |
| `color` | `Color` | Unselected color | `#BDBDBD` |
| `selected_color` | `Color` | Selected color | `#1976D2` |
| `label_color` | `Color` | Label text color | `#000000` |
| `size` | `f32` | Radio button size in pixels | `20.0` |

## Methods

### Selection State

```rust
// Check if selected
if radio.is_selected() {
    println!("This option is selected");
}

// Select this radio button
radio.set_selected(true);

// When selecting, deselect others in the same group manually
// (or use a RadioGroup helper - see examples)
```

### Label and Value

```rust
// Get label
let label = radio.label();

// Set label
radio.set_label("New label");

// Get value
let value = radio.value();

// Set value
radio.set_value("new_value");
```

### Group Management

```rust
// Get group name
let group = radio.group();

// Change group
radio.set_group("different_group");
```

### Styling

```rust
use engage_ux_core::color::Color;

// Set unselected color
radio.set_color(Color::from_hex("#E0E0E0").unwrap());

// Set selected color
radio.set_selected_color(Color::from_hex("#4CAF50").unwrap());

// Set label color
radio.set_label_color(Color::from_hex("#333333").unwrap());

// Set size
radio.set_size(22.0);
```

### State Control

```rust
// Disable radio button
radio.set_disabled(true);

// Check if disabled
if radio.is_disabled() {
    println!("Radio button is disabled");
}
```

### Events

```rust
use engage_ux_core::events::Event;

// Set change handler
radio.set_on_change(|event: &Event| {
    println!("Radio button {} selected", event.target_id);
});
```

## Examples

### Radio Group

```rust
struct RadioGroup {
    buttons: Vec<RadioButton>,
    group_name: String,
}

impl RadioGroup {
    fn new(group_name: &str, options: &[(&str, &str)]) -> Self {
        let buttons = options
            .iter()
            .enumerate()
            .map(|(i, (label, value))| {
                RadioButton::new(
                    i as u64,
                    *label,
                    *value,
                    group_name
                )
            })
            .collect();
        
        Self {
            buttons,
            group_name: group_name.to_string(),
        }
    }
    
    fn select(&mut self, index: usize) {
        // Deselect all
        for button in &mut self.buttons {
            button.set_selected(false);
        }
        
        // Select the chosen one
        if let Some(button) = self.buttons.get_mut(index) {
            button.set_selected(true);
        }
    }
    
    fn get_selected_value(&self) -> Option<String> {
        self.buttons
            .iter()
            .find(|b| b.is_selected())
            .map(|b| b.value().to_string())
    }
    
    fn get_selected_index(&self) -> Option<usize> {
        self.buttons
            .iter()
            .position(|b| b.is_selected())
    }
}

// Usage
let mut size_group = RadioGroup::new("size", &[
    ("Small", "S"),
    ("Medium", "M"),
    ("Large", "L"),
    ("X-Large", "XL"),
]);

size_group.select(1); // Select "Medium"
let value = size_group.get_selected_value(); // Some("M")
```

### Themed Radio Buttons

```rust
use engage_ux_themes::Theme;

fn create_themed_radio_group(
    group_name: &str,
    options: &[(&str, &str)],
    theme: &Theme,
) -> Vec<RadioButton> {
    options
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let mut radio = RadioButton::new(
                i as u64,
                *label,
                *value,
                group_name
            );
            radio.set_selected_color(theme.colors.primary);
            radio.set_label_color(theme.colors.text_primary);
            radio.set_size(22.0);
            radio
        })
        .collect()
}

// Usage
let theme = Theme::dark();
let payment_methods = create_themed_radio_group(
    "payment",
    &[
        ("Credit Card", "card"),
        ("PayPal", "paypal"),
        ("Bank Transfer", "bank"),
    ],
    &theme,
);
```

### Form Integration

```rust
struct FormData {
    payment_method: String,
    shipping_speed: String,
}

impl FormData {
    fn from_radio_groups(
        payment_group: &RadioGroup,
        shipping_group: &RadioGroup,
    ) -> Result<Self, String> {
        let payment_method = payment_group
            .get_selected_value()
            .ok_or("Please select a payment method")?;
        
        let shipping_speed = shipping_group
            .get_selected_value()
            .ok_or("Please select a shipping speed")?;
        
        Ok(Self {
            payment_method,
            shipping_speed,
        })
    }
}

// Usage
let form_data = FormData::from_radio_groups(&payment, &shipping)?;
```

### Conditional Radio Buttons

```rust
struct ConditionalRadioGroup {
    condition: RadioButton,
    dependent_group: RadioGroup,
}

impl ConditionalRadioGroup {
    fn update(&mut self) {
        let enabled = self.condition.is_selected();
        
        for button in &mut self.dependent_group.buttons {
            button.set_disabled(!enabled);
            if !enabled {
                button.set_selected(false);
            }
        }
    }
}

// Usage
let mut conditional = ConditionalRadioGroup {
    condition: RadioButton::new(1, "Enable shipping", "yes", "condition"),
    dependent_group: RadioGroup::new("shipping", &[
        ("Standard", "standard"),
        ("Express", "express"),
        ("Overnight", "overnight"),
    ]),
};

// When condition changes
conditional.update();
```

### Visual Radio Group

```rust
fn create_visual_radio_group(
    group_name: &str,
    start_y: f32,
    options: &[(&str, &str)],
    theme: &Theme,
) -> Vec<RadioButton> {
    options
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let mut radio = RadioButton::new(
                i as u64,
                *label,
                *value,
                group_name
            );
            
            radio.set_bounds(Rect::new(
                20.0,
                start_y + (i as f32 * 35.0),
                200.0,
                24.0,
            ));
            
            radio.set_selected_color(theme.colors.primary);
            radio.set_label_color(theme.colors.text_primary);
            
            radio
        })
        .collect()
}

// Usage
let colors = create_visual_radio_group(
    "color",
    100.0,
    &[
        ("Red", "red"),
        ("Green", "green"),
        ("Blue", "blue"),
    ],
    &theme,
);
```

### Default Selection

```rust
fn create_radio_group_with_default(
    group_name: &str,
    options: &[(&str, &str)],
    default_index: usize,
) -> RadioGroup {
    let mut group = RadioGroup::new(group_name, options);
    group.select(default_index);
    group
}

// Usage
let priority = create_radio_group_with_default(
    "priority",
    &[
        ("Low", "low"),
        ("Medium", "medium"),
        ("High", "high"),
    ],
    1, // Default to "Medium"
);
```

## Accessibility

Radio buttons automatically include accessibility features:

- **Role**: `radio`
- **Label**: Radio button label text
- **Keyboard**: 
  - `Tab` to move between groups
  - Arrow keys to move within group
  - `Space` to select
- **Focus**: Can receive keyboard focus
- **States**: Selected state is announced

```rust
// Accessibility is built-in
radio.set_aria_label("Select payment method");
radio.set_aria_describedby("payment-description");
```

### Accessible Radio Group

```rust
struct AccessibleRadioGroup {
    group_label: String,
    buttons: Vec<RadioButton>,
}

impl AccessibleRadioGroup {
    fn new(group_label: &str, options: &[(&str, &str)]) -> Self {
        let buttons = options
            .iter()
            .enumerate()
            .map(|(i, (label, value))| {
                let mut radio = RadioButton::new(i as u64, *label, *value, group_label);
                radio.set_aria_label(&format!("{}: {}", group_label, label));
                radio
            })
            .collect();
        
        Self {
            group_label: group_label.to_string(),
            buttons,
        }
    }
}
```

## Best Practices

### Clear Labels

Use descriptive labels that make the choice obvious:

```rust
// Good
RadioButton::new(1, "Receive email notifications", "email", "notifications");
RadioButton::new(2, "Receive SMS notifications", "sms", "notifications");

// Avoid
RadioButton::new(3, "Email", "email", "notifications");
```

### Logical Grouping

Group related options together:

```rust
// Shipping options group
let shipping_group = RadioGroup::new("shipping", &[
    ("Standard (5-7 days)", "standard"),
    ("Express (2-3 days)", "express"),
    ("Overnight (next day)", "overnight"),
]);

// Keep separate groups for different choices
let payment_group = RadioGroup::new("payment", &[
    ("Credit Card", "card"),
    ("PayPal", "paypal"),
]);
```

### Default Selection

Always provide a sensible default:

```rust
let mut group = RadioGroup::new("priority", &[
    ("Low", "low"),
    ("Medium", "medium"),
    ("High", "high"),
]);

// Select a default
group.select(1); // Medium is default
```

### Validation

Ensure a selection is made when required:

```rust
fn validate_selection(group: &RadioGroup) -> Result<(), String> {
    if group.get_selected_value().is_none() {
        Err("Please select an option".to_string())
    } else {
        Ok(())
    }
}
```

### Appropriate Sizing

Size radio buttons for easy interaction:

```rust
// Standard size
radio.set_size(20.0);

// Touch-friendly size
radio.set_size(24.0);
```

## When to Use Radio vs Other Components

### Use Radio Buttons When:
- Selecting one option from 2-7 choices
- All options should be visible
- User needs to see all options to make an informed choice

### Use Select Dropdown When:
- More than 7 options
- Space is limited
- Options are familiar to users

### Use Checkbox When:
- Multiple selections allowed
- Choices are not mutually exclusive

### Use Toggle When:
- Simple on/off choice
- Immediate effect

## Related Components

- [Checkbox](checkbox.md) - For multiple selections
- [Select](select.md) - For dropdown selections
- [Toggle](toggle.md) - For on/off states
- [Button](button.md) - For action triggers

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Accessibility](../guides/accessibility.md) - WCAG compliance
- [Theming Guide](../guides/theming.md) - Customizing appearance

---

[← Back to Components](index.md)
