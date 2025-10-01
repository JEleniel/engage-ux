//! List component for displaying collections of items

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// List item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
	pub text: String,
	pub value: String,
	pub disabled: bool,
}

impl ListItem {
	pub fn new(text: impl Into<String>, value: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			value: value.into(),
			disabled: false,
		}
	}
}

/// List component for displaying collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
	properties: ComponentProperties,
	items: Vec<ListItem>,
	selected_index: Option<usize>,
	multi_select: bool,
	selected_indices: Vec<usize>,
	item_height: f32,
	color: Color,
	background_color: Color,
	selected_color: Color,
	hover_color: Color,
}

impl List {
	/// Create a new list
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			items: Vec::new(),
			selected_index: None,
			multi_select: false,
			selected_indices: Vec::new(),
			item_height: 40.0,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			selected_color: Color::from_hex("#1976D2").unwrap(),
			hover_color: Color::from_hex("#E3F2FD").unwrap(),
		}
	}

	/// Add an item to the list
	pub fn add_item(&mut self, item: ListItem) {
		self.items.push(item);
	}

	/// Set items
	pub fn set_items(&mut self, items: Vec<ListItem>) {
		self.items = items;
	}

	/// Get items
	pub fn items(&self) -> &[ListItem] {
		&self.items
	}

	/// Select an item by index
	pub fn select(&mut self, index: usize) {
		if index < self.items.len() {
			if self.multi_select {
				if !self.selected_indices.contains(&index) {
					self.selected_indices.push(index);
				}
			} else {
				self.selected_index = Some(index);
			}
		}
	}

	/// Deselect an item by index
	pub fn deselect(&mut self, index: usize) {
		if self.multi_select {
			self.selected_indices.retain(|&i| i != index);
		} else if self.selected_index == Some(index) {
			self.selected_index = None;
		}
	}

	/// Get selected index (single select)
	pub fn selected_index(&self) -> Option<usize> {
		self.selected_index
	}

	/// Get selected indices (multi select)
	pub fn selected_indices(&self) -> &[usize] {
		&self.selected_indices
	}

	/// Enable multi-select
	pub fn set_multi_select(&mut self, multi_select: bool) {
		self.multi_select = multi_select;
	}

	/// Check if multi-select is enabled
	pub fn is_multi_select(&self) -> bool {
		self.multi_select
	}

	/// Set item height
	pub fn set_item_height(&mut self, height: f32) {
		self.item_height = height;
	}

	/// Get item height
	pub fn item_height(&self) -> f32 {
		self.item_height
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set selected item color
	pub fn set_selected_color(&mut self, color: Color) {
		self.selected_color = color;
	}

	/// Set hover color
	pub fn set_hover_color(&mut self, color: Color) {
		self.hover_color = color;
	}
}

impl Component for List {
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
	fn test_list_creation() {
		let list = List::new(1);
		assert_eq!(list.id(), 1);
		assert_eq!(list.items().len(), 0);
	}

	#[test]
	fn test_list_add_items() {
		let mut list = List::new(1);
		list.add_item(ListItem::new("Item 1", "1"));
		list.add_item(ListItem::new("Item 2", "2"));
		assert_eq!(list.items().len(), 2);
	}

	#[test]
	fn test_list_selection() {
		let mut list = List::new(1);
		list.add_item(ListItem::new("Item 1", "1"));
		list.add_item(ListItem::new("Item 2", "2"));
		
		list.select(0);
		assert_eq!(list.selected_index(), Some(0));
		
		list.deselect(0);
		assert_eq!(list.selected_index(), None);
	}

	#[test]
	fn test_list_multi_select() {
		let mut list = List::new(1);
		list.set_multi_select(true);
		list.add_item(ListItem::new("Item 1", "1"));
		list.add_item(ListItem::new("Item 2", "2"));
		list.add_item(ListItem::new("Item 3", "3"));
		
		list.select(0);
		list.select(2);
		assert_eq!(list.selected_indices().len(), 2);
		assert!(list.selected_indices().contains(&0));
		assert!(list.selected_indices().contains(&2));
	}
}
