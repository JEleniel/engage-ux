# Events

The `events` module defines the generic event system used by components and the runtime.

Key types:

- `Event` — data structure representing an event.
- `EventType` — enum of known event kinds.
- `EventHandler` — trait for objects that handle events.
- `EventCallback` — type alias for callback-based handlers (re-exported).

Usage:

```rust
use engage_ux_core::events::{Event, EventType, EventHandler};
```
