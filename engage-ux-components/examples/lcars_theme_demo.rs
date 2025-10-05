//! LCARS Theme Demo
//!
//! Demonstrates the futuristic LCARS theme inspired by Star Trek.
//! Named after Captain Picard's famous "Engage" command.

use engage_ux_themes::Theme;

fn main() {
	println!("=== Engage UX - LCARS Theme Demo ===\n");
	println!("\"Make it so.\" - Captain Jean-Luc Picard\n");

	// Demo LCARS Light theme
	println!("╔══════════════════════════════════════════════════════════════╗");
	println!("║  LCARS LIGHT THEME - Futuristic Interface                   ║");
	println!("╚══════════════════════════════════════════════════════════════╝\n");

	let lcars_light = Theme::light();
	display_theme(&lcars_light);

	println!("\n");

	// Demo LCARS Dark theme
	println!("╔══════════════════════════════════════════════════════════════╗");
	println!("║  LCARS DARK THEME - Futuristic Interface                    ║");
	println!("╚══════════════════════════════════════════════════════════════╝\n");

	let lcars_dark = Theme::dark();
	display_theme(&lcars_dark);

	println!("\n");
	println!("═══════════════════════════════════════════════════════════════");
	println!("  LCARS Design Elements - Voyager Style");
	println!("═══════════════════════════════════════════════════════════════\n");

	println!("Key Features:");
	println!("  • Voyager-inspired indigo/blue colors (#6699FF, #5566CC)");
	println!("  • Distinctive curved borders (radius: 20.0)");
	println!("  • Bold border width (3.0px)");
	println!("  • Rich custom color palette:");
	println!("    - Voyager Indigo, Blue, Cyan, Teal");
	println!("    - Purple, Lavender, Periwinkle, Steel");
	println!("  • Glowing blue shadow effects for depth");
	println!("  • Clean Helvetica Neue typography");
	println!("  • Compact spacing for information density\n");

	println!("═══════════════════════════════════════════════════════════════\n");

	// Display custom colors
	println!("Custom Voyager Color Palette:");
	if let Some(voyager_indigo) = lcars_light.colors.custom.get("voyager_indigo") {
		println!("  Voyager Indigo:     {:?}", voyager_indigo);
	}
	if let Some(voyager_blue) = lcars_light.colors.custom.get("voyager_blue") {
		println!("  Voyager Blue:       {:?}", voyager_blue);
	}
	if let Some(voyager_cyan) = lcars_light.colors.custom.get("voyager_cyan") {
		println!("  Voyager Cyan:       {:?}", voyager_cyan);
	}
	if let Some(voyager_teal) = lcars_light.colors.custom.get("voyager_teal") {
		println!("  Voyager Teal:       {:?}", voyager_teal);
	}
	if let Some(voyager_purple) = lcars_light.colors.custom.get("voyager_purple") {
		println!("  Voyager Purple:     {:?}", voyager_purple);
	}
	if let Some(voyager_lavender) = lcars_light.colors.custom.get("voyager_lavender") {
		println!("  Voyager Lavender:   {:?}", voyager_lavender);
	}
	if let Some(voyager_periwinkle) = lcars_light.colors.custom.get("voyager_periwinkle") {
		println!("  Voyager Periwinkle: {:?}", voyager_periwinkle);
	}
	if let Some(voyager_steel) = lcars_light.colors.custom.get("voyager_steel") {
		println!("  Voyager Steel:      {:?}", voyager_steel);
	}

	println!("\n═══════════════════════════════════════════════════════════════");
	println!("  \"Engage!\" - The Spirit of Engage UX");
	println!("═══════════════════════════════════════════════════════════════\n");
}

fn display_theme(theme: &Theme) {
	println!("Theme: {}", theme.name);
	println!("───────────────────────────────────────────────────────────────\n");

	println!("Colors:");
	println!("  Primary:        {:?}", theme.colors.primary);
	println!("  Secondary:      {:?}", theme.colors.secondary);
	println!("  Background:     {:?}", theme.colors.background);
	println!("  Surface:        {:?}", theme.colors.surface);
	println!("  Error:          {:?}", theme.colors.error);
	println!("  Warning:        {:?}", theme.colors.warning);
	println!("  Success:        {:?}", theme.colors.success);
	println!("  Info:           {:?}", theme.colors.info);
	println!("  Text Primary:   {:?}", theme.colors.text_primary);
	println!("  Text Secondary: {:?}", theme.colors.text_secondary);
	println!("  Custom Colors:  {} defined", theme.colors.custom.len());
	println!();

	println!("Typography:");
	println!("  Font Family: {}", theme.typography.font_family);
	println!("  Base Size:   {}px", theme.typography.font_size_base);
	println!("  Small Size:  {}px", theme.typography.font_size_small);
	println!("  Large Size:  {}px", theme.typography.font_size_large);
	println!("  Line Height: {}", theme.typography.line_height);
	println!();

	println!("Spacing:");
	println!("  Unit:   {}px", theme.spacing.unit);
	println!("  Small:  {}px", theme.spacing.small);
	println!("  Medium: {}px", theme.spacing.medium);
	println!("  Large:  {}px", theme.spacing.large);
	println!();

	println!("Borders:");
	println!("  Width:  {}px", theme.borders.width);
	println!("  Radius: {}px (curved LCARS style)", theme.borders.radius);
	println!("  Color:  {:?}", theme.borders.color);
	println!();

	println!("Shadows:");
	println!("  Enabled:     {}", theme.shadows.enabled);
	println!("  Blur Radius: {}px", theme.shadows.blur_radius);
	println!("  Offset:      ({}, {})", theme.shadows.offset_x, theme.shadows.offset_y);
	println!("  Color:       {:?}", theme.shadows.color);
}
