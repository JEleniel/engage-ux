```markdown
# engage-ux-core: API reorganization and animation behavior (2025-11-01)

This document describes recent, developer-facing changes made to the `engage-ux-core` crate.
It summarizes the reorganization of small shared primitives, the one-struct-per-file rule applied to components,
the curated re-exports at the crate root, introduction of a `prelude`, and the new zero-duration animation semantics.

## Summary of changes

- Centralized small primitives into a `types` module (`engage_ux_core::types`).
  - Examples: `ComponentId`, `Rect`, `MouseButton`, `KeyModifiers`.
  - Rationale: reduces duplication and ensures a single authoritative definition for commonly-used small types.

- Split `Component` and its properties into separate files.
  - `component.rs` now contains the `Component` trait and related trait-only helpers.
  - `component_properties.rs` contains the `ComponentProperties` struct and its implementation.
  - Rationale: enforces the repository rule "one primary struct/trait per file" to improve discoverability.

- Curated, explicit re-exports in the crate root (`lib.rs`).
  - Replaced broad `pub use module::*;` exports with targeted `pub use` lines for the public surface.
  - Added a `prelude` module to expose the most common types (convenience imports).
  - Rationale: avoids hidden/ambiguous exports and name collisions, and makes the public API contract explicit.

- Animation behavior change: zero-duration animations are treated as no-ops.
  - When `Animation::start()` is called and the animation `duration == Duration::ZERO`, the animation is immediately
    marked `AnimationState::Completed`.
  - Subsequent `update()` calls return `None` because the animation is already completed.
  - Rationale: zero-duration animations typically denote an instantaneous change; treating them as completed avoids
    divide-by-zero or ambiguous progress calculations and matches intuitive expectations.

## What changed for users (import examples)

Before (older codebases may have used wildcard re-exports):

```rust
use engage_ux_core::component::*; // ambiguous imports
use engage_ux_core::types::Rect; // duplicate definitions sometimes existed
```

After (recommended):

```rust
// Explicit imports
use engage_ux_core::component::Component;
use engage_ux_core::component_properties::ComponentProperties;
use engage_ux_core::types::{ComponentId, Rect, KeyModifiers, MouseButton};


# engage-ux-core: API reorganization and animation behavior (2025-11-01)

This document describes recent, developer-facing changes made to the `engage-ux-core` crate.
It summarizes the reorganization of small shared primitives, the one-struct-per-file rule applied to components,
the curated re-exports at the crate root, introduction of a `prelude`, and the new zero-duration animation semantics.

## Summary of changes

- Centralized small primitives into a `types` module (`engage_ux_core::types`).

  + Examples: `ComponentId`, `Rect`, `MouseButton`, `KeyModifiers`.

  + Rationale: reduces duplication and ensures a single authoritative definition for commonly-used small types.

- Split `Component` and its properties into separate files.

  + `component.rs` now contains the `Component` trait and related trait-only helpers.

  + `component_properties.rs` contains the `ComponentProperties` struct and its implementation.

  + Rationale: enforces the repository rule "one primary struct/trait per file" to improve discoverability.

- Curated, explicit re-exports in the crate root (`lib.rs`).

  + Replaced broad `pub use module::*;` exports with targeted `pub use` lines for the public surface.

  + Added a `prelude` module to expose the most common types (convenience imports).

  + Rationale: avoids hidden/ambiguous exports and name collisions, and makes the public API contract explicit.

- Animation behavior change: zero-duration animations are treated as no-ops.

  + When `Animation::start()` is called and the animation `duration == Duration::ZERO`, the animation is immediately
    marked `AnimationState::Completed`.

  + Subsequent `update()` calls return `None` because the animation is already completed.

  + Rationale: zero-duration animations typically denote an instantaneous change; treating them as completed avoids
    divide-by-zero or ambiguous progress calculations and matches intuitive expectations.

## What changed for users (import examples)

Before (older codebases may have used wildcard re-exports):

```rust
use engage_ux_core::component::*; // ambiguous imports
use engage_ux_core::types::Rect; // duplicate definitions sometimes existed
```

After (recommended):

```rust
// Explicit imports
use engage_ux_core::component::Component;
use engage_ux_core::component_properties::ComponentProperties;
use engage_ux_core::types::{ComponentId, Rect, KeyModifiers, MouseButton};
// or, for convenience:
use engage_ux_core::prelude::*;
```

The `prelude` contains commonly used items (types and traits) to make small examples concise while
preserving the explicit crate root re-exports for more controlled use.

## Migration notes

- If your code referenced locally-defined `ComponentId`, `Rect`, or other small primitives from internal modules,
  update imports to `engage_ux_core::types::{...}`.

- If your code depended on wildcard re-exports from `engage_ux_core::` (for example `use engage_ux_core::*;`),
  switch to one of:

```rust
use engage_ux_core::prelude::*; // convenience import for common items
// or
use engage_ux_core::component::Component;
use engage_ux_core::types::ComponentId;
```

## Tests and behavior guarantees

-- Unit tests were added/updated to reflect these changes. In particular:

- Tests asserting animation semantics were updated so that zero-duration animations are marked completed and
- Negative tests for drag/drop and custom input parsing were added.

## Rationale and design notes

- The "one primary struct per file" rule helps code readers quickly find the implementation and reduces file size.

- Centralizing small primitives avoids subtle divergence across modules and makes serde derives and trait bounds
  easier to manage (one place to add derives like `Serialize`/`Deserialize`).

- Explicit re-exports make the public contract easier to review in code reviews and reduce accidental API surface area.

## Where to look in the code

- `engage-ux-core/src/types.rs` — centralized small primitives
- `engage-ux-core/src/component.rs` — `Component` trait (primary file for component behavior)
- `engage-ux-core/src/component_properties.rs` — `ComponentProperties` struct
- `engage-ux-core/src/lib.rs` — curated re-exports and `prelude` module
- `engage-ux-core/src/animation.rs` — animation system (see the `start()` semantics for zero-duration handling)

## Changelog note

This change was recorded in the project `CHANGELOG.md` under the Unreleased section (2025-11-01).

## Questions or follow-ups

- If you want zero-duration animations to instead produce a single final value on the next `update()` call,
  I can change the implementation and tests to return `Some(final_value)` on the first `update()`.

- If you'd like me to expand the `prelude` or further split modules (e.g., one enum/struct per file), I can prepare
  a follow-up PR with a suggested split and migration notes.

---

Document generated: 2025-11-01
