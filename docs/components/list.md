# List Component

A component for displaying a list of selectable items.

## Overview

The List component displays a collection of items in a vertical or horizontal layout, supporting single or multiple selection, custom items, and styling.

## Basic Usage

```rust
use engage_ux_components::{List, ListItem};
use engage_ux_core::component::{Component, Rect};

// Create a list
let mut list = List::new(1);
list.set_bounds(Rect::new(20.0, 20.0, 300.0, 400.0));

// Add items
list.add_item(ListItem::new("Item 1"));
list.add_item(ListItem::new("Item 2"));
list.add_item(ListItem::new("Item 3"));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `items` | `Vec<ListItem>` | List items | `[]` |
| `selected_indices` | `Vec<usize>` | Selected item indices | `[]` |
| `multi_select` | `bool` | Allow multiple selection | `false` |
| `item_height` | `f32` | Height per item | `44.0` |

## Methods

```rust
// Add items
list.add_item(ListItem::new("Item"));

// Set all items
list.set_items(vec![
    ListItem::new("First"),
    ListItem::new("Second"),
    ListItem::new("Third"),
]);

// Selection
list.select(0); // Select first item
list.select_multiple(vec![0, 2]); // Select multiple
list.clear_selection();

// Get selected
let selected = list.selected_indices();
let values = list.selected_values();

// Configuration
list.set_multi_select(true);
list.set_item_height(50.0);
```

## Examples

### Simple List

```rust
let mut list = List::new(1);
list.set_items(vec![
    ListItem::new("Apple"),
    ListItem::new("Banana"),
    ListItem::new("Cherry"),
]);

list.set_on_select(|index| {
    println!("Selected item {}", index);
});
```

### Multi-Select List

```rust
let mut list = List::new(1);
list.set_multi_select(true);
list.set_items(vec![
    ListItem::new("Option 1"),
    ListItem::new("Option 2"),
    ListItem::new("Option 3"),
]);

// Later, get all selected
let selected = list.selected_indices();
```

### Themed List

```rust
use engage_ux_themes::Theme;

let theme = Theme::dark();
let mut list = List::new(1);

list.set_background_color(theme.colors.surface);
list.set_text_color(theme.colors.text_primary);
list.set_selected_color(theme.colors.primary);
```

## Accessibility

Lists are keyboard navigable:

- **Role**: `listbox`
- **Keyboard**: Arrow keys to navigate, Space/Enter to select
- **Multi-select**: Shift+Arrow for range, Ctrl+Click for individual

```rust
list.set_aria_label("Select items");
list.set_aria_multiselectable(true);
```

## Related Components

- [Select](select.md) - Dropdown selection
- [Checkbox](checkbox.md) - For multiple choices
- [Radio Button](radio.md) - For single choice

---

[← Back to Components](index.md)
