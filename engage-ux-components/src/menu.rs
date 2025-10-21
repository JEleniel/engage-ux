//! Menu components (dropdown, drawer, hamburger)

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Menu item
#[derive(Clone, Serialize, Deserialize)]
pub struct MenuItem {
	pub id: String,
	pub label: String,
	pub icon: Option<String>,
	pub disabled: bool,
	pub separator: bool,
	pub shortcut: Option<String>,
	pub submenu: Vec<MenuItem>,
	#[serde(skip)]
	pub on_click: Option<EventCallback>,
}

impl MenuItem {
	/// Create a new menu item
	pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			label: label.into(),
			icon: None,
			disabled: false,
			separator: false,
			shortcut: None,
			submenu: Vec::new(),
			on_click: None,
		}
	}

	/// Create a separator
	pub fn separator() -> Self {
		Self {
			id: String::new(),
			label: String::new(),
			icon: None,
			disabled: false,
			separator: true,
			shortcut: None,
			submenu: Vec::new(),
			on_click: None,
		}
	}

	/// Add submenu item
	pub fn add_submenu_item(&mut self, item: MenuItem) {
		self.submenu.push(item);
	}

	/// Set click callback
	pub fn set_on_click(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_click = Some(std::sync::Arc::new(callback));
	}

	/// Handle click
	pub fn handle_click(&self, event: &Event) {
		if let Some(ref callback) = self.on_click {
			callback(event);
		}
	}
}

/// Dropdown menu component
#[derive(Clone, Serialize, Deserialize)]
pub struct Dropdown {
	properties: ComponentProperties,
	items: Vec<MenuItem>,
	open: bool,
	trigger_id: Option<ComponentId>,
	color: Color,
	background_color: Color,
	hover_color: Color,
	border_color: Color,
}

impl Dropdown {
	/// Create a new dropdown menu
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			items: Vec::new(),
			open: false,
			trigger_id: None,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			hover_color: Color::from_hex("#E0E0E0").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
		}
	}

	/// Add menu item
	pub fn add_item(&mut self, item: MenuItem) {
		self.items.push(item);
	}

	/// Get items
	pub fn items(&self) -> &[MenuItem] {
		&self.items
	}

	/// Get items (mutable)
	pub fn items_mut(&mut self) -> &mut Vec<MenuItem> {
		&mut self.items
	}

	/// Check if open
	pub fn is_open(&self) -> bool {
		self.open
	}

	/// Open menu
	pub fn open(&mut self) {
		self.open = true;
	}

	/// Close menu
	pub fn close(&mut self) {
		self.open = false;
	}

	/// Toggle menu
	pub fn toggle(&mut self) {
		self.open = !self.open;
	}

	/// Get trigger component ID
	pub fn trigger_id(&self) -> Option<ComponentId> {
		self.trigger_id
	}

	/// Set trigger component ID
	pub fn set_trigger_id(&mut self, trigger_id: Option<ComponentId>) {
		self.trigger_id = trigger_id;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set hover color
	pub fn set_hover_color(&mut self, color: Color) {
		self.hover_color = color;
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Color) {
		self.border_color = color;
	}
}

impl Component for Dropdown {
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

/// Drawer position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawerPosition {
	Left,
	Right,
	Top,
	Bottom,
}

/// Drawer component (side panel)
#[derive(Clone, Serialize, Deserialize)]
pub struct Drawer {
	properties: ComponentProperties,
	position: DrawerPosition,
	open: bool,
	overlay: bool,
	width: f32,
	color: Color,
	background_color: Color,
	overlay_color: Color,
	#[serde(skip)]
	on_close: Option<EventCallback>,
}

impl Drawer {
	/// Create a new drawer
	pub fn new(id: ComponentId, position: DrawerPosition) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			position,
			open: false,
			overlay: true,
			width: 280.0,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			overlay_color: Color::from_hex("#00000080").unwrap(),
			on_close: None,
		}
	}

	/// Get position
	pub fn position(&self) -> DrawerPosition {
		self.position
	}

	/// Set position
	pub fn set_position(&mut self, position: DrawerPosition) {
		self.position = position;
	}

	/// Check if open
	pub fn is_open(&self) -> bool {
		self.open
	}

	/// Open drawer
	pub fn open(&mut self) {
		self.open = true;
	}

	/// Close drawer
	pub fn close(&mut self) {
		self.open = false;
	}

	/// Toggle drawer
	pub fn toggle(&mut self) {
		self.open = !self.open;
	}

	/// Check if overlay mode
	pub fn has_overlay(&self) -> bool {
		self.overlay
	}

	/// Set overlay mode
	pub fn set_overlay(&mut self, overlay: bool) {
		self.overlay = overlay;
	}

	/// Get width
	pub fn width(&self) -> f32 {
		self.width
	}

	/// Set width
	pub fn set_width(&mut self, width: f32) {
		self.width = width.max(100.0);
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set overlay color
	pub fn set_overlay_color(&mut self, color: Color) {
		self.overlay_color = color;
	}

	/// Set close callback
	pub fn set_on_close(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_close = Some(std::sync::Arc::new(callback));
	}

	/// Handle close event
	pub fn handle_close(&self, event: &Event) {
		if let Some(ref callback) = self.on_close {
			callback(event);
		}
	}
}

impl Component for Drawer {
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

/// Hamburger menu button (typically opens a drawer or menu)
#[derive(Clone, Serialize, Deserialize)]
pub struct HamburgerMenu {
	properties: ComponentProperties,
	open: bool,
	animated: bool,
	color: Color,
	size: f32,
	#[serde(skip)]
	on_toggle: Option<EventCallback>,
}

impl HamburgerMenu {
	/// Create a new hamburger menu button
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			open: false,
			animated: true,
			color: Color::from_hex("#000000").unwrap(),
			size: 24.0,
			on_toggle: None,
		}
	}

	/// Check if open/active
	pub fn is_open(&self) -> bool {
		self.open
	}

	/// Open/activate
	pub fn open(&mut self) {
		self.open = true;
	}

	/// Close/deactivate
	pub fn close(&mut self) {
		self.open = false;
	}

	/// Toggle state
	pub fn toggle(&mut self) {
		self.open = !self.open;
	}

	/// Check if animated
	pub fn is_animated(&self) -> bool {
		self.animated
	}

	/// Set animated
	pub fn set_animated(&mut self, animated: bool) {
		self.animated = animated;
	}

	/// Get color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get size
	pub fn size(&self) -> f32 {
		self.size
	}

	/// Set size
	pub fn set_size(&mut self, size: f32) {
		self.size = size.max(16.0);
	}

	/// Set toggle callback
	pub fn set_on_toggle(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_toggle = Some(std::sync::Arc::new(callback));
	}

	/// Handle toggle event
	pub fn handle_toggle(&self, event: &Event) {
		if let Some(ref callback) = self.on_toggle {
			callback(event);
		}
	}
}

impl Component for HamburgerMenu {
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
	fn test_menu_item_creation() {
		let item = MenuItem::new("file", "File");
		assert_eq!(item.id, "file");
		assert_eq!(item.label, "File");
		assert!(!item.disabled);
		assert!(!item.separator);
	}

	#[test]
	fn test_menu_item_separator() {
		let separator = MenuItem::separator();
		assert!(separator.separator);
	}

	#[test]
	fn test_menu_item_submenu() {
		let mut item = MenuItem::new("file", "File");
		item.add_submenu_item(MenuItem::new("new", "New"));
		assert_eq!(item.submenu.len(), 1);
	}

	#[test]
	fn test_dropdown_creation() {
		let dropdown = Dropdown::new(1);
		assert_eq!(dropdown.id(), 1);
		assert!(!dropdown.is_open());
		assert_eq!(dropdown.items().len(), 0);
	}

	#[test]
	fn test_dropdown_toggle() {
		let mut dropdown = Dropdown::new(1);
		dropdown.toggle();
		assert!(dropdown.is_open());
		dropdown.toggle();
		assert!(!dropdown.is_open());
	}

	#[test]
	fn test_dropdown_items() {
		let mut dropdown = Dropdown::new(1);
		dropdown.add_item(MenuItem::new("1", "Item 1"));
		dropdown.add_item(MenuItem::new("2", "Item 2"));
		assert_eq!(dropdown.items().len(), 2);
	}

	#[test]
	fn test_drawer_creation() {
		let drawer = Drawer::new(1, DrawerPosition::Left);
		assert_eq!(drawer.id(), 1);
		assert_eq!(drawer.position(), DrawerPosition::Left);
		assert!(!drawer.is_open());
	}

	#[test]
	fn test_drawer_toggle() {
		let mut drawer = Drawer::new(1, DrawerPosition::Left);
		drawer.toggle();
		assert!(drawer.is_open());
		drawer.toggle();
		assert!(!drawer.is_open());
	}

	#[test]
	fn test_drawer_overlay() {
		let mut drawer = Drawer::new(1, DrawerPosition::Left);
		assert!(drawer.has_overlay());
		drawer.set_overlay(false);
		assert!(!drawer.has_overlay());
	}

	#[test]
	fn test_hamburger_menu_creation() {
		let hamburger = HamburgerMenu::new(1);
		assert_eq!(hamburger.id(), 1);
		assert!(!hamburger.is_open());
		assert!(hamburger.is_animated());
	}

	#[test]
	fn test_hamburger_menu_toggle() {
		let mut hamburger = HamburgerMenu::new(1);
		hamburger.toggle();
		assert!(hamburger.is_open());
		hamburger.toggle();
		assert!(!hamburger.is_open());
	}
}
