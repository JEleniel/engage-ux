# Avatar Component

A component for displaying user avatars with image, initials, or icon.

## Overview

The Avatar component displays a user's profile picture, initials, or an icon. It supports different shapes, sizes, and states.

## Basic Usage

```rust
use engage_ux_components::{Avatar, AvatarMode};
use engage_ux_core::component::{Component, Rect};

// Create an avatar with image
let mut avatar = Avatar::new(1);
avatar.set_mode(AvatarMode::Image("/users/profile.jpg".to_string()));
avatar.set_bounds(Rect::new(20.0, 20.0, 48.0, 48.0));
```

## Modes

```rust
use engage_ux_components::AvatarMode;

// Image mode
avatar.set_mode(AvatarMode::Image("path/to/image.jpg".to_string()));

// Initials mode
avatar.set_mode(AvatarMode::Initials("JD".to_string()));

// Icon mode
avatar.set_mode(AvatarMode::Icon("user".to_string()));
```

## Shapes

```rust
use engage_ux_components::AvatarShape;

// Circular (default)
avatar.set_shape(AvatarShape::Circle);

// Square
avatar.set_shape(AvatarShape::Square);

// Rounded square
avatar.set_shape(AvatarShape::RoundedSquare);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `mode` | `AvatarMode` | Display mode | `Icon("user")` |
| `shape` | `AvatarShape` | Avatar shape | `Circle` |
| `size` | `f32` | Avatar size | `48.0` |
| `border_width` | `f32` | Border width | `0.0` |
| `border_color` | `Color` | Border color | Transparent |
| `background_color` | `Color` | Background color | Theme primary |

## Methods

```rust
// Set mode
avatar.set_mode(AvatarMode::Initials("AB".to_string()));

// Set shape
avatar.set_shape(AvatarShape::Circle);

// Set size
avatar.set_size(64.0);

// Set border
avatar.set_border_width(2.0);
avatar.set_border_color(Color::from_hex("#FFFFFF").unwrap());

// Set background
avatar.set_background_color(Color::from_hex("#1976D2").unwrap());
```

## Examples

### Image Avatar

```rust
let mut avatar = Avatar::new(1);
avatar.set_mode(AvatarMode::Image("/users/jane.jpg".to_string()));
avatar.set_shape(AvatarShape::Circle);
avatar.set_size(48.0);
avatar.set_border_width(2.0);
avatar.set_border_color(Color::from_hex("#FFFFFF").unwrap());
```

### Initials Avatar

```rust
let mut avatar = Avatar::new(1);
avatar.set_mode(AvatarMode::Initials("JD".to_string()));
avatar.set_background_color(Color::from_hex("#4CAF50").unwrap());
avatar.set_shape(AvatarShape::Circle);
```

### Avatar with Badge

```rust
struct AvatarWithBadge {
    avatar: Avatar,
    badge: Badge,
}

impl AvatarWithBadge {
    fn new(id: u64, image_src: &str, badge_count: usize) -> Self {
        let mut avatar = Avatar::new(id);
        avatar.set_mode(AvatarMode::Image(image_src.to_string()));
        
        let mut badge = Badge::new(id + 1, &badge_count.to_string());
        badge.set_variant(BadgeVariant::Error);
        
        Self { avatar, badge }
    }
}
```

### Avatar Group

```rust
fn create_avatar_group(users: &[(&str, &str)]) -> Vec<Avatar> {
    users
        .iter()
        .enumerate()
        .map(|(i, (name, image))| {
            let mut avatar = Avatar::new(i as u64);
            avatar.set_mode(AvatarMode::Image((*image).to_string()));
            avatar.set_size(32.0);
            avatar.set_border_width(2.0);
            avatar
        })
        .collect()
}
```

## Accessibility

Avatars should have accessible names:

```rust
avatar.set_aria_label("Jane Doe");
avatar.set_alt_text("Profile picture of Jane Doe");
```

## Related Components

- [Image](image.md) - For general images
- [Badge](badge.md) - Often combined with avatars
- [Icon](icon.md) - For icon avatars

---

[← Back to Components](index.md)
