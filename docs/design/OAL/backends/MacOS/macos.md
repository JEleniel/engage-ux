# MacOS Backend Design

## Overview

Defines the API surface for the MacOS backend implementation of the OS Abstraction Layer (OAL), updated to use [`winit`](https://github.com/rust-windowing/winit) for window and event management. All windowing and event loop handling should use winit for cross-platform support. For drawing on window surfaces, [`wgpu`](https://github.com/gfx-rs/wgpu) is recommended for GPU-accelerated, cross-platform rendering. Cocoa/CoreGraphics APIs are wrapped by winit and only used for integration points not covered by winit.

## API Surface

- Window/event management: Use winit for all window creation, event loop, and input handling. Builder and event-driven patterns should reference winit's APIs for ergonomic setup and event delivery.
- Surface drawing: Use wgpu for rendering to winit surfaces. CoreGraphics/CoreAnimation may be used as fallback only if strictly required.
- Accessibility: Integrate with macOS Accessibility API (AXUIElement) as needed.

## Backend Selection

- OAL selects MacOSBackend if running on macOS. All window/event plumbing should use winit for cross-platform support. Surface drawing should use wgpu unless a platform-native API is strictly required.

## Integration Points

- EventBus for cross-thread communication.
- Thread safety via Rust Send/Sync traits.
- Builder pattern for configuration, referencing winit's APIs for window setup.

## Platform-Specific Notes

- Cocoa/CoreGraphics APIs are wrapped by winit and only used for integration points not covered by winit. Handle Retina display scaling and multi-space desktops via winit's APIs where possible.

## OAL Integration Notes

- Uses EventBus for all input event delivery and cross-thread communication.
- Layout is expressed in Units (U), default 1U=1cm, with runtime scale changes supported (Metric, Imperial, Point, Pixel presets).
- Unit-to-pixel conversion is performed using device DPCM/DPI as part of the rendering pipeline.
- Exposes logical Canvas, View, and Window types matching the OAL API.
- Rendering is always on the main thread; all other logic is off-main-thread via EventBus.
- Platform-specific errors and configuration changes are surfaced via Result types and EventBus events.

See `../OAL.md` for the full API contract and Mermaid diagram.
