//! Integration tests for theme system

use engage_ux_themes::Theme;

#[test]
fn test_theme_user_friendly_formats() {
	// Test that all color formats work together in a theme
	let theme_json = r##"{
		"name": "Integration Test Theme",
		"colors": {
			"primary": {"hex": "#1976D2"},
			"secondary": {"rgb": [66, 66, 66]},
			"background": {"hsl": [0, 0, 1.0]},
			"surface": {"hex": "#F5F5F5"},
			"error": {"rgb": [211, 47, 47]},
			"warning": {"hsl": [38, 1.0, 0.5]},
			"success": {"hex": "#388E3C"},
			"info": {"rgb": [2, 136, 209]},
			"text_primary": {"hsl": [0, 0, 0.13]},
			"text_secondary": {"hex": "#757575"},
			"text_disabled": {"rgb": [189, 189, 189, 0.5]},
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

	let theme = Theme::from_json(theme_json).expect("Failed to parse theme");
	assert_eq!(theme.name, "Integration Test Theme");

	// Test serialization roundtrip
	let serialized = theme.to_json().expect("Failed to serialize theme");
	let deserialized = Theme::from_json(&serialized).expect("Failed to deserialize theme");
	assert_eq!(deserialized.name, theme.name);
}

#[test]
fn test_theme_defaults() {
	let light = Theme::light();
	let dark = Theme::dark();

	// Default themes are now LCARS themes
	assert_eq!(light.name, "LCARS Light");
	assert_eq!(dark.name, "LCARS Dark");

	// Verify both themes have all required properties
	assert_eq!(light.typography.font_size_base, 16.0);
	assert_eq!(dark.typography.font_size_base, 16.0);

	assert_eq!(light.spacing.unit, 8.0);
	assert_eq!(dark.spacing.unit, 8.0);
}
