# engage-ux-derive

This crate provides the `#[derive(Event)]` procedural macro used by Engage UX to convert consumer types into `engage_ux_core::events::EventType` and `engage_ux_core::events::Event` values.

Supported forms

- Type-level shorthand: `#[event("my_event_name")]`
- Keyed form: `#[event(name = "my_event_name", crate = "my::crate::path")]
- Per-enum-variant overrides: `#[event(name = "variant_name")]` on enum variants.

Examples

1) Struct shorthand

```rust
use serde::Serialize;
use engage_ux_derive::Event;

#[derive(Serialize, Event)]
#[event("my_struct_event")]
struct MyStruct {
    x: u32,
}

// MyStruct::try_to_event(target_id) -> Result<engage_ux_core::events::Event, MyStructEventError>
```

2) Struct keyed form with custom crate path

```rust
use serde::Serialize;
use engage_ux_derive::Event;

#[derive(Serialize, Event)]
#[event(name = "my_custom", crate = "engage_ux_core::events")]
struct Other {
    s: String,
}
```

3) Enum with per-variant names

```rust
use serde::Serialize;
use engage_ux_derive::Event;

#[derive(Serialize, Event)]
enum AppEvent {
    #[event("ping")]
    Ping { v: u32 },

    #[event("pong")]
    Pong(u32),

    // default name will be "AppEvent::Other"
    Other,
}
```

Notes

- The derive macro requires `serde::Serialize` on your type because it uses `serde_json` to serialize the payload.
- The macro generates a consumer-local error enum named `<TypeName>EventError` derived with `thiserror::Error`.
- Use the generated `try_to_event(self, target: ComponentId) -> Result<Event, <TypeNameEventError>>` helper to construct events for a given target.

Testing

- UI negative tests live under `tests/ui/` and are exercised by the `trybuild` harness in `tests/trybuild.rs`.
