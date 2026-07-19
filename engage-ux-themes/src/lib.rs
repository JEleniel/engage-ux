//! Theme system for Engage UX
//!
//! Provides JSON-based theme configuration and management.

use engage_ux_core::Color;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
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
	/// Component-specific layouts mapped by component ID or name
	#[serde(default)]
	pub component_layouts: HashMap<String, JsonValue>,
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
	/// Create default LCARS light theme (inspired by Star Trek)
	///
	/// This is the default theme for Engage UX, named after Captain Picard's
	/// famous "Engage" command. Features vibrant LCARS colors with curved borders.
	pub fn light() -> engage_ux_core::EngageResult<Self> {
		Self::lcars_light()
	}

	/// Create default LCARS dark theme (inspired by Star Trek)
	pub fn dark() -> engage_ux_core::EngageResult<Self> {
		Self::lcars_dark()
	}

	/// Create LCARS light theme - slick futuristic interface inspired by Star Trek Voyager
	pub fn lcars_light() -> engage_ux_core::EngageResult<Self> {
		let mut custom_colors = HashMap::new();
		custom_colors.insert("voyager_indigo".to_string(), Color::from_hex("#5566CC")?);
		custom_colors.insert("voyager_blue".to_string(), Color::from_hex("#6699FF")?);
		custom_colors.insert("voyager_cyan".to_string(), Color::from_hex("#66CCFF")?);
		custom_colors.insert("voyager_teal".to_string(), Color::from_hex("#66CCCC")?);
		custom_colors.insert("voyager_purple".to_string(), Color::from_hex("#9966CC")?);
		custom_colors.insert("voyager_lavender".to_string(), Color::from_hex("#9999FF")?);
		custom_colors.insert(
			"voyager_periwinkle".to_string(),
			Color::from_hex("#AAAAFF")?,
		);
		custom_colors.insert("voyager_steel".to_string(), Color::from_hex("#8899CC")?);
		custom_colors.insert("panel_dark".to_string(), Color::from_hex("#000000")?);
		custom_colors.insert("panel_medium".to_string(), Color::from_hex("#1A1A1A")?);
		custom_colors.insert("accent_glow".to_string(), Color::from_hex("#6699FFCC")?);

		Ok(Self {
			name: "LCARS Light".to_string(),
			colors: ColorPalette {
				primary: Color::from_hex("#6699FF")?,
				secondary: Color::from_hex("#9966CC")?,
				background: Color::from_hex("#000000")?,
				surface: Color::from_hex("#1A1A1A")?,
				error: Color::from_hex("#CC6666")?,
				warning: Color::from_hex("#FFAA66")?,
				success: Color::from_hex("#66CCAA")?,
				info: Color::from_hex("#66CCFF")?,
				text_primary: Color::from_hex("#AAAAFF")?,
				text_secondary: Color::from_hex("#9999FF")?,
				text_disabled: Color::from_hex("#666666")?,
				custom: custom_colors,
			},
			typography: Typography {
				font_family: "Helvetica Neue, Arial, sans-serif".to_string(),
				font_size_base: 16.0,
				font_size_small: 13.0,
				font_size_large: 22.0,
				line_height: 1.4,
			},
			spacing: Spacing {
				unit: 8.0,
				small: 6.0,
				medium: 12.0,
				large: 20.0,
			},
			borders: BorderStyle {
				width: 3.0,
				radius: 20.0,
				color: Color::from_hex("#6699FF")?,
			},
			shadows: ShadowStyle {
				enabled: true,
				blur_radius: 8.0,
				offset_x: 0.0,
				offset_y: 4.0,
				color: Color::from_hex("#6699FF33")?,
			},
			component_layouts: HashMap::new(),
		})
	}

	/// Create LCARS dark theme - slick futuristic interface inspired by Star Trek Voyager
	pub fn lcars_dark() -> engage_ux_core::EngageResult<Self> {
		let mut custom_colors = HashMap::new();
		custom_colors.insert("voyager_indigo".to_string(), Color::from_hex("#5566CC")?);
		custom_colors.insert("voyager_blue".to_string(), Color::from_hex("#6699FF")?);
		custom_colors.insert("voyager_cyan".to_string(), Color::from_hex("#66CCFF")?);
		custom_colors.insert("voyager_teal".to_string(), Color::from_hex("#66CCCC")?);
		custom_colors.insert("voyager_purple".to_string(), Color::from_hex("#9966CC")?);
		custom_colors.insert("voyager_lavender".to_string(), Color::from_hex("#9999FF")?);
		custom_colors.insert(
			"voyager_periwinkle".to_string(),
			Color::from_hex("#AAAAFF")?,
		);
		custom_colors.insert("voyager_steel".to_string(), Color::from_hex("#8899CC")?);
		custom_colors.insert("panel_dark".to_string(), Color::from_hex("#000000")?);
		custom_colors.insert("panel_medium".to_string(), Color::from_hex("#0D0D0D")?);
		custom_colors.insert("panel_light".to_string(), Color::from_hex("#1A1A1A")?);
		custom_colors.insert("accent_glow".to_string(), Color::from_hex("#6699FFAA")?);
		custom_colors.insert("button_inactive".to_string(), Color::from_hex("#333333")?);
		custom_colors.insert("separator".to_string(), Color::from_hex("#6699FF66")?);

		Ok(Self {
			name: "LCARS Dark".to_string(),
			colors: ColorPalette {
				primary: Color::from_hex("#6699FF")?,
				secondary: Color::from_hex("#9999FF")?,
				background: Color::from_hex("#000000")?,
				surface: Color::from_hex("#0D0D0D")?,
				error: Color::from_hex("#FF6666")?,
				warning: Color::from_hex("#FFAA66")?,
				success: Color::from_hex("#66CCAA")?,
				info: Color::from_hex("#66CCFF")?,
				text_primary: Color::from_hex("#AAAAFF")?,
				text_secondary: Color::from_hex("#9999FF")?,
				text_disabled: Color::from_hex("#555555")?,
				custom: custom_colors,
			},
			typography: Typography {
				font_family: "Helvetica Neue, Arial, sans-serif".to_string(),
				font_size_base: 16.0,
				font_size_small: 13.0,
				font_size_large: 22.0,
				line_height: 1.4,
			},
			spacing: Spacing {
				unit: 8.0,
				small: 6.0,
				medium: 12.0,
				large: 20.0,
			},
			borders: BorderStyle {
				width: 3.0,
				radius: 20.0,
				color: Color::from_hex("#6699FF")?,
			},
			shadows: ShadowStyle {
				enabled: true,
				blur_radius: 10.0,
				offset_x: 0.0,
				offset_y: 4.0,
				color: Color::from_hex("#6699FF44")?,
			},
			component_layouts: HashMap::new(),
		})
	}

	/// Create classic light theme (original design)
	pub fn classic_light() -> engage_ux_core::EngageResult<Self> {
		Ok(Self {
			name: "Classic Light".to_string(),
			colors: ColorPalette {
				primary: Color::from_hex("#1976D2")?,
				secondary: Color::from_hex("#424242")?,
				background: Color::from_hex("#FFFFFF")?,
				surface: Color::from_hex("#F5F5F5")?,
				error: Color::from_hex("#D32F2F")?,
				warning: Color::from_hex("#FFA000")?,
				success: Color::from_hex("#388E3C")?,
				info: Color::from_hex("#0288D1")?,
				text_primary: Color::from_hex("#212121")?,
				text_secondary: Color::from_hex("#757575")?,
				text_disabled: Color::from_hex("#BDBDBD")?,
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
				color: Color::from_hex("#E0E0E0")?,
			},
			shadows: ShadowStyle {
				enabled: true,
				blur_radius: 4.0,
				offset_x: 0.0,
				offset_y: 2.0,
				color: Color::new(0, 0, 0, 50),
			},
			component_layouts: HashMap::new(),
		})
	}

	/// Create classic dark theme (original design)
	pub fn classic_dark() -> engage_ux_core::EngageResult<Self> {
		Ok(Self {
			name: "Classic Dark".to_string(),
			colors: ColorPalette {
				primary: Color::from_hex("#90CAF9")?,
				secondary: Color::from_hex("#CE93D8")?,
				background: Color::from_hex("#121212")?,
				surface: Color::from_hex("#1E1E1E")?,
				error: Color::from_hex("#EF5350")?,
				warning: Color::from_hex("#FFB74D")?,
				success: Color::from_hex("#66BB6A")?,
				info: Color::from_hex("#4FC3F7")?,
				text_primary: Color::from_hex("#FFFFFF")?,
				text_secondary: Color::from_hex("#B0B0B0")?,
				text_disabled: Color::from_hex("#6E6E6E")?,
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
				color: Color::from_hex("#424242")?,
			},
			shadows: ShadowStyle {
				enabled: true,
				blur_radius: 4.0,
				offset_x: 0.0,
				offset_y: 2.0,
				color: Color::new(0, 0, 0, 100),
			},
			component_layouts: HashMap::new(),
		})
	}

	/// Load theme from JSON string
	pub fn from_json(json: &str) -> engage_ux_core::EngageResult<Self> {
		let t: Theme = serde_json::from_str(json)?;
		Ok(t)
	}

	/// Save theme to JSON string
	pub fn to_json(&self) -> engage_ux_core::EngageResult<String> {
		let s = serde_json::to_string_pretty(self)?;
		Ok(s)
	}
}

impl Default for Theme {
	fn default() -> Self {
		// Prefer constructing the canonical light theme; if parsing fails,
		// fall back to a minimal default theme to avoid panics in Default.
		if let Ok(t) = Self::light() {
			t
		} else {
			Theme {
				name: "Default Theme".to_string(),
				colors: ColorPalette {
					primary: Color::default(),
					secondary: Color::default(),
					background: Color::default(),
					surface: Color::default(),
					error: Color::default(),
					warning: Color::default(),
					success: Color::default(),
					info: Color::default(),
					text_primary: Color::default(),
					text_secondary: Color::default(),
					text_disabled: Color::default(),
					custom: HashMap::new(),
				},
				typography: Typography {
					font_family: "sans-serif".to_string(),
					font_size_base: 16.0,
					font_size_small: 12.0,
					font_size_large: 20.0,
					line_height: 1.4,
				},
				spacing: Spacing {
					unit: 8.0,
					small: 6.0,
					medium: 12.0,
					large: 20.0,
				},
				borders: BorderStyle {
					width: 1.0,
					radius: 4.0,
					color: Color::default(),
				},
				shadows: ShadowStyle {
					enabled: false,
					blur_radius: 0.0,
					offset_x: 0.0,
					offset_y: 0.0,
					color: Color::default(),
				},
				component_layouts: HashMap::new(),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_light_theme() {
		let theme = Theme::light().expect("build light theme");
		assert_eq!(theme.name, "LCARS Light");
		assert_eq!(theme.typography.font_size_base, 16.0);
		// Verify Voyager-style indigo/blue color palette
		assert_eq!(theme.colors.primary, Color::from_hex("#6699FF").unwrap());
	}

	#[test]
	fn test_dark_theme() {
		let theme = Theme::dark().expect("build dark theme");
		assert_eq!(theme.name, "LCARS Dark");
		assert_eq!(theme.typography.font_size_base, 16.0);
		// Verify Voyager-style indigo/blue color palette
		assert_eq!(theme.colors.primary, Color::from_hex("#6699FF").unwrap());
	}

	#[test]
	fn test_theme_serialization() {
		let theme = Theme::light().expect("build light theme");
		let json = theme.to_json().expect("serialize theme");
		assert!(json.contains("LCARS Light"));

		let deserialized = Theme::from_json(&json).expect("deserialize theme");
		assert_eq!(deserialized.name, theme.name);
	}

	#[test]
	fn test_spacing() {
		let theme = Theme::default();
		assert_eq!(theme.spacing.unit, 8.0);
		assert_eq!(theme.spacing.small, 6.0);
		assert_eq!(theme.spacing.medium, 12.0);
	}

	#[test]
	fn test_classic_themes() {
		let light = Theme::classic_light().expect("classic light");
		assert_eq!(light.name, "Classic Light");
		assert_eq!(light.colors.primary, Color::from_hex("#1976D2").unwrap());

		let dark = Theme::classic_dark().expect("classic dark");
		assert_eq!(dark.name, "Classic Dark");
		assert_eq!(dark.colors.primary, Color::from_hex("#90CAF9").unwrap());
	}

	#[test]
	fn test_load_user_friendly_hex_format() {
		let json = r##"{
			"name": "Test Theme",
			"colors": {
				"primary": {"hex": "#1976D2"},
				"secondary": {"hex": "#424242"},
				"background": {"hex": "#FFFFFF"},
				"surface": {"hex": "#F5F5F5"},
				"error": {"hex": "#D32F2F"},
				"warning": {"hex": "#FFA000"},
				"success": {"hex": "#388E3C"},
				"info": {"hex": "#0288D1"},
				"text_primary": {"hex": "#212121"},
				"text_secondary": {"hex": "#757575"},
				"text_disabled": {"hex": "#BDBDBD"},
				"custom": {}
			},
			"typography": {
				"font_family": "system-ui",
				"font_size_base": 16.0,
				"font_size_small": 14.0,
				"font_size_large": 20.0,
				"line_height": 1.5
			},
			"spacing": {
				"unit": 8.0,
				"small": 8.0,
				"medium": 16.0,
				"large": 24.0
			},
			"borders": {
				"width": 1.0,
				"radius": 4.0,
				"color": {"hex": "#E0E0E0"}
			},
			"shadows": {
				"enabled": true,
				"blur_radius": 4.0,
				"offset_x": 0.0,
				"offset_y": 2.0,
				"color": {"rgb": [0, 0, 0, 0.2]}
			}
		}"##;

		let theme = Theme::from_json(json).unwrap();
		assert_eq!(theme.name, "Test Theme");
	}

	#[test]
	fn test_load_user_friendly_rgb_format() {
		let json = r#"{
			"name": "RGB Test",
			"colors": {
				"primary": {"rgb": [25, 118, 210]},
				"secondary": {"rgb": [66, 66, 66]},
				"background": {"rgb": [255, 255, 255]},
				"surface": {"rgb": [245, 245, 245]},
				"error": {"rgb": [211, 47, 47]},
				"warning": {"rgb": [255, 160, 0]},
				"success": {"rgb": [56, 142, 60]},
				"info": {"rgb": [2, 136, 209]},
				"text_primary": {"rgb": [33, 33, 33]},
				"text_secondary": {"rgb": [117, 117, 117]},
				"text_disabled": {"rgb": [189, 189, 189]},
				"custom": {}
			},
			"typography": {
				"font_family": "system-ui",
				"font_size_base": 16.0,
				"font_size_small": 14.0,
				"font_size_large": 20.0,
				"line_height": 1.5
			},
			"spacing": {
				"unit": 8.0,
				"small": 8.0,
				"medium": 16.0,
				"large": 24.0
			},
			"borders": {
				"width": 1.0,
				"radius": 4.0,
				"color": {"rgb": [224, 224, 224]}
			},
			"shadows": {
				"enabled": true,
				"blur_radius": 4.0,
				"offset_x": 0.0,
				"offset_y": 2.0,
				"color": {"rgb": [0, 0, 0, 0.2]}
			}
		}"#;

		let theme = Theme::from_json(json).unwrap();
		assert_eq!(theme.name, "RGB Test");
	}

	#[test]
	fn test_load_user_friendly_hsl_format() {
		let json = r#"{
			"name": "HSL Test",
			"colors": {
				"primary": {"hsl": [207, 0.79, 0.46]},
				"secondary": {"hsl": [0, 0, 0.26]},
				"background": {"hsl": [0, 0, 1.0]},
				"surface": {"hsl": [0, 0, 0.96]},
				"error": {"hsl": [0, 0.65, 0.51]},
				"warning": {"hsl": [38, 1.0, 0.5]},
				"success": {"hsl": [122, 0.43, 0.39]},
				"info": {"hsl": [199, 0.98, 0.41]},
				"text_primary": {"hsl": [0, 0, 0.13]},
				"text_secondary": {"hsl": [0, 0, 0.46]},
				"text_disabled": {"hsl": [0, 0, 0.74]},
				"custom": {}
			},
			"typography": {
				"font_family": "system-ui",
				"font_size_base": 16.0,
				"font_size_small": 14.0,
				"font_size_large": 20.0,
				"line_height": 1.5
			},
			"spacing": {
				"unit": 8.0,
				"small": 8.0,
				"medium": 16.0,
				"large": 24.0
			},
			"borders": {
				"width": 1.0,
				"radius": 4.0,
				"color": {"hsl": [0, 0, 0.88]}
			},
			"shadows": {
				"enabled": true,
				"blur_radius": 4.0,
				"offset_x": 0.0,
				"offset_y": 2.0,
				"color": {"hsl": [0, 0, 0, 0.2]}
			}
		}"#;

		let theme = Theme::from_json(json).unwrap();
		assert_eq!(theme.name, "HSL Test");
	}

	#[test]
	fn test_load_mixed_format() {
		let json = r##"{
			"name": "Mixed Format Test",
			"colors": {
				"primary": {"hex": "#1976D2"},
				"secondary": {"rgb": [66, 66, 66]},
				"background": {"hsl": [0, 0, 1.0]},
				"surface": {"space": "RGB", "components": [0.96, 0.96, 0.96, 1.0]},
				"error": {"hex": "#D32F2F"},
				"warning": {"rgb": [255, 160, 0, 0.9]},
				"success": {"hsl": [122, 0.43, 0.39, 0.95]},
				"info": {"hex": "#0288D1FF"},
				"text_primary": {"rgb": [33, 33, 33]},
				"text_secondary": {"hsl": [0, 0, 0.46]},
				"text_disabled": {"hex": "#BDBDBD"},
				"custom": {}
			},
			"typography": {
				"font_family": "system-ui",
				"font_size_base": 16.0,
				"font_size_small": 14.0,
				"font_size_large": 20.0,
				"line_height": 1.5
			},
			"spacing": {
				"unit": 8.0,
				"small": 8.0,
				"medium": 16.0,
				"large": 24.0
			},
			"borders": {
				"width": 1.0,
				"radius": 4.0,
				"color": {"hex": "#E0E0E0"}
			},
			"shadows": {
				"enabled": true,
				"blur_radius": 4.0,
				"offset_x": 0.0,
				"offset_y": 2.0,
				"color": {"rgb": [0, 0, 0, 0.2]}
			}
		}"##;

		let theme = Theme::from_json(json).unwrap();
		assert_eq!(theme.name, "Mixed Format Test");
	}
}
