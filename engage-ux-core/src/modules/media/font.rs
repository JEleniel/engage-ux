//! Font loading and management system

use super::MediaError;
use std::collections::HashMap;

/// Font weight
#[derive(Debug, Clone)]
pub enum FontWeight {
	/// Extra Light (200)
	ExtraLight,
	/// Light (300)
	Light,
	/// Normal/Regular (500)
	Normal,
	/// Bold (700)
	Bold,
	/// Extra Bold (800)
	ExtraBold,
}

/// Font style
#[derive(Debug, Clone)]
pub enum FontStyle {
	/// Normal/upright style
	Normal,
	/// Italic style
	Italic,
}

/// Font family
#[derive(Debug, Clone)]
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
	pub fn with_fallbacks(name: impl Into<String>, fallbacks: Vec<impl Into<String>>) -> Self {
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
	pub fn with_style(family: FontFamily, size: f32, weight: FontWeight, style: FontStyle) -> Self {
		Self {
			family,
			weight,
			style,
			size,
			data: None,
		}
	}

	/// Load font from file path
	pub fn load_from_file(path: &str) -> Result<Self, MediaError> {
		let data = std::fs::read(path)
			.map_err(|e| MediaError::LoadFailed(format!("Failed to read font file: {}", e)))?;

		Self::load_from_bytes(data, 16.0)
	}

	/// Load font from bytes
	pub fn load_from_bytes(data: Vec<u8>, size: f32) -> Result<Self, MediaError> {
		// Basic validation
		if data.is_empty() {
			return Err(MediaError::InvalidData("Empty font data".to_string()));
		}

		// Validate font data with fontdue
		let font_settings = fontdue::FontSettings::default();
		let _parsed_font = fontdue::Font::from_bytes(data.as_slice(), font_settings)
			.map_err(|e| MediaError::InvalidData(format!("Invalid font data: {}", e)))?;

		// Use default family name since fontdue doesn't expose it directly
		let family_name = "Loaded Font".to_string();

		// Create font with data
		Ok(Self {
			family: FontFamily::new(family_name),
			weight: FontWeight::Normal,
			style: FontStyle::Normal,
			size,
			data: Some(data),
		})
	}
}

/// Font registry for managing loaded fonts
#[derive(Debug, Default)]
#[allow(dead_code)]
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
		self.fonts.entry(family_name).or_default().push(font);
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
