# Video Component

A video player component with playback controls.

## Overview

The Video component provides video playback with controls for play/pause, seeking, volume, and fullscreen.

## Basic Usage

```rust
use engage_ux_components::Video;

let mut video = Video::new(1, "video.mp4");
video.set_bounds(Rect::new(0.0, 0.0, 640.0, 360.0));

// Show controls
video.set_controls(true);
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `source` | `String` | Video URL or path | Required |
| `controls` | `bool` | Show playback controls | `true` |
| `autoplay` | `bool` | Auto-play on load | `false` |
| `loop` | `bool` | Loop playback | `false` |
| `muted` | `bool` | Start muted | `false` |
| `poster` | `Option<String>` | Poster image URL | `None` |

## Methods

```rust
// Playback control
video.play();
video.pause();
video.stop();

// Seeking
video.seek(30.0); // Seek to 30 seconds

// Volume
video.set_volume(0.75); // 75% volume
video.mute();
video.unmute();

// Configuration
video.set_controls(true);
video.set_autoplay(false);
video.set_loop(true);
video.set_poster("poster.jpg");

// Events
video.set_on_play(|_| {
    println!("Playing");
});

video.set_on_pause(|_| {
    println!("Paused");
});

video.set_on_ended(|_| {
    println!("Ended");
});
```

## Examples

### Background Video

```rust
let mut bg_video = Video::new(1, "background.mp4");
bg_video.set_autoplay(true);
bg_video.set_loop(true);
bg_video.set_muted(true);
bg_video.set_controls(false);
```

### Tutorial Video

```rust
let mut tutorial = Video::new(1, "tutorial.mp4");
tutorial.set_controls(true);
tutorial.set_poster("tutorial-poster.jpg");

tutorial.set_on_ended(|_| {
    show_next_lesson();
});
```

## Accessibility

Videos should have captions and transcripts:

```rust
video.set_aria_label("Tutorial video");
video.add_subtitle_track("subtitles.vtt");
```

## Related Components

- [Image](image.md) - For static images
- [Carousel](carousel.md) - For slideshows

---

[← Back to Components](index.md)
