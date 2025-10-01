//! Link component for navigation and actions

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Link component
#[derive(Clone, Serialize, Deserialize)]
pub struct Link {
	properties: ComponentProperties,
	text: String,
	href: String,
	target: LinkTarget,
	underline: bool,
	color: Color,
	hover_color: Color,
	visited_color: Color,
	font_size: f32,
	#[serde(skip)]
	on_click: Option<EventCallback>,
}

/// Link target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkTarget {
	/// Open in the same window/view
	Self_,
	/// Open in a new window/tab
	Blank,
	/// Open in parent frame
	Parent,
	/// Open in top frame
	Top,
}

impl Link {
	/// Create a new link
	pub fn new(id: ComponentId, text: impl Into<String>, href: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			text: text.into(),
			href: href.into(),
			target: LinkTarget::Self_,
			underline: true,
			color: Color::from_hex("#1976D2").unwrap(),
			hover_color: Color::from_hex("#1565C0").unwrap(),
			visited_color: Color::from_hex("#7B1FA2").unwrap(),
			font_size: 14.0,
			on_click: None,
		}
	}

	/// Get link text
	pub fn text(&self) -> &str {
		&self.text
	}

	/// Set link text
	pub fn set_text(&mut self, text: impl Into<String>) {
		self.text = text.into();
	}

	/// Get href
	pub fn href(&self) -> &str {
		&self.href
	}

	/// Set href
	pub fn set_href(&mut self, href: impl Into<String>) {
		self.href = href.into();
	}

	/// Get target
	pub fn target(&self) -> LinkTarget {
		self.target
	}

	/// Set target
	pub fn set_target(&mut self, target: LinkTarget) {
		self.target = target;
	}

	/// Check if underlined
	pub fn is_underlined(&self) -> bool {
		self.underline
	}

	/// Set underline
	pub fn set_underline(&mut self, underline: bool) {
		self.underline = underline;
	}

	/// Set link color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get link color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set hover color
	pub fn set_hover_color(&mut self, color: Color) {
		self.hover_color = color;
	}

	/// Get hover color
	pub fn hover_color(&self) -> &Color {
		&self.hover_color
	}

	/// Set visited color
	pub fn set_visited_color(&mut self, color: Color) {
		self.visited_color = color;
	}

	/// Get visited color
	pub fn visited_color(&self) -> &Color {
		&self.visited_color
	}

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}

	/// Set click event handler
	pub fn set_on_click(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_click = Some(std::sync::Arc::new(callback));
	}

	/// Handle click event
	pub fn handle_click(&self, event: &Event) {
		if let Some(ref callback) = self.on_click {
			callback(event);
		}
	}
}

impl Component for Link {
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
	fn test_link_creation() {
		let link = Link::new(1, "Click here", "https://example.com");
		assert_eq!(link.id(), 1);
		assert_eq!(link.text(), "Click here");
		assert_eq!(link.href(), "https://example.com");
	}

	#[test]
	fn test_link_text_modification() {
		let mut link = Link::new(1, "Original", "https://example.com");
		link.set_text("Updated");
		assert_eq!(link.text(), "Updated");
	}

	#[test]
	fn test_link_href_modification() {
		let mut link = Link::new(1, "Link", "https://example.com");
		link.set_href("https://newurl.com");
		assert_eq!(link.href(), "https://newurl.com");
	}

	#[test]
	fn test_link_target() {
		let mut link = Link::new(1, "Link", "https://example.com");
		assert_eq!(link.target(), LinkTarget::Self_);
		
		link.set_target(LinkTarget::Blank);
		assert_eq!(link.target(), LinkTarget::Blank);
	}

	#[test]
	fn test_link_underline() {
		let mut link = Link::new(1, "Link", "https://example.com");
		assert!(link.is_underlined());
		
		link.set_underline(false);
		assert!(!link.is_underlined());
	}
}
