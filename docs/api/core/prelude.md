# Prelude

The `engage_ux_core::prelude` module exposes a curated set of commonly-used types to make examples concise.

Typical items in the prelude:

- `Component`, `ComponentRef`, `ComponentProperties`
- `ComponentId`, `Rect`, `KeyModifiers`, `MouseButton`
- `Event`, `EventHandler`
- `Animation`, `AnimationController`

Use the prelude when writing examples or small programs to reduce import noise:

```rust
use engage_ux_core::prelude::*;
```
