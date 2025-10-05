# Title Menu Component

An application menu bar component (File, Edit, View, etc.).

## Overview

The Title Menu component provides a traditional application menu bar with dropdown menus, commonly found in desktop applications.

## Basic Usage

```rust
use engage_ux_components::{TitleMenu, MenuItem};

let mut menu_bar = TitleMenu::new(1);
menu_bar.set_bounds(Rect::new(0.0, 0.0, 800.0, 30.0));

// Add menus
menu_bar.add_menu("File", vec![
    MenuItem::new("New", "new"),
    MenuItem::new("Open...", "open"),
    MenuItem::new("Save", "save"),
    MenuItem::separator(),
    MenuItem::new("Exit", "exit"),
]);

menu_bar.set_on_select(|action| {
    handle_menu_action(action);
});
```

## Menu Structure

```rust
// File menu
menu_bar.add_menu("File", vec![
    MenuItem::with_shortcut("New File", "new", "Ctrl+N"),
    MenuItem::with_shortcut("Open File...", "open", "Ctrl+O"),
    MenuItem::with_shortcut("Save", "save", "Ctrl+S"),
    MenuItem::with_shortcut("Save As...", "save_as", "Ctrl+Shift+S"),
    MenuItem::separator(),
    MenuItem::new("Recent Files", "recent"),
    MenuItem::separator(),
    MenuItem::new("Exit", "exit"),
]);

// Edit menu
menu_bar.add_menu("Edit", vec![
    MenuItem::with_shortcut("Undo", "undo", "Ctrl+Z"),
    MenuItem::with_shortcut("Redo", "redo", "Ctrl+Y"),
    MenuItem::separator(),
    MenuItem::with_shortcut("Cut", "cut", "Ctrl+X"),
    MenuItem::with_shortcut("Copy", "copy", "Ctrl+C"),
    MenuItem::with_shortcut("Paste", "paste", "Ctrl+V"),
]);

// View menu
menu_bar.add_menu("View", vec![
    MenuItem::new("Zoom In", "zoom_in"),
    MenuItem::new("Zoom Out", "zoom_out"),
    MenuItem::new("Reset Zoom", "zoom_reset"),
    MenuItem::separator(),
    MenuItem::new("Fullscreen", "fullscreen"),
]);

// Help menu
menu_bar.add_menu("Help", vec![
    MenuItem::new("Documentation", "docs"),
    MenuItem::new("About", "about"),
]);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `menus` | `Vec<(String, Vec<MenuItem>)>` | Menu definitions | `[]` |
| `active_menu` | `Option<usize>` | Currently open menu | `None` |

## Methods

```rust
// Add menu
menu_bar.add_menu("File", file_menu_items);

// Get menu
let menu = menu_bar.get_menu("File");

// Handle selection
menu_bar.set_on_select(|action| {
    match action {
        "new" => create_new_file(),
        "open" => open_file(),
        "save" => save_file(),
        _ => {}
    }
});
```

## Examples

### Full Application Menu

```rust
fn create_app_menu() -> TitleMenu {
    let mut menu_bar = TitleMenu::new(1);
    
    // File
    menu_bar.add_menu("File", vec![
        MenuItem::with_shortcut("New", "new", "Ctrl+N"),
        MenuItem::with_shortcut("Open...", "open", "Ctrl+O"),
        MenuItem::separator(),
        MenuItem::new("Exit", "exit"),
    ]);
    
    // Edit
    menu_bar.add_menu("Edit", vec![
        MenuItem::with_shortcut("Undo", "undo", "Ctrl+Z"),
        MenuItem::with_shortcut("Redo", "redo", "Ctrl+Y"),
    ]);
    
    // Help
    menu_bar.add_menu("Help", vec![
        MenuItem::new("About", "about"),
    ]);
    
    menu_bar
}
```

### Disabled Menu Items

```rust
let mut save_item = MenuItem::new("Save", "save");
if !has_unsaved_changes {
    save_item.set_disabled(true);
}
```

## Accessibility

Menu bars should be keyboard accessible:

```rust
menu_bar.set_aria_label("Application menu");
menu_bar.set_role("menubar");
```

## Related Components

- [Menu](menu.md) - Dropdown menus
- [Window](window.md) - Application windows

---

[← Back to Components](index.md)
