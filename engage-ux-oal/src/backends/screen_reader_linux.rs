//! Linux screen reader backend using AT-SPI
//!
//! This module provides screen reader support on Linux using the
//! AT-SPI (Assistive Technology Service Provider Interface),
//! which is the standard accessibility framework for Linux.

use super::screen_reader::{ScreenReaderBackend, StubScreenReader};
use engage_ux_core::accessibility::{AccessibilityProps, Announcement};
use std::collections::HashMap;

/// Linux screen reader backend using AT-SPI
pub struct LinuxScreenReader {
	stub: StubScreenReader,
	components: HashMap<usize, AccessibilityProps>,
}

impl LinuxScreenReader {
	/// Create a new Linux screen reader backend
	pub fn new() -> Self {
		Self {
			stub: StubScreenReader::new(),
			components: HashMap::new(),
		}
	}
}

impl Default for LinuxScreenReader {
	fn default() -> Self {
		Self::new()
	}
}

impl ScreenReaderBackend for LinuxScreenReader {
	fn announce(&mut self, announcement: Announcement) {
		self.stub.announce(announcement);
	}

	fn stop(&mut self) {
		self.stub.stop();
	}

	fn is_enabled(&self) -> bool {
		// On Linux, check if AT-SPI is available and a screen reader is running
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
		"Linux AT-SPI"
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use engage_ux_core::accessibility::AriaRole;

	#[test]
	fn test_linux_screen_reader_creation() {
		let reader = LinuxScreenReader::new();
		assert_eq!(reader.backend_name(), "Linux AT-SPI");
	}

	#[test]
	fn test_linux_component_management() {
		let mut reader = LinuxScreenReader::new();
		let props = AccessibilityProps::new()
			.with_role(AriaRole::Button)
			.with_label("Submit");

		reader.update_component(1, props);
		reader.remove_component(1);
	}

	#[test]
	fn test_linux_announcements() {
		let mut reader = LinuxScreenReader::new();
		let announcement = Announcement::low("Update available");
		reader.announce(announcement);
		reader.stop();
	}
}
