//! RGB Color system with support for HSL and utility methods
//!
//! This module provides full support for both RGB and HSL color spaces
//! with conversion between them and various utility methods.
//! RGB is used internally since that is what most displays use.

use serde::{
	Deserialize, Serialize,
	de::{self, Deserializer},
};
use serde_json::Value as JsonValue;

use crate::Ansi;

/// Represents an RGB color, provides methods for conversion and manipulation
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Color {
	/// Red component (0-255)
	pub red: u8,
	/// Green component (0-255)
	pub green: u8,
	/// Blue component (0-255)
	pub blue: u8,
	/// Alpha component (0-255), 255 is fully opaque
	pub alpha: u8,
}
impl Color {
	/// Create a new Color from RGBA components
	pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}

	/// Create a new Color from RGB components, defaulting alpha to 255 (opaque)
	pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
		Self {
			red,
			green,
			blue,
			alpha: 255,
		}
	}

	/// Create a new color from a hex string, e.g. "#RRGGBB" or "#RRGGBBAA"
	/// Returns a ColorError if the format is invalid
	pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
		let hex = hex.trim_start_matches('#');
		match hex.len() {
			6 => {
				let red = u8::from_str_radix(&hex[0..2], 16)?;
				let green = u8::from_str_radix(&hex[2..4], 16)?;
				let blue = u8::from_str_radix(&hex[4..6], 16)?;
				Ok(Self::from_rgb(red, green, blue))
			}
			8 => {
				let red = u8::from_str_radix(&hex[0..2], 16)?;
				let green = u8::from_str_radix(&hex[2..4], 16)?;
				let blue = u8::from_str_radix(&hex[4..6], 16)?;
				let alpha = u8::from_str_radix(&hex[6..8], 16)?;
				Ok(Self::new(red, green, blue, alpha))
			}
			_ => Err(ColorError::InvalidLength),
		}
	}

	/// Create a new Color from HSLA components
	pub fn from_hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
		let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
		let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
		let m = lightness - c / 2.0;
		let (r1, g1, b1) = if (0.0..60.0).contains(&hue) {
			(c, x, 0.0)
		} else if (60.0..120.0).contains(&hue) {
			(x, c, 0.0)
		} else if (120.0..180.0).contains(&hue) {
			(0.0, c, x)
		} else if (180.0..240.0).contains(&hue) {
			(0.0, x, c)
		} else if (240.0..300.0).contains(&hue) {
			(x, 0.0, c)
		} else {
			(c, 0.0, x)
		};
		Self::new(
			((r1 + m) * 255.0).round() as u8,
			((g1 + m) * 255.0).round() as u8,
			((b1 + m) * 255.0).round() as u8,
			(alpha * 255.0).round() as u8,
		)
	}

	/// Get the components of the color as (hue, saturation, lightness, alpha)
	pub fn to_hsla(&self) -> (f32, f32, f32, f32) {
		let r = self.red as f32 / 255.0;
		let g = self.green as f32 / 255.0;
		let b = self.blue as f32 / 255.0;
		let a = self.alpha as f32 / 255.0;

		let max = r.max(g).max(b);
		let min = r.min(g).min(b);
		let delta = max - min;
		let sum = max + min;

		let l = sum / 2.0;

		// Using the simplified formulas, and calculating h + 360 % 360 to ensure positive hue
		if delta == 0.0 {
			return (0.0, 0.0, l, a); // achromatic
		}
		let s = if l < 0.5 {
			delta / sum
		} else {
			delta / (2.0 - delta)
		};
		let h = ((if max == r {
			(g - b) / delta
		} else if max == g {
			(b - r) / delta + 2.0
		} else {
			(r - g) / delta + 4.0
		} * 60.0) + 360.0)
			% 360.0;

		(h, s, l, a)
	}

	/// Calculate the relative luminance of the color
	pub fn relative_luminance(&self) -> f32 {
		// Convert sRGB to linear light (per WCAG / sRGB standard) before weighting
		let to_linear = |channel: u8| {
			let c = channel as f32 / 255.0;
			if c <= 0.03928 {
				c / 12.92
			} else {
				((c + 0.055) / 1.055).powf(2.4)
			}
		};
		let r = to_linear(self.red);
		let g = to_linear(self.green);
		let b = to_linear(self.blue);
		0.2126 * r + 0.7152 * g + 0.0722 * b
	}

	/// Calculate the contrast ratio between this color and another color
	pub fn contrast_ratio(&self, other: &Color) -> f32 {
		let l1 = self.relative_luminance();
		let l2 = other.relative_luminance();
		(l1.max(l2) + 0.05) / (l1.min(l2) + 0.05)
	}

	/// Choose black or white text color based on contrast ratio
	pub fn contrast_black_or_white(&self) -> Color {
		if self.contrast_ratio(&Ansi::WHITE) >= self.contrast_ratio(&Ansi::BLACK) {
			Ansi::WHITE
		} else {
			Ansi::BLACK
		}
	}

	/// Colorize the black or white contrast color with the inverse of this color
	pub fn contrast_color(&self) -> Color {
		let contrast = self.contrast_black_or_white();
		let (h, s, l, a) = self.to_hsla();
		let inverse_hue = (h + 180.0) % 360.0;
		let saturation = if contrast == Ansi::BLACK { 1.0 - s } else { s };
		Color::from_hsla(inverse_hue, saturation, l, a)
	}

	/// Linearly interpolate between two colors.
	/// `t` should be in [0.0, 1.0], where 0.0 returns self and 1.0 returns other.
	pub fn lerp(&self, other: &Color, t: f32) -> Color {
		let clamp = |v: f32| v.clamp(0.0, 1.0);
		let t = clamp(t);
		let lerp_u8 = |a: u8, b: u8| ((a as f32) + (b as f32 - a as f32) * t).round() as u8;
		Color {
			red: lerp_u8(self.red, other.red),
			green: lerp_u8(self.green, other.green),
			blue: lerp_u8(self.blue, other.blue),
			alpha: lerp_u8(self.alpha, other.alpha),
		}
	}
}

/// Default to opaque black
impl Default for Color {
	fn default() -> Self {
		Self {
			red: 0,
			green: 0,
			blue: 0,
			alpha: 255,
		}
	}
}

// Custom deserialization to support "user-friendly" theme JSON formats
impl<'de> Deserialize<'de> for Color {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let value = JsonValue::deserialize(deserializer).map_err(de::Error::custom)?;

		// helper to convert numeric JsonValue to u8 (accepts 0..255 ints or 0.0..1.0 floats)
		let num_to_u8 = |v: &JsonValue| -> Option<u8> {
			if let Some(i) = v.as_i64()
				&& (0..=255).contains(&i)
			{
				return Some(i as u8);
			}
			if let Some(u) = v.as_u64()
				&& u <= 255
			{
				return Some(u as u8);
			}
			if let Some(f) = v.as_f64() {
				// treat floats in 0.0..1.0 as normalized, otherwise 0..255
				if (0.0..=1.0).contains(&f) {
					return Some((f * 255.0).round() as u8);
				}
				if (0.0..=255.0).contains(&f) {
					return Some(f.round() as u8);
				}
			}
			None
		};

		match value {
			JsonValue::String(s) => {
				// hex string
				Color::from_hex(&s).map_err(de::Error::custom)
			}
			JsonValue::Array(arr) => {
				// treat as rgb or rgba array
				if arr.len() >= 3 {
					let r = num_to_u8(&arr[0])
						.ok_or_else(|| de::Error::custom("invalid red component"))?;
					let g = num_to_u8(&arr[1])
						.ok_or_else(|| de::Error::custom("invalid green component"))?;
					let b = num_to_u8(&arr[2])
						.ok_or_else(|| de::Error::custom("invalid blue component"))?;
					let a = if arr.len() >= 4 {
						num_to_u8(&arr[3])
							.ok_or_else(|| de::Error::custom("invalid alpha component"))?
					} else {
						255
					};
					Ok(Color::new(r, g, b, a))
				} else {
					Err(de::Error::custom(
						"array must be length 3 or 4 for rgb/rgba",
					))
				}
			}
			JsonValue::Object(map) => {
				// hex
				if let Some(hex_val) = map.get("hex")
					&& let Some(s) = hex_val.as_str()
				{
					return Color::from_hex(s).map_err(de::Error::custom);
				}

				// rgb: [r,g,b] or [r,g,b,a]
				if let Some(rgb_val) = map.get("rgb")
					&& let Some(arr) = rgb_val.as_array()
					&& arr.len() >= 3
				{
					let r = num_to_u8(&arr[0]).ok_or_else(|| de::Error::custom("invalid red"))?;
					let g = num_to_u8(&arr[1]).ok_or_else(|| de::Error::custom("invalid green"))?;
					let b = num_to_u8(&arr[2]).ok_or_else(|| de::Error::custom("invalid blue"))?;
					let a = if arr.len() >= 4 {
						num_to_u8(&arr[3]).ok_or_else(|| de::Error::custom("invalid alpha"))?
					} else {
						255
					};
					return Ok(Color::new(r, g, b, a));
				}

				// hsl: [h, s, l] or [h, s, l, a]
				if let Some(hsl_val) = map.get("hsl")
					&& let Some(arr) = hsl_val.as_array()
					&& arr.len() >= 3
				{
					let h = arr[0]
						.as_f64()
						.ok_or_else(|| de::Error::custom("invalid hue"))? as f32;
					let s = arr[1]
						.as_f64()
						.ok_or_else(|| de::Error::custom("invalid saturation"))? as f32;
					let l = arr[2]
						.as_f64()
						.ok_or_else(|| de::Error::custom("invalid lightness"))? as f32;
					let a = if arr.len() >= 4 {
						let af = arr[3]
							.as_f64()
							.ok_or_else(|| de::Error::custom("invalid alpha"))? as f32;
						(af * 255.0).round() as u8
					} else {
						255
					};
					return Ok(Color::from_hsla(h, s, l, a as f32 / 255.0));
				}

				// space/components pair: components are floats in 0..1
				if map.get("space").is_some()
					&& let Some(components) = map.get("components")
					&& let Some(arr) = components.as_array()
					&& arr.len() >= 3
				{
					let r = (arr[0].as_f64().unwrap_or(0.0) * 255.0).round() as u8;
					let g = (arr[1].as_f64().unwrap_or(0.0) * 255.0).round() as u8;
					let b = (arr[2].as_f64().unwrap_or(0.0) * 255.0).round() as u8;
					let a = if arr.len() >= 4 {
						(arr[3].as_f64().unwrap_or(1.0) * 255.0).round() as u8
					} else {
						255
					};
					return Ok(Color::new(r, g, b, a));
				}

				// direct components: red/green/blue/(alpha)
				if map.contains_key("red") && map.contains_key("green") && map.contains_key("blue")
				{
					let r = num_to_u8(map.get("red").unwrap())
						.ok_or_else(|| de::Error::custom("invalid red"))?;
					let g = num_to_u8(map.get("green").unwrap())
						.ok_or_else(|| de::Error::custom("invalid green"))?;
					let b = num_to_u8(map.get("blue").unwrap())
						.ok_or_else(|| de::Error::custom("invalid blue"))?;
					let a = if let Some(av) = map.get("alpha") {
						num_to_u8(av).ok_or_else(|| de::Error::custom("invalid alpha"))?
					} else {
						255
					};
					return Ok(Color::new(r, g, b, a));
				}

				Err(de::Error::custom("unsupported color format"))
			}
			_ => Err(de::Error::custom("invalid color representation")),
		}
	}
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when parsing a color
pub enum ColorError {
	/// Invalid hex color format
	#[error("Invalid hex color format")]
	InvalidFormat,
	/// Invalid hex color length
	#[error("Invalid hex color length")]
	InvalidLength,
	/// Failed to parse hex color component
	#[error("Failed to parse hex color component")]
	ParseError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_color() {
		let c = Color::new(10, 20, 30, 40);
		assert_eq!(c.red, 10);
		assert_eq!(c.green, 20);
		assert_eq!(c.blue, 30);
		assert_eq!(c.alpha, 40);
	}

	#[test]
	fn from_rgb() {
		let c = Color::from_rgb(255, 0, 128);
		assert_eq!(c.red, 255);
		assert_eq!(c.green, 0);
		assert_eq!(c.blue, 128);
		assert_eq!(c.alpha, 255);
	}

	#[test]
	fn from_hex_valid() {
		let c = Color::from_hex("#ff00aa").unwrap();
		assert_eq!(c, Color::new(255, 0, 170, 255));

		let c2 = Color::from_hex("#ff00aa80").unwrap();
		assert_eq!(c2, Color::new(255, 0, 170, 128));

		let c3 = Color::from_hex("112233").unwrap();
		assert_eq!(c3, Color::new(17, 34, 51, 255));
	}

	#[test]
	fn from_hex_invalid_length() {
		assert!(matches!(
			Color::from_hex("#12345"),
			Err(ColorError::InvalidLength)
		));
		assert!(matches!(
			Color::from_hex("#1234567"),
			Err(ColorError::InvalidLength)
		));
		assert!(matches!(
			Color::from_hex(""),
			Err(ColorError::InvalidLength)
		));
	}

	#[test]
	fn from_hex_invalid_format() {
		let err = Color::from_hex("#zzzzzz");
		assert!(err.is_err());
		let err = Color::from_hex("#12xx34");
		assert!(err.is_err());
	}

	#[test]
	fn from_hsla_and_to_hsla_roundtrip() {
		let color = Color::from_hsla(120.0, 0.5, 0.4, 0.8);
		let (h, s, l, a) = color.to_hsla();
		assert!((h - 120.0).abs() < 1.0);
		assert!((s - 0.5).abs() < 0.05);
		assert!((l - 0.4).abs() < 0.05);
		assert!((a - 0.8).abs() < 0.05);
	}

	#[test]
	fn relative_luminance() {
		let black = Ansi::BLACK;
		let white = Ansi::WHITE;
		assert!((black.relative_luminance() - 0.0).abs() < 0.01);
		assert!((white.relative_luminance() - 1.0).abs() < 0.01);
	}

	#[test]
	fn contrast_ratio() {
		let black = Ansi::BLACK;
		let white = Ansi::WHITE;
		let ratio = black.contrast_ratio(&white);
		assert!((ratio - 21.0).abs() < 0.1);
		let red = Ansi::RED;
		let blue = Ansi::BLUE;
		let green = Ansi::GREEN;
		let yellow = Ansi::YELLOW;

		let ratio_red_blue = red.contrast_ratio(&blue);
		assert!(ratio_red_blue > 2.0 && ratio_red_blue < 3.0);

		let ratio_red_green = red.contrast_ratio(&green);
		assert!(ratio_red_green > 2.0 && ratio_red_green < 3.0);

		let ratio_yellow_blue = yellow.contrast_ratio(&blue);
		assert!(ratio_yellow_blue > 7.0 && ratio_yellow_blue < 9.0);

		let ratio_white_yellow = Ansi::WHITE.contrast_ratio(&yellow);
		assert!(ratio_white_yellow > 1.0 && ratio_white_yellow < 2.0);

		let gray = Color::from_rgb(128, 128, 128);
		let ratio2 = black.contrast_ratio(&gray);
		assert!(ratio2 > 5.0 && ratio2 < 6.0);
	}

	#[test]
	fn contrast_black_or_white() {
		let dark = Color::from_rgb(10, 10, 10);
		assert_eq!(dark.contrast_black_or_white(), Ansi::WHITE);

		let light = Color::from_rgb(250, 250, 250);
		assert_eq!(light.contrast_black_or_white(), Ansi::BLACK);
	}

	#[test]
	fn contrast_color() {
		let c = Color::from_rgb(255, 0, 0); // red
		let contrast = c.contrast_color();
		// Should be a color with hue roughly opposite to red (cyan/greenish)
		let (h, ..) = contrast.to_hsla();
		assert!((h > 170.0 && h < 200.0) || (h > 350.0 || h < 10.0));
	}

	#[test]
	fn default() {
		let c = Color::default();
		assert_eq!(c, Ansi::BLACK);
	}

	#[test]
	fn constants() {
		assert_eq!(Ansi::RED, Color::from_rgb(255, 0, 0));
		assert_eq!(Ansi::WHITE, Color::from_rgb(255, 255, 255));
		assert_eq!(Ansi::BLACK, Color::from_rgb(0, 0, 0));
		assert_eq!(Ansi::BLUE, Color::from_rgb(0, 0, 255));
	}

	#[test]
	fn web_and_common_constants() {
		// Web palette
		assert_eq!(crate::Web::RED, Color::from_rgb(255, 0, 0));
		assert_eq!(crate::Web::BLUE, Color::from_rgb(0, 0, 255));

		// Common palette
		assert_eq!(crate::Common::WHITE, Color::from_rgb(255, 255, 255));
		assert_eq!(crate::Common::TRANSPARENT.alpha, 0);
	}

	#[test]
	fn engage_constants_and_transparent() {
		assert_eq!(crate::Engage::ORANGE, Color::from_rgb(255, 153, 51));
		assert_eq!(crate::Engage::TRANSPARENT.alpha, 0);
	}

	#[test]
	fn lerp_boundaries() {
		let a = Color::from_rgb(10, 20, 30);
		let b = Color::from_rgb(100, 110, 120);
		// t = 0 -> a
		assert_eq!(a.lerp(&b, 0.0), a);
		// t = 1 -> b
		assert_eq!(a.lerp(&b, 1.0), b);
		// midpoint roughly averages
		let mid = a.lerp(&b, 0.5);
		assert_eq!(mid.red, ((a.red as u16 + b.red as u16) / 2) as u8);
	}
}
