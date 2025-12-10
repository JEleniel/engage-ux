use engage_ux_core::event::Event;
use std::sync::Arc;

/// EventCallback type used by components: an Arc'd Fn(&Event)
pub type EventCallback = Arc<dyn Fn(&Event) + Send + Sync + 'static>;
