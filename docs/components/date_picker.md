# Date Picker Component

A component for selecting dates from a calendar interface.

## Overview

The Date Picker component provides a calendar interface for selecting dates, with support for date ranges, min/max dates, and custom formatting.

## Basic Usage

```rust
use engage_ux_components::DatePicker;
use chrono::NaiveDate;

let mut date_picker = DatePicker::new(1);
date_picker.set_placeholder("Select a date");

// Handle date selection
date_picker.set_on_select(|date| {
    println!("Selected: {}", date);
});
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `selected_date` | `Option<NaiveDate>` | Selected date | `None` |
| `min_date` | `Option<NaiveDate>` | Minimum selectable date | `None` |
| `max_date` | `Option<NaiveDate>` | Maximum selectable date | `None` |
| `format` | `String` | Date display format | `"YYYY-MM-DD"` |
| `inline` | `bool` | Show inline (not popup) | `false` |

## Methods

```rust
// Set selected date
date_picker.set_selected_date(Some(NaiveDate::from_ymd(2024, 1, 15)));

// Set date range
date_picker.set_min_date(Some(NaiveDate::from_ymd(2024, 1, 1)));
date_picker.set_max_date(Some(NaiveDate::from_ymd(2024, 12, 31)));

// Set format
date_picker.set_format("MM/DD/YYYY");

// Inline mode
date_picker.set_inline(true);

// Events
date_picker.set_on_select(|date| {
    handle_date_selection(date);
});
```

## Examples

### Birthday Picker

```rust
let mut birthday = DatePicker::new(1);
birthday.set_placeholder("Select your birthday");

// Limit to reasonable date range
birthday.set_min_date(Some(NaiveDate::from_ymd(1920, 1, 1)));
birthday.set_max_date(Some(chrono::Local::now().naive_local().date()));
```

### Appointment Picker

```rust
let mut appointment = DatePicker::new(1);

// Only allow future dates
appointment.set_min_date(Some(chrono::Local::now().naive_local().date()));

// Format for US locale
appointment.set_format("MM/DD/YYYY");
```

### Inline Calendar

```rust
let mut calendar = DatePicker::new(1);
calendar.set_inline(true);
calendar.set_bounds(Rect::new(20.0, 20.0, 300.0, 320.0));
```

## Accessibility

Date pickers should be keyboard accessible:

```rust
date_picker.set_aria_label("Select date");
date_picker.set_role("application");
```

## Related Components

- [Text Input](text-input.md) - For text entry
- [Select](select.md) - For dropdown selection

---

[← Back to Components](index.md)
