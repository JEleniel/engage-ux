//! Banner component for important messages

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Banner variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerVariant {
	Info,
	Success,
	Warning,
	Error,
}

/// Banner position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BannerPosition {
	Top,
	Bottom,
}

/// Banner component
#[derive(Clone, Serialize, Deserialize)]
pub struct Banner {
	properties: ComponentProperties,
	message: String,
	title: Option<String>,
	variant: BannerVariant,
	position: BannerPosition,
	visible: bool,
	dismissible: bool,
	icon: Option<String>,
	color: Color,
	background_color: Color,
	border_color: Color,
	#[serde(skip)]
	on_dismiss: Option<EventCallback>,
	#[serde(skip)]
	on_action: Option<EventCallback>,
	action_text: Option<String>,
}

impl Banner {
	/// Create a new banner
	pub fn new(id: ComponentId, message: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			message: message.into(),
			title: None,
			variant: BannerVariant::Info,
			position: BannerPosition::Top,
			visible: true,
			dismissible: true,
			icon: None,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#E3F2FD").unwrap(),
			border_color: Color::from_hex("#2196F3").unwrap(),
			on_dismiss: None,
			on_action: None,
			action_text: None,
		}
	}

	/// Create an info banner
	pub fn info(id: ComponentId, message: impl Into<String>) -> Self {
		let mut banner = Self::new(id, message);
		banner.set_variant(BannerVariant::Info);
		banner
	}

	/// Create a success banner
	pub fn success(id: ComponentId, message: impl Into<String>) -> Self {
		let mut banner = Self::new(id, message);
		banner.set_variant(BannerVariant::Success);
		banner
	}

	/// Create a warning banner
	pub fn warning(id: ComponentId, message: impl Into<String>) -> Self {
		let mut banner = Self::new(id, message);
		banner.set_variant(BannerVariant::Warning);
		banner
	}

	/// Create an error banner
	pub fn error(id: ComponentId, message: impl Into<String>) -> Self {
		let mut banner = Self::new(id, message);
		banner.set_variant(BannerVariant::Error);
		banner
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
	pub fn title(&self) -> Option<&str> {
		self.title.as_deref()
	}

	/// Set title
	pub fn set_title(&mut self, title: Option<String>) {
		self.title = title;
	}

	/// Get variant
	pub fn variant(&self) -> BannerVariant {
		self.variant
	}

	/// Set variant
	pub fn set_variant(&mut self, variant: BannerVariant) {
		self.variant = variant;
		// Update colors based on variant
		match variant {
			BannerVariant::Info => {
				self.background_color = Color::from_hex("#E3F2FD").unwrap();
				self.border_color = Color::from_hex("#2196F3").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
			}
			BannerVariant::Success => {
				self.background_color = Color::from_hex("#E8F5E9").unwrap();
				self.border_color = Color::from_hex("#4CAF50").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
			}
			BannerVariant::Warning => {
				self.background_color = Color::from_hex("#FFF3E0").unwrap();
				self.border_color = Color::from_hex("#FF9800").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
			}
			BannerVariant::Error => {
				self.background_color = Color::from_hex("#FFEBEE").unwrap();
				self.border_color = Color::from_hex("#F44336").unwrap();
				self.color = Color::from_hex("#000000").unwrap();
			}
		}
	}

	/// Get position
	pub fn position(&self) -> BannerPosition {
		self.position
	}

	/// Set position
	pub fn set_position(&mut self, position: BannerPosition) {
		self.position = position;
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show banner
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide banner
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Check if dismissible
	pub fn is_dismissible(&self) -> bool {
		self.dismissible
	}

	/// Set dismissible
	pub fn set_dismissible(&mut self, dismissible: bool) {
		self.dismissible = dismissible;
	}

	/// Get icon
	pub fn icon(&self) -> Option<&str> {
		self.icon.as_deref()
	}

	/// Set icon
	pub fn set_icon(&mut self, icon: Option<String>) {
		self.icon = icon;
	}

	/// Get action text
	pub fn action_text(&self) -> Option<&str> {
		self.action_text.as_deref()
	}

	/// Set action text
	pub fn set_action_text(&mut self, text: Option<String>) {
		self.action_text = text;
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

	/// Set dismiss callback
	pub fn set_on_dismiss(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_dismiss = Some(std::sync::Arc::new(callback));
	}

	/// Handle dismiss event
	pub fn handle_dismiss(&self, event: &Event) {
		if let Some(ref callback) = self.on_dismiss {
			callback(event);
		}
	}

	/// Set action callback
	pub fn set_on_action(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_action = Some(std::sync::Arc::new(callback));
	}

	/// Handle action event
	pub fn handle_action(&self, event: &Event) {
		if let Some(ref callback) = self.on_action {
			callback(event);
		}
	}
}

impl Component for Banner {
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
	fn test_banner_creation() {
		let banner = Banner::new(1, "Important message");
		assert_eq!(banner.id(), 1);
		assert_eq!(banner.message(), "Important message");
		assert!(banner.is_visible());
	}

	#[test]
	fn test_banner_variants() {
		let info = Banner::info(1, "Info");
		assert_eq!(info.variant(), BannerVariant::Info);

		let success = Banner::success(2, "Success");
		assert_eq!(success.variant(), BannerVariant::Success);

		let warning = Banner::warning(3, "Warning");
		assert_eq!(warning.variant(), BannerVariant::Warning);

		let error = Banner::error(4, "Error");
		assert_eq!(error.variant(), BannerVariant::Error);
	}

	#[test]
	fn test_banner_visibility() {
		let mut banner = Banner::new(1, "Message");
		assert!(banner.is_visible());
		
		banner.hide();
		assert!(!banner.is_visible());
		
		banner.show();
		assert!(banner.is_visible());
	}

	#[test]
	fn test_banner_dismissible() {
		let mut banner = Banner::new(1, "Message");
		assert!(banner.is_dismissible());
		
		banner.set_dismissible(false);
		assert!(!banner.is_dismissible());
	}

	#[test]
	fn test_banner_title() {
		let mut banner = Banner::new(1, "Message");
		assert_eq!(banner.title(), None);
		
		banner.set_title(Some("Title".to_string()));
		assert_eq!(banner.title(), Some("Title"));
	}

	#[test]
	fn test_banner_position() {
		let mut banner = Banner::new(1, "Message");
		assert_eq!(banner.position(), BannerPosition::Top);
		
		banner.set_position(BannerPosition::Bottom);
		assert_eq!(banner.position(), BannerPosition::Bottom);
	}

	#[test]
	fn test_banner_action() {
		let mut banner = Banner::new(1, "Message");
		assert_eq!(banner.action_text(), None);
		
		banner.set_action_text(Some("Learn More".to_string()));
		assert_eq!(banner.action_text(), Some("Learn More"));
	}
}
