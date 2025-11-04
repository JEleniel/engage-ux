# derive::Event

The `Event` derive (provided by the `engage-ux-derive` crate) makes it
easy to convert strongly-typed payload types into the framework's
`Event` representation.

Why use the derive

- Reduces boilerplate for creating `engage_ux_core::events::Event`
- Works for both `struct` and `enum` payload types
- Integrates with `serde::Serialize` so payloads are sent as JSON

Quick example (struct)

```no_run
use engage_ux_derive::Event;
use serde::Serialize;
use engage_ux_core::types::ComponentId;

#[derive(Serialize, Event)]
#[event("my_custom_event")]
struct MyPayload { value: i32 }

fn make_event(target: ComponentId) -> engage_ux_core::events::Event {
    MyPayload { value: 42 }.try_to_event(target).unwrap()
}
```

Enum usage and publishing variants

The derive also works on enums. When you `derive(Event)` for an enum,
each variant can be emitted as an event. You can optionally attach
`#[event("name")]` to a variant to control the event name emitted for
that variant. If a variant does not specify a name the derive will
generate a reasonable fallback.

```no_run
use engage_ux_derive::Event;
use serde::Serialize;
use engage_ux_core::types::ComponentId;

#[derive(Serialize, Event)]
enum AppEvent {
    #[event("user.logged_in")]
    UserLoggedIn { user_id: u64 },

    // fallback name will be generated for this variant
    DataUpdated { key: String, value: String },
}

fn publish_login(target: ComponentId, uid: u64) {
    AppEvent::UserLoggedIn { user_id: uid }
        .try_to_event(target)
        .unwrap();
}
```

Subscriber-side filtering and deserializing

Subscribers typically receive an `Event` value. The derive stores a
string `name` and JSON `data` for custom events. A typical subscriber
can filter by the event name and then deserialize the JSON back into
the original type using `serde_json` to pattern-match the enum variants.

```no_run
use engage_ux_core::events::Event;

fn handle(evt: Event) {
    if let engage_ux_core::events::EventType::Custom { name, data } = evt.r#type {
        if name == "user.logged_in" {
            // Attempt to decode into the AppEvent type and match variants
            # derive::Event

            The `Event` derive (provided by the `engage-ux-derive` crate) makes it
            easy to convert strongly-typed payload types into the framework's
            `Event` representation.

            Why use the derive

            - Reduces boilerplate for creating `engage_ux_core::events::Event`
            - Works for both `struct` and `enum` payload types
            - Integrates with `serde::Serialize` so payloads are sent as JSON

            Quick example (struct)

            ```no_run
            use engage_ux_derive::Event;
            use serde::Serialize;
            use engage_ux_core::types::ComponentId;

            #[derive(Serialize, Event)]
            #[event("my_custom_event")]
            struct MyPayload { value: i32 }

            fn make_event(target: ComponentId) -> engage_ux_core::events::Event {
                MyPayload { value: 42 }.try_to_event(target).unwrap()
            }
            ```

            Enum usage and publishing variants

            The derive also works on enums. When you `derive(Event)` for an enum,
            each variant can be emitted as an event. You can optionally attach
            `#[event("name")]` to a variant to control the event name emitted for
            that variant. If a variant does not specify a name the derive will
            generate a reasonable fallback.

            ```no_run
            use engage_ux_derive::Event;
            use serde::Serialize;
            use engage_ux_core::types::ComponentId;

            #[derive(Serialize, Event)]
            enum AppEvent {
                #[event("user.logged_in")]
                UserLoggedIn { user_id: u64 },

                // fallback name will be generated for this variant
                DataUpdated { key: String, value: String },
            }

            fn publish_login(target: ComponentId, uid: u64) {
                AppEvent::UserLoggedIn { user_id: uid }
                    .try_to_event(target)
                    .unwrap();
            }
            ```

            Subscriber-side filtering and deserializing

            Subscribers typically receive an `Event` value. The derive stores a
            string `name` and JSON `data` for custom events. A typical subscriber
            can filter by the event name and then deserialize the JSON back into
            the original type using `serde_json` to pattern-match the enum variants.

            ```no_run
            use engage_ux_core::events::Event;

            fn handle(evt: Event) {
                if let engage_ux_core::events::EventType::Custom { name, data } = evt.r#type {
                    if name == "user.logged_in" {
                        // Attempt to decode into the AppEvent type and match variants
                        if let Ok(app_evt) = serde_json::from_str::<AppEvent>(&data) {
                            match app_evt {
                                AppEvent::UserLoggedIn { user_id } => {
                                    // handle login
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            ```

            Attributes and configuration

            - `#[event("name")]` — shorthand to set the event name for a `struct` or
              an enum variant.
            - `#[event(name = "...")]` — explicit named form.
            - `#[event(crate = "my::events")]` — override the path to the events
              crate used by the generated code (default: `engage_ux_core::events`).

            Notes and expectations

            - The input type must implement `serde::Serialize`.
            - Serialization uses `serde_json` and serialization errors are mapped to a
              generated `*EventError` enum that the derive creates alongside your type.
            - For enums the derive attempts to honor any per-variant `#[event(...)]`
              attributes; otherwise it falls back to generated names.

            See also

            - `docs/api/core/events.md` — core events types and traits.
