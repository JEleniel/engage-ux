# Link Component

A clickable hyperlink component for navigation.

## Overview

The Link component provides clickable text links for navigation within the application or to external URLs.

## Basic Usage

```rust
use engage_ux_components::Link;
use engage_ux_core::component::{Component, Rect};

// Create a link
let mut link = Link::new(1, "Learn More");
link.set_href("https://example.com");
link.set_bounds(Rect::new(20.0, 20.0, 100.0, 20.0));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `text` | `String` | Link text | Required |
| `href` | `String` | Target URL | `""` |
| `target` | `LinkTarget` | Where to open link | `SameWindow` |
| `underline` | `bool` | Show underline | `true` |
| `color` | `Color` | Link color | `#1976D2` |
| `visited_color` | `Color` | Visited link color | `#9C27B0` |
| `hover_color` | `Color` | Hover color | `#1565C0` |

## Methods

```rust
// Set target URL
link.set_href("/page");

// Set link text
link.set_text("Click here");

// Set target
link.set_target(LinkTarget::NewWindow);

// Styling
link.set_underline(true);
link.set_color(Color::from_hex("#1976D2").unwrap());
```

## Examples

### External Link

```rust
let mut external = Link::new(1, "Visit our website");
external.set_href("https://example.com");
external.set_target(LinkTarget::NewWindow);
```

### Internal Navigation

```rust
let mut nav_link = Link::new(1, "Go to Settings");
nav_link.set_href("/settings");
nav_link.set_target(LinkTarget::SameWindow);
```

### Styled Link

```rust
let mut link = Link::new(1, "Important Link");
link.set_color(Color::from_hex("#D32F2F").unwrap());
link.set_underline(false);
```

## Accessibility

Links are keyboard accessible and should have descriptive text:

```rust
// Good: Descriptive
link.set_text("Read the full documentation");

// Avoid: Generic
link.set_text("Click here");
```

## Related Components

- [Button](button.md) - For actions
- [Text](text.md) - For styled text
- [Label](label.md) - For static text

---

[← Back to Components](index.md)
