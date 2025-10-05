# Banner Component

A persistent message component displayed at the top or bottom of the interface.

## Overview

The Banner component displays important, persistent messages to users. Unlike toasts, banners remain visible until explicitly dismissed and can contain actions.

## Basic Usage

```rust
use engage_ux_components::{Banner, BannerType};
use engage_ux_core::component::{Component, Rect};

// Create a banner
let mut banner = Banner::new(1, "System update available");
banner.set_type(BannerType::Info);
banner.set_dismissible(true);
```

## Types

```rust
use engage_ux_components::BannerType;

// Information (blue)
banner.set_type(BannerType::Info);

// Warning (yellow)
banner.set_type(BannerType::Warning);

// Error (red)
banner.set_type(BannerType::Error);

// Success (green)
banner.set_type(BannerType::Success);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `message` | `String` | Banner message | Required |
| `type` | `BannerType` | Message type | `Info` |
| `dismissible` | `bool` | Can be dismissed | `true` |
| `icon` | `Option<String>` | Icon name | Auto |
| `actions` | `Vec<String>` | Action button labels | `[]` |

## Methods

```rust
// Set message
banner.set_message("New features available");

// Set type
banner.set_type(BannerType::Success);

// Set dismissible
banner.set_dismissible(true);

// Add actions
banner.add_action("Update Now");
banner.add_action("Learn More");
```

## Examples

### Info Banner

```rust
let mut banner = Banner::new(1, "Cookies help us deliver our services.");
banner.set_type(BannerType::Info);
banner.add_action("Accept");
banner.add_action("Learn More");
```

### Warning Banner

```rust
let mut banner = Banner::new(1, "Your session will expire in 5 minutes");
banner.set_type(BannerType::Warning);
banner.add_action("Extend Session");
```

### Error Banner

```rust
let mut banner = Banner::new(1, "Connection lost. Please check your network.");
banner.set_type(BannerType::Error);
banner.set_dismissible(false); // Can't dismiss
banner.add_action("Retry");
```

## Related Components

- [Toast](toast.md) - For temporary messages
- [Dialog](dialog.md) - For modal messages

---

[← Back to Components](index.md)
