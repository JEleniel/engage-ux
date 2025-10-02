//! Font loading and management system

use super::MediaError;
use std::collections::HashMap;

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
	/// Thin (100)
	Thin,
	/// Extra Light (200)
	ExtraLight,
	/// Light (300)
	Light,
	/// Normal/Regular (400)
	Normal,
	/// Medium (500)
	Medium,
	/// Semi Bold (600)
	SemiBold,
	/// Bold (700)
	Bold,
	/// Extra Bold (800)
	ExtraBold,
	/// Black (900)
	Black,
}

impl FontWeight {
	/// Get numeric value
	pub fn value(&self) -> u16 {
		match self {
			FontWeight::Thin => 100,
			FontWeight::ExtraLight => 200,
			FontWeight::Light => 300,
			FontWeight::Normal => 400,
			FontWeight::Medium => 500,
			FontWeight::SemiBold => 600,
			FontWeight::Bold => 700,
			FontWeight::ExtraBold => 800,
			FontWeight::Black => 900,
		}
	}

	/// Create from numeric value
	pub fn from_value(value: u16) -> Self {
		match value {
			0..=150 => FontWeight::Thin,
			151..=250 => FontWeight::ExtraLight,
			251..=350 => FontWeight::Light,
			351..=450 => FontWeight::Normal,
			451..=550 => FontWeight::Medium,
			551..=650 => FontWeight::SemiBold,
			651..=750 => FontWeight::Bold,
			751..=850 => FontWeight::ExtraBold,
			_ => FontWeight::Black,
		}
	}
}

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
	/// Normal/upright style
	Normal,
	/// Italic style
	Italic,
	/// Oblique style
	Oblique,
}

/// Font family
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontFamily {
	/// Family name
	pub name: String,
	/// Fallback families
	pub fallbacks: Vec<String>,
}

impl FontFamily {
	/// Create a new font family
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			fallbacks: Vec::new(),
		}
	}

	/// Create with fallbacks
	pub fn with_fallbacks(
		name: impl Into<String>,
		fallbacks: Vec<impl Into<String>>,
	) -> Self {
		Self {
			name: name.into(),
			fallbacks: fallbacks.into_iter().map(|s| s.into()).collect(),
		}
	}

	/// Add a fallback font
	pub fn add_fallback(&mut self, family: impl Into<String>) {
		self.fallbacks.push(family.into());
	}
}

/// Font representation
#[derive(Debug, Clone)]
pub struct Font {
	/// Font family
	pub family: FontFamily,
	/// Font weight
	pub weight: FontWeight,
	/// Font style
	pub style: FontStyle,
	/// Font size
	pub size: f32,
	/// Font data (if loaded from file)
	pub data: Option<Vec<u8>>,
}

impl Font {
	/// Create a new font
	pub fn new(family: FontFamily, size: f32) -> Self {
		Self {
			family,
			weight: FontWeight::Normal,
			style: FontStyle::Normal,
			size,
			data: None,
		}
	}

	/// Create with weight and style
	pub fn with_style(
		family: FontFamily,
		size: f32,
		weight: FontWeight,
		style: FontStyle,
	) -> Self {
		Self {
			family,
			weight,
			style,
			size,
			data: None,
		}
	}

	/// Load font from file path (stub for now)
	pub fn load_from_file(_path: &str) -> Result<Self, MediaError> {
		// TODO: Implement actual font loading
		Err(MediaError::UnsupportedFormat(
			"Font loading not yet implemented".to_string(),
		))
	}

	/// Load font from bytes
	pub fn load_from_bytes(data: Vec<u8>, size: f32) -> Result<Self, MediaError> {
		// Basic validation
		if data.is_empty() {
			return Err(MediaError::InvalidData("Empty font data".to_string()));
		}

		// Create font with data
		Ok(Self {
			family: FontFamily::new("Loaded Font"),
			weight: FontWeight::Normal,
			style: FontStyle::Normal,
			size,
			data: Some(data),
		})
	}
}

/// Font registry for managing loaded fonts
#[derive(Debug, Default)]
pub struct FontRegistry {
	/// Registered fonts by family name
	fonts: HashMap<String, Vec<Font>>,
}

impl FontRegistry {
	/// Create a new font registry
	pub fn new() -> Self {
		Self::default()
	}

	/// Register a font
	pub fn register(&mut self, font: Font) {
		let family_name = font.family.name.clone();
		self.fonts
			.entry(family_name)
			.or_insert_with(Vec::new)
			.push(font);
	}

	/// Get font by family and style
	pub fn get(&self, family: &str, weight: FontWeight, style: FontStyle) -> Option<&Font> {
		self.fonts.get(family)?.iter().find(|f| {
			f.weight == weight && f.style == style
		})
	}

	/// Get all fonts for a family
	pub fn get_family(&self, family: &str) -> Option<&Vec<Font>> {
		self.fonts.get(family)
	}

	/// Check if a font family is registered
	pub fn has_family(&self, family: &str) -> bool {
		self.fonts.contains_key(family)
	}

	/// Get all registered family names
	pub fn families(&self) -> Vec<&String> {
		self.fonts.keys().collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_font_weight_value() {
		assert_eq!(FontWeight::Normal.value(), 400);
		assert_eq!(FontWeight::Bold.value(), 700);
		assert_eq!(FontWeight::Thin.value(), 100);
	}

	#[test]
	fn test_font_weight_from_value() {
		assert_eq!(FontWeight::from_value(400), FontWeight::Normal);
		assert_eq!(FontWeight::from_value(700), FontWeight::Bold);
		assert_eq!(FontWeight::from_value(550), FontWeight::Medium);
	}

	#[test]
	fn test_font_family() {
		let family = FontFamily::new("Arial");
		assert_eq!(family.name, "Arial");
		assert!(family.fallbacks.is_empty());

		let family_with_fallbacks =
			FontFamily::with_fallbacks("Helvetica", vec!["Arial", "sans-serif"]);
		assert_eq!(family_with_fallbacks.fallbacks.len(), 2);
	}

	#[test]
	fn test_font_creation() {
		let family = FontFamily::new("Georgia");
		let font = Font::new(family.clone(), 16.0);
		assert_eq!(font.family.name, "Georgia");
		assert_eq!(font.size, 16.0);
		assert_eq!(font.weight, FontWeight::Normal);
	}

	#[test]
	fn test_font_with_style() {
		let family = FontFamily::new("Times New Roman");
		let font = Font::with_style(family, 18.0, FontWeight::Bold, FontStyle::Italic);
		assert_eq!(font.weight, FontWeight::Bold);
		assert_eq!(font.style, FontStyle::Italic);
	}

	#[test]
	fn test_font_load_from_bytes() {
		let data = vec![1, 2, 3, 4]; // Fake font data
		let font = Font::load_from_bytes(data.clone(), 14.0).unwrap();
		assert_eq!(font.data, Some(data));
		assert_eq!(font.size, 14.0);
	}

	#[test]
	fn test_font_registry() {
		let mut registry = FontRegistry::new();

		let family = FontFamily::new("Open Sans");
		let font = Font::new(family.clone(), 16.0);
		registry.register(font);

		assert!(registry.has_family("Open Sans"));
		assert!(!registry.has_family("Arial"));

		let retrieved = registry.get("Open Sans", FontWeight::Normal, FontStyle::Normal);
		assert!(retrieved.is_some());
	}

	#[test]
	fn test_font_registry_families() {
		let mut registry = FontRegistry::new();

		registry.register(Font::new(FontFamily::new("Arial"), 12.0));
		registry.register(Font::new(FontFamily::new("Georgia"), 14.0));

		let families = registry.families();
		assert_eq!(families.len(), 2);
	}
}
