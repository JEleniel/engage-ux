//! Android screen reader backend using TalkBack
//!
//! This module provides screen reader support on Android using
//! TalkBack integration through the Android accessibility APIs.

use super::screen_reader::{ScreenReaderBackend, StubScreenReader};
use engage_ux_core::accessibility::{AccessibilityProps, Announcement};
use std::collections::HashMap;

/// Android screen reader backend using TalkBack
pub struct AndroidScreenReader {
	stub: StubScreenReader,
	components: HashMap<usize, AccessibilityProps>,
}

impl AndroidScreenReader {
	/// Create a new Android screen reader backend
	pub fn new() -> Self {
		Self {
			stub: StubScreenReader::new(),
			components: HashMap::new(),
		}
	}
}

impl Default for AndroidScreenReader {
	fn default() -> Self {
		Self::new()
	}
}

impl ScreenReaderBackend for AndroidScreenReader {
	fn announce(&mut self, announcement: Announcement) {
		self.stub.announce(announcement);
	}

	fn stop(&mut self) {
		self.stub.stop();
	}

	fn is_enabled(&self) -> bool {
		// On Android, check if TalkBack is enabled
		// For now, return false as we need platform-specific APIs
		false
	}

	fn update_component(&mut self, component_id: usize, props: AccessibilityProps) {
		self.components.insert(component_id, props);
	}

	fn remove_component(&mut self, component_id: usize) {
		self.components.remove(&component_id);
	}

	fn set_focus(&mut self, component_id: usize) {
		self.stub.set_focus(component_id);
	}

	fn clear_focus(&mut self) {
		self.stub.clear_focus();
	}

	fn backend_name(&self) -> &'static str {
		"Android TalkBack"
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use engage_ux_core::accessibility::{AnnouncementPriority, AriaRole};

	#[test]
	fn test_android_screen_reader_creation() {
		let reader = AndroidScreenReader::new();
		assert_eq!(reader.backend_name(), "Android TalkBack");
	}

	#[test]
	fn test_android_component_management() {
		let mut reader = AndroidScreenReader::new();
		let props = AccessibilityProps::new()
			.with_role(AriaRole::Button)
			.with_label("Submit");

		reader.update_component(1, props);
		reader.remove_component(1);
	}

	#[test]
	fn test_android_announcements() {
		let mut reader = AndroidScreenReader::new();
		let announcement = Announcement::high("Connection error");
		reader.announce(announcement);
		reader.stop();
	}
}
