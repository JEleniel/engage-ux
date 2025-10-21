//! Group component for organizing related elements

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Group orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupOrientation {
	Horizontal,
	Vertical,
}

/// Group component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
	properties: ComponentProperties,
	title: String,
	orientation: GroupOrientation,
	children: Vec<ComponentId>,
	spacing: f32,
	padding: f32,
	collapsible: bool,
	collapsed: bool,
	show_border: bool,
	color: Color,
	background_color: Color,
	border_color: Color,
	title_color: Color,
	title_background: Color,
}

impl Group {
	/// Create a new group
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: String::new(),
			orientation: GroupOrientation::Vertical,
			children: Vec::new(),
			spacing: 8.0,
			padding: 12.0,
			collapsible: false,
			collapsed: false,
			show_border: true,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
			title_color: Color::from_hex("#000000").unwrap(),
			title_background: Color::from_hex("#F5F5F5").unwrap(),
		}
	}

	/// Create a group with title
	pub fn with_title(id: ComponentId, title: impl Into<String>) -> Self {
		let mut group = Self::new(id);
		group.title = title.into();
		group
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Get orientation
	pub fn orientation(&self) -> GroupOrientation {
		self.orientation
	}

	/// Set orientation
	pub fn set_orientation(&mut self, orientation: GroupOrientation) {
		self.orientation = orientation;
	}

	/// Add child component
	pub fn add_child(&mut self, child_id: ComponentId) {
		self.children.push(child_id);
	}

	/// Remove child component
	pub fn remove_child(&mut self, child_id: ComponentId) {
		self.children.retain(|&id| id != child_id);
	}

	/// Get children
	pub fn children(&self) -> &[ComponentId] {
		&self.children
	}

	/// Clear all children
	pub fn clear_children(&mut self) {
		self.children.clear();
	}

	/// Get spacing between children
	pub fn spacing(&self) -> f32 {
		self.spacing
	}

	/// Set spacing between children
	pub fn set_spacing(&mut self, spacing: f32) {
		self.spacing = spacing.max(0.0);
	}

	/// Get padding
	pub fn padding(&self) -> f32 {
		self.padding
	}

	/// Set padding
	pub fn set_padding(&mut self, padding: f32) {
		self.padding = padding.max(0.0);
	}

	/// Check if collapsible
	pub fn is_collapsible(&self) -> bool {
		self.collapsible
	}

	/// Set collapsible
	pub fn set_collapsible(&mut self, collapsible: bool) {
		self.collapsible = collapsible;
	}

	/// Check if collapsed
	pub fn is_collapsed(&self) -> bool {
		self.collapsed
	}

	/// Set collapsed
	pub fn set_collapsed(&mut self, collapsed: bool) {
		if self.collapsible {
			self.collapsed = collapsed;
		}
	}

	/// Toggle collapsed state
	pub fn toggle_collapsed(&mut self) {
		if self.collapsible {
			self.collapsed = !self.collapsed;
		}
	}

	/// Check if border is shown
	pub fn shows_border(&self) -> bool {
		self.show_border
	}

	/// Set whether to show border
	pub fn set_show_border(&mut self, show: bool) {
		self.show_border = show;
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

	/// Set title color
	pub fn set_title_color(&mut self, color: Color) {
		self.title_color = color;
	}

	/// Set title background color
	pub fn set_title_background(&mut self, color: Color) {
		self.title_background = color;
	}
}

impl Component for Group {
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
	fn test_group_creation() {
		let group = Group::new(1);
		assert_eq!(group.id(), 1);
		assert_eq!(group.children().len(), 0);
	}

	#[test]
	fn test_group_with_title() {
		let group = Group::with_title(1, "My Group");
		assert_eq!(group.title(), "My Group");
	}

	#[test]
	fn test_group_children() {
		let mut group = Group::new(1);
		group.add_child(10);
		group.add_child(20);
		group.add_child(30);

		assert_eq!(group.children().len(), 3);
		assert_eq!(group.children()[0], 10);

		group.remove_child(20);
		assert_eq!(group.children().len(), 2);
	}

	#[test]
	fn test_group_orientation() {
		let mut group = Group::new(1);
		assert_eq!(group.orientation(), GroupOrientation::Vertical);

		group.set_orientation(GroupOrientation::Horizontal);
		assert_eq!(group.orientation(), GroupOrientation::Horizontal);
	}

	#[test]
	fn test_group_collapsible() {
		let mut group = Group::new(1);
		assert!(!group.is_collapsible());
		assert!(!group.is_collapsed());

		group.set_collapsible(true);
		group.set_collapsed(true);
		assert!(group.is_collapsed());

		group.toggle_collapsed();
		assert!(!group.is_collapsed());
	}

	#[test]
	fn test_group_spacing_padding() {
		let mut group = Group::new(1);
		group.set_spacing(16.0);
		group.set_padding(20.0);

		assert_eq!(group.spacing(), 16.0);
		assert_eq!(group.padding(), 20.0);
	}
}
