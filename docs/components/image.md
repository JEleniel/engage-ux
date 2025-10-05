# Image Component

A component for displaying images with various fit modes and accessibility features.

## Overview

The Image component provides flexible image display with support for different fit modes, lazy loading, and accessibility features including alt text.

## Basic Usage

```rust
use engage_ux_components::Image;
use engage_ux_core::component::{Component, Rect};

// Create an image
let mut image = Image::new(1, "https://example.com/photo.jpg");
image.set_bounds(Rect::new(20.0, 20.0, 400.0, 300.0));

// Set alt text for accessibility
image.set_alt_text("A beautiful sunset over the ocean");
```

## Fit Modes

Images can be displayed with different fit modes:

```rust
use engage_ux_components::ImageFit;

// Cover: Fill the entire bounds, may crop
image.set_fit(ImageFit::Cover);

// Contain: Fit within bounds, maintain aspect ratio
image.set_fit(ImageFit::Contain);

// Fill: Stretch to fill bounds, may distort
image.set_fit(ImageFit::Fill);

// None: Display at original size
image.set_fit(ImageFit::None);

// ScaleDown: Like contain, but never scale up
image.set_fit(ImageFit::ScaleDown);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `source` | `String` | Image URL or path | Required |
| `alt_text` | `String` | Alt text for accessibility | `""` |
| `width` | `Option<f32>` | Image width (pixels) | `None` |
| `height` | `Option<f32>` | Image height (pixels) | `None` |
| `fit` | `ImageFit` | How image fits in bounds | `Cover` |
| `lazy` | `bool` | Enable lazy loading | `false` |

## Methods

### Source

```rust
// Get source URL
let src = image.source();

// Set source URL
image.set_source("https://example.com/new-image.jpg");
image.set_source("/assets/local-image.png");
```

### Alt Text

```rust
// Get alt text
let alt = image.alt_text();

// Set alt text (important for accessibility)
image.set_alt_text("Profile photo of Jane Doe");
```

### Dimensions

```rust
// Get dimensions
let width = image.width();  // Option<f32>
let height = image.height(); // Option<f32>

// Set dimensions
image.set_width(Some(400.0));
image.set_height(Some(300.0));

// Clear dimensions (use natural size)
image.set_width(None);
image.set_height(None);
```

### Fit Mode

```rust
// Get fit mode
let fit = image.fit();

// Set fit mode
image.set_fit(ImageFit::Contain);
```

### Lazy Loading

```rust
// Enable lazy loading
image.set_lazy(true);

// Check if lazy
if image.is_lazy() {
    println!("Image will be lazy loaded");
}
```

## Examples

### Profile Picture

```rust
let mut avatar = Image::new(1, "/users/profile.jpg");
avatar.set_bounds(Rect::new(20.0, 20.0, 100.0, 100.0));
avatar.set_fit(ImageFit::Cover);
avatar.set_alt_text("User profile picture");
```

### Responsive Image

```rust
fn create_responsive_image(
    id: u64,
    src: &str,
    alt: &str,
    screen_width: f32,
) -> Image {
    let mut image = Image::new(id, src);
    image.set_alt_text(alt);
    
    // Adjust size based on screen
    let width = if screen_width < 768.0 {
        screen_width - 40.0 // Mobile
    } else {
        600.0 // Desktop
    };
    
    image.set_width(Some(width));
    image.set_fit(ImageFit::Contain);
    image
}
```

### Gallery Image

```rust
struct GalleryImage {
    thumbnail: Image,
    full_size: Image,
}

impl GalleryImage {
    fn new(id: u64, thumb_src: &str, full_src: &str, alt: &str) -> Self {
        let mut thumbnail = Image::new(id, thumb_src);
        thumbnail.set_bounds(Rect::new(0.0, 0.0, 150.0, 150.0));
        thumbnail.set_fit(ImageFit::Cover);
        thumbnail.set_alt_text(alt);
        
        let mut full_size = Image::new(id + 1, full_src);
        full_size.set_fit(ImageFit::Contain);
        full_size.set_alt_text(alt);
        
        Self { thumbnail, full_size }
    }
}
```

### Lazy Loaded Images

```rust
fn create_lazy_image_list(images: &[(&str, &str)]) -> Vec<Image> {
    images
        .iter()
        .enumerate()
        .map(|(i, (src, alt))| {
            let mut img = Image::new(i as u64, *src);
            img.set_alt_text(*alt);
            img.set_lazy(true);
            img.set_fit(ImageFit::Cover);
            img
        })
        .collect()
}
```

### Placeholder Image

```rust
struct ImageWithPlaceholder {
    image: Image,
    placeholder_src: String,
    loaded: bool,
}

impl ImageWithPlaceholder {
    fn new(id: u64, src: &str, placeholder: &str, alt: &str) -> Self {
        let mut image = Image::new(id, placeholder);
        image.set_alt_text(alt);
        image.set_fit(ImageFit::Cover);
        
        Self {
            image,
            placeholder_src: placeholder.to_string(),
            loaded: false,
        }
    }
    
    async fn load(&mut self, actual_src: &str) {
        // Load the actual image
        self.image.set_source(actual_src);
        self.loaded = true;
    }
}
```

## Accessibility

Images require proper alt text for accessibility:

```rust
// Decorative image
image.set_alt_text(""); // Empty for decorative
image.set_aria_hidden(true);

// Informative image
image.set_alt_text("Bar chart showing sales growth from 2020 to 2024");

// Functional image (e.g., button)
image.set_alt_text("Submit form");
image.set_role("button");
```

### Alt Text Guidelines

- Be descriptive but concise
- Describe the content and function
- Don't start with "Image of..." or "Picture of..."
- For decorative images, use empty alt text

```rust
// Good alt text
image.set_alt_text("Mountain landscape at sunset");

// Avoid
image.set_alt_text("Image of a mountain landscape at sunset");
```

## Best Practices

### Always Provide Alt Text

```rust
// Good
image.set_alt_text("Company logo");

// Bad
// No alt text provided
```

### Choose Appropriate Fit Mode

```rust
// Profile pictures: Cover
avatar.set_fit(ImageFit::Cover);

// Product photos: Contain
product.set_fit(ImageFit::Contain);

// Backgrounds: Cover
background.set_fit(ImageFit::Cover);

// Icons/diagrams: ScaleDown
icon.set_fit(ImageFit::ScaleDown);
```

### Optimize Image Sizes

```rust
// Provide appropriate sizes
thumbnail.set_width(Some(150.0));
thumbnail.set_height(Some(150.0));

// Use lazy loading for below-fold images
image.set_lazy(true);
```

### Handle Loading States

```rust
enum ImageState {
    Loading,
    Loaded,
    Error,
}

struct ManagedImage {
    image: Image,
    state: ImageState,
}
```

## Related Components

- [Icon](icon.md) - For vector icons
- [Avatar](avatar.md) - For user avatars
- [Video](video.md) - For video content
- [Card](card.md) - Often contains images

## See Also

- [Accessibility](../guides/accessibility.md) - Accessible images
- [Theming Guide](../guides/theming.md) - Styling images
- [Performance](../guides/performance.md) - Optimizing images

---

[← Back to Components](index.md)
