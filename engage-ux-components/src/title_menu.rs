//! Title menu component (menu bar with dropdown menus)

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

use crate::menu::MenuItem;

/// Title menu item (top-level menu button)
#[derive(Clone, Serialize, Deserialize)]
pub struct TitleMenuItem {
	pub id: String,
	pub label: String,
	pub items: Vec<MenuItem>,
	pub disabled: bool,
}

impl TitleMenuItem {
	pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			label: label.into(),
			items: Vec::new(),
			disabled: false,
		}
	}

	pub fn add_item(&mut self, item: MenuItem) {
		self.items.push(item);
	}
}

/// Title menu component (application menu bar)
#[derive(Clone, Serialize, Deserialize)]
pub struct TitleMenu {
	properties: ComponentProperties,
	menus: Vec<TitleMenuItem>,
	active_menu: Option<String>,
	color: Color,
	background_color: Color,
	hover_color: Color,
	hover_background: Color,
	active_color: Color,
	active_background: Color,
	height: f32,
}

impl TitleMenu {
	/// Create a new title menu
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			menus: Vec::new(),
			active_menu: None,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#F5F5F5").unwrap(),
			hover_color: Color::from_hex("#000000").unwrap(),
			hover_background: Color::from_hex("#E0E0E0").unwrap(),
			active_color: Color::from_hex("#FFFFFF").unwrap(),
			active_background: Color::from_hex("#1976D2").unwrap(),
			height: 30.0,
		}
	}

	/// Add a menu
	pub fn add_menu(&mut self, menu: TitleMenuItem) {
		self.menus.push(menu);
	}

	/// Get menus
	pub fn menus(&self) -> &[TitleMenuItem] {
		&self.menus
	}

	/// Get menus (mutable)
	pub fn menus_mut(&mut self) -> &mut Vec<TitleMenuItem> {
		&mut self.menus
	}

	/// Get active menu ID
	pub fn active_menu(&self) -> Option<&str> {
		self.active_menu.as_deref()
	}

	/// Set active menu
	pub fn set_active_menu(&mut self, menu_id: Option<String>) {
		self.active_menu = menu_id;
	}

	/// Open menu
	pub fn open_menu(&mut self, menu_id: impl Into<String>) {
		self.active_menu = Some(menu_id.into());
	}

	/// Close active menu
	pub fn close_menu(&mut self) {
		self.active_menu = None;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set hover text color
	pub fn set_hover_color(&mut self, color: Color) {
		self.hover_color = color;
	}

	/// Set hover background color
	pub fn set_hover_background(&mut self, color: Color) {
		self.hover_background = color;
	}

	/// Set active text color
	pub fn set_active_color(&mut self, color: Color) {
		self.active_color = color;
	}

	/// Set active background color
	pub fn set_active_background(&mut self, color: Color) {
		self.active_background = color;
	}

	/// Get height
	pub fn height(&self) -> f32 {
		self.height
	}

	/// Set height
	pub fn set_height(&mut self, height: f32) {
		self.height = height.max(20.0);
	}
}

impl Component for TitleMenu {
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
	fn test_title_menu_creation() {
		let menu = TitleMenu::new(1);
		assert_eq!(menu.id(), 1);
		assert_eq!(menu.menus().len(), 0);
		assert_eq!(menu.active_menu(), None);
	}

	#[test]
	fn test_title_menu_add_menus() {
		let mut menu = TitleMenu::new(1);

		let mut file_menu = TitleMenuItem::new("file", "File");
		file_menu.add_item(MenuItem::new("new", "New"));
		file_menu.add_item(MenuItem::new("open", "Open"));

		let mut edit_menu = TitleMenuItem::new("edit", "Edit");
		edit_menu.add_item(MenuItem::new("cut", "Cut"));
		edit_menu.add_item(MenuItem::new("copy", "Copy"));

		menu.add_menu(file_menu);
		menu.add_menu(edit_menu);

		assert_eq!(menu.menus().len(), 2);
	}

	#[test]
	fn test_title_menu_active() {
		let mut menu = TitleMenu::new(1);
		menu.add_menu(TitleMenuItem::new("file", "File"));

		menu.open_menu("file");
		assert_eq!(menu.active_menu(), Some("file"));

		menu.close_menu();
		assert_eq!(menu.active_menu(), None);
	}

	#[test]
	fn test_title_menu_height() {
		let mut menu = TitleMenu::new(1);
		menu.set_height(40.0);
		assert_eq!(menu.height(), 40.0);
	}
}
