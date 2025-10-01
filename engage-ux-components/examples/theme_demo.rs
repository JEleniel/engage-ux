//! Theme demonstration example
//!
//! This example shows how to create and customize themes in Engage UX.
//!
//! Run with: cargo run --example theme_demo

use engage_ux_core::color::Color;
use engage_ux_themes::Theme;

fn main() {
	println!("=== Engage UX - Theme System Demo ===\n");

	// Load default light theme
	println!("--- Light Theme ---");
	let light = Theme::light();
	display_theme(&light);

	// Load default dark theme
	println!("\n--- Dark Theme ---");
	let dark = Theme::dark();
	display_theme(&dark);

	// Demonstrate color conversions
	println!("\n--- Color System ---");
	demonstrate_colors();

	// Export theme to JSON
	println!("\n--- Theme Serialization ---");
	if let Ok(json) = light.to_json() {
		println!("Light theme exported to JSON:");
		// Print first few lines
		for (i, line) in json.lines().take(10).enumerate() {
			println!("{}: {}", i + 1, line);
		}
		println!("... ({} total lines)", json.lines().count());

		// Demonstrate round-trip serialization
		if let Ok(restored) = Theme::from_json(&json) {
			println!("\n✓ Theme successfully restored from JSON");
			println!("  Name: {}", restored.name);
			println!("  Font size: {}", restored.typography.font_size_base);
		}
	}

	println!("\n=== Demo Complete ===");
}

fn display_theme(theme: &Theme) {
	println!("Theme: {}", theme.name);
	println!("\nColors:");
	println!("  Primary:    {:?}", theme.colors.primary.components());
	println!("  Secondary:  {:?}", theme.colors.secondary.components());
	println!("  Background: {:?}", theme.colors.background.components());
	println!("  Text:       {:?}", theme.colors.text_primary.components());
	
	println!("\nTypography:");
	println!("  Font family: {}", theme.typography.font_family);
	println!("  Base size:   {}px", theme.typography.font_size_base);
	println!("  Line height: {}", theme.typography.line_height);
	
	println!("\nSpacing:");
	println!("  Unit:   {}px", theme.spacing.unit);
	println!("  Small:  {}px", theme.spacing.small);
	println!("  Medium: {}px", theme.spacing.medium);
	println!("  Large:  {}px", theme.spacing.large);
	
	println!("\nBorders:");
	println!("  Width:  {}px", theme.borders.width);
	println!("  Radius: {}px", theme.borders.radius);
	
	println!("\nShadows:");
	println!("  Enabled:     {}", theme.shadows.enabled);
	println!("  Blur radius: {}px", theme.shadows.blur_radius);
	println!("  Offset:      ({}, {})", theme.shadows.offset_x, theme.shadows.offset_y);
}

fn demonstrate_colors() {
	// RGB colors
	let red = Color::rgb(1.0, 0.0, 0.0, 1.0);
	let green = Color::rgb(0.0, 1.0, 0.0, 1.0);
	let blue = Color::rgb(0.0, 0.0, 1.0, 1.0);
	
	println!("RGB Colors:");
	println!("  Red:   {:?}", red.components());
	println!("  Green: {:?}", green.components());
	println!("  Blue:  {:?}", blue.components());
	
	// HSL colors
	println!("\nHSL Conversions:");
	println!("  Red as HSL:   {:?}", red.to_hsl().components());
	println!("  Green as HSL: {:?}", green.to_hsl().components());
	println!("  Blue as HSL:  {:?}", blue.to_hsl().components());
	
	// Hex colors
	println!("\nFrom Hex:");
	let cyan = Color::from_hex("#00FFFF").unwrap();
	let magenta = Color::from_hex("#FF00FF").unwrap();
	let yellow = Color::from_hex("#FFFF00").unwrap();
	println!("  Cyan:    {:?}", cyan.components());
	println!("  Magenta: {:?}", magenta.components());
	println!("  Yellow:  {:?}", yellow.components());
	
	// Alpha/transparency
	println!("\nTransparency:");
	let opaque = Color::rgb(1.0, 0.0, 0.0, 1.0);
	let semi = opaque.with_alpha(0.5);
	let transparent = opaque.with_alpha(0.0);
	println!("  Opaque:       alpha = {}", opaque.alpha());
	println!("  Semi-trans:   alpha = {}", semi.alpha());
	println!("  Transparent:  alpha = {}", transparent.alpha());
}
