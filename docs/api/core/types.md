# Types

Centralized small primitives for `engage-ux-core`.

Public items (re-exported from `engage-ux-core::types`):

- `ComponentId` — alias for `u64` used as a stable identifier for components.
- `Rect` — rectangle primitive (x, y, width, height) used for bounds and layout.
- `MouseButton` — enum for primary/secondary/middle mouse buttons.
- `KeyModifiers` — struct representing modifier keys (shift/ctrl/alt/meta).

Usage example:

```rust
use engage_ux_core::types::{ComponentId, Rect};

let id: ComponentId = 42;
let bounds = Rect::new(0.0, 0.0, 100.0, 20.0);
```

Rationale: these small primitives were centralized to avoid duplication and to ease serde derives and trait implementations.
