# Tabs Component

A tabbed interface for organizing content into separate views.

## Overview

The Tabs component allows users to switch between different content sections. Only one tab's content is visible at a time.

## Basic Usage

```rust
use engage_ux_components::{Tabs, Tab};
use engage_ux_core::component::{Component, Rect};

// Create tabs
let mut tabs = Tabs::new(1);
tabs.set_bounds(Rect::new(20.0, 20.0, 600.0, 400.0));

// Add tabs
tabs.add_tab(Tab::new("Home", "Home content"));
tabs.add_tab(Tab::new("Profile", "Profile content"));
tabs.add_tab(Tab::new("Settings", "Settings content"));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `tabs` | `Vec<Tab>` | Tab definitions | `[]` |
| `active_index` | `usize` | Active tab index | `0` |
| `closeable` | `bool` | Tabs can be closed | `false` |
| `overflow` | `TabOverflow` | Handle overflow | `Scroll` |

## Methods

```rust
// Add tab
tabs.add_tab(Tab::new("Label", "Content"));

// Set active tab
tabs.set_active(1);

// Get active index
let active = tabs.active_index();

// Remove tab
tabs.remove_tab(2);

// Enable closeable tabs
tabs.set_closeable(true);
```

## Examples

### Document Tabs

```rust
let mut docs = Tabs::new(1);
docs.set_closeable(true);

docs.add_tab(Tab::new("Document 1", "Content 1"));
docs.add_tab(Tab::new("Document 2", "Content 2"));

docs.set_on_close(|index| {
    println!("Closed tab {}", index);
});
```

### Settings Tabs

```rust
let mut settings = Tabs::new(1);
settings.add_tab(Tab::new("General", general_panel));
settings.add_tab(Tab::new("Privacy", privacy_panel));
settings.add_tab(Tab::new("Security", security_panel));
```

## Related Components

- [Accordion](accordion.md) - For expandable panels
- [Card](card.md) - For content cards

---

[← Back to Components](index.md)
