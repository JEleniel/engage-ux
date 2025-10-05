//! Window component for application windows

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowState {
	Normal,
	Minimized,
	Maximized,
	Fullscreen,
}

/// Window component
#[derive(Clone, Serialize, Deserialize)]
pub struct Window {
	properties: ComponentProperties,
	title: String,
	state: WindowState,
	resizable: bool,
	movable: bool,
	closable: bool,
	minimizable: bool,
	maximizable: bool,
	modal: bool,
	always_on_top: bool,
	show_title_bar: bool,
	show_menu_bar: bool,
	min_width: Option<f32>,
	min_height: Option<f32>,
	max_width: Option<f32>,
	max_height: Option<f32>,
	children: Vec<ComponentId>,
	title_bar_height: f32,
	menu_bar_height: f32,
	color: Color,
	background_color: Color,
	title_bar_color: Color,
	title_bar_background: Color,
	border_color: Color,
	#[serde(skip)]
	on_close: Option<EventCallback>,
	#[serde(skip)]
	on_minimize: Option<EventCallback>,
	#[serde(skip)]
	on_maximize: Option<EventCallback>,
	#[serde(skip)]
	on_resize: Option<EventCallback>,
	#[serde(skip)]
	on_move: Option<EventCallback>,
}

impl Window {
	/// Create a new window
	pub fn new(id: ComponentId, title: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: title.into(),
			state: WindowState::Normal,
			resizable: true,
			movable: true,
			closable: true,
			minimizable: true,
			maximizable: true,
			modal: false,
			always_on_top: false,
			show_title_bar: true,
			show_menu_bar: false,
			min_width: Some(200.0),
			min_height: Some(100.0),
			max_width: None,
			max_height: None,
			children: Vec::new(),
			title_bar_height: 32.0,
			menu_bar_height: 30.0,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			title_bar_color: Color::from_hex("#FFFFFF").unwrap(),
			title_bar_background: Color::from_hex("#2C2C2C").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
			on_close: None,
			on_minimize: None,
			on_maximize: None,
			on_resize: None,
			on_move: None,
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

	/// Get state
	pub fn state(&self) -> WindowState {
		self.state
	}

	/// Set state
	pub fn set_state(&mut self, state: WindowState) {
		self.state = state;
	}

	/// Check if window is minimized
	pub fn is_minimized(&self) -> bool {
		self.state == WindowState::Minimized
	}

	/// Minimize window
	pub fn minimize(&mut self) {
		if self.minimizable {
			self.state = WindowState::Minimized;
		}
	}

	/// Check if window is maximized
	pub fn is_maximized(&self) -> bool {
		self.state == WindowState::Maximized
	}

	/// Maximize window
	pub fn maximize(&mut self) {
		if self.maximizable {
			self.state = WindowState::Maximized;
		}
	}

	/// Restore window to normal state
	pub fn restore(&mut self) {
		self.state = WindowState::Normal;
	}

	/// Check if window is fullscreen
	pub fn is_fullscreen(&self) -> bool {
		self.state == WindowState::Fullscreen
	}

	/// Set fullscreen
	pub fn set_fullscreen(&mut self, fullscreen: bool) {
		self.state = if fullscreen {
			WindowState::Fullscreen
		} else {
			WindowState::Normal
		};
	}

	/// Check if resizable
	pub fn is_resizable(&self) -> bool {
		self.resizable
	}

	/// Set resizable
	pub fn set_resizable(&mut self, resizable: bool) {
		self.resizable = resizable;
	}

	/// Check if movable
	pub fn is_movable(&self) -> bool {
		self.movable
	}

	/// Set movable
	pub fn set_movable(&mut self, movable: bool) {
		self.movable = movable;
	}

	/// Check if closable
	pub fn is_closable(&self) -> bool {
		self.closable
	}

	/// Set closable
	pub fn set_closable(&mut self, closable: bool) {
		self.closable = closable;
	}

	/// Check if minimizable
	pub fn is_minimizable(&self) -> bool {
		self.minimizable
	}

	/// Set minimizable
	pub fn set_minimizable(&mut self, minimizable: bool) {
		self.minimizable = minimizable;
	}

	/// Check if maximizable
	pub fn is_maximizable(&self) -> bool {
		self.maximizable
	}

	/// Set maximizable
	pub fn set_maximizable(&mut self, maximizable: bool) {
		self.maximizable = maximizable;
	}

	/// Check if modal
	pub fn is_modal(&self) -> bool {
		self.modal
	}

	/// Set modal
	pub fn set_modal(&mut self, modal: bool) {
		self.modal = modal;
	}

	/// Check if always on top
	pub fn is_always_on_top(&self) -> bool {
		self.always_on_top
	}

	/// Set always on top
	pub fn set_always_on_top(&mut self, always_on_top: bool) {
		self.always_on_top = always_on_top;
	}

	/// Check if title bar is shown
	pub fn shows_title_bar(&self) -> bool {
		self.show_title_bar
	}

	/// Set whether to show title bar
	pub fn set_show_title_bar(&mut self, show: bool) {
		self.show_title_bar = show;
	}

	/// Check if menu bar is shown
	pub fn shows_menu_bar(&self) -> bool {
		self.show_menu_bar
	}

	/// Set whether to show menu bar
	pub fn set_show_menu_bar(&mut self, show: bool) {
		self.show_menu_bar = show;
	}

	/// Get minimum width
	pub fn min_width(&self) -> Option<f32> {
		self.min_width
	}

	/// Set minimum width
	pub fn set_min_width(&mut self, width: Option<f32>) {
		self.min_width = width;
	}

	/// Get minimum height
	pub fn min_height(&self) -> Option<f32> {
		self.min_height
	}

	/// Set minimum height
	pub fn set_min_height(&mut self, height: Option<f32>) {
		self.min_height = height;
	}

	/// Get maximum width
	pub fn max_width(&self) -> Option<f32> {
		self.max_width
	}

	/// Set maximum width
	pub fn set_max_width(&mut self, width: Option<f32>) {
		self.max_width = width;
	}

	/// Get maximum height
	pub fn max_height(&self) -> Option<f32> {
		self.max_height
	}

	/// Set maximum height
	pub fn set_max_height(&mut self, height: Option<f32>) {
		self.max_height = height;
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

	/// Get title bar height
	pub fn title_bar_height(&self) -> f32 {
		self.title_bar_height
	}

	/// Set title bar height
	pub fn set_title_bar_height(&mut self, height: f32) {
		self.title_bar_height = height.max(20.0);
	}

	/// Get menu bar height
	pub fn menu_bar_height(&self) -> f32 {
		self.menu_bar_height
	}

	/// Set menu bar height
	pub fn set_menu_bar_height(&mut self, height: f32) {
		self.menu_bar_height = height.max(20.0);
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set title bar text color
	pub fn set_title_bar_color(&mut self, color: Color) {
		self.title_bar_color = color;
	}

	/// Set title bar background color
	pub fn set_title_bar_background(&mut self, color: Color) {
		self.title_bar_background = color;
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Color) {
		self.border_color = color;
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

	/// Set minimize callback
	pub fn set_on_minimize(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_minimize = Some(std::sync::Arc::new(callback));
	}

	/// Handle minimize event
	pub fn handle_minimize(&self, event: &Event) {
		if let Some(ref callback) = self.on_minimize {
			callback(event);
		}
	}

	/// Set maximize callback
	pub fn set_on_maximize(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_maximize = Some(std::sync::Arc::new(callback));
	}

	/// Handle maximize event
	pub fn handle_maximize(&self, event: &Event) {
		if let Some(ref callback) = self.on_maximize {
			callback(event);
		}
	}

	/// Set resize callback
	pub fn set_on_resize(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_resize = Some(std::sync::Arc::new(callback));
	}

	/// Handle resize event
	pub fn handle_resize(&self, event: &Event) {
		if let Some(ref callback) = self.on_resize {
			callback(event);
		}
	}

	/// Set move callback
	pub fn set_on_move(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_move = Some(std::sync::Arc::new(callback));
	}

	/// Handle move event
	pub fn handle_move(&self, event: &Event) {
		if let Some(ref callback) = self.on_move {
			callback(event);
		}
	}
}

impl Component for Window {
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
	fn test_window_creation() {
		let window = Window::new(1, "My Application");
		assert_eq!(window.id(), 1);
		assert_eq!(window.title(), "My Application");
		assert_eq!(window.state(), WindowState::Normal);
	}

	#[test]
	fn test_window_states() {
		let mut window = Window::new(1, "Test");

		window.minimize();
		assert!(window.is_minimized());

		window.restore();
		assert_eq!(window.state(), WindowState::Normal);

		window.maximize();
		assert!(window.is_maximized());

		window.set_fullscreen(true);
		assert!(window.is_fullscreen());
	}

	#[test]
	fn test_window_properties() {
		let mut window = Window::new(1, "Test");

		assert!(window.is_resizable());
		assert!(window.is_movable());
		assert!(window.is_closable());

		window.set_resizable(false);
		window.set_movable(false);
		window.set_closable(false);

		assert!(!window.is_resizable());
		assert!(!window.is_movable());
		assert!(!window.is_closable());
	}

	#[test]
	fn test_window_size_constraints() {
		let mut window = Window::new(1, "Test");

		window.set_min_width(Some(400.0));
		window.set_min_height(Some(300.0));
		window.set_max_width(Some(1920.0));
		window.set_max_height(Some(1080.0));

		assert_eq!(window.min_width(), Some(400.0));
		assert_eq!(window.min_height(), Some(300.0));
		assert_eq!(window.max_width(), Some(1920.0));
		assert_eq!(window.max_height(), Some(1080.0));
	}

	#[test]
	fn test_window_children() {
		let mut window = Window::new(1, "Test");
		window.add_child(10);
		window.add_child(20);
		window.add_child(30);

		assert_eq!(window.children().len(), 3);

		window.remove_child(20);
		assert_eq!(window.children().len(), 2);
	}

	#[test]
	fn test_window_modal() {
		let mut window = Window::new(1, "Test");
		assert!(!window.is_modal());

		window.set_modal(true);
		assert!(window.is_modal());
	}
}
