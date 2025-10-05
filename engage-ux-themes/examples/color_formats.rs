//! Example demonstrating the various color formats supported by Engage UX
//!
//! This example shows how to use hex, RGB, HSL, and legacy formats in theme files.

use engage_ux_themes::Theme;

fn main() {
	println!("Engage UX Color Format Examples\n");
	println!("================================\n");

	// Example 1: Hex format (most common)
	println!("1. Hex Format (with and without alpha):");
	let hex_theme_json = r##"{
		"name": "Hex Theme",
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
			"text_disabled": {"hex": "#BDBDBD80"},
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
			"color": {"hex": "#00000033"}
		}
	}"##;

	match Theme::from_json(hex_theme_json) {
		Ok(theme) => println!("   ✓ Successfully loaded theme: {}\n", theme.name),
		Err(e) => println!("   ✗ Failed to load theme: {}\n", e),
	}

	// Example 2: RGB array format
	println!("2. RGB Array Format (with and without alpha):");
	let rgb_theme_json = r#"{
		"name": "RGB Theme",
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

	match Theme::from_json(rgb_theme_json) {
		Ok(theme) => println!("   ✓ Successfully loaded theme: {}\n", theme.name),
		Err(e) => println!("   ✗ Failed to load theme: {}\n", e),
	}

	// Example 3: HSL array format
	println!("3. HSL Array Format (with and without alpha):");
	let hsl_theme_json = r#"{
		"name": "HSL Theme",
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
			"text_disabled": {"hsl": [0, 0, 0.74, 0.5]},
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

	match Theme::from_json(hsl_theme_json) {
		Ok(theme) => println!("   ✓ Successfully loaded theme: {}\n", theme.name),
		Err(e) => println!("   ✗ Failed to load theme: {}\n", e),
	}

	// Example 4: Mixed formats
	println!("4. Mixed Format Theme (using all formats together):");
	let mixed_theme_json = r##"{
		"name": "Mixed Format Theme",
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

	match Theme::from_json(mixed_theme_json) {
		Ok(theme) => println!("   ✓ Successfully loaded theme: {}\n", theme.name),
		Err(e) => println!("   ✗ Failed to load theme: {}\n", e),
	}

	// Example 5: Legacy format (still supported)
	println!("5. Legacy Format (backward compatible):");
	let legacy_theme_json = r#"{
		"name": "Legacy Theme",
		"colors": {
			"primary": {"space": "RGB", "components": [0.098, 0.463, 0.824, 1.0]},
			"secondary": {"space": "RGB", "components": [0.259, 0.259, 0.259, 1.0]},
			"background": {"space": "RGB", "components": [1.0, 1.0, 1.0, 1.0]},
			"surface": {"space": "RGB", "components": [0.961, 0.961, 0.961, 1.0]},
			"error": {"space": "RGB", "components": [0.827, 0.184, 0.184, 1.0]},
			"warning": {"space": "RGB", "components": [1.0, 0.627, 0.0, 1.0]},
			"success": {"space": "RGB", "components": [0.220, 0.557, 0.235, 1.0]},
			"info": {"space": "RGB", "components": [0.008, 0.533, 0.820, 1.0]},
			"text_primary": {"space": "RGB", "components": [0.129, 0.129, 0.129, 1.0]},
			"text_secondary": {"space": "RGB", "components": [0.459, 0.459, 0.459, 1.0]},
			"text_disabled": {"space": "RGB", "components": [0.741, 0.741, 0.741, 0.5]},
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
			"color": {"space": "RGB", "components": [0.878, 0.878, 0.878, 1.0]}
		},
		"shadows": {
			"enabled": true,
			"blur_radius": 4.0,
			"offset_x": 0.0,
			"offset_y": 2.0,
			"color": {"space": "RGB", "components": [0.0, 0.0, 0.0, 0.2]}
		}
	}"#;

	match Theme::from_json(legacy_theme_json) {
		Ok(theme) => println!("   ✓ Successfully loaded theme: {}\n", theme.name),
		Err(e) => println!("   ✗ Failed to load theme: {}\n", e),
	}

	println!("\n================================");
	println!("All color format examples completed successfully!");
	println!("\nKey Takeaways:");
	println!("- Hex format: Most concise, perfect for known colors");
	println!("- RGB format: Great for programmatic color generation");
	println!("- HSL format: Ideal for color manipulation and theming");
	println!("- Legacy format: Backward compatible, uses normalized values");
	println!("- Mix and match: Use the best format for each color");
}
