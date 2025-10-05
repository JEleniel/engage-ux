# Pagination Component

A component for navigating through pages of content.

## Overview

The Pagination component provides controls for navigating through paginated content, displaying page numbers and next/previous buttons.

## Basic Usage

```rust
use engage_ux_components::Pagination;
use engage_ux_core::component::{Component, Rect};

// Create pagination
let mut pagination = Pagination::new(1);
pagination.set_bounds(Rect::new(20.0, 20.0, 400.0, 40.0));

// Configure
pagination.set_total_pages(10);
pagination.set_current_page(1);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `total_pages` | `usize` | Total number of pages | `1` |
| `current_page` | `usize` | Current page (1-based) | `1` |
| `page_size` | `usize` | Items per page | `10` |
| `max_buttons` | `usize` | Max page buttons shown | `7` |

## Methods

```rust
// Set total pages
pagination.set_total_pages(20);

// Set current page
pagination.set_current_page(5);

// Navigate
pagination.next_page();
pagination.previous_page();
pagination.first_page();
pagination.last_page();

// Handle page change
pagination.set_on_page_change(|page| {
    load_page(page);
});
```

## Examples

### Simple Pagination

```rust
let mut pagination = Pagination::new(1);
pagination.set_total_pages(15);
pagination.set_current_page(1);

pagination.set_on_page_change(|page| {
    fetch_data(page).await;
});
```

### Table Pagination

```rust
struct PaginatedTable {
    table: Table,
    pagination: Pagination,
    total_items: usize,
    page_size: usize,
}

impl PaginatedTable {
    fn new(id: u64, page_size: usize) -> Self {
        let table = Table::new(id);
        let mut pagination = Pagination::new(id + 1);
        pagination.set_page_size(page_size);
        
        Self {
            table,
            pagination,
            total_items: 0,
            page_size,
        }
    }
    
    fn set_total_items(&mut self, total: usize) {
        self.total_items = total;
        let pages = (total + self.page_size - 1) / self.page_size;
        self.pagination.set_total_pages(pages);
    }
}
```

## Accessibility

Pagination should be keyboard navigable:

```rust
pagination.set_aria_label("Pagination navigation");
```

## Related Components

- [Table](table.md) - Often used with tables
- [List](list.md) - For paginated lists

---

[← Back to Components](index.md)
