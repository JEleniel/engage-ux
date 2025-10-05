//! Color system supporting RGB and HSL color models
//!
//! This module provides full support for both RGB and HSL color spaces
//! with conversion between them and various utility methods.

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a color space (RGB or HSL)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColorSpace {
	/// Red, Green, Blue color space
	RGB,
	/// Hue, Saturation, Lightness color space
	HSL,
}

/// A color with RGBA or HSLA representation
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
	space: ColorSpace,
	// For RGB: r, g, b (0.0-1.0), a (0.0-1.0)
	// For HSL: h (0.0-360.0), s (0.0-1.0), l (0.0-1.0), a (0.0-1.0)
	components: [f32; 4],
}

impl Color {
	/// Create a new RGB color
	///
	/// # Arguments
	/// * `r` - Red component (0.0-1.0)
	/// * `g` - Green component (0.0-1.0)
	/// * `b` - Blue component (0.0-1.0)
	/// * `a` - Alpha/opacity (0.0-1.0, default 1.0)
	pub fn rgb(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			space: ColorSpace::RGB,
			components: [
				r.clamp(0.0, 1.0),
				g.clamp(0.0, 1.0),
				b.clamp(0.0, 1.0),
				a.clamp(0.0, 1.0),
			],
		}
	}

	/// Create a new HSL color
	///
	/// # Arguments
	/// * `h` - Hue (0.0-360.0)
	/// * `s` - Saturation (0.0-1.0)
	/// * `l` - Lightness (0.0-1.0)
	/// * `a` - Alpha/opacity (0.0-1.0, default 1.0)
	pub fn hsl(h: f32, s: f32, l: f32, a: f32) -> Self {
		Self {
			space: ColorSpace::HSL,
			components: [
				h % 360.0,
				s.clamp(0.0, 1.0),
				l.clamp(0.0, 1.0),
				a.clamp(0.0, 1.0),
			],
		}
	}

	/// Create color from hex string (e.g., "#RRGGBB" or "#RRGGBBAA")
	pub fn from_hex(hex: &str) -> Result<Self, String> {
		let hex = hex.trim_start_matches('#');

		let (r, g, b, a) = match hex.len() {
			6 => {
				let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
				let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
				let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
				(r, g, b, 255)
			}
			8 => {
				let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
				let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
				let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
				let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;
				(r, g, b, a)
			}
			_ => return Err(format!("Invalid hex color length: {}", hex.len())),
		};

		Ok(Self::rgb(
			r as f32 / 255.0,
			g as f32 / 255.0,
			b as f32 / 255.0,
			a as f32 / 255.0,
		))
	}

	/// Convert color to RGB color space
	pub fn to_rgb(&self) -> Self {
		if self.space == ColorSpace::RGB {
			return self.clone();
		}

		// Convert HSL to RGB
		let h = self.components[0];
		let s = self.components[1];
		let l = self.components[2];
		let a = self.components[3];

		let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
		let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
		let m = l - c / 2.0;

		let (r, g, b) = match h {
			h if h < 60.0 => (c, x, 0.0),
			h if h < 120.0 => (x, c, 0.0),
			h if h < 180.0 => (0.0, c, x),
			h if h < 240.0 => (0.0, x, c),
			h if h < 300.0 => (x, 0.0, c),
			_ => (c, 0.0, x),
		};

		Self::rgb(r + m, g + m, b + m, a)
	}

	/// Convert color to HSL color space
	pub fn to_hsl(&self) -> Self {
		if self.space == ColorSpace::HSL {
			return self.clone();
		}

		// Convert RGB to HSL
		let r = self.components[0];
		let g = self.components[1];
		let b = self.components[2];
		let a = self.components[3];

		let max = r.max(g).max(b);
		let min = r.min(g).min(b);
		let delta = max - min;

		let l = (max + min) / 2.0;

		let s = if delta == 0.0 {
			0.0
		} else {
			delta / (1.0 - (2.0 * l - 1.0).abs())
		};

		let h = if delta == 0.0 {
			0.0
		} else if max == r {
			60.0 * (((g - b) / delta) % 6.0)
		} else if max == g {
			60.0 * (((b - r) / delta) + 2.0)
		} else {
			60.0 * (((r - g) / delta) + 4.0)
		};

		let h = if h < 0.0 { h + 360.0 } else { h };

		Self::hsl(h, s, l, a)
	}

	/// Get the alpha (opacity) component
	pub fn alpha(&self) -> f32 {
		self.components[3]
	}

	/// Set the alpha (opacity) component
	pub fn with_alpha(&self, alpha: f32) -> Self {
		let mut color = self.clone();
		color.components[3] = alpha.clamp(0.0, 1.0);
		color
	}

	/// Get color space
	pub fn color_space(&self) -> ColorSpace {
		self.space
	}

	/// Get raw components
	pub fn components(&self) -> [f32; 4] {
		self.components
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::rgb(0.0, 0.0, 0.0, 1.0)
	}
}

// Custom serialization - keep backward compatibility with current format
impl Serialize for Color {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		use serde::ser::SerializeStruct;
		let mut state = serializer.serialize_struct("Color", 2)?;
		state.serialize_field("space", &self.space)?;
		state.serialize_field("components", &self.components)?;
		state.end()
	}
}

// Custom deserialization - support multiple user-friendly formats
impl<'de> Deserialize<'de> for Color {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(untagged)]
		enum ColorFormat {
			// Legacy format: {"space": "RGB", "components": [r, g, b, a]}
			Legacy {
				space: ColorSpace,
				components: [f32; 4],
			},
			// RGB array format: {"rgb": [r, g, b]} or {"rgb": [r, g, b, a]}
			Rgb {
				rgb: Vec<f32>,
			},
			// Hex format: {"hex": "#RRGGBB"} or {"hex": "#RRGGBBAA"}
			Hex {
				hex: String,
			},
			// HSL format: {"hsl": [h, s, l]} or {"hsl": [h, s, l, a]}
			Hsl {
				hsl: Vec<f32>,
			},
		}

		let format = ColorFormat::deserialize(deserializer)?;

		match format {
			ColorFormat::Legacy { space, components } => Ok(Color { space, components }),
			ColorFormat::Rgb { rgb } => {
				if rgb.len() == 3 {
					Ok(Color::rgb(
						rgb[0] / 255.0,
						rgb[1] / 255.0,
						rgb[2] / 255.0,
						1.0,
					))
				} else if rgb.len() == 4 {
					Ok(Color::rgb(
						rgb[0] / 255.0,
						rgb[1] / 255.0,
						rgb[2] / 255.0,
						rgb[3],
					))
				} else {
					Err(de::Error::custom(format!(
						"RGB array must have 3 or 4 components, got {}",
						rgb.len()
					)))
				}
			}
			ColorFormat::Hex { hex } => Color::from_hex(&hex).map_err(de::Error::custom),
			ColorFormat::Hsl { hsl } => {
				if hsl.len() == 3 {
					Ok(Color::hsl(hsl[0], hsl[1], hsl[2], 1.0))
				} else if hsl.len() == 4 {
					Ok(Color::hsl(hsl[0], hsl[1], hsl[2], hsl[3]))
				} else {
					Err(de::Error::custom(format!(
						"HSL array must have 3 or 4 components, got {}",
						hsl.len()
					)))
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rgb_creation() {
		let color = Color::rgb(1.0, 0.5, 0.0, 1.0);
		assert_eq!(color.color_space(), ColorSpace::RGB);
		assert_eq!(color.alpha(), 1.0);
	}

	#[test]
	fn test_hsl_creation() {
		let color = Color::hsl(180.0, 0.5, 0.5, 1.0);
		assert_eq!(color.color_space(), ColorSpace::HSL);
		assert_eq!(color.alpha(), 1.0);
	}

	#[test]
	fn test_from_hex() {
		let color = Color::from_hex("#FF8000").unwrap();
		let rgb = color.to_rgb();
		let components = rgb.components();
		assert!((components[0] - 1.0).abs() < 0.01);
		assert!((components[1] - 0.5).abs() < 0.01);
		assert!((components[2] - 0.0).abs() < 0.01);
	}

	#[test]
	fn test_rgb_to_hsl_conversion() {
		let rgb = Color::rgb(1.0, 0.0, 0.0, 1.0);
		let hsl = rgb.to_hsl();
		assert_eq!(hsl.color_space(), ColorSpace::HSL);
	}

	#[test]
	fn test_hsl_to_rgb_conversion() {
		let hsl = Color::hsl(0.0, 1.0, 0.5, 1.0);
		let rgb = hsl.to_rgb();
		assert_eq!(rgb.color_space(), ColorSpace::RGB);
		let components = rgb.components();
		assert!((components[0] - 1.0).abs() < 0.01);
	}

	#[test]
	fn test_alpha_manipulation() {
		let color = Color::rgb(1.0, 0.5, 0.0, 1.0);
		let transparent = color.with_alpha(0.5);
		assert_eq!(transparent.alpha(), 0.5);
	}

	#[test]
	fn test_clamping() {
		let color = Color::rgb(2.0, -0.5, 0.5, 1.5);
		let components = color.components();
		assert_eq!(components[0], 1.0);
		assert_eq!(components[1], 0.0);
		assert_eq!(components[3], 1.0);
	}

	#[test]
	fn test_deserialize_rgb_array_without_alpha() {
		let json = r#"{"rgb": [128, 255, 255]}"#;
		let color: Color = serde_json::from_str(json).unwrap();
		let rgb = color.to_rgb();
		let components = rgb.components();
		assert!((components[0] - 0.502).abs() < 0.01); // 128/255 ≈ 0.502
		assert!((components[1] - 1.0).abs() < 0.01); // 255/255 = 1.0
		assert!((components[2] - 1.0).abs() < 0.01); // 255/255 = 1.0
		assert_eq!(components[3], 1.0); // Alpha = 1.0
	}

	#[test]
	fn test_deserialize_rgb_array_with_alpha() {
		let json = r#"{"rgb": [128, 255, 255, 0.5]}"#;
		let color: Color = serde_json::from_str(json).unwrap();
		let rgb = color.to_rgb();
		let components = rgb.components();
		assert!((components[0] - 0.502).abs() < 0.01);
		assert!((components[1] - 1.0).abs() < 0.01);
		assert!((components[2] - 1.0).abs() < 0.01);
		assert_eq!(components[3], 0.5);
	}

	#[test]
	fn test_deserialize_hex_without_alpha() {
		let json = r##"{"hex": "#80FFFF"}"##;
		let color: Color = serde_json::from_str(json).unwrap();
		let rgb = color.to_rgb();
		let components = rgb.components();
		assert!((components[0] - 0.502).abs() < 0.01);
		assert!((components[1] - 1.0).abs() < 0.01);
		assert!((components[2] - 1.0).abs() < 0.01);
		assert_eq!(components[3], 1.0);
	}

	#[test]
	fn test_deserialize_hex_with_alpha() {
		let json = r##"{"hex": "#80FFFF80"}"##;
		let color: Color = serde_json::from_str(json).unwrap();
		let rgb = color.to_rgb();
		let components = rgb.components();
		assert!((components[0] - 0.502).abs() < 0.01);
		assert!((components[1] - 1.0).abs() < 0.01);
		assert!((components[2] - 1.0).abs() < 0.01);
		assert!((components[3] - 0.502).abs() < 0.01); // 128/255 ≈ 0.502
	}

	#[test]
	fn test_deserialize_hsl_without_alpha() {
		let json = r#"{"hsl": [180, 0.5, 0.8]}"#;
		let color: Color = serde_json::from_str(json).unwrap();
		assert_eq!(color.color_space(), ColorSpace::HSL);
		let components = color.components();
		assert_eq!(components[0], 180.0);
		assert_eq!(components[1], 0.5);
		assert_eq!(components[2], 0.8);
		assert_eq!(components[3], 1.0);
	}

	#[test]
	fn test_deserialize_hsl_with_alpha() {
		let json = r#"{"hsl": [180, 0.5, 0.8, 0.5]}"#;
		let color: Color = serde_json::from_str(json).unwrap();
		assert_eq!(color.color_space(), ColorSpace::HSL);
		let components = color.components();
		assert_eq!(components[0], 180.0);
		assert_eq!(components[1], 0.5);
		assert_eq!(components[2], 0.8);
		assert_eq!(components[3], 0.5);
	}

	#[test]
	fn test_deserialize_legacy_format() {
		let json = r#"{"space": "RGB", "components": [0.5, 1.0, 1.0, 1.0]}"#;
		let color: Color = serde_json::from_str(json).unwrap();
		assert_eq!(color.color_space(), ColorSpace::RGB);
		let components = color.components();
		assert_eq!(components[0], 0.5);
		assert_eq!(components[1], 1.0);
		assert_eq!(components[2], 1.0);
		assert_eq!(components[3], 1.0);
	}

	#[test]
	fn test_serialize_maintains_legacy_format() {
		let color = Color::rgb(0.5, 1.0, 1.0, 1.0);
		let json = serde_json::to_string(&color).unwrap();
		assert!(json.contains("\"space\""));
		assert!(json.contains("\"components\""));
	}

	#[test]
	fn test_roundtrip_serialization() {
		let original = Color::rgb(0.5, 0.75, 0.25, 0.8);
		let json = serde_json::to_string(&original).unwrap();
		let deserialized: Color = serde_json::from_str(&json).unwrap();
		assert_eq!(original, deserialized);
	}
}
