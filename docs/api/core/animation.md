# Animation

The animation module contains a small, composable animation system used by components.

Key types:

- `Animation` — holds an `AnimationType`, `duration`, `easing`, `delay`, `repeat`, `alternate` and state.
- `AnimationType` — enum variants: `Fade`, `Slide`, `Scale`, `Rotate`, `Color`.
- `Easing` — easing functions: `Linear`, `EaseIn`, `EaseOut`, `EaseInOut`, `CubicBezier`, `Elastic`, `Bounce`.
- `AnimationController` — helper type managing multiple animations.

Behavior change (2025-11-01):

- Zero-duration animations are treated as completed at `start()` time. `start()` sets the state to `Completed` and `update()` returns `None` for completed animations. This avoids ambiguous progress calculations and matches the expectation that a zero-duration animation denotes an instantaneous change.
