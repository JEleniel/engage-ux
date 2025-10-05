# Text Editor Component

A rich text editor with formatting capabilities.

## Overview

The Text Editor component provides a WYSIWYG editor for formatted text, supporting bold, italic, lists, and other formatting options.

## Basic Usage

```rust
use engage_ux_components::TextEditor;

let mut editor = TextEditor::new(1);
editor.set_bounds(Rect::new(20.0, 20.0, 600.0, 400.0));
editor.set_placeholder("Start typing...");
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `content` | `String` | Editor content | `""` |
| `toolbar_visible` | `bool` | Show formatting toolbar | `true` |
| `read_only` | `bool` | Read-only mode | `false` |
| `max_length` | `Option<usize>` | Max character count | `None` |

## Methods

```rust
// Content
editor.set_content("Initial content");
let content = editor.get_content();

// Formatting
editor.toggle_bold();
editor.toggle_italic();
editor.toggle_underline();
editor.insert_link("https://example.com", "Link text");
editor.insert_image("image.jpg");

// Lists
editor.toggle_bullet_list();
editor.toggle_numbered_list();

// Undo/Redo
editor.undo();
editor.redo();

// Configuration
editor.set_toolbar_visible(true);
editor.set_read_only(false);

// Events
editor.set_on_change(|content| {
    save_draft(content);
});
```

## Examples

### Document Editor

```rust
let mut document = TextEditor::new(1);
document.set_bounds(Rect::new(0.0, 40.0, 800.0, 560.0));

// Auto-save
document.set_on_change(|content| {
    auto_save_document(content);
});
```

### Comment Editor

```rust
let mut comment = TextEditor::new(1);
comment.set_toolbar_visible(true);
comment.set_placeholder("Write a comment...");
comment.set_max_length(Some(1000));
```

## Accessibility

Editors should be keyboard accessible:

```rust
editor.set_aria_label("Rich text editor");
editor.set_role("textbox");
editor.set_aria_multiline(true);
```

## Related Components

- [Text Area](text-area.md) - For plain text
- [Text Input](text-input.md) - For single line

---

[← Back to Components](index.md)
