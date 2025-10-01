//! Theme system for Engage UX
//!
//! Provides JSON-based theme configuration and management.

use engage_ux_core::color::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme definition with colors, fonts, and styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
	pub name: String,
	pub colors: ColorPalette,
	pub typography: Typography,
	pub spacing: Spacing,
	pub borders: BorderStyle,
	pub shadows: ShadowStyle,
}

/// Color palette for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
	pub primary: Color,
	pub secondary: Color,
	pub background: Color,
	pub surface: Color,
	pub error: Color,
	pub warning: Color,
	pub success: Color,
	pub info: Color,
	pub text_primary: Color,
	pub text_secondary: Color,
	pub text_disabled: Color,
	#[serde(default)]
	pub custom: HashMap<String, Color>,
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
	pub font_family: String,
	pub font_size_base: f32,
	pub font_size_small: f32,
	pub font_size_large: f32,
	pub line_height: f32,
}

/// Spacing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
	pub unit: f32,
	pub small: f32,
	pub medium: f32,
	pub large: f32,
}

/// Border style settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderStyle {
	pub width: f32,
	pub radius: f32,
	pub color: Color,
}

/// Shadow style settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowStyle {
	pub enabled: bool,
	pub blur_radius: f32,
	pub offset_x: f32,
	pub offset_y: f32,
	pub color: Color,
}

impl Theme {
	/// Create default light theme
	pub fn light() -> Self {
		Self {
			name: "Light".to_string(),
			colors: ColorPalette {
				primary: Color::from_hex("#1976D2").unwrap(),
				secondary: Color::from_hex("#424242").unwrap(),
				background: Color::from_hex("#FFFFFF").unwrap(),
				surface: Color::from_hex("#F5F5F5").unwrap(),
				error: Color::from_hex("#D32F2F").unwrap(),
				warning: Color::from_hex("#FFA000").unwrap(),
				success: Color::from_hex("#388E3C").unwrap(),
				info: Color::from_hex("#0288D1").unwrap(),
				text_primary: Color::from_hex("#212121").unwrap(),
				text_secondary: Color::from_hex("#757575").unwrap(),
				text_disabled: Color::from_hex("#BDBDBD").unwrap(),
				custom: HashMap::new(),
			},
			typography: Typography {
				font_family: "system-ui, -apple-system, sans-serif".to_string(),
				font_size_base: 16.0,
				font_size_small: 14.0,
				font_size_large: 20.0,
				line_height: 1.5,
			},
			spacing: Spacing {
				unit: 8.0,
				small: 8.0,
				medium: 16.0,
				large: 24.0,
			},
			borders: BorderStyle {
				width: 1.0,
				radius: 4.0,
				color: Color::from_hex("#E0E0E0").unwrap(),
			},
			shadows: ShadowStyle {
				enabled: true,
				blur_radius: 4.0,
				offset_x: 0.0,
				offset_y: 2.0,
				color: Color::rgb(0.0, 0.0, 0.0, 0.2),
			},
		}
	}

	/// Create default dark theme
	pub fn dark() -> Self {
		Self {
			name: "Dark".to_string(),
			colors: ColorPalette {
				primary: Color::from_hex("#90CAF9").unwrap(),
				secondary: Color::from_hex("#CE93D8").unwrap(),
				background: Color::from_hex("#121212").unwrap(),
				surface: Color::from_hex("#1E1E1E").unwrap(),
				error: Color::from_hex("#EF5350").unwrap(),
				warning: Color::from_hex("#FFB74D").unwrap(),
				success: Color::from_hex("#66BB6A").unwrap(),
				info: Color::from_hex("#4FC3F7").unwrap(),
				text_primary: Color::from_hex("#FFFFFF").unwrap(),
				text_secondary: Color::from_hex("#B0B0B0").unwrap(),
				text_disabled: Color::from_hex("#6E6E6E").unwrap(),
				custom: HashMap::new(),
			},
			typography: Typography {
				font_family: "system-ui, -apple-system, sans-serif".to_string(),
				font_size_base: 16.0,
				font_size_small: 14.0,
				font_size_large: 20.0,
				line_height: 1.5,
			},
			spacing: Spacing {
				unit: 8.0,
				small: 8.0,
				medium: 16.0,
				large: 24.0,
			},
			borders: BorderStyle {
				width: 1.0,
				radius: 4.0,
				color: Color::from_hex("#424242").unwrap(),
			},
			shadows: ShadowStyle {
				enabled: true,
				blur_radius: 4.0,
				offset_x: 0.0,
				offset_y: 2.0,
				color: Color::rgb(0.0, 0.0, 0.0, 0.4),
			},
		}
	}

	/// Load theme from JSON string
	pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
		serde_json::from_str(json)
	}

	/// Save theme to JSON string
	pub fn to_json(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string_pretty(self)
	}
}

impl Default for Theme {
	fn default() -> Self {
		Self::light()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_light_theme() {
		let theme = Theme::light();
		assert_eq!(theme.name, "Light");
		assert_eq!(theme.typography.font_size_base, 16.0);
	}

	#[test]
	fn test_dark_theme() {
		let theme = Theme::dark();
		assert_eq!(theme.name, "Dark");
		assert_eq!(theme.typography.font_size_base, 16.0);
	}

	#[test]
	fn test_theme_serialization() {
		let theme = Theme::light();
		let json = theme.to_json().unwrap();
		assert!(json.contains("Light"));
		
		let deserialized = Theme::from_json(&json).unwrap();
		assert_eq!(deserialized.name, theme.name);
	}

	#[test]
	fn test_spacing() {
		let theme = Theme::default();
		assert_eq!(theme.spacing.unit, 8.0);
		assert_eq!(theme.spacing.small, 8.0);
		assert_eq!(theme.spacing.medium, 16.0);
	}
}
