# Line Numbers Component

A component for displaying line numbers alongside code.

## Overview

The Line Numbers component displays line numbers for code editors or text displays, with support for highlighting and custom styling.

## Basic Usage

```rust
use engage_ux_components::LineNumbers;

let mut line_numbers = LineNumbers::new(1);
line_numbers.set_line_count(50);
line_numbers.set_bounds(Rect::new(0.0, 0.0, 50.0, 600.0));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `line_count` | `usize` | Number of lines | `1` |
| `start_line` | `usize` | Starting line number | `1` |
| `highlighted_lines` | `Vec<usize>` | Highlighted line numbers | `[]` |
| `font_size` | `f32` | Font size | `14.0` |

## Methods

```rust
// Set line count
line_numbers.set_line_count(100);

// Set start line
line_numbers.set_start_line(1);

// Highlight lines
line_numbers.highlight_line(42);
line_numbers.clear_highlights();

// Styling
line_numbers.set_font_size(14.0);
line_numbers.set_color(Color::from_hex("#999999").unwrap());
line_numbers.set_highlight_color(Color::from_hex("#FFE082").unwrap());
```

## Examples

### Code Editor Line Numbers

```rust
let mut line_nums = LineNumbers::new(1);
line_nums.set_line_count(code_lines.len());
line_nums.set_font_size(14.0);

// Highlight current line
line_nums.highlight_line(current_line);
```

### Log Viewer Line Numbers

```rust
let mut log_lines = LineNumbers::new(1);
log_lines.set_line_count(log_entries.len());

// Highlight errors
for (i, entry) in log_entries.iter().enumerate() {
    if entry.level == LogLevel::Error {
        log_lines.highlight_line(i + 1);
    }
}
```

## Related Components

- [Text Editor](text-editor.md) - Rich text editor
- [Console](console.md) - Terminal display

---

[← Back to Components](index.md)
