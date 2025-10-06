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
//! Screen reader integration tests
//!
//! Tests screen reader support infrastructure across different platforms:
//! - Windows: MSAA (Microsoft Active Accessibility) / UI Automation
//! - macOS: NSAccessibility
//! - Linux: AT-SPI (Assistive Technology Service Provider Interface)
//! - Android: TalkBack
//! - iOS: VoiceOver

use engage_ux_components::*;
use engage_ux_core::accessibility::{
	AccessibilityProps, Announcement, AnnouncementPriority, AriaRole,
};
use engage_ux_core::component::Component;

/// Get current platform screen reader technology
fn screen_reader_technology() -> &'static str {
	#[cfg(target_os = "windows")]
	return "MSAA/UI Automation";

	#[cfg(target_os = "macos")]
	return "NSAccessibility";

	#[cfg(target_os = "linux")]
	return "AT-SPI";

	#[cfg(target_os = "android")]
	return "TalkBack";

	#[cfg(target_os = "ios")]
	return "VoiceOver";

	#[cfg(not(any(
		target_os = "windows",
		target_os = "macos",
		target_os = "linux",
		target_os = "android",
		target_os = "ios"
	)))]
	return "None";
}

/// Test platform-specific screen reader identification
#[test]
fn test_screen_reader_platform_identification() {
	let technology = screen_reader_technology();

	// Verify we identify the correct technology for each platform
	#[cfg(target_os = "windows")]
	assert_eq!(technology, "MSAA/UI Automation");

	#[cfg(target_os = "macos")]
	assert_eq!(technology, "NSAccessibility");

	#[cfg(target_os = "linux")]
	assert_eq!(technology, "AT-SPI");

	#[cfg(target_os = "android")]
	assert_eq!(technology, "TalkBack");

	#[cfg(target_os = "ios")]
	assert_eq!(technology, "VoiceOver");

	println!("Platform screen reader technology: {}", technology);
}

/// Test announcement creation and priorities
#[test]
fn test_screen_reader_announcements() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	// Low priority announcement
	let low_announcement = Announcement::new("File saved", AnnouncementPriority::Low);
	assert_eq!(low_announcement.message, "File saved");
	assert_eq!(low_announcement.priority, AnnouncementPriority::Low);

	// Medium priority announcement
	let medium_announcement = Announcement::new("Form submitted", AnnouncementPriority::Medium);
	assert_eq!(medium_announcement.message, "Form submitted");
	assert_eq!(medium_announcement.priority, AnnouncementPriority::Medium);

	// High priority announcement (error)
	let high_announcement = Announcement::new("Error: Invalid input", AnnouncementPriority::High);
	assert_eq!(high_announcement.message, "Error: Invalid input");
	assert_eq!(high_announcement.priority, AnnouncementPriority::High);
}

/// Test ARIA roles for screen reader compatibility
#[test]
fn test_screen_reader_aria_roles() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	// Create accessibility props with different roles
	let button_props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Submit Form");

	assert_eq!(button_props.role, Some(AriaRole::Button));
	assert_eq!(button_props.label, Some("Submit Form".to_string()));

	// Screen readers use role to announce component type
	let textbox_props = AccessibilityProps::new()
		.with_role(AriaRole::Textbox)
		.with_label("Email address");

	assert_eq!(textbox_props.role, Some(AriaRole::Textbox));
	assert_eq!(textbox_props.label, Some("Email address".to_string()));
}

/// Test live region announcements
#[test]
fn test_screen_reader_live_regions() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	// Polite live region (waits for pause in speech)
	let props = AccessibilityProps::new().with_role(AriaRole::Status);

	// In full implementation, would set live: Some(AriaLive::Polite)
	assert_eq!(props.role, Some(AriaRole::Status));

	// Assertive live region (interrupts speech)
	let alert_props = AccessibilityProps::new().with_role(AriaRole::Alert);

	// In full implementation, would set live: Some(AriaLive::Assertive)
	assert_eq!(alert_props.role, Some(AriaRole::Alert));
}

/// Test button label announcement
#[test]
fn test_screen_reader_button_labels() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let button = Button::new(1, "Click Me");

	// Screen reader should announce: "Click Me, button"
	assert_eq!(button.id(), 1);
	assert!(button.is_enabled());

	// Props would include label for screen reader
	let props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Click Me");

	assert_eq!(props.label, Some("Click Me".to_string()));
}

/// Test text input screen reader support
#[test]
fn test_screen_reader_text_input() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let mut input = TextInput::new(1);
	input.set_placeholder("Enter your email");

	// Screen reader should announce:
	// - Role: text box
	// - Placeholder text when empty
	// - Current text content

	assert_eq!(input.placeholder(), "Enter your email");
}

/// Test checkbox state announcements
#[test]
fn test_screen_reader_checkbox_states() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let mut checkbox = Checkbox::new(1, "I agree to terms");

	// Initially unchecked - screen reader announces: "I agree to terms, checkbox, not checked"
	assert!(!checkbox.is_checked());

	// Check it - screen reader announces: "I agree to terms, checkbox, checked"
	checkbox.set_checked(true);
	assert!(checkbox.is_checked());
}

/// Test slider value announcements
#[test]
fn test_screen_reader_slider_values() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let slider = Slider::new(1, 0.0, 100.0);

	// Screen reader should announce:
	// - Role: slider
	// - Current value
	// - Range
	// - Label (if provided)

	assert_eq!(slider.id(), 1);
}

/// Test progress indicator announcements
#[test]
fn test_screen_reader_progress_updates() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let mut progress = Progress::new(1);
	progress.set_value(50.0);

	// Screen reader should announce progress percentage
	assert_eq!(progress.value(), 50.0);
}

/// Test disabled component announcements
#[test]
fn test_screen_reader_disabled_components() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let mut button = Button::new(1, "Submit");
	button.set_enabled(false);

	// Screen reader should announce: "Submit, button, disabled"
	assert!(!button.is_enabled());

	let props = AccessibilityProps::new()
		.with_role(AriaRole::Button)
		.with_label("Submit");

	// In full implementation:
	// props.disabled = true;

	assert_eq!(props.label, Some("Submit".to_string()));
}

/// Test image alt text announcements
#[test]
fn test_screen_reader_image_alt_text() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let mut image = Image::new(1, "photo.jpg");
	image.set_alt_text("Team photo at 2024 conference");

	// Screen reader should read: "Team photo at 2024 conference, image"
	assert_eq!(image.alt_text(), "Team photo at 2024 conference");
}

/// Test link announcements
#[test]
fn test_screen_reader_link_descriptions() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let link = Link::new(1, "Learn more", "https://example.com/docs");

	// Screen reader should announce:
	// - Text: "Learn more"
	// - Role: "link"

	assert_eq!(link.text(), "Learn more");
	assert_eq!(link.id(), 1);
}

/// Test alert announcements
#[test]
fn test_screen_reader_alert_announcements() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let alert = AlertDialog::new(1, "Error", "Connection lost");

	// Screen reader should immediately announce (assertive):
	// "Alert: Connection lost"

	assert_eq!(alert.id(), 1);
}

/// Test toast notification announcements
#[test]
fn test_screen_reader_toast_notifications() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let toast = Toast::new(1, "File saved successfully");

	// Screen reader should politely announce (when user pauses):
	// "File saved successfully"

	assert_eq!(toast.message(), "File saved successfully");
}

/// Test accordion expand/collapse announcements
#[test]
fn test_screen_reader_accordion_states() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let accordion = Accordion::new(1);

	// Screen reader should announce panel states
	assert_eq!(accordion.id(), 1);
}

/// Test radio button group announcements
#[test]
fn test_screen_reader_radio_groups() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let radio1 = RadioButton::new(1, "Option 1", "opt1", "group1");
	let radio2 = RadioButton::new(2, "Option 2", "opt2", "group1");

	// Screen reader should announce:
	// - "Option 1, radio button, 1 of 2"
	// - Whether selected or not

	assert_eq!(radio1.id(), 1);
	assert_eq!(radio2.id(), 2);
}

/// Test toggle switch announcements
#[test]
fn test_screen_reader_toggle_switches() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let mut toggle = Toggle::with_label(1, "Dark Mode");

	// Off state - screen reader announces: "Dark Mode, switch, off"
	assert!(!toggle.is_active());

	// On state - screen reader announces: "Dark Mode, switch, on"
	toggle.set_active(true);
	assert!(toggle.is_active());
}

/// Test list structure announcements
#[test]
fn test_screen_reader_list_navigation() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let list = List::new(1);

	// Screen reader should announce:
	// - "List"
	// - Number of items
	// - Current position when navigating

	assert_eq!(list.id(), 1);
}

/// Test table navigation announcements
#[test]
fn test_screen_reader_table_navigation() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let table = Table::new(1);

	// Screen reader should announce:
	// - "Table"
	// - Number of rows and columns
	// - Header cells
	// - Current cell position: "Row 2, column 3"

	assert_eq!(table.id(), 1);
}

/// Test dropdown menu announcements
#[test]
fn test_screen_reader_dropdown_menus() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let dropdown = Dropdown::new(1);

	// Screen reader should announce:
	// - Current selection
	// - "Menu" or "Combo box"
	// - Whether expanded or collapsed

	assert_eq!(dropdown.id(), 1);
}

/// Test breadcrumb navigation announcements
#[test]
fn test_screen_reader_breadcrumb_navigation() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let breadcrumb = Breadcrumb::new(1);

	// Screen reader should announce:
	// - "Navigation"
	// - Current location in hierarchy
	// - "Home / Products / Item"

	assert_eq!(breadcrumb.id(), 1);
}

/// Test pagination announcements
#[test]
fn test_screen_reader_pagination() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let pagination = Pagination::new(1, 10);

	// Screen reader should announce:
	// - Current page: "Page 3"
	// - Total pages: "of 10"
	// - Navigation controls

	assert_eq!(pagination.id(), 1);
}

/// Test label and input associations
#[test]
fn test_screen_reader_label_associations() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let label = Label::new(1, "Email address:");
	let input = TextInput::new(2);

	// Screen reader should announce label with input:
	// "Email address: text box"

	assert_eq!(label.text(), "Email address:");
	assert_eq!(input.id(), 2);
}

/// Test error message announcements
#[test]
fn test_screen_reader_error_messages() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	// Error announcements should be assertive
	let announcement = Announcement::new("Invalid email format", AnnouncementPriority::High);

	assert_eq!(announcement.message, "Invalid email format");
	assert_eq!(announcement.priority, AnnouncementPriority::High);
}

/// Test success message announcements
#[test]
fn test_screen_reader_success_messages() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	// Success announcements should be polite
	let announcement = Announcement::new("Form submitted successfully", AnnouncementPriority::Low);

	assert_eq!(announcement.message, "Form submitted successfully");
	assert_eq!(announcement.priority, AnnouncementPriority::Low);
}

/// Test icon screen reader labels
#[test]
fn test_screen_reader_icon_labels() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let icon = Icon::new(1, "warning");

	// Screen reader should announce icon meaning
	// In full implementation, would have aria-label
	assert_eq!(icon.name(), "warning");
}

/// Test container landmark announcements
#[test]
fn test_screen_reader_landmarks() {
	let technology = screen_reader_technology();
	println!("Testing with: {}", technology);

	let container = Container::new(1);

	// Containers can be navigation landmarks
	// Screen reader would announce:
	// - "Navigation region"
	// - "Main content"
	// - "Complementary"

	assert_eq!(container.id(), 1);
}

/// Test Windows-specific screen reader support
#[test]
#[cfg(target_os = "windows")]
fn test_windows_screen_reader_support() {
	let technology = screen_reader_technology();
	assert_eq!(technology, "MSAA/UI Automation");

	// Windows uses:
	// - MSAA (legacy)
	// - UI Automation (modern)
	// - NVDA, JAWS, Narrator
}

/// Test macOS-specific screen reader support
#[test]
#[cfg(target_os = "macos")]
fn test_macos_screen_reader_support() {
	let technology = screen_reader_technology();
	assert_eq!(technology, "NSAccessibility");

	// macOS uses:
	// - NSAccessibility framework
	// - VoiceOver
}

/// Test Linux-specific screen reader support
#[test]
#[cfg(target_os = "linux")]
fn test_linux_screen_reader_support() {
	let technology = screen_reader_technology();
	assert_eq!(technology, "AT-SPI");

	// Linux uses:
	// - AT-SPI (Assistive Technology Service Provider Interface)
	// - Orca screen reader
}

/// Test Android-specific screen reader support
#[test]
#[cfg(target_os = "android")]
fn test_android_screen_reader_support() {
	let technology = screen_reader_technology();
	assert_eq!(technology, "TalkBack");

	// Android uses:
	// - Android Accessibility Framework
	// - TalkBack screen reader
}

/// Test iOS-specific screen reader support
#[test]
#[cfg(target_os = "ios")]
fn test_ios_screen_reader_support() {
	let technology = screen_reader_technology();
	assert_eq!(technology, "VoiceOver");

	// iOS uses:
	// - iOS Accessibility API
	// - VoiceOver screen reader
}
