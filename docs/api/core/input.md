# Input

Input handling covers keyboard, mouse, touch and custom input devices.

Key types:

- `KeyboardEvent`, `KeyCode`, `KeyModifiers`
- `MouseEvent`, `MouseButton`, `MouseState`
- `TouchEvent`, `Touch`, `TouchPhase`
- `InputEvent` — unified enum covering all input event variants
- `CustomInputEvent` and `CustomInputValue` — extensible custom device events

Notes:

- A negative test was added to ensure malformed lookups from `CustomInputEvent` return `None` rather than panicking.
