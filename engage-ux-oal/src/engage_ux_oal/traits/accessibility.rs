//! Accessibility bridge trait.

use crate::{OalError, engage_ux_oal::types::AccessibilityPayload};

/// Accessibility bridge trait. Implementations are expected to be main-thread-only.
#[allow(dead_code)]
///
/// The core crate exposes `AccessibilityEvent` as a platform-agnostic event type; the
/// OAL bridge accepts those events and forwards them to the OS-specific accessibility APIs.
pub trait AccessibilityBridge: Send + Sync {
	fn publish_event(&self, event: AccessibilityPayload) -> Result<(), OalError>;
}
