//! Window control components (close, minimize, maximize)

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Window control button type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowControlType {
	Close,
	Minimize,
	Maximize,
	Restore,
}

/// Window control button
#[derive(Clone, Serialize, Deserialize)]
pub struct WindowControlButton {
	properties: ComponentProperties,
	control_type: WindowControlType,
	color: Color,
	hover_color: Color,
	background_color: Color,
	hover_background: Color,
	size: f32,
	#[serde(skip)]
	on_click: Option<EventCallback>,
}

impl WindowControlButton {
	/// Create a new window control button
	pub fn new(id: ComponentId, control_type: WindowControlType) -> Self {
		let (color, hover_color, hover_background) = match control_type {
			WindowControlType::Close => (
				Color::from_hex("#000000").unwrap(),
				Color::from_hex("#FFFFFF").unwrap(),
				Color::from_hex("#E81123").unwrap(),
			),
			WindowControlType::Minimize
			| WindowControlType::Maximize
			| WindowControlType::Restore => (
				Color::from_hex("#000000").unwrap(),
				Color::from_hex("#000000").unwrap(),
				Color::from_hex("#E0E0E0").unwrap(),
			),
		};

		Self {
			properties: ComponentProperties::new(id),
			control_type,
			color,
			hover_color,
			background_color: Color::from_hex("#00000000").unwrap(), // Transparent
			hover_background,
			size: 46.0,
			on_click: None,
		}
	}

	/// Get control type
	pub fn control_type(&self) -> WindowControlType {
		self.control_type
	}

	/// Set icon color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get icon color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set hover color
	pub fn set_hover_color(&mut self, color: Color) {
		self.hover_color = color;
	}

	/// Get hover color
	pub fn hover_color(&self) -> &Color {
		&self.hover_color
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Get background color
	pub fn background_color(&self) -> &Color {
		&self.background_color
	}

	/// Set hover background color
	pub fn set_hover_background(&mut self, color: Color) {
		self.hover_background = color;
	}

	/// Get hover background color
	pub fn hover_background(&self) -> &Color {
		&self.hover_background
	}

	/// Set size
	pub fn set_size(&mut self, size: f32) {
		self.size = size.max(20.0);
	}

	/// Get size
	pub fn size(&self) -> f32 {
		self.size
	}

	/// Set click event handler
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

impl Component for WindowControlButton {
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

/// Window controls container with all standard buttons
#[derive(Clone, Serialize, Deserialize)]
pub struct WindowControls {
	properties: ComponentProperties,
	minimize_button: Option<WindowControlButton>,
	maximize_button: Option<WindowControlButton>,
	restore_button: Option<WindowControlButton>,
	close_button: WindowControlButton,
	is_maximized: bool,
	spacing: f32,
}

impl WindowControls {
	/// Create new window controls
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			minimize_button: Some(WindowControlButton::new(
				id + 1,
				WindowControlType::Minimize,
			)),
			maximize_button: Some(WindowControlButton::new(
				id + 2,
				WindowControlType::Maximize,
			)),
			restore_button: Some(WindowControlButton::new(id + 3, WindowControlType::Restore)),
			close_button: WindowControlButton::new(id + 4, WindowControlType::Close),
			is_maximized: false,
			spacing: 0.0,
		}
	}

	/// Get minimize button
	pub fn minimize_button(&self) -> Option<&WindowControlButton> {
		self.minimize_button.as_ref()
	}

	/// Get minimize button (mutable)
	pub fn minimize_button_mut(&mut self) -> Option<&mut WindowControlButton> {
		self.minimize_button.as_mut()
	}

	/// Enable/disable minimize button
	pub fn set_minimize_enabled(&mut self, enabled: bool) {
		if enabled && self.minimize_button.is_none() {
			self.minimize_button = Some(WindowControlButton::new(
				self.id() + 1,
				WindowControlType::Minimize,
			));
		} else if !enabled {
			self.minimize_button = None;
		}
	}

	/// Get maximize/restore button (shows appropriate one based on state)
	pub fn maximize_restore_button(&self) -> Option<&WindowControlButton> {
		if self.is_maximized {
			self.restore_button.as_ref()
		} else {
			self.maximize_button.as_ref()
		}
	}

	/// Get maximize/restore button (mutable)
	pub fn maximize_restore_button_mut(&mut self) -> Option<&mut WindowControlButton> {
		if self.is_maximized {
			self.restore_button.as_mut()
		} else {
			self.maximize_button.as_mut()
		}
	}

	/// Enable/disable maximize/restore buttons
	pub fn set_maximize_enabled(&mut self, enabled: bool) {
		if enabled {
			if self.maximize_button.is_none() {
				self.maximize_button = Some(WindowControlButton::new(
					self.id() + 2,
					WindowControlType::Maximize,
				));
			}
			if self.restore_button.is_none() {
				self.restore_button = Some(WindowControlButton::new(
					self.id() + 3,
					WindowControlType::Restore,
				));
			}
		} else {
			self.maximize_button = None;
			self.restore_button = None;
		}
	}

	/// Get close button
	pub fn close_button(&self) -> &WindowControlButton {
		&self.close_button
	}

	/// Get close button (mutable)
	pub fn close_button_mut(&mut self) -> &mut WindowControlButton {
		&mut self.close_button
	}

	/// Check if window is maximized
	pub fn is_maximized(&self) -> bool {
		self.is_maximized
	}

	/// Set maximized state
	pub fn set_maximized(&mut self, maximized: bool) {
		self.is_maximized = maximized;
	}

	/// Toggle maximized state
	pub fn toggle_maximized(&mut self) {
		self.is_maximized = !self.is_maximized;
	}

	/// Get spacing between buttons
	pub fn spacing(&self) -> f32 {
		self.spacing
	}

	/// Set spacing between buttons
	pub fn set_spacing(&mut self, spacing: f32) {
		self.spacing = spacing.max(0.0);
	}
}

impl Component for WindowControls {
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
	fn test_window_control_button_creation() {
		let button = WindowControlButton::new(1, WindowControlType::Close);
		assert_eq!(button.id(), 1);
		assert_eq!(button.control_type(), WindowControlType::Close);
	}

	#[test]
	fn test_window_control_button_colors() {
		let mut button = WindowControlButton::new(1, WindowControlType::Close);
		let custom_color = Color::from_hex("#FF0000").unwrap();
		button.set_color(custom_color.clone());
		assert_eq!(button.color(), &custom_color);
	}

	#[test]
	fn test_window_controls_creation() {
		let controls = WindowControls::new(1);
		assert_eq!(controls.id(), 1);
		assert!(controls.minimize_button().is_some());
		assert!(controls.maximize_restore_button().is_some());
		assert!(!controls.is_maximized());
	}

	#[test]
	fn test_window_controls_maximize_state() {
		let mut controls = WindowControls::new(1);
		assert!(!controls.is_maximized());

		controls.set_maximized(true);
		assert!(controls.is_maximized());

		controls.toggle_maximized();
		assert!(!controls.is_maximized());
	}

	#[test]
	fn test_window_controls_button_enabling() {
		let mut controls = WindowControls::new(1);

		controls.set_minimize_enabled(false);
		assert!(controls.minimize_button().is_none());

		controls.set_minimize_enabled(true);
		assert!(controls.minimize_button().is_some());

		controls.set_maximize_enabled(false);
		assert!(controls.maximize_restore_button().is_none());

		controls.set_maximize_enabled(true);
		assert!(controls.maximize_restore_button().is_some());
	}

	#[test]
	fn test_window_controls_spacing() {
		let mut controls = WindowControls::new(1);
		assert_eq!(controls.spacing(), 0.0);

		controls.set_spacing(4.0);
		assert_eq!(controls.spacing(), 4.0);
	}
}
