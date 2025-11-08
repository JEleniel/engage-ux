# Accessibility — Design & Guidelines

This document captures accessibility requirements, platform mappings, testing recommendations, and a WCAG‑AAA focused checklist for Engage UX. It complements the OAL design (`docs/design/oal.md`) by describing the platform-specific APIs to integrate with and the practical steps to make the UX library accessible by default.

## Goals

- Provide a platform-neutral accessibility model in the core and a small OAL bridge that maps it to platform accessibility facilities (UIA, NSAccessibility, AccessibilityNodeProvider, AT‑SPI).
- Make the default themes and components target WCAG AAA accessibility where reasonable and provide helpers so app authors can reach AAA when needed.
- Offer robust virtual accessibility nodes for drawn (non-native) widgets, stable node ids, and low-latency event publishing so assistive technology can interact reliably.

## Platform Accessibility APIs (inventory)

- Windows: UI Automation (UIA) as the primary API; MSAA / IAccessible as legacy compatibility.
- macOS: NSAccessibility / NSAccessibilityElement, and native notifications (VoiceOver).
- iOS: UIAccessibility with accessibilityTraits, UIAccessibilityElement, accessibility notifications.
- Android: AccessibilityNodeInfo / AccessibilityNodeProvider and AccessibilityEvent; integrate with TalkBack.
- Linux (desktop): AT‑SPI2 via D‑Bus (exposed by GTK/Qt) for Orca and other screen readers.

Each platform has nuances (e.g., UIA patterns vs Android actions). Platform crates implement the mapping and publish the accessible node tree.

## Core accessibility model (summary)

The core maintains a platform-neutral accessibility tree of nodes with the following canonical properties:

- id: stable node id
- role: semantic role (Button, TextField, Image, Slider, List, ListItem, Table, etc.)
- name: accessible name (label / alt text)
- description/hint: longer explanation or hint text
- value: current value (optional)
- states: flags (focused, selected, disabled, checked, expanded, busy)
- actions: list of supported actions (Invoke/Click, SetValue, Increment, Decrement, ScrollTo, Custom)
- bounds: logical rectangle in core coordinates
- children: ordered list of child node ids
- live_region: optional live region kind (off, polite, assertive)

The core is responsible for computing accessible names and ensuring nodes that are essential for assistive tech have meaningful names and roles.

## Mapping guidance (core -> platform)

- Windows (UIA): map roles to control types and implement appropriate UIA patterns such as InvokePattern, ValuePattern, TextPattern, SelectionPattern, RangeValuePattern, TogglePattern, and Table/Grid patterns.
- macOS/iOS: set accessibilityLabel, accessibilityValue, accessibilityHint and use NSAccessibilityElement for virtual nodes; post notifications for changes.
- Android: publish AccessibilityNodeInfo objects, set className, text, contentDescription, bounds, and actions; implement AccessibilityNodeProvider for virtual view hierarchies.
- AT‑SPI (Linux): expose accessible objects and roles, and publish events for state/structure changes.

## OAL AccessibilityBridge responsibilities

(See `docs/design/oal.md` for the OAL AccessibilityBridge API sketch.) In short, the bridge must:

- Accept create/update/remove operations from the core and publish them via the platform accessibility API.
- Convert logical coordinates to platform physical coordinates using the window scale factor.
- Translate core roles/states/actions to platform-specific attributes, patterns, and actions.
- Dispatch actions from assistive tech back into core event handlers (e.g., user activated a virtual button).
- Publish events (focus, structure change, value change, live region announcements) in a rate-limited manner to avoid event storms.

## WCAG‑AAA checklist (library responsibilities)

Note: some AAA requirements are content/app-specific; the library's role is to provide defaults, helpers, and enforcement where feasible.

Perceivable

- Text alternatives: require/encourage alt text APIs for images and icons. Provide tooling to warn when missing.
- Contrast: default themes must meet AAA contrast ratios for normal text where practical (7:1). Provide a high-contrast theme and runtime switch.
- Resize text: respect system font scaling and provide API to scale text up to 200% without clipping.
- Non-color cues: ensure states are represented by icons/text in addition to color.

Operable

- Keyboard navigation: default keyboard order, focus outlines, and ARIA-like tab semantics.
- Focus visibility: visible focus indicator with sufficient contrast.
- Reduced motion: respect system motion preferences and offer a reduce-motion mode.

Understandable

- Clear labels and instructions: helpers for tooltips and instructions tied to form controls.
- Consistent behavior: modal dialogs trap focus, and focus is restored on close.

Robust

- Complete semantics: expose name/role/value/actions consistently, stable ids, and live region support.

Media

- Captions/subtitles: video components must accept subtitle tracks and expose caption controls.
- Audio descriptions: allow apps to provide audio descriptions for media.

Testing and verification

- Automated checks:
    + Accessibility tree completeness tests (every visible interactive node has a name/role).
    + Contrast checks for default themes.
    + Keyboard navigation order tests for core components.
- Manual checks:
    + VoiceOver (macOS/iOS), Narrator and NVDA (Windows), TalkBack (Android), Orca (Linux).
    + Accessibility Inspector tools on each platform.
- CI:
    + Run unit tests that validate accessibility metadata for components.
    + Optionally run headless checker tools where available.

Developer ergonomics

- Provide builders and helpers so component authors can set accessibility attributes easily (e.g., Button::with_accessible_name("...")).
- Provide lint/audit tooling in dev mode to warn about missing accessibility attributes.
- Document common patterns and show examples for custom-drawn widgets (how to create virtual accessibility nodes).

## Examples and patterns

- Virtual button example: how to create a drawn button and publish a virtual accessible node with Invoke action hooked into the core event when platform accessibility requests an Invoke.
- Text input example: mapping caret, selection, and IME events to platform text APIs or, if implementing custom text, mapping to TextPattern / selection APIs.

## Next steps

- Implement `AccessibilityBridge` trait in `engage-ux-oal` with concrete types and tests.
- Add automated accessibility unit tests for each component in `engage-ux-components`.
- Create example apps demonstrating screen reader flows on each platform (VoiceOver, TalkBack, Narrator, Orca).

## References

- WCAG 2.2: <https://www.w3.org/WAI/standards-guidelines/wcag/>
- UIAutomation (Windows) docs
- NSAccessibility Guide (Apple)
- Android Accessibility Developer Guide
- AT-SPI2 specification
