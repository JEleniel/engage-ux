# Breadcrumb Component

A navigation component showing the current page's location in the site hierarchy.

## Overview

The Breadcrumb component displays a trail of links showing the user's navigation path, helping them understand their location and navigate back.

## Basic Usage

```rust
use engage_ux_components::{Breadcrumb, BreadcrumbItem};
use engage_ux_core::component::{Component, Rect};

// Create breadcrumb
let mut breadcrumb = Breadcrumb::new(1);
breadcrumb.set_bounds(Rect::new(20.0, 20.0, 600.0, 30.0));

// Add items
breadcrumb.add_item(BreadcrumbItem::new("Home", "/"));
breadcrumb.add_item(BreadcrumbItem::new("Products", "/products"));
breadcrumb.add_item(BreadcrumbItem::new("Electronics", "/products/electronics"));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `items` | `Vec<BreadcrumbItem>` | Breadcrumb items | `[]` |
| `separator` | `String` | Separator character | `"/"` |
| `max_items` | `Option<usize>` | Max visible items | `None` |

## Methods

```rust
// Add item
breadcrumb.add_item(BreadcrumbItem::new("Label", "/path"));

// Set separator
breadcrumb.set_separator(">");

// Set max items
breadcrumb.set_max_items(Some(3));

// Handle click
breadcrumb.set_on_click(|href| {
    navigate_to(href);
});
```

## Examples

### File Path Breadcrumb

```rust
let mut path = Breadcrumb::new(1);
path.set_separator("/");

path.add_item(BreadcrumbItem::new("Home", "~"));
path.add_item(BreadcrumbItem::new("Documents", "~/Documents"));
path.add_item(BreadcrumbItem::new("Projects", "~/Documents/Projects"));
```

### E-commerce Breadcrumb

```rust
let mut nav = Breadcrumb::new(1);
nav.add_item(BreadcrumbItem::new("Home", "/"));
nav.add_item(BreadcrumbItem::new("Electronics", "/electronics"));
nav.add_item(BreadcrumbItem::new("Laptops", "/electronics/laptops"));
nav.add_item(BreadcrumbItem::new("Gaming Laptops", "/electronics/laptops/gaming"));
```

## Accessibility

Breadcrumbs should use proper navigation landmarks:

```rust
breadcrumb.set_aria_label("Breadcrumb navigation");
```

## Related Components

- [Link](link.md) - For navigation links
- [Menu](menu.md) - For navigation menus

---

[← Back to Components](index.md)
