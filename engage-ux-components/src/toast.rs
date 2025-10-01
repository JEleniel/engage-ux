//! Toast notification component

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Toast position on screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastPosition {
	TopLeft,
	TopCenter,
	TopRight,
	BottomLeft,
	BottomCenter,
	BottomRight,
}

/// Toast variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastVariant {
	Success,
	Error,
	Warning,
	Info,
	Default,
}

/// Toast component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toast {
	properties: ComponentProperties,
	message: String,
	title: String,
	variant: ToastVariant,
	position: ToastPosition,
	duration_ms: Option<u64>,
	dismissible: bool,
	visible: bool,
	icon: Option<String>,
	color: Color,
	background_color: Color,
	border_color: Color,
}

impl Toast {
	/// Create a new toast
	pub fn new(id: ComponentId, message: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			message: message.into(),
			title: String::new(),
			variant: ToastVariant::Default,
			position: ToastPosition::TopRight,
			duration_ms: Some(3000),
			dismissible: true,
			visible: false,
			icon: None,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#E0E0E0").unwrap(),
		}
	}

	/// Create a success toast
	pub fn success(id: ComponentId, message: impl Into<String>) -> Self {
		let mut toast = Self::new(id, message);
		toast.set_variant(ToastVariant::Success);
		toast
	}

	/// Create an error toast
	pub fn error(id: ComponentId, message: impl Into<String>) -> Self {
		let mut toast = Self::new(id, message);
		toast.set_variant(ToastVariant::Error);
		toast
	}

	/// Create a warning toast
	pub fn warning(id: ComponentId, message: impl Into<String>) -> Self {
		let mut toast = Self::new(id, message);
		toast.set_variant(ToastVariant::Warning);
		toast
	}

	/// Create an info toast
	pub fn info(id: ComponentId, message: impl Into<String>) -> Self {
		let mut toast = Self::new(id, message);
		toast.set_variant(ToastVariant::Info);
		toast
	}

	/// Get message
	pub fn message(&self) -> &str {
		&self.message
	}

	/// Set message
	pub fn set_message(&mut self, message: impl Into<String>) {
		self.message = message.into();
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Get variant
	pub fn variant(&self) -> ToastVariant {
		self.variant
	}

	/// Set variant
	pub fn set_variant(&mut self, variant: ToastVariant) {
		self.variant = variant;
		// Update colors based on variant
		match variant {
			ToastVariant::Success => {
				self.background_color = Color::from_hex("#4CAF50").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
				self.border_color = Color::from_hex("#388E3C").unwrap();
			}
			ToastVariant::Error => {
				self.background_color = Color::from_hex("#F44336").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
				self.border_color = Color::from_hex("#D32F2F").unwrap();
			}
			ToastVariant::Warning => {
				self.background_color = Color::from_hex("#FF9800").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
				self.border_color = Color::from_hex("#F57C00").unwrap();
			}
			ToastVariant::Info => {
				self.background_color = Color::from_hex("#2196F3").unwrap();
				self.color = Color::from_hex("#FFFFFF").unwrap();
				self.border_color = Color::from_hex("#1976D2").unwrap();
			}
			ToastVariant::Default => {
				self.background_color = Color::from_hex("#FFFFFF").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
				self.border_color = Color::from_hex("#E0E0E0").unwrap();
			}
		}
	}

	/// Get position
	pub fn position(&self) -> ToastPosition {
		self.position
	}

	/// Set position
	pub fn set_position(&mut self, position: ToastPosition) {
		self.position = position;
	}

	/// Get duration in milliseconds
	pub fn duration_ms(&self) -> Option<u64> {
		self.duration_ms
	}

	/// Set duration in milliseconds (None for persistent)
	pub fn set_duration_ms(&mut self, duration_ms: Option<u64>) {
		self.duration_ms = duration_ms;
	}

	/// Check if dismissible
	pub fn is_dismissible(&self) -> bool {
		self.dismissible
	}

	/// Set dismissible
	pub fn set_dismissible(&mut self, dismissible: bool) {
		self.dismissible = dismissible;
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show toast
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide toast
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Get icon
	pub fn icon(&self) -> Option<&str> {
		self.icon.as_deref()
	}

	/// Set icon
	pub fn set_icon(&mut self, icon: Option<String>) {
		self.icon = icon;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Color) {
		self.border_color = color;
	}
}

impl Component for Toast {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn properties(&self) -> &ComponentProperties {
		&self.properties
	}

	fn properties_mut(&mut self) -> &mut ComponentProperties {
		&mut self.properties
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_toast_creation() {
		let toast = Toast::new(1, "Hello");
		assert_eq!(toast.id(), 1);
		assert_eq!(toast.message(), "Hello");
		assert!(!toast.is_visible());
	}

	#[test]
	fn test_toast_variants() {
		let success = Toast::success(1, "Success!");
		assert_eq!(success.variant(), ToastVariant::Success);

		let error = Toast::error(2, "Error!");
		assert_eq!(error.variant(), ToastVariant::Error);

		let warning = Toast::warning(3, "Warning!");
		assert_eq!(warning.variant(), ToastVariant::Warning);

		let info = Toast::info(4, "Info!");
		assert_eq!(info.variant(), ToastVariant::Info);
	}

	#[test]
	fn test_toast_visibility() {
		let mut toast = Toast::new(1, "Test");
		assert!(!toast.is_visible());
		
		toast.show();
		assert!(toast.is_visible());
		
		toast.hide();
		assert!(!toast.is_visible());
	}

	#[test]
	fn test_toast_duration() {
		let mut toast = Toast::new(1, "Test");
		assert_eq!(toast.duration_ms(), Some(3000));
		
		toast.set_duration_ms(Some(5000));
		assert_eq!(toast.duration_ms(), Some(5000));
		
		toast.set_duration_ms(None);
		assert_eq!(toast.duration_ms(), None);
	}

	#[test]
	fn test_toast_position() {
		let mut toast = Toast::new(1, "Test");
		toast.set_position(ToastPosition::BottomLeft);
		assert_eq!(toast.position(), ToastPosition::BottomLeft);
	}

	#[test]
	fn test_toast_dismissible() {
		let mut toast = Toast::new(1, "Test");
		assert!(toast.is_dismissible());
		
		toast.set_dismissible(false);
		assert!(!toast.is_dismissible());
	}

	#[test]
	fn test_toast_title() {
		let mut toast = Toast::new(1, "Message");
		toast.set_title("Title");
		assert_eq!(toast.title(), "Title");
	}
}
