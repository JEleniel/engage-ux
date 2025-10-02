# Tasks to Complete Implementation

## Phase 2

1. **Priority Features**

    - Build platform-specific OAL backends
    - Native window management for each OS
    - Graphics rendering backends
    - Implement WCAG AAA features
    - Full support for navigation using:
        + Keyboard
        + Mouse
        + Touch
    - Screen reader support
    - SVG rendering (without script execution)
    - Font loading and rendering system
    - Image format support (PNG, JPEG, WebP, etc.)

2. **Bugfixes / Spec Conformance Fixes**

    - Change the theme configuration to accept more user friendly color formats, e.g.:
        + `{"primary": {"rgb":[128,255,255]}}` - RGB, Alpha=1
        + `{"primary": {"rgb":[128,255,255,0.5]}}` - RGB with Alpha
        + `{"primary": {"hex":"#80FFFF"}}` - Hex, Alpha=255 (1)
        + `{"primary": {"hex":"#80FFFF80"}}` - Hex with Alpha
        + `{"primary": {"hsl":[180, 0.5, 0.8]}}` - HSL, Alpha=1
        + `{"primary": {"hsl":[180, 0.5, 0.8,0.5]}}` - HSL with Alpha

3. **Testing**
    - Add integration tests
    - Add end-to-end functional tests
    - Platform-specific testing

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
