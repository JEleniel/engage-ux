//! Integration tests for screen reader functionality
//!
//! Tests the complete screen reader integration across all platforms,
//! verifying proper backend creation, announcement handling, and
//! accessibility tree management.

use engage_ux_core::accessibility::{
	AccessibilityProps, Announcement, AnnouncementPriority, AriaRole,
};
use engage_ux_oal::{Platform, get_backend_factory};

#[test]
fn test_screen_reader_backend_creation() {
	// Verify that a screen reader backend can be created for all platforms
	let factory = get_backend_factory();
	let screen_reader = factory.create_screen_reader();
	let backend_name = screen_reader.backend_name();

	// Verify backend name matches expected platform
	let platform = Platform::current();
	match platform {
		Platform::Windows => assert_eq!(backend_name, "Windows UI Automation"),
		Platform::MacOS => assert_eq!(backend_name, "macOS NSAccessibility"),
		Platform::Linux => assert_eq!(backend_name, "Linux AT-SPI"),
		Platform::Android => assert_eq!(backend_name, "Android TalkBack"),
		Platform::iOS => assert_eq!(backend_name, "iOS VoiceOver"),
		Platform::Unknown => assert_eq!(backend_name, "Stub"),
	}
}

#[test]
fn test_screen_reader_announcements() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Test different announcement priorities
	let low_announcement = Announcement::low("Background task complete");
	screen_reader.announce(low_announcement);

	let medium_announcement = Announcement::medium("New message received");
	screen_reader.announce(medium_announcement);

	let high_announcement = Announcement::high("Critical error occurred");
	screen_reader.announce(high_announcement);

	// Test stopping announcements
	screen_reader.stop();

	// Test custom announcement creation
	let custom_announcement = Announcement::new("Custom message", AnnouncementPriority::Medium);
	screen_reader.announce(custom_announcement);
}

#[test]
fn test_screen_reader_component_management() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Create button with accessibility properties
	let button_props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Submit Form")
		.with_description("Submits the form data to the server")
		.with_focusable(true);

	screen_reader.update_component(1, button_props);

	// Create textbox with accessibility properties
	let textbox_props = AccessibilityProps::new()
		.with_role(AriaRole::Textbox)
		.with_label("Email Address")
		.with_focusable(true);

	screen_reader.update_component(2, textbox_props);

	// Create checkbox with accessibility properties
	let checkbox_props = AccessibilityProps::new()
		.with_role(AriaRole::Checkbox)
		.with_label("Remember me")
		.with_focusable(true);

	screen_reader.update_component(3, checkbox_props);

	// Remove components
	screen_reader.remove_component(1);
	screen_reader.remove_component(2);
	screen_reader.remove_component(3);
}

#[test]
fn test_screen_reader_focus_management() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Create components
	let button_props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Click Me")
		.with_focusable(true);

	let link_props = AccessibilityProps::new()
		.with_role(AriaRole::Link)
		.with_label("Learn More")
		.with_focusable(true);

	screen_reader.update_component(1, button_props);
	screen_reader.update_component(2, link_props);

	// Test focus navigation
	screen_reader.set_focus(1);
	screen_reader.set_focus(2);
	screen_reader.clear_focus();

	// Cleanup
	screen_reader.remove_component(1);
	screen_reader.remove_component(2);
}

#[test]
fn test_screen_reader_aria_roles() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Test various ARIA roles
	let roles_to_test = vec![
		(AriaRole::Button, "Button Element"),
		(AriaRole::Link, "Link Element"),
		(AriaRole::Textbox, "Input Field"),
		(AriaRole::Checkbox, "Checkbox Element"),
		(AriaRole::Radio, "Radio Button"),
		(AriaRole::Slider, "Slider Control"),
		(AriaRole::List, "List Container"),
		(AriaRole::ListItem, "List Item"),
		(AriaRole::Menu, "Menu Container"),
		(AriaRole::MenuItem, "Menu Item"),
		(AriaRole::Dialog, "Dialog Window"),
		(AriaRole::Alert, "Alert Message"),
		(AriaRole::Status, "Status Display"),
		(AriaRole::Navigation, "Navigation Area"),
		(AriaRole::Main, "Main Content"),
		(AriaRole::Complementary, "Sidebar Content"),
	];

	for (idx, (role, label)) in roles_to_test.iter().enumerate() {
		let props = AccessibilityProps::new()
			.with_role(role.clone())
			.with_label(*label)
			.with_focusable(true);

		screen_reader.update_component(idx + 1, props);
	}

	// Cleanup all components
	for idx in 1..=roles_to_test.len() {
		screen_reader.remove_component(idx);
	}
}

#[test]
fn test_screen_reader_state_changes() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Create a component and update its state
	let mut props = AccessibilityProps::new()
		.with_role(AriaRole::Checkbox)
		.with_label("Accept Terms");

	props.checked = Some(false);
	screen_reader.update_component(1, props.clone());

	// Announce state change
	screen_reader.announce(Announcement::medium("Checkbox unchecked"));

	// Update state
	props.checked = Some(true);
	screen_reader.update_component(1, props.clone());

	// Announce state change
	screen_reader.announce(Announcement::medium("Checkbox checked"));

	// Cleanup
	screen_reader.remove_component(1);
}

#[test]
fn test_screen_reader_live_regions() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Create live region for status updates
	let status_props = AccessibilityProps::new()
		.with_role(AriaRole::Status)
		.with_label("Status");

	screen_reader.update_component(1, status_props);

	// Announce updates to the live region
	screen_reader.announce(Announcement::low("Loading..."));
	screen_reader.announce(Announcement::medium("50% complete"));
	screen_reader.announce(Announcement::high("Complete!"));

	// Cleanup
	screen_reader.remove_component(1);
}

#[test]
fn test_screen_reader_complex_ui() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Simulate a complex form with multiple components
	let components = vec![
		(AriaRole::Navigation, "Main Navigation", true),
		(AriaRole::Main, "Main Content", false),
		(AriaRole::Textbox, "Username", true),
		(AriaRole::Textbox, "Password", true),
		(AriaRole::Checkbox, "Remember Me", true),
		(AriaRole::Button, "Sign In", true),
		(AriaRole::Link, "Forgot Password?", true),
		(AriaRole::Alert, "Error Messages", false),
	];

	for (idx, (role, label, focusable)) in components.iter().enumerate() {
		let props = AccessibilityProps::new()
			.with_role(role.clone())
			.with_label(*label)
			.with_focusable(*focusable);

		screen_reader.update_component(idx + 1, props);
	}

	// Test focus navigation through form
	screen_reader.set_focus(3); // Username
	screen_reader.set_focus(4); // Password
	screen_reader.set_focus(5); // Remember Me
	screen_reader.set_focus(6); // Sign In

	// Announce form submission
	screen_reader.announce(Announcement::high("Form submitted"));

	// Cleanup all components
	for idx in 1..=components.len() {
		screen_reader.remove_component(idx);
	}
}

#[test]
fn test_screen_reader_backend_consistency() {
	// Verify that multiple screen readers can be created
	let factory = get_backend_factory();

	let reader1 = factory.create_screen_reader();
	let reader2 = factory.create_screen_reader();

	// Both should have the same backend name
	assert_eq!(reader1.backend_name(), reader2.backend_name());
}

#[test]
fn test_screen_reader_disabled_state() {
	let factory = get_backend_factory();
	let mut screen_reader = factory.create_screen_reader();

	// Create disabled component
	let mut props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Disabled Button")
		.with_focusable(false);

	props.disabled = true;
	screen_reader.update_component(1, props);

	// Announce disabled state
	screen_reader.announce(Announcement::low("Button is disabled"));

	// Cleanup
	screen_reader.remove_component(1);
}
