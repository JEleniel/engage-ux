# Carousel Component

A slideshow component for cycling through images or content.

## Overview

The Carousel component displays a rotating series of images or content panels with navigation controls and indicators.

## Basic Usage

```rust
use engage_ux_components::Carousel;

let mut carousel = Carousel::new(1);
carousel.set_bounds(Rect::new(0.0, 0.0, 800.0, 400.0));

// Add slides
carousel.add_slide(image1);
carousel.add_slide(image2);
carousel.add_slide(image3);

// Configure
carousel.set_auto_play(true);
carousel.set_interval(5000); // 5 seconds
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `slides` | `Vec<Box<dyn Component>>` | Carousel slides | `[]` |
| `current_index` | `usize` | Current slide index | `0` |
| `auto_play` | `bool` | Auto-advance slides | `false` |
| `interval` | `u32` | Auto-play interval (ms) | `3000` |
| `show_indicators` | `bool` | Show slide indicators | `true` |
| `show_controls` | `bool` | Show prev/next buttons | `true` |

## Methods

```rust
// Add slides
carousel.add_slide(content);

// Navigation
carousel.next();
carousel.previous();
carousel.go_to(2);

// Configuration
carousel.set_auto_play(true);
carousel.set_interval(4000);
carousel.set_show_indicators(true);
carousel.set_show_controls(true);

// Events
carousel.set_on_slide_change(|index| {
    println!("Now showing slide {}", index);
});
```

## Examples

### Image Carousel

```rust
let mut carousel = Carousel::new(1);

for image_src in &image_urls {
    let img = Image::new(id, image_src);
    carousel.add_slide(Box::new(img));
}

carousel.set_auto_play(true);
carousel.set_interval(5000);
```

### Testimonial Carousel

```rust
let mut testimonials = Carousel::new(1);
testimonials.set_auto_play(true);
testimonials.set_interval(8000);

for testimonial in &testimonials_data {
    let card = create_testimonial_card(testimonial);
    testimonials.add_slide(Box::new(card));
}
```

## Accessibility

Carousels should be keyboard accessible and announce changes:

```rust
carousel.set_aria_label("Image carousel");
carousel.set_aria_live("polite");
```

## Related Components

- [Image](image.md) - For image slides
- [Card](card.md) - For content slides

---

[← Back to Components](index.md)
