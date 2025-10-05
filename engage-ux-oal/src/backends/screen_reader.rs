//! Screen reader backend abstraction
//!
//! Provides platform-specific screen reader integration for:
//! - Windows: MSAA/UI Automation
//! - macOS: NSAccessibility
//! - Linux: AT-SPI
//! - Android: TalkBack
//! - iOS: VoiceOver

use engage_ux_core::accessibility::{AccessibilityProps, Announcement};

#[cfg(test)]
use engage_ux_core::accessibility::AriaRole;

/// Screen reader backend trait
///
/// Platform-specific implementations provide integration with native
/// screen reader and accessibility APIs.
pub trait ScreenReaderBackend: Send {
	/// Announce a message to the screen reader
	fn announce(&mut self, announcement: Announcement);

	/// Stop current announcement
	fn stop(&mut self);

	/// Check if screen reader is enabled
	fn is_enabled(&self) -> bool;

	/// Update accessibility properties for a component
	fn update_component(&mut self, component_id: usize, props: AccessibilityProps);

	/// Remove a component from the accessibility tree
	fn remove_component(&mut self, component_id: usize);

	/// Set focus to a component
	fn set_focus(&mut self, component_id: usize);

	/// Clear focus
	fn clear_focus(&mut self);

	/// Get the backend name for debugging
	fn backend_name(&self) -> &'static str;
}

/// Stub screen reader backend for testing and unsupported platforms
#[derive(Debug, Default)]
pub struct StubScreenReader {
	enabled: bool,
	last_announcement: Option<String>,
	focused_component: Option<usize>,
}

impl StubScreenReader {
	/// Create a new stub screen reader
	pub fn new() -> Self {
		Self {
			enabled: false,
			last_announcement: None,
			focused_component: None,
		}
	}

	/// Get the last announcement (for testing)
	#[allow(dead_code)]
	pub fn last_announcement(&self) -> Option<&str> {
		self.last_announcement.as_deref()
	}

	/// Get the focused component ID (for testing)
	#[allow(dead_code)]
	pub fn focused_component(&self) -> Option<usize> {
		self.focused_component
	}
}

impl ScreenReaderBackend for StubScreenReader {
	fn announce(&mut self, announcement: Announcement) {
		self.last_announcement = Some(announcement.message);
	}

	fn stop(&mut self) {
		self.last_announcement = None;
	}

	fn is_enabled(&self) -> bool {
		self.enabled
	}

	fn update_component(&mut self, _component_id: usize, _props: AccessibilityProps) {
		// Stub implementation - does nothing
	}

	fn remove_component(&mut self, _component_id: usize) {
		// Stub implementation - does nothing
	}

	fn set_focus(&mut self, component_id: usize) {
		self.focused_component = Some(component_id);
	}

	fn clear_focus(&mut self) {
		self.focused_component = None;
	}

	fn backend_name(&self) -> &'static str {
		"Stub"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_stub_screen_reader() {
		let mut reader = StubScreenReader::new();
		assert!(!reader.is_enabled());
		assert_eq!(reader.backend_name(), "Stub");

		let announcement = Announcement::high("Test message");
		reader.announce(announcement);
		assert_eq!(reader.last_announcement(), Some("Test message"));

		reader.stop();
		assert_eq!(reader.last_announcement(), None);
	}

	#[test]
	fn test_stub_focus_management() {
		let mut reader = StubScreenReader::new();
		assert_eq!(reader.focused_component(), None);

		reader.set_focus(42);
		assert_eq!(reader.focused_component(), Some(42));

		reader.clear_focus();
		assert_eq!(reader.focused_component(), None);
	}

	#[test]
	fn test_stub_component_updates() {
		let mut reader = StubScreenReader::new();
		let props = AccessibilityProps::new()
			.with_role(AriaRole::Button)
			.with_label("Test");

		// These should not panic
		reader.update_component(1, props);
		reader.remove_component(1);
	}
}
