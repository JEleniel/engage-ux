//! Button component for user interaction

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Button variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonVariant {
	Primary,
	Secondary,
	Outlined,
	Text,
}

/// Button component
#[derive(Clone, Serialize, Deserialize)]
pub struct Button {
	properties: ComponentProperties,
	text: String,
	variant: ButtonVariant,
	color: Color,
	background_color: Color,
	#[serde(skip)]
	on_click: Option<EventCallback>,
}

impl Button {
	/// Create a new button
	pub fn new(id: ComponentId, text: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			text: text.into(),
			variant: ButtonVariant::Primary,
			color: Color::from_hex("#FFFFFF").unwrap(),
			background_color: Color::from_hex("#1976D2").unwrap(),
			on_click: None,
		}
	}

	/// Get button text
	pub fn text(&self) -> &str {
		&self.text
	}

	/// Set button text
	pub fn set_text(&mut self, text: impl Into<String>) {
		self.text = text.into();
	}

	/// Get button variant
	pub fn variant(&self) -> ButtonVariant {
		self.variant
	}

	/// Set button variant
	pub fn set_variant(&mut self, variant: ButtonVariant) {
		self.variant = variant;
	}

	/// Get text color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get background color
	pub fn background_color(&self) -> &Color {
		&self.background_color
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set click handler
	pub fn set_on_click<F>(&mut self, callback: F)
	where
		F: Fn(&Event) + Send + Sync + 'static,
	{
		self.on_click = Some(Arc::new(callback));
	}

	/// Handle click event
	pub fn handle_click(&self, event: &Event) {
		if let Some(ref callback) = self.on_click {
			callback(event);
		}
	}
}

impl Component for Button {
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

// Manual Debug implementation due to callback
impl std::fmt::Debug for Button {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Button")
			.field("properties", &self.properties)
			.field("text", &self.text)
			.field("variant", &self.variant)
			.field("color", &self.color)
			.field("background_color", &self.background_color)
			.field("on_click", &self.on_click.is_some())
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use engage_ux_core::events::EventType;

	#[test]
	fn test_button_creation() {
		let button = Button::new(1, "Click me");
		assert_eq!(button.id(), 1);
		assert_eq!(button.text(), "Click me");
		assert_eq!(button.variant(), ButtonVariant::Primary);
	}

	#[test]
	fn test_button_text() {
		let mut button = Button::new(1, "Initial");
		button.set_text("Updated");
		assert_eq!(button.text(), "Updated");
	}

	#[test]
	fn test_button_variant() {
		let mut button = Button::new(1, "Button");
		button.set_variant(ButtonVariant::Outlined);
		assert_eq!(button.variant(), ButtonVariant::Outlined);
	}

	#[test]
	fn test_button_colors() {
		let mut button = Button::new(1, "Button");
		let color = Color::from_hex("#FF0000").unwrap();
		button.set_color(color.clone());
		assert_eq!(button.color(), &color);
	}

	#[test]
	fn test_button_click_handler() {
		let mut button = Button::new(1, "Button");
		let clicked = Arc::new(std::sync::atomic::AtomicBool::new(false));
		let clicked_clone = clicked.clone();
		
		button.set_on_click(move |_event| {
			clicked_clone.store(true, std::sync::atomic::Ordering::Relaxed);
		});
		
		let event = Event::new(1, EventType::Click);
		button.handle_click(&event);
		
		assert!(clicked.load(std::sync::atomic::Ordering::Relaxed));
	}

	#[test]
	fn test_button_component_trait() {
		let mut button = Button::new(1, "Button");
		assert!(button.is_visible());
		assert!(button.is_enabled());
		
		button.set_enabled(false);
		assert!(!button.is_enabled());
	}
}
