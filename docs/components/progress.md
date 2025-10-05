# Progress Component

A component for displaying progress indicators in linear or circular format.

## Overview

The Progress component shows the completion status of an operation. It supports linear (horizontal bar), circular (spinner), and indeterminate modes.

## Basic Usage

```rust
use engage_ux_components::{Progress, ProgressType};
use engage_ux_core::component::{Component, Rect};

// Create a linear progress bar
let mut progress = Progress::new(1);
progress.set_value(0.75); // 75% complete
progress.set_bounds(Rect::new(20.0, 20.0, 300.0, 8.0));
```

## Progress Types

```rust
use engage_ux_components::ProgressType;

// Linear horizontal bar
progress.set_type(ProgressType::Linear);

// Circular spinner
progress.set_type(ProgressType::Circular);

// Indeterminate (unknown duration)
progress.set_type(ProgressType::Indeterminate);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `value` | `f32` | Progress value (0.0 to 1.0) | `0.0` |
| `type` | `ProgressType` | Display type | `Linear` |
| `color` | `Color` | Progress color | Theme primary |
| `background_color` | `Color` | Track color | Theme surface |
| `show_label` | `bool` | Show percentage label | `false` |

## Methods

```rust
// Set value (0.0 to 1.0)
progress.set_value(0.5); // 50%

// Set type
progress.set_type(ProgressType::Circular);

// Show/hide label
progress.set_show_label(true);

// Styling
progress.set_color(Color::from_hex("#4CAF50").unwrap());
progress.set_background_color(Color::from_hex("#E0E0E0").unwrap());
```

## Examples

### File Upload Progress

```rust
let mut upload_progress = Progress::new(1);
upload_progress.set_type(ProgressType::Linear);
upload_progress.set_show_label(true);

// Update during upload
upload_progress.set_value(bytes_uploaded as f32 / total_bytes as f32);

// Show percentage
let percent = (upload_progress.value() * 100.0) as i32;
println!("{}% complete", percent);
```

### Loading Spinner

```rust
let mut spinner = Progress::new(1);
spinner.set_type(ProgressType::Indeterminate);
spinner.set_bounds(Rect::new(150.0, 150.0, 40.0, 40.0));
```

### Circular Progress

```rust
let mut circular = Progress::new(1);
circular.set_type(ProgressType::Circular);
circular.set_value(0.65); // 65%
circular.set_show_label(true);
```

### Multi-Step Progress

```rust
struct MultiStepProgress {
    progress: Progress,
    steps: Vec<String>,
    current_step: usize,
}

impl MultiStepProgress {
    fn new(id: u64, steps: Vec<String>) -> Self {
        let mut progress = Progress::new(id);
        progress.set_type(ProgressType::Linear);
        progress.set_show_label(true);
        
        Self {
            progress,
            steps,
            current_step: 0,
        }
    }
    
    fn update(&mut self, step: usize) {
        self.current_step = step;
        let value = step as f32 / self.steps.len() as f32;
        self.progress.set_value(value);
    }
}
```

## Accessibility

Progress indicators should have accessible labels:

```rust
progress.set_aria_label("Upload progress");
progress.set_aria_valuenow(75.0);
progress.set_aria_valuemin(0.0);
progress.set_aria_valuemax(100.0);
```

## Related Components

- [Slider](slider.md) - For user-controlled values
- [Spinner](spinner.md) - For loading states

---

[← Back to Components](index.md)
