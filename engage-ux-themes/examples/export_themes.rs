//! Export default themes to JSON files
//!
//! Run with: cargo run --example export_themes -p engage-ux-themes

use engage_ux_themes::Theme;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
	// Create themes directory if it doesn't exist
	fs::create_dir_all("themes")?;

	// Export light theme
	let light = Theme::light();
	let light_json = light.to_json().expect("Failed to serialize light theme");
	fs::write("themes/light.json", light_json)?;
	println!("✓ Exported light theme to themes/light.json");

	// Export dark theme
	let dark = Theme::dark();
	let dark_json = dark.to_json().expect("Failed to serialize dark theme");
	fs::write("themes/dark.json", dark_json)?;
	println!("✓ Exported dark theme to themes/dark.json");

	Ok(())
}
