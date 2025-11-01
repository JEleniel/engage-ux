# Component

The `Component` trait is the base interface for UI elements in Engage UX. Implementation details are in `engage-ux-core/src/component.rs`.

Key items:

- `Component` trait — methods for rendering, events, properties access.
- `ComponentProperties` — common runtime properties such as `id`, `visible`, `enabled`, and `bounds` (now in `component_properties.rs`).

Imports (recommended):

```rust
use engage_ux_core::component::Component;
use engage_ux_core::component_properties::ComponentProperties;
use engage_ux_core::types::ComponentId;
```

Migration note: code that previously expected `ComponentProperties` to be in `component.rs` should now import it from `engage_ux_core::component_properties`.
