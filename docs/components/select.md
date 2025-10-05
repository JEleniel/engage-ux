# Select Component

A dropdown selection component for choosing from a list of options.

## Overview

The Select component (also known as a dropdown) allows users to choose one option from a list. It supports searchable options, custom styling, and keyboard navigation.

## Basic Usage

```rust
use engage_ux_components::{Select, SelectOption};
use engage_ux_core::component::{Component, Rect};

// Create a select component
let mut select = Select::new(1);
select.set_placeholder("Choose an option...");
select.set_bounds(Rect::new(20.0, 20.0, 250.0, 40.0));

// Add options
select.add_option(SelectOption::new("Option 1", "value1"));
select.add_option(SelectOption::new("Option 2", "value2"));
select.add_option(SelectOption::new("Option 3", "value3"));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `options` | `Vec<SelectOption>` | Available options | `[]` |
| `selected_index` | `Option<usize>` | Selected option index | `None` |
| `placeholder` | `String` | Placeholder text | `"Select an option..."` |
| `searchable` | `bool` | Enable search/filter | `false` |
| `open` | `bool` | Dropdown open state | `false` |

## Methods

### Options Management

```rust
// Add single option
select.add_option(SelectOption::new("Label", "value"));

// Set all options at once
select.set_options(vec![
    SelectOption::new("Small", "S"),
    SelectOption::new("Medium", "M"),
    SelectOption::new("Large", "L"),
]);

// Get all options
let options = select.options();
```

### Selection

```rust
// Select by index
select.select(1); // Select second option

// Get selected index
let index = select.selected_index(); // Option<usize>

// Get selected value
let value = select.selected_value(); // Option<&str>

// Get selected label
let label = select.selected_label(); // Option<&str>
```

### Configuration

```rust
// Set placeholder
select.set_placeholder("Select a size...");

// Enable search
select.set_searchable(true);

// Open/close dropdown
select.open();
select.close();

// Check if open
if select.is_open() {
    println!("Dropdown is open");
}
```

### Events

```rust
select.set_on_change(|event| {
    println!("Selection changed");
});
```

## Examples

### Simple Dropdown

```rust
let mut country = Select::new(1);
country.set_placeholder("Select a country...");
country.set_options(vec![
    SelectOption::new("United States", "US"),
    SelectOption::new("United Kingdom", "UK"),
    SelectOption::new("Canada", "CA"),
    SelectOption::new("Australia", "AU"),
]);
```

### Searchable Select

```rust
let mut city = Select::new(1);
city.set_placeholder("Search for a city...");
city.set_searchable(true);

// Add many options
let cities = vec![
    ("New York", "NYC"),
    ("Los Angeles", "LA"),
    ("Chicago", "CHI"),
    // ... many more
];

for (label, value) in cities {
    city.add_option(SelectOption::new(label, value));
}
```

### Form Integration

```rust
struct SelectField {
    label: Label,
    select: Select,
}

impl SelectField {
    fn new(id: u64, label_text: &str, options: Vec<SelectOption>) -> Self {
        let label = Label::new(id, label_text);
        
        let mut select = Select::new(id + 1);
        select.set_options(options);
        select.set_aria_labelled_by(label.id());
        
        Self { label, select }
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.select.selected_value().is_none() {
            Err("Please select an option".to_string())
        } else {
            Ok(())
        }
    }
    
    fn get_value(&self) -> Option<String> {
        self.select.selected_value().map(|s| s.to_string())
    }
}
```

### Disabled Options

```rust
let mut select = Select::new(1);
select.add_option(SelectOption {
    label: "Available Option".to_string(),
    value: "available".to_string(),
    disabled: false,
});
select.add_option(SelectOption {
    label: "Disabled Option".to_string(),
    value: "disabled".to_string(),
    disabled: true,
});
```

### Grouped Options

```rust
struct GroupedSelect {
    select: Select,
    groups: Vec<(String, Vec<SelectOption>)>,
}

impl GroupedSelect {
    fn new(id: u64) -> Self {
        Self {
            select: Select::new(id),
            groups: Vec::new(),
        }
    }
    
    fn add_group(&mut self, name: &str, options: Vec<SelectOption>) {
        self.groups.push((name.to_string(), options));
        
        // Flatten for the select component
        for option in options {
            self.select.add_option(option);
        }
    }
}
```

### Themed Select

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut select = Select::new(1);

select.set_color(theme.colors.text_primary);
select.set_background_color(theme.colors.surface);
select.set_border_color(theme.colors.outline);
```

### Default Selection

```rust
let mut select = Select::new(1);
select.set_options(vec![
    SelectOption::new("Low", "low"),
    SelectOption::new("Medium", "medium"),
    SelectOption::new("High", "high"),
]);

// Select default
select.select(1); // Select "Medium"
```

## Accessibility

Select components are keyboard accessible:

- **Role**: `combobox`
- **Keyboard**: 
  - `Space`/`Enter` to open
  - Arrow keys to navigate
  - `Enter` to select
  - `Esc` to close
- **States**: Open/closed, selected option announced

```rust
select.set_aria_label("Choose a category");
select.set_aria_describedby(help_text_id);
```

## Best Practices

### Clear Labels

```rust
// Good: Descriptive placeholder
select.set_placeholder("Select your country...");

// Avoid: Generic
select.set_placeholder("Select...");
```

### Reasonable Number of Options

```rust
// Good: 3-15 options
let select = create_select_with_options(small_list);

// Consider search for many options
if options.len() > 15 {
    select.set_searchable(true);
}
```

### Default Selection

```rust
// Provide sensible default when appropriate
select.select(0); // Select first option
```

### Validation

```rust
fn validate_select(select: &Select) -> Result<(), String> {
    if select.selected_value().is_none() {
        Err("Please make a selection".to_string())
    } else {
        Ok(())
    }
}
```

## When to Use

### Use Select When:
- Choosing one option from many (7+ options)
- Space is limited
- Options are familiar to users

### Use Radio Buttons When:
- Few options (2-7)
- All options should be visible
- User needs to see options to decide

### Use Autocomplete When:
- Very many options (50+)
- Users know what they're looking for

## Related Components

- [Radio Button](radio.md) - For few options
- [Checkbox](checkbox.md) - For multiple selections
- [Text Input](text-input.md) - For free text entry
- [List](list.md) - For displaying lists

## See Also

- [Forms Guide](../guides/forms.md) - Building forms
- [Accessibility](../guides/accessibility.md) - Accessible selects
- [Validation Guide](../guides/validation.md) - Validation

---

[← Back to Components](index.md)
