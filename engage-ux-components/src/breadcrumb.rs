//! Breadcrumb navigation component

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Breadcrumb item
#[derive(Clone, Serialize, Deserialize)]
pub struct BreadcrumbItem {
	pub label: String,
	pub href: Option<String>,
	pub icon: Option<String>,
	#[serde(skip)]
	pub on_click: Option<EventCallback>,
}

impl BreadcrumbItem {
	/// Create a new breadcrumb item
	pub fn new(label: impl Into<String>) -> Self {
		Self {
			label: label.into(),
			href: None,
			icon: None,
			on_click: None,
		}
	}

	/// Create a breadcrumb item with href
	pub fn with_href(label: impl Into<String>, href: impl Into<String>) -> Self {
		Self {
			label: label.into(),
			href: Some(href.into()),
			icon: None,
			on_click: None,
		}
	}

	/// Set click callback
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

/// Breadcrumb component
#[derive(Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
	properties: ComponentProperties,
	items: Vec<BreadcrumbItem>,
	separator: String,
	max_items: Option<usize>,
	color: Color,
	active_color: Color,
	separator_color: Color,
	font_size: f32,
}

impl Breadcrumb {
	/// Create a new breadcrumb
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			items: Vec::new(),
			separator: "/".to_string(),
			max_items: None,
			color: Color::from_hex("#757575").unwrap(),
			active_color: Color::from_hex("#000000").unwrap(),
			separator_color: Color::from_hex("#CCCCCC").unwrap(),
			font_size: 14.0,
		}
	}

	/// Add an item
	pub fn add_item(&mut self, item: BreadcrumbItem) {
		self.items.push(item);
	}

	/// Insert an item at a specific position
	pub fn insert_item(&mut self, index: usize, item: BreadcrumbItem) {
		if index <= self.items.len() {
			self.items.insert(index, item);
		}
	}

	/// Remove an item at a specific position
	pub fn remove_item(&mut self, index: usize) {
		if index < self.items.len() {
			self.items.remove(index);
		}
	}

	/// Get items
	pub fn items(&self) -> &[BreadcrumbItem] {
		&self.items
	}

	/// Get items (mutable)
	pub fn items_mut(&mut self) -> &mut Vec<BreadcrumbItem> {
		&mut self.items
	}

	/// Clear all items
	pub fn clear(&mut self) {
		self.items.clear();
	}

	/// Get separator
	pub fn separator(&self) -> &str {
		&self.separator
	}

	/// Set separator
	pub fn set_separator(&mut self, separator: impl Into<String>) {
		self.separator = separator.into();
	}

	/// Get max items
	pub fn max_items(&self) -> Option<usize> {
		self.max_items
	}

	/// Set max items (will show ellipsis if exceeded)
	pub fn set_max_items(&mut self, max_items: Option<usize>) {
		self.max_items = max_items;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set active (current) item color
	pub fn set_active_color(&mut self, color: Color) {
		self.active_color = color;
	}

	/// Set separator color
	pub fn set_separator_color(&mut self, color: Color) {
		self.separator_color = color;
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

impl Component for Breadcrumb {
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
	fn test_breadcrumb_item_creation() {
		let item = BreadcrumbItem::new("Home");
		assert_eq!(item.label, "Home");
		assert_eq!(item.href, None);
	}

	#[test]
	fn test_breadcrumb_item_with_href() {
		let item = BreadcrumbItem::with_href("Home", "/home");
		assert_eq!(item.label, "Home");
		assert_eq!(item.href, Some("/home".to_string()));
	}

	#[test]
	fn test_breadcrumb_creation() {
		let breadcrumb = Breadcrumb::new(1);
		assert_eq!(breadcrumb.id(), 1);
		assert_eq!(breadcrumb.items().len(), 0);
		assert_eq!(breadcrumb.separator(), "/");
	}

	#[test]
	fn test_breadcrumb_add_items() {
		let mut breadcrumb = Breadcrumb::new(1);
		breadcrumb.add_item(BreadcrumbItem::new("Home"));
		breadcrumb.add_item(BreadcrumbItem::new("Products"));
		breadcrumb.add_item(BreadcrumbItem::new("Laptop"));

		assert_eq!(breadcrumb.items().len(), 3);
	}

	#[test]
	fn test_breadcrumb_insert_remove() {
		let mut breadcrumb = Breadcrumb::new(1);
		breadcrumb.add_item(BreadcrumbItem::new("Home"));
		breadcrumb.add_item(BreadcrumbItem::new("Laptop"));

		breadcrumb.insert_item(1, BreadcrumbItem::new("Products"));
		assert_eq!(breadcrumb.items().len(), 3);
		assert_eq!(breadcrumb.items()[1].label, "Products");

		breadcrumb.remove_item(1);
		assert_eq!(breadcrumb.items().len(), 2);
	}

	#[test]
	fn test_breadcrumb_separator() {
		let mut breadcrumb = Breadcrumb::new(1);
		breadcrumb.set_separator(">");
		assert_eq!(breadcrumb.separator(), ">");
	}

	#[test]
	fn test_breadcrumb_clear() {
		let mut breadcrumb = Breadcrumb::new(1);
		breadcrumb.add_item(BreadcrumbItem::new("Home"));
		breadcrumb.add_item(BreadcrumbItem::new("Products"));

		breadcrumb.clear();
		assert_eq!(breadcrumb.items().len(), 0);
	}
}
