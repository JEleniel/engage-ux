//! Tooltip and popover component

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Tooltip position relative to target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TooltipPosition {
	Top,
	Bottom,
	Left,
	Right,
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
}

/// Tooltip component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltip {
	properties: ComponentProperties,
	content: String,
	target_id: ComponentId,
	position: TooltipPosition,
	visible: bool,
	delay_ms: u64,
	max_width: Option<f32>,
	color: Color,
	background_color: Color,
	border_color: Color,
	font_size: f32,
}

impl Tooltip {
	/// Create a new tooltip
	pub fn new(id: ComponentId, target_id: ComponentId, content: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			content: content.into(),
			target_id,
			position: TooltipPosition::Top,
			visible: false,
			delay_ms: 500,
			max_width: Some(300.0),
			color: Color::from_hex("#FFFFFF").unwrap(),
			background_color: Color::from_hex("#424242").unwrap(),
			border_color: Color::from_hex("#424242").unwrap(),
			font_size: 12.0,
		}
	}

	/// Get content
	pub fn content(&self) -> &str {
		&self.content
	}

	/// Set content
	pub fn set_content(&mut self, content: impl Into<String>) {
		self.content = content.into();
	}

	/// Get target component ID
	pub fn target_id(&self) -> ComponentId {
		self.target_id
	}

	/// Set target component ID
	pub fn set_target_id(&mut self, target_id: ComponentId) {
		self.target_id = target_id;
	}

	/// Get position
	pub fn position(&self) -> TooltipPosition {
		self.position
	}

	/// Set position
	pub fn set_position(&mut self, position: TooltipPosition) {
		self.position = position;
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show tooltip
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide tooltip
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Toggle visibility
	pub fn toggle(&mut self) {
		self.visible = !self.visible;
	}

	/// Get delay in milliseconds
	pub fn delay_ms(&self) -> u64 {
		self.delay_ms
	}

	/// Set delay in milliseconds before showing
	pub fn set_delay_ms(&mut self, delay_ms: u64) {
		self.delay_ms = delay_ms;
	}

	/// Get max width
	pub fn max_width(&self) -> Option<f32> {
		self.max_width
	}

	/// Set max width
	pub fn set_max_width(&mut self, max_width: Option<f32>) {
		self.max_width = max_width;
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

	/// Set font size
	pub fn set_font_size(&mut self, size: f32) {
		self.font_size = size;
	}

	/// Get font size
	pub fn font_size(&self) -> f32 {
		self.font_size
	}
}

impl Component for Tooltip {
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

/// Popover component (extended tooltip with interactive content)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Popover {
	properties: ComponentProperties,
	title: String,
	content: String,
	target_id: ComponentId,
	position: TooltipPosition,
	visible: bool,
	dismissible: bool,
	max_width: Option<f32>,
	color: Color,
	background_color: Color,
	border_color: Color,
}

impl Popover {
	/// Create a new popover
	pub fn new(id: ComponentId, target_id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: String::new(),
			content: String::new(),
			target_id,
			position: TooltipPosition::Bottom,
			visible: false,
			dismissible: true,
			max_width: Some(400.0),
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
		}
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Get content
	pub fn content(&self) -> &str {
		&self.content
	}

	/// Set content
	pub fn set_content(&mut self, content: impl Into<String>) {
		self.content = content.into();
	}

	/// Get target component ID
	pub fn target_id(&self) -> ComponentId {
		self.target_id
	}

	/// Get position
	pub fn position(&self) -> TooltipPosition {
		self.position
	}

	/// Set position
	pub fn set_position(&mut self, position: TooltipPosition) {
		self.position = position;
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show popover
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide popover
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Toggle visibility
	pub fn toggle(&mut self) {
		self.visible = !self.visible;
	}

	/// Check if dismissible
	pub fn is_dismissible(&self) -> bool {
		self.dismissible
	}

	/// Set dismissible
	pub fn set_dismissible(&mut self, dismissible: bool) {
		self.dismissible = dismissible;
	}

	/// Get max width
	pub fn max_width(&self) -> Option<f32> {
		self.max_width
	}

	/// Set max width
	pub fn set_max_width(&mut self, max_width: Option<f32>) {
		self.max_width = max_width;
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

impl Component for Popover {
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
	fn test_tooltip_creation() {
		let tooltip = Tooltip::new(1, 100, "Helpful text");
		assert_eq!(tooltip.id(), 1);
		assert_eq!(tooltip.target_id(), 100);
		assert_eq!(tooltip.content(), "Helpful text");
		assert!(!tooltip.is_visible());
	}

	#[test]
	fn test_tooltip_visibility() {
		let mut tooltip = Tooltip::new(1, 100, "Text");
		tooltip.show();
		assert!(tooltip.is_visible());
		tooltip.hide();
		assert!(!tooltip.is_visible());
	}

	#[test]
	fn test_tooltip_position() {
		let mut tooltip = Tooltip::new(1, 100, "Text");
		tooltip.set_position(TooltipPosition::Bottom);
		assert_eq!(tooltip.position(), TooltipPosition::Bottom);
	}

	#[test]
	fn test_popover_creation() {
		let popover = Popover::new(1, 100);
		assert_eq!(popover.id(), 1);
		assert_eq!(popover.target_id(), 100);
		assert!(!popover.is_visible());
	}

	#[test]
	fn test_popover_content() {
		let mut popover = Popover::new(1, 100);
		popover.set_title("Help");
		popover.set_content("Detailed help text");
		assert_eq!(popover.title(), "Help");
		assert_eq!(popover.content(), "Detailed help text");
	}

	#[test]
	fn test_popover_dismissible() {
		let mut popover = Popover::new(1, 100);
		assert!(popover.is_dismissible());
		popover.set_dismissible(false);
		assert!(!popover.is_dismissible());
	}
}
