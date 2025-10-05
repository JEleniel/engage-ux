# Table Component

A data table component with sorting, filtering, and selection capabilities.

## Overview

The Table component displays data in rows and columns with support for sorting, filtering, pagination, and row selection.

## Basic Usage

```rust
use engage_ux_components::Table;
use engage_ux_core::component::{Component, Rect};

// Create a table
let mut table = Table::new(1);
table.set_bounds(Rect::new(20.0, 20.0, 800.0, 600.0));

// Set columns
table.set_columns(vec!["Name", "Email", "Status"]);

// Add rows
table.add_row(vec!["John Doe", "john@example.com", "Active"]);
table.add_row(vec!["Jane Smith", "jane@example.com", "Active"]);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `columns` | `Vec<String>` | Column headers | `[]` |
| `rows` | `Vec<Vec<String>>` | Table data | `[]` |
| `sortable` | `bool` | Enable sorting | `true` |
| `selectable` | `bool` | Enable row selection | `false` |
| `paginated` | `bool` | Enable pagination | `false` |

## Methods

```rust
// Set columns
table.set_columns(vec!["Col1", "Col2", "Col3"]);

// Add row
table.add_row(vec!["Data1", "Data2", "Data3"]);

// Sort by column
table.sort_by_column(0, true); // ascending

// Enable features
table.set_sortable(true);
table.set_selectable(true);
table.set_paginated(true);

// Set page size
table.set_page_size(20);
```

## Examples

### User Table

```rust
let mut users = Table::new(1);
users.set_columns(vec!["ID", "Name", "Email", "Role"]);
users.set_sortable(true);
users.set_selectable(true);

for user in user_list {
    users.add_row(vec![
        &user.id.to_string(),
        &user.name,
        &user.email,
        &user.role,
    ]);
}
```

### Paginated Table

```rust
let mut table = Table::new(1);
table.set_paginated(true);
table.set_page_size(25);

table.set_on_page_change(|page| {
    load_page_data(page);
});
```

## Related Components

- [List](list.md) - For simple lists
- [Card](card.md) - For card layouts

---

[← Back to Components](index.md)
