# Menu Components

Menu components including Dropdown, Drawer, HamburgerMenu, and TitleMenu.

## Overview

Menu components provide navigation and command interfaces in various forms - dropdowns, side drawers, hamburger menus, and title bar menus.

## Dropdown Menu

A dropdown menu that appears below a trigger.

```rust
use engage_ux_components::{Dropdown, MenuItem};

let mut dropdown = Dropdown::new(1);
dropdown.set_trigger_text("Options");

// Add menu items
dropdown.add_item(MenuItem::new("New", "new"));
dropdown.add_item(MenuItem::new("Open", "open"));
dropdown.add_item(MenuItem::separator());
dropdown.add_item(MenuItem::new("Exit", "exit"));

dropdown.set_on_select(|value| {
    match value {
        "new" => create_new(),
        "open" => open_file(),
        "exit" => exit_app(),
        _ => {}
    }
});
```

## Drawer

A side panel that slides in from the edge.

```rust
use engage_ux_components::{Drawer, DrawerPosition};

let mut drawer = Drawer::new(1);
drawer.set_position(DrawerPosition::Left);
drawer.set_width(300.0);

// Add content
drawer.add_child(Box::new(navigation_menu));

// Show/hide
drawer.open();
drawer.close();
drawer.toggle();
```

## Hamburger Menu

A mobile menu with animated icon.

```rust
use engage_ux_components::HamburgerMenu;

let mut hamburger = HamburgerMenu::new(1);

hamburger.set_on_toggle(|is_open| {
    if is_open {
        show_mobile_menu();
    } else {
        hide_mobile_menu();
    }
});
```

## Title Menu

Application menu bar (File, Edit, View, etc.).

```rust
use engage_ux_components::{TitleMenu, MenuItem};

let mut menu = TitleMenu::new(1);

// File menu
menu.add_menu("File", vec![
    MenuItem::new("New", "new"),
    MenuItem::new("Open...", "open"),
    MenuItem::new("Save", "save"),
    MenuItem::separator(),
    MenuItem::new("Exit", "exit"),
]);

// Edit menu
menu.add_menu("Edit", vec![
    MenuItem::new("Undo", "undo"),
    MenuItem::new("Redo", "redo"),
    MenuItem::separator(),
    MenuItem::new("Cut", "cut"),
    MenuItem::new("Copy", "copy"),
    MenuItem::new("Paste", "paste"),
]);
```

## Menu Items

```rust
// Regular item
let item = MenuItem::new("Label", "value");

// With icon
let item = MenuItem::with_icon("Save", "save", "disk-icon");

// With shortcut
let item = MenuItem::with_shortcut("Save", "save", "Ctrl+S");

// Separator
let sep = MenuItem::separator();

// Disabled item
let mut item = MenuItem::new("Disabled", "disabled");
item.set_disabled(true);

// Nested submenu
let mut parent = MenuItem::new("Recent Files", "recent");
parent.add_submenu(vec![
    MenuItem::new("file1.txt", "file1"),
    MenuItem::new("file2.txt", "file2"),
]);
```

## Examples

### Application Menu

```rust
let mut app_menu = TitleMenu::new(1);

app_menu.add_menu("File", vec![
    MenuItem::with_shortcut("New File", "new", "Ctrl+N"),
    MenuItem::with_shortcut("Open File", "open", "Ctrl+O"),
    MenuItem::with_shortcut("Save", "save", "Ctrl+S"),
    MenuItem::separator(),
    MenuItem::new("Exit", "exit"),
]);

app_menu.set_on_select(|value| {
    handle_menu_action(value);
});
```

### Navigation Drawer

```rust
let mut nav_drawer = Drawer::new(1);
nav_drawer.set_position(DrawerPosition::Left);

let mut menu = List::new(2);
menu.add_item(ListItem::new("Dashboard"));
menu.add_item(ListItem::new("Projects"));
menu.add_item(ListItem::new("Settings"));

nav_drawer.add_child(Box::new(menu));
```

## Accessibility

Menus should be keyboard accessible:

```rust
menu.set_aria_label("Main menu");
menu.set_role("menu");
```

## Related Components

- [Button](button.md) - Menu triggers
- [List](list.md) - Menu item lists

---

[← Back to Components](index.md)
