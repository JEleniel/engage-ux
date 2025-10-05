# Accordion Component

A vertically stacked set of expandable/collapsible panels.

## Overview

The Accordion component displays collapsible content panels. It's useful for organizing content into sections that users can expand or collapse as needed.

## Basic Usage

```rust
use engage_ux_components::{Accordion, AccordionPanel};
use engage_ux_core::component::{Component, Rect};

// Create an accordion
let mut accordion = Accordion::new(1);
accordion.set_bounds(Rect::new(20.0, 20.0, 400.0, 600.0));

// Add panels
accordion.add_panel(AccordionPanel::new(
    "Section 1",
    "Content for section 1"
));
accordion.add_panel(AccordionPanel::new(
    "Section 2",
    "Content for section 2"
));
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `panels` | `Vec<AccordionPanel>` | Accordion panels | `[]` |
| `allow_multiple` | `bool` | Multiple panels open | `false` |
| `expanded_indices` | `Vec<usize>` | Expanded panel indices | `[]` |

## Methods

```rust
// Add panel
accordion.add_panel(AccordionPanel::new("Title", "Content"));

// Expand panel
accordion.expand(0); // Expand first panel

// Collapse panel
accordion.collapse(0);

// Toggle panel
accordion.toggle(0);

// Allow multiple open
accordion.set_allow_multiple(true);
```

## Examples

### FAQ Accordion

```rust
let mut faq = Accordion::new(1);
faq.add_panel(AccordionPanel::new(
    "What is Engage UX?",
    "Engage UX is a cross-platform UI toolkit..."
));
faq.add_panel(AccordionPanel::new(
    "How do I get started?",
    "Start by adding the dependencies to your Cargo.toml..."
));
```

### Settings Accordion

```rust
let mut settings = Accordion::new(1);
settings.set_allow_multiple(true); // Multiple sections open

settings.add_panel(AccordionPanel::new(
    "General Settings",
    "General preferences content..."
));
settings.add_panel(AccordionPanel::new(
    "Privacy Settings",
    "Privacy options content..."
));
```

## Related Components

- [Tabs](tabs.md) - For tabbed content
- [Card](card.md) - For card layouts

---

[← Back to Components](index.md)
