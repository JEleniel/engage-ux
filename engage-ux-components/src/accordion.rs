//! Accordion component for collapsible content panels

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Accordion panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccordionPanel {
	pub id: String,
	pub title: String,
	pub content: String,
	pub expanded: bool,
	pub disabled: bool,
	pub icon: Option<String>,
}

impl AccordionPanel {
	/// Create a new accordion panel
	pub fn new(
		id: impl Into<String>,
		title: impl Into<String>,
		content: impl Into<String>,
	) -> Self {
		Self {
			id: id.into(),
			title: title.into(),
			content: content.into(),
			expanded: false,
			disabled: false,
			icon: None,
		}
	}
}

/// Accordion component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accordion {
	properties: ComponentProperties,
	panels: Vec<AccordionPanel>,
	allow_multiple: bool,
	color: Color,
	background_color: Color,
	border_color: Color,
	active_color: Color,
}

impl Accordion {
	/// Create a new accordion
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			panels: Vec::new(),
			allow_multiple: false,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			border_color: Color::from_hex("#E0E0E0").unwrap(),
			active_color: Color::from_hex("#F5F5F5").unwrap(),
		}
	}

	/// Add a panel
	pub fn add_panel(&mut self, panel: AccordionPanel) {
		self.panels.push(panel);
	}

	/// Get panels
	pub fn panels(&self) -> &[AccordionPanel] {
		&self.panels
	}

	/// Get panels (mutable)
	pub fn panels_mut(&mut self) -> &mut Vec<AccordionPanel> {
		&mut self.panels
	}

	/// Expand panel by ID
	pub fn expand(&mut self, panel_id: &str) {
		if !self.allow_multiple {
			// Collapse all other panels
			for panel in &mut self.panels {
				panel.expanded = false;
			}
		}

		// Expand requested panel
		if let Some(panel) = self.panels.iter_mut().find(|p| p.id == panel_id)
			&& !panel.disabled
		{
			panel.expanded = true;
		}
	}

	/// Collapse panel by ID
	pub fn collapse(&mut self, panel_id: &str) {
		if let Some(panel) = self.panels.iter_mut().find(|p| p.id == panel_id) {
			panel.expanded = false;
		}
	}

	/// Toggle panel by ID
	pub fn toggle(&mut self, panel_id: &str) {
		if let Some(panel) = self.panels.iter().find(|p| p.id == panel_id) {
			if panel.expanded {
				self.collapse(panel_id);
			} else {
				self.expand(panel_id);
			}
		}
	}

	/// Check if multiple panels can be expanded
	pub fn allows_multiple(&self) -> bool {
		self.allow_multiple
	}

	/// Set allow multiple
	pub fn set_allow_multiple(&mut self, allow_multiple: bool) {
		self.allow_multiple = allow_multiple;
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

	/// Set active panel color
	pub fn set_active_color(&mut self, color: Color) {
		self.active_color = color;
	}
}

impl Component for Accordion {
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
	fn test_accordion_creation() {
		let accordion = Accordion::new(1);
		assert_eq!(accordion.id(), 1);
		assert_eq!(accordion.panels().len(), 0);
		assert!(!accordion.allows_multiple());
	}

	#[test]
	fn test_accordion_add_panel() {
		let mut accordion = Accordion::new(1);
		accordion.add_panel(AccordionPanel::new("1", "Panel 1", "Content 1"));
		accordion.add_panel(AccordionPanel::new("2", "Panel 2", "Content 2"));
		assert_eq!(accordion.panels().len(), 2);
	}

	#[test]
	fn test_accordion_expand_single() {
		let mut accordion = Accordion::new(1);
		accordion.add_panel(AccordionPanel::new("1", "Panel 1", "Content 1"));
		accordion.add_panel(AccordionPanel::new("2", "Panel 2", "Content 2"));

		accordion.expand("1");
		assert!(accordion.panels()[0].expanded);
		assert!(!accordion.panels()[1].expanded);

		accordion.expand("2");
		assert!(!accordion.panels()[0].expanded); // First should be collapsed
		assert!(accordion.panels()[1].expanded);
	}

	#[test]
	fn test_accordion_expand_multiple() {
		let mut accordion = Accordion::new(1);
		accordion.set_allow_multiple(true);
		accordion.add_panel(AccordionPanel::new("1", "Panel 1", "Content 1"));
		accordion.add_panel(AccordionPanel::new("2", "Panel 2", "Content 2"));

		accordion.expand("1");
		accordion.expand("2");
		assert!(accordion.panels()[0].expanded);
		assert!(accordion.panels()[1].expanded);
	}

	#[test]
	fn test_accordion_toggle() {
		let mut accordion = Accordion::new(1);
		accordion.add_panel(AccordionPanel::new("1", "Panel 1", "Content 1"));

		accordion.toggle("1");
		assert!(accordion.panels()[0].expanded);

		accordion.toggle("1");
		assert!(!accordion.panels()[0].expanded);
	}
}
