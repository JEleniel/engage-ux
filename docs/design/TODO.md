# Tasks to Complete Implementation

## Phase 2

1. **Priority Features**

    - 🚧 **PARTIAL** - Build platform-specific OAL backends
        + ✅ Backend architecture and factory pattern implemented
        + ✅ Renderer abstraction (RenderBackend, RenderContext, RenderCommand)
        + ✅ Window backend abstraction (WindowBackend, WindowState, WindowBounds)
        + ✅ Platform-specific factory stubs for Windows, macOS, Linux, Android, iOS
        + ⏳ TODO: Implement actual platform-specific rendering (requires external dependencies)
    - 🚧 **PARTIAL** - Native window management for each OS
        + ✅ Window management interface complete
        + ✅ Window state tracking (Normal, Minimized, Maximized, Fullscreen)
        + ⏳ TODO: Platform-specific implementations (requires OS-specific APIs)
    - ✅ **COMPLETED** - Graphics rendering backends
        + ✅ Rendering command system
        + ✅ Backend factory pattern
        + ✅ Stub renderer for testing
    - ✅ **COMPLETED** - Implement WCAG AAA features (infrastructure)
        + ✅ ARIA roles and attributes
        + ✅ Accessibility properties system
        + ✅ Focus management
        + ✅ Screen reader announcement system
    - ✅ **COMPLETED** - Full support for navigation using:
        + ✅ Keyboard (23 tests)
        + ✅ Mouse (4 tests)
        + ✅ Touch (6 tests, including gestures)
        + ✅ Unified InputHandler trait
    - 🚧 **PARTIAL** - Screen reader support
        + ✅ Screen reader interface and announcement system
        + ⏳ TODO: Platform-specific integration (requires OS APIs)
    - ✅ **COMPLETED** - SVG rendering (without script execution)
        + ✅ SVG parser with security restrictions
        + ✅ Script detection and blocking
        + ✅ Event handler blocking
        + ✅ External resource blocking
        + ✅ 6 tests for security features
    - ✅ **COMPLETED** - Font loading and rendering system
        + ✅ Font family, weight, and style support
        + ✅ Font registry for managing fonts
        + ✅ Font loading from bytes
        + ✅ 9 tests for font system
    - ✅ **COMPLETED** - Image format support (PNG, JPEG, WebP, etc.)
        + ✅ Format detection from extension and magic bytes
        + ✅ ImageData structure for pixel data
        + ✅ Support for PNG, JPEG, WebP, GIF, BMP, TIFF
        + ✅ 8 tests for image handling

2. **Bugfixes / Spec Conformance Fixes**

    - ✅ **COMPLETED** - Change the theme configuration to accept more user friendly color formats:
        + ✅ `{"primary": {"rgb":[128,255,255]}}` - RGB, Alpha=1
        + ✅ `{"primary": {"rgb":[128,255,255,0.5]}}` - RGB with Alpha
        + ✅ `{"primary": {"hex":"#80FFFF"}}` - Hex, Alpha=255 (1)
        + ✅ `{"primary": {"hex":"#80FFFF80"}}` - Hex with Alpha
        + ✅ `{"primary": {"hsl":[180, 0.5, 0.8]}}` - HSL, Alpha=1
        + ✅ `{"primary": {"hsl":[180, 0.5, 0.8,0.5]}}` - HSL with Alpha
        + Documentation: [docs/color-formats.md](../color-formats.md)
        + Example: `cargo run --example color_formats -p engage-ux-themes`
        + Tests: 13 new tests added

3. **Testing**
    - ✅ **COMPLETED** - Add integration tests (8 tests)
        + ✅ Input system integration tests
        + ✅ Rendering pipeline tests
        + ✅ Theme integration tests
    - ⏳ **TODO** - Add end-to-end functional tests (requires platform backends)
    - ⏳ **TODO** - Platform-specific testing (requires platform implementations)

## Phase 3

1. **Priority Features**

    - Framework and documentation for developers to build their own components using Engage UX.

2. **Additional Features**

    - Animation system
    - Drag and drop support
    - Ability for developers to provide their own input handler for other devices.

3. **Testing**
    - Add integration tests
    - Add end-to-end functional tests
    - Platform-specific testing

## Phase 4

1. **Priority Features**

    - Support relative values for properties, e.g. `rb` and `rp`, where `rb` operates similarly to `em` in that it is a scaling relative to the theme's base size, and `rp` operates similarly to `rem` by scaling relative to the inherited size.
    - Support layout properties in the theme for components mapped to the `id` or a `name` property. Each component should be able to be positioned absolutely or relative to it's direct parent. Support `width`, `height`, `top`, `left`,`bottom`,`right`,`min_width`,`max_width`,`min_height`,`max_height` properties. Support an alternative sizing mode of `fill` that takes no sizes and fills the parent (an enum would be appropriate here). Support relative values, such as `rb`, `rp`, and `%`.

2. **Additional Features**
    - Support for multi-monitor setups, allowing devs to treat them as one pane, multiple separate panes, or a mix (for >2 monitors). Support for runtime configuration is required, as devs may want to allow users to choose.

## Phase 5

- Support for client/server rendering.
    - Mode 1 (default): The server renders the image, using the monitor layout of the client, and sends the compressed, rendered view to the client. The client then displays the view, and returns any input to the server. This is meant for use cases where the server has the rendering horsepower.
    - Mode 2: The server sends all information required to render the UX to the client who then renders the view. The client sends any input events to the server. This is meant for the use case where the client has rendering horsepower.
    - The entire connection must be encrypted with a minimum equivalent to TLS 1.3 (you may use HTTPS and TLS 1.3 if it will be performant enough). Both client and server must support using the OS Certificate Authorities as well as configurable additional CAs. The server must support both encrypted and unencrypted key files.
    - All connections must be fully authenticated. Support for built in Windows authentication (including Active Directory), Linux PAM, LDAP, and OAuth are required. Support for basic user/password authentication will not be supported.
    - Minimally, the system must be able to render, send, and display 24fps video without noticable stuttering or any degradation. Ideally, it should be able to support 4k 120fps, given sufficient bandwidth.
