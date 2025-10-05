//! Tabbed interface component

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Tab item
#[derive(Clone, Serialize, Deserialize)]
pub struct Tab {
	pub id: String,
	pub label: String,
	pub icon: Option<String>,
	pub closable: bool,
	pub disabled: bool,
	pub badge: Option<String>,
	#[serde(skip)]
	pub on_close: Option<EventCallback>,
}

impl Tab {
	/// Create a new tab
	pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			label: label.into(),
			icon: None,
			closable: false,
			disabled: false,
			badge: None,
			on_close: None,
		}
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

/// Tab position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabPosition {
	Top,
	Bottom,
	Left,
	Right,
}

/// Tabs component
#[derive(Clone, Serialize, Deserialize)]
pub struct Tabs {
	properties: ComponentProperties,
	tabs: Vec<Tab>,
	active_tab: Option<String>,
	position: TabPosition,
	color: Color,
	background_color: Color,
	active_color: Color,
	active_background: Color,
	border_color: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl Tabs {
	/// Create a new tabs component
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			tabs: Vec::new(),
			active_tab: None,
			position: TabPosition::Top,
			color: Color::from_hex("#757575").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			active_color: Color::from_hex("#1976D2").unwrap(),
			active_background: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#E0E0E0").unwrap(),
			on_change: None,
		}
	}

	/// Add a tab
	pub fn add_tab(&mut self, tab: Tab) {
		// If this is the first tab, make it active
		if self.tabs.is_empty() {
			self.active_tab = Some(tab.id.clone());
		}
		self.tabs.push(tab);
	}

	/// Remove a tab by ID
	pub fn remove_tab(&mut self, tab_id: &str) {
		if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
			self.tabs.remove(pos);

			// If we removed the active tab, select another one
			if self.active_tab.as_deref() == Some(tab_id) {
				self.active_tab = self.tabs.first().map(|t| t.id.clone());
			}
		}
	}

	/// Get tabs
	pub fn tabs(&self) -> &[Tab] {
		&self.tabs
	}

	/// Get tabs (mutable)
	pub fn tabs_mut(&mut self) -> &mut Vec<Tab> {
		&mut self.tabs
	}

	/// Get active tab ID
	pub fn active_tab(&self) -> Option<&str> {
		self.active_tab.as_deref()
	}

	/// Set active tab by ID
	pub fn set_active_tab(&mut self, tab_id: impl Into<String>) {
		let tab_id = tab_id.into();
		if self.tabs.iter().any(|t| t.id == tab_id && !t.disabled) {
			self.active_tab = Some(tab_id);
		}
	}

	/// Get active tab index
	pub fn active_index(&self) -> Option<usize> {
		self.active_tab
			.as_ref()
			.and_then(|id| self.tabs.iter().position(|t| &t.id == id))
	}

	/// Set active tab by index
	pub fn set_active_index(&mut self, index: usize) {
		if let Some(tab) = self.tabs.get(index) {
			if !tab.disabled {
				self.active_tab = Some(tab.id.clone());
			}
		}
	}

	/// Get tab position
	pub fn position(&self) -> TabPosition {
		self.position
	}

	/// Set tab position
	pub fn set_position(&mut self, position: TabPosition) {
		self.position = position;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set active tab color
	pub fn set_active_color(&mut self, color: Color) {
		self.active_color = color;
	}

	/// Set active tab background
	pub fn set_active_background(&mut self, color: Color) {
		self.active_background = color;
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Color) {
		self.border_color = color;
	}

	/// Set change callback
	pub fn set_on_change(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_change = Some(std::sync::Arc::new(callback));
	}

	/// Handle change event
	pub fn handle_change(&self, event: &Event) {
		if let Some(ref callback) = self.on_change {
			callback(event);
		}
	}
}

impl Component for Tabs {
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
	fn test_tab_creation() {
		let tab = Tab::new("tab1", "Tab 1");
		assert_eq!(tab.id, "tab1");
		assert_eq!(tab.label, "Tab 1");
		assert!(!tab.closable);
		assert!(!tab.disabled);
	}

	#[test]
	fn test_tabs_creation() {
		let tabs = Tabs::new(1);
		assert_eq!(tabs.id(), 1);
		assert_eq!(tabs.tabs().len(), 0);
		assert_eq!(tabs.active_tab(), None);
	}

	#[test]
	fn test_tabs_add_tab() {
		let mut tabs = Tabs::new(1);
		tabs.add_tab(Tab::new("tab1", "Tab 1"));
		tabs.add_tab(Tab::new("tab2", "Tab 2"));

		assert_eq!(tabs.tabs().len(), 2);
		assert_eq!(tabs.active_tab(), Some("tab1")); // First tab is active by default
	}

	#[test]
	fn test_tabs_set_active() {
		let mut tabs = Tabs::new(1);
		tabs.add_tab(Tab::new("tab1", "Tab 1"));
		tabs.add_tab(Tab::new("tab2", "Tab 2"));

		tabs.set_active_tab("tab2");
		assert_eq!(tabs.active_tab(), Some("tab2"));

		tabs.set_active_index(0);
		assert_eq!(tabs.active_tab(), Some("tab1"));
	}

	#[test]
	fn test_tabs_remove_tab() {
		let mut tabs = Tabs::new(1);
		tabs.add_tab(Tab::new("tab1", "Tab 1"));
		tabs.add_tab(Tab::new("tab2", "Tab 2"));
		tabs.add_tab(Tab::new("tab3", "Tab 3"));

		tabs.set_active_tab("tab2");
		tabs.remove_tab("tab2");

		assert_eq!(tabs.tabs().len(), 2);
		assert_eq!(tabs.active_tab(), Some("tab1")); // Should fallback to first tab
	}

	#[test]
	fn test_tabs_position() {
		let mut tabs = Tabs::new(1);
		assert_eq!(tabs.position(), TabPosition::Top);

		tabs.set_position(TabPosition::Left);
		assert_eq!(tabs.position(), TabPosition::Left);
	}
}
