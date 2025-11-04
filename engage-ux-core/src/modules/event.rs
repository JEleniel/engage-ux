//! Event module (modern layout)
//!
//! Provides event types, categories, an Event struct and an EventBus with
//! optional category-filtered subscriptions.

pub mod bus;
pub mod category;
pub mod event;
pub mod event_type;

pub use bus::EventBus;
pub use category::EventCategory;
pub use event::Event;
pub use event_type::EventType;

// tests can be added in submodules; keep the root module small.
