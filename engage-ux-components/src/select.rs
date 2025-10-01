//! Select/Dropdown component for selecting from options

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Select option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
	pub label: String,
	pub value: String,
	pub disabled: bool,
}

impl SelectOption {
	pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
		Self {
			label: label.into(),
			value: value.into(),
			disabled: false,
		}
	}
}

/// Select/Dropdown component
#[derive(Clone, Serialize, Deserialize)]
pub struct Select {
	properties: ComponentProperties,
	options: Vec<SelectOption>,
	selected_index: Option<usize>,
	placeholder: String,
	searchable: bool,
	open: bool,
	color: Color,
	background_color: Color,
	border_color: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl Select {
	/// Create a new select component
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			options: Vec::new(),
			selected_index: None,
			placeholder: "Select an option...".to_string(),
			searchable: false,
			open: false,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#CCCCCC").unwrap(),
			on_change: None,
		}
	}

	/// Add an option
	pub fn add_option(&mut self, option: SelectOption) {
		self.options.push(option);
	}

	/// Set options
	pub fn set_options(&mut self, options: Vec<SelectOption>) {
		self.options = options;
	}

	/// Get options
	pub fn options(&self) -> &[SelectOption] {
		&self.options
	}

	/// Select an option by index
	pub fn select(&mut self, index: usize) {
		if index < self.options.len() && !self.options[index].disabled {
			self.selected_index = Some(index);
		}
	}

	/// Get selected index
	pub fn selected_index(&self) -> Option<usize> {
		self.selected_index
	}

	/// Get selected value
	pub fn selected_value(&self) -> Option<&str> {
		self.selected_index
			.and_then(|i| self.options.get(i))
			.map(|opt| opt.value.as_str())
	}

	/// Get selected label
	pub fn selected_label(&self) -> Option<&str> {
		self.selected_index
			.and_then(|i| self.options.get(i))
			.map(|opt| opt.label.as_str())
	}

	/// Set placeholder text
	pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
		self.placeholder = placeholder.into();
	}

	/// Get placeholder text
	pub fn placeholder(&self) -> &str {
		&self.placeholder
	}

	/// Enable/disable searchable dropdown
	pub fn set_searchable(&mut self, searchable: bool) {
		self.searchable = searchable;
	}

	/// Check if searchable
	pub fn is_searchable(&self) -> bool {
		self.searchable
	}

	/// Open the dropdown
	pub fn open(&mut self) {
		self.open = true;
	}

	/// Close the dropdown
	pub fn close(&mut self) {
		self.open = false;
	}

	/// Toggle dropdown state
	pub fn toggle(&mut self) {
		self.open = !self.open;
	}

	/// Check if dropdown is open
	pub fn is_open(&self) -> bool {
		self.open
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

	/// Set change event handler
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

impl Component for Select {
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
	fn test_select_creation() {
		let select = Select::new(1);
		assert_eq!(select.id(), 1);
		assert_eq!(select.options().len(), 0);
		assert_eq!(select.selected_index(), None);
	}

	#[test]
	fn test_select_add_options() {
		let mut select = Select::new(1);
		select.add_option(SelectOption::new("Option 1", "1"));
		select.add_option(SelectOption::new("Option 2", "2"));
		assert_eq!(select.options().len(), 2);
	}

	#[test]
	fn test_select_selection() {
		let mut select = Select::new(1);
		select.add_option(SelectOption::new("Option 1", "1"));
		select.add_option(SelectOption::new("Option 2", "2"));
		
		select.select(0);
		assert_eq!(select.selected_index(), Some(0));
		assert_eq!(select.selected_value(), Some("1"));
		assert_eq!(select.selected_label(), Some("Option 1"));
	}

	#[test]
	fn test_select_disabled_option() {
		let mut select = Select::new(1);
		let mut option = SelectOption::new("Disabled", "disabled");
		option.disabled = true;
		select.add_option(option);
		
		select.select(0);
		assert_eq!(select.selected_index(), None); // Should not select disabled option
	}

	#[test]
	fn test_select_toggle() {
		let mut select = Select::new(1);
		assert!(!select.is_open());
		
		select.toggle();
		assert!(select.is_open());
		
		select.toggle();
		assert!(!select.is_open());
	}
}
