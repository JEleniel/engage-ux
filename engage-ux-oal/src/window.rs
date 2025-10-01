//! Window management abstraction
//!
//! Provides cross-platform window creation and management.

use engage_ux_core::component::{ComponentId, Rect};

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
	pub title: String,
	pub width: u32,
	pub height: u32,
	pub resizable: bool,
	pub decorated: bool,
	pub transparent: bool,
}

impl Default for WindowConfig {
	fn default() -> Self {
		Self {
			title: "Engage UX Window".to_string(),
			width: 800,
			height: 600,
			resizable: true,
			decorated: true,
			transparent: false,
		}
	}
}

/// Abstract window interface
pub trait Window: Send + Sync {
	/// Get window's component ID
	fn id(&self) -> ComponentId;

	/// Get window title
	fn title(&self) -> &str;

	/// Set window title
	fn set_title(&mut self, title: &str);

	/// Get window size
	fn size(&self) -> (u32, u32);

	/// Set window size
	fn set_size(&mut self, width: u32, height: u32);

	/// Get window position
	fn position(&self) -> (i32, i32);

	/// Set window position
	fn set_position(&mut self, x: i32, y: i32);

	/// Show the window
	fn show(&mut self);

	/// Hide the window
	fn hide(&mut self);

	/// Close the window
	fn close(&mut self);

	/// Check if window is visible
	fn is_visible(&self) -> bool;

	/// Get client area bounds
	fn client_bounds(&self) -> Rect;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_window_config_default() {
		let config = WindowConfig::default();
		assert_eq!(config.title, "Engage UX Window");
		assert_eq!(config.width, 800);
		assert_eq!(config.height, 600);
		assert!(config.resizable);
		assert!(config.decorated);
		assert!(!config.transparent);
	}

	#[test]
	fn test_window_config_custom() {
		let config = WindowConfig {
			title: "Custom Window".to_string(),
			width: 1024,
			height: 768,
			resizable: false,
			decorated: false,
			transparent: true,
		};
		assert_eq!(config.title, "Custom Window");
		assert_eq!(config.width, 1024);
		assert!(!config.resizable);
	}
}
