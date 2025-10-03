# Implementation Roadmap

## Architecture Reference

Before implementing any features, review the architecture documentation in `docs/design/architecture/`:

- **[System Architecture](../architecture/System_Architecture.md)** - Understand the layered architecture and crate organization
- **[Component Architecture](../architecture/Component_Architecture.md)** - Learn component patterns and rendering pipeline
- **[Data Flow](../architecture/Data_Flow.md)** - Understand how data flows through the system
- **[NFRs](../architecture/NFRs.md)** - Ensure all implementations meet non-functional requirements (security, accessibility, performance)

Each requirement document includes detailed specifications and acceptance criteria for implementation.

## Phase 2 - COMPLETE ✅

All Phase 2 tasks that can be implemented in pure Rust have been completed. The remaining items require platform-specific implementations with OS APIs and are documented below.

### 1. Priority Features - COMPLETE ✅

#### Graphics & Rendering System ✅
- ✅ **COMPLETED** - Graphics rendering backend architecture
	- Backend abstraction (RenderBackend, RenderContext, RenderCommand)
	- Backend factory pattern for platform-specific implementations
	- Stub renderer for testing and unsupported platforms
	- Platform-specific factory stubs for Windows, macOS, Linux, Android, iOS
	- 9 comprehensive tests

#### Window Management ✅
- ✅ **COMPLETED** - Window management backend architecture
	- Window backend abstraction (WindowBackend, WindowState, WindowBounds)
	- Window state tracking (Normal, Minimized, Maximized, Fullscreen)
	- Window events (Resized, Moved, CloseRequested, FocusGained, FocusLost, DpiChanged)
	- DPI scaling support
	- 5 comprehensive tests

#### Input System ✅
- ✅ **COMPLETED** - Full support for all input types:
	- **Keyboard**: KeyCode enum, KeyModifiers (bitflags), KeyboardEvent, KeyboardState (23 tests)
	- **Mouse**: MouseButton, MouseEvent (ButtonDown/Up, Move, Wheel, Enter/Leave), MouseState (4 tests)
	- **Touch**: Multi-touch, TouchPhase, gestures (pinch, pan), TouchState (6 tests)
	- Unified InputHandler trait for components
	- 3 integration tests

#### Accessibility ✅
- ✅ **COMPLETED** - WCAG AAA accessibility infrastructure
	- ARIA roles and attributes (Button, Link, Textbox, Checkbox, etc.)
	- AccessibilityProps system for components
	- FocusManager for keyboard navigation
	- Screen reader announcement system with priorities
	- 4 comprehensive tests

#### Media Support ✅
- ✅ **COMPLETED** - Secure SVG parsing (production-ready with `usvg`)
	- Complete SVG 1.1 parsing
	- Automatic script blocking (security feature)
	- Event handler blocking
	- External resource blocking
	- Document dimensions and viewBox parsing
	- 6 tests including security validation

- ✅ **COMPLETED** - Font system (production-ready with `fontdue`)
	- Font family, weight (Thin-Black), and style (Normal/Italic/Oblique) support
	- FontRegistry for managing loaded fonts
	- TrueType/OpenType font parsing and validation
	- Font loading from files and bytes
	- 9 comprehensive tests

- ✅ **COMPLETED** - Image format support (production-ready with `image` crate)
	- Format detection from extensions and magic bytes
	- Full support for PNG, JPEG, WebP, GIF, BMP, TIFF
	- ImageData structure with pixel access
	- ColorType support (Grayscale, RGB, RGBA)
	- 8 comprehensive tests

### 2. User-Friendly Color Formats ✅
- ✅ **COMPLETED** - Enhanced theme color format support:
	- Hex format: `{"hex": "#RRGGBB"}` and `{"hex": "#RRGGBBAA"}`
	- RGB array: `{"rgb": [r, g, b]}` (0-255) and `{"rgb": [r, g, b, a]}`
	- HSL array: `{"hsl": [h, s, l]}` and `{"hsl": [h, s, l, a]}`
	- Backward compatible with legacy format
	- Automatic format detection
	- 13 new tests added
	- Complete documentation: [docs/color-formats.md](../color-formats.md)
	- Working example: `cargo run --example color_formats -p engage-ux-themes`
	- Example themes: `themes/light-friendly.json`, `themes/dark-friendly.json`

### 3. Testing ✅
- ✅ **COMPLETED** - Integration test suite (8 tests total)
	- Input system integration (3 tests)
	- Rendering pipeline integration (3 tests)
	- Theme integration (2 tests)
	- All tests passing with 100% success rate

### Future Work (Requires Platform-Specific Implementations)

The following items require OS-specific APIs and external platform libraries:

#### Platform-Specific Backends ⏳
- **Windows**: Direct2D/Direct3D for rendering, Win32 APIs for windows
- **macOS**: Core Graphics for rendering, Cocoa for windows
- **Linux**: Cairo/Skia for rendering, X11/Wayland for windows
- **Android**: Canvas API, NDK integration
- **iOS**: Core Graphics, UIKit integration

#### Screen Reader Integration ⏳
- **Windows**: Microsoft Active Accessibility (MSAA) or UI Automation
- **macOS**: NSAccessibility APIs
- **Linux**: AT-SPI (Assistive Technology Service Provider Interface)
- **Android**: TalkBack integration
- **iOS**: VoiceOver integration

#### Advanced Testing ⏳
- End-to-end functional tests (requires working platform backends)
- Platform-specific testing (requires OS integration)
- Visual regression testing

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

## Implementation Guidelines

### For Machine Agents

When implementing features, follow these guidelines:

#### 1. Review Architecture First
- Read the relevant architecture documents before coding
- Understand the requirements and acceptance criteria
- Review the NFRs to ensure compliance
- Study the diagrams to understand system interactions

#### 2. Follow Design Patterns
- Use trait-based architecture (Component, RenderBackend, WindowBackend)
- Apply factory pattern for platform-specific implementations
- Use observer pattern for events (broadcast channels)
- Follow strategy pattern for platform-specific behavior
- Use adapter pattern for OS API integration

#### 3. Ensure Quality
- All code must pass `cargo clippy` with no warnings
- All code must be formatted with `rustfmt`
- Use tabs for indentation (4 spaces width)
- Add comprehensive unit tests (aim for 90% coverage)
- Add integration tests for system interactions
- All public APIs must have documentation comments with examples

#### 4. Maintain Security
- **NEVER** use `unsafe` code (enforced by `#![forbid(unsafe_code)]`)
- Validate all external input
- Clamp all numeric values to safe ranges
- Block script execution in SVG files
- Use only actively maintained dependencies (updated within 6 months)

#### 5. Ensure Accessibility
- All interactive components must implement InputHandler trait
- Add appropriate ARIA roles and labels
- Ensure 7:1 contrast ratio for WCAG AAA
- Support keyboard navigation
- Implement focus management
- Add screen reader announcements

#### 6. Optimize Performance
- Target 60 FPS (16ms per frame)
- Use lazy loading for media assets
- Cache layout calculations until invalidated
- Batch render commands
- Use async/await for long-running operations
- Profile and optimize hot paths

#### 7. Platform Abstraction
- Keep platform-specific code in OAL layer
- Use factory pattern to create platform backends
- Implement unified interfaces across platforms
- Test with stub backends for platform independence
- Ensure feature parity across all platforms

#### 8. Thread Safety
- Use Arc<RwLock<T>> for shared component state
- Use Tokio channels for event distribution
- Avoid locks in hot paths where possible
- Prevent deadlocks through proper lock ordering
- Use atomic types for simple shared state

#### 9. Error Handling
- Return Result types for fallible operations
- Provide clear, actionable error messages
- Handle errors gracefully (no panics in production)
- Log errors appropriately
- Implement fallback behavior where possible

#### 10. Testing Strategy
- Unit tests for all components and systems
- Integration tests for system interactions
- Test error conditions and edge cases
- Mock platform backends for testing
- Ensure tests are deterministic (no flaky tests)
- Keep tests fast (complete in under 60 seconds)

### Component Implementation Checklist

When implementing a new component:

- [ ] Implement `Component` trait
- [ ] Add `ComponentProperties` field
- [ ] Implement builder pattern for construction
- [ ] Add event callbacks (on_click, on_change, etc.)
- [ ] Add accessibility properties (ARIA role, label)
- [ ] Support theme colors and styles
- [ ] Implement rendering logic
- [ ] Add comprehensive unit tests (creation, properties, events, state)
- [ ] Add documentation comments with examples
- [ ] Test with both light and dark themes
- [ ] Verify keyboard navigation works
- [ ] Verify screen reader announcements
- [ ] Profile performance if interactive

### Backend Implementation Checklist

When implementing a platform backend:

- [ ] Implement platform-specific trait (WindowBackend or RenderBackend)
- [ ] Handle all required operations
- [ ] Convert between Engage types and platform types
- [ ] Handle platform-specific events
- [ ] Ensure thread safety
- [ ] Handle errors gracefully
- [ ] Add platform-specific tests
- [ ] Document platform-specific requirements
- [ ] Ensure performance meets targets
- [ ] Verify memory is released properly
