//! RGB Color system with support for HSL and utility methods
//!
//! This module provides full support for both RGB and HSL color spaces
//! with conversion between them and various utility methods.
//! RGB is used internally since that is what most displays use.

use serde::{Deserialize, Serialize};

/// Represents an RGB color, provides methods for conversion and manipulation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
		let red = u8::from_str_radix(&hex[0..2], 16)?;
		let green = u8::from_str_radix(&hex[2..4], 16)?;
		let blue = u8::from_str_radix(&hex[4..6], 16)?;
		match hex.len() {
			6 => Ok(Self::from_rgb(red, green, blue)),
			8 => {
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
		let r = self.red as f32 / 255.0;
		let g = self.green as f32 / 255.0;
		let b = self.blue as f32 / 255.0;
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
		if self.contrast_ratio(&Color::WHITE) >= self.contrast_ratio(&Color::BLACK) {
			Color::WHITE
		} else {
			Color::BLACK
		}
	}

	/// Colorize the black or white contrast color with the inverse of this color
	pub fn contrast_color(&self) -> Color {
		let contrast = self.contrast_black_or_white();
		let (h, s, l, a) = self.to_hsla();
		let inverse_hue = (h + 180.0) % 360.0;
		let saturation = if contrast == Color::BLACK { 1.0 - s } else { s };
		Color::from_hsla(inverse_hue, saturation, l, a)
	}

	/// Linearly interpolate between two colors.
	/// `t` should be in [0.0, 1.0], where 0.0 returns self and 1.0 returns other.
	pub fn lerp(&self, other: &Color, t: f32) -> Color {
		let clamp = |v: f32| v.max(0.0).min(1.0);
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

/// Constants for commonly used colors
impl Color {
	/// ANSI black color (0, 0, 0, 255)
	pub const ANSI_BLACK: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// ANSI red color (128, 0, 0, 255)
	pub const ANSI_RED: Color = Color {
		red: 128,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// ANSI green color (0, 128, 0, 255)
	pub const ANSI_GREEN: Color = Color {
		red: 0,
		green: 128,
		blue: 0,
		alpha: 255,
	};
	/// ANSI yellow color (128, 128, 0, 255)
	pub const ANSI_YELLOW: Color = Color {
		red: 128,
		green: 128,
		blue: 0,
		alpha: 255,
	};
	/// ANSI blue color (0, 0, 128, 255)
	pub const ANSI_BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 128,
		alpha: 255,
	};
	/// ANSI magenta color (128, 0, 128, 255)
	pub const ANSI_MAGENTA: Color = Color {
		red: 128,
		green: 0,
		blue: 128,
		alpha: 255,
	};
	/// ANSI cyan color (0, 128, 128, 255)
	pub const ANSI_CYAN: Color = Color {
		red: 0,
		green: 128,
		blue: 128,
		alpha: 255,
	};
	/// ANSI white color (192, 192, 192, 255)
	pub const ANSI_WHITE: Color = Color {
		red: 192,
		green: 192,
		blue: 192,
		alpha: 255,
	};
	/// ANSI bright black color (128, 128, 128, 255)
	pub const ANSI_BRIGHT_BLACK: Color = Color {
		red: 128,
		green: 128,
		blue: 128,
		alpha: 255,
	};
	/// ANSI bright red color (255, 0, 0, 255)
	pub const ANSI_BRIGHT_RED: Color = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// ANSI bright green color (0, 255, 0, 255)
	pub const ANSI_BRIGHT_GREEN: Color = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 255,
	};
	/// ANSI bright yellow color (255, 255, 0, 255)
	pub const ANSI_BRIGHT_YELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 255,
	};
	/// ANSI bright blue color (0, 0, 255, 255)
	pub const ANSI_BRIGHT_BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 255,
	};
	/// ANSI bright magenta color (255, 0, 255, 255)
	pub const ANSI_BRIGHT_MAGENTA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};
	/// ANSI bright cyan color (0, 255, 255, 255)
	pub const ANSI_BRIGHT_CYAN: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};
	/// ANSI bright white color (255, 255, 255, 255)
	pub const ANSI_BRIGHT_WHITE: Color = Color {
		red: 255,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	// Web safe colors (140 support by most browsers)
	/// Web color: black (0, 0, 0, 255)
	pub const BLACK: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// Web color: navy (0, 0, 128, 255)
	pub const NAVY: Color = Color {
		red: 0,
		green: 0,
		blue: 128,
		alpha: 255,
	};
	/// Web color: dark blue (0, 0, 139, 255)
	pub const DARKBLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 139,
		alpha: 255,
	};
	/// Web color: medium blue (0, 0, 205, 255)
	pub const MEDIUMBLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 205,
		alpha: 255,
	};
	/// Web color: blue (0, 0, 255, 255)
	pub const BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 255,
	};
	/// Web color: dark green (0, 100, 0, 255)
	pub const DARKGREEN: Color = Color {
		red: 0,
		green: 100,
		blue: 0,
		alpha: 255,
	};
	/// Web color: green (0, 128, 0, 255)
	pub const GREEN: Color = Color {
		red: 0,
		green: 128,
		blue: 0,
		alpha: 255,
	};
	/// Web color: teal (0, 128, 128, 255)
	pub const TEAL: Color = Color {
		red: 0,
		green: 128,
		blue: 128,
		alpha: 255,
	};
	/// Web color: dark cyan (0, 139, 139, 255)
	pub const DARKCYAN: Color = Color {
		red: 0,
		green: 139,
		blue: 139,
		alpha: 255,
	};
	/// Web color: deep sky blue (0, 191, 255, 255)
	pub const DEEPSKYBLUE: Color = Color {
		red: 0,
		green: 191,
		blue: 255,
		alpha: 255,
	};
	/// Web color: dark turquoise (0, 206, 209, 255)
	pub const DARKTURQUOISE: Color = Color {
		red: 0,
		green: 206,
		blue: 209,
		alpha: 255,
	};
	/// Web color: medium spring green (0, 250, 154, 255)
	pub const MEDIUMSPRINGGREEN: Color = Color {
		red: 0,
		green: 250,
		blue: 154,
		alpha: 255,
	};
	/// Web color: lime (0, 255, 0, 255)
	pub const LIME: Color = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 255,
	};
	/// Web color: spring green (0, 255, 127, 255)
	pub const SPRINGGREEN: Color = Color {
		red: 0,
		green: 255,
		blue: 127,
		alpha: 255,
	};
	/// Web color: aqua (0, 255, 255, 255)
	pub const AQUA: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};
	/// Web color: cyan (0, 255, 255, 255)
	pub const CYAN: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};
	/// Web color: midnight blue (25, 25, 112, 255)
	pub const MIDNIGHTBLUE: Color = Color {
		red: 25,
		green: 25,
		blue: 112,
		alpha: 255,
	};
	/// Web color: dodger blue (30, 144, 255, 255)
	pub const DODGERBLUE: Color = Color {
		red: 30,
		green: 144,
		blue: 255,
		alpha: 255,
	};
	/// Web color: light sea green (32, 178, 170, 255)
	pub const LIGHTSEAGREEN: Color = Color {
		red: 32,
		green: 178,
		blue: 170,
		alpha: 255,
	};
	/// Web color: forest green (34, 139, 34, 255)
	pub const FORESTGREEN: Color = Color {
		red: 34,
		green: 139,
		blue: 34,
		alpha: 255,
	};
	/// Web color: sea green (46, 139, 87, 255)
	pub const SEAGREEN: Color = Color {
		red: 46,
		green: 139,
		blue: 87,
		alpha: 255,
	};
	/// Web color: dark slate gray (47, 79, 79, 255)
	pub const DARKSLATEGRAY: Color = Color {
		red: 47,
		green: 79,
		blue: 79,
		alpha: 255,
	};
	/// Web color: dark slate grey (47, 79, 79, 255)
	pub const DARKSLATEGREY: Color = Color {
		red: 47,
		green: 79,
		blue: 79,
		alpha: 255,
	};
	/// Web color: lime green (50, 205, 50, 255)
	pub const LIMEGREEN: Color = Color {
		red: 50,
		green: 205,
		blue: 50,
		alpha: 255,
	};
	/// Web color: medium sea green (60, 179, 113, 255)
	pub const MEDIUMSEAGREEN: Color = Color {
		red: 60,
		green: 179,
		blue: 113,
		alpha: 255,
	};
	/// Web color: turquoise (64, 224, 208, 255)
	pub const TURQUOISE: Color = Color {
		red: 64,
		green: 224,
		blue: 208,
		alpha: 255,
	};
	/// Web color: royal blue (65, 105, 225, 255)
	pub const ROYALBLUE: Color = Color {
		red: 65,
		green: 105,
		blue: 225,
		alpha: 255,
	};
	/// Web color: steel blue (70, 130, 180, 255)
	pub const STEELBLUE: Color = Color {
		red: 70,
		green: 130,
		blue: 180,
		alpha: 255,
	};
	/// Web color: dark slate blue (72, 61, 139, 255)
	pub const DARKSLATEBLUE: Color = Color {
		red: 72,
		green: 61,
		blue: 139,
		alpha: 255,
	};
	/// Web color: medium turquoise (72, 209, 204, 255)
	pub const MEDIUMTURQUOISE: Color = Color {
		red: 72,
		green: 209,
		blue: 204,
		alpha: 255,
	};
	/// Web color: indigo (75, 0, 130, 255)
	pub const INDIGO: Color = Color {
		red: 75,
		green: 0,
		blue: 130,
		alpha: 255,
	};
	/// Web color: dark olive green (85, 107, 47, 255)
	pub const DARKOLIVEGREEN: Color = Color {
		red: 85,
		green: 107,
		blue: 47,
		alpha: 255,
	};
	/// Web color: cadet blue (95, 158, 160, 255)
	pub const CADETBLUE: Color = Color {
		red: 95,
		green: 158,
		blue: 160,
		alpha: 255,
	};
	/// Web color: cornflower blue (100, 149, 237, 255)
	pub const CORNFLOWERBLUE: Color = Color {
		red: 100,
		green: 149,
		blue: 237,
		alpha: 255,
	};
	/// Web color: rebecca purple (102, 51, 153, 255)
	pub const REBECCAPURPLE: Color = Color {
		red: 102,
		green: 51,
		blue: 153,
		alpha: 255,
	};
	/// Web color: medium aquamarine (102, 205, 170, 255)
	pub const MEDIUMAQUAMARINE: Color = Color {
		red: 102,
		green: 205,
		blue: 170,
		alpha: 255,
	};
	/// Web color: dim gray (105, 105, 105, 255)
	pub const DIMGRAY: Color = Color {
		red: 105,
		green: 105,
		blue: 105,
		alpha: 255,
	};
	/// Web color: dim grey (105, 105, 105, 255)
	pub const DIMGREY: Color = Color {
		red: 105,
		green: 105,
		blue: 105,
		alpha: 255,
	};
	/// Web color: slate blue (106, 90, 205, 255)
	pub const SLATEBLUE: Color = Color {
		red: 106,
		green: 90,
		blue: 205,
		alpha: 255,
	};
	/// Web color: olive drab (107, 142, 35, 255)
	pub const OLIVEDRAB: Color = Color {
		red: 107,
		green: 142,
		blue: 35,
		alpha: 255,
	};
	/// Web color: slate gray (112, 128, 144, 255)
	pub const SLATEGRAY: Color = Color {
		red: 112,
		green: 128,
		blue: 144,
		alpha: 255,
	};
	/// Web color: slate grey (112, 128, 144, 255)
	pub const SLATEGREY: Color = Color {
		red: 112,
		green: 128,
		blue: 144,
		alpha: 255,
	};
	/// Web color: light slate gray (119, 136, 153, 255)
	pub const LIGHTSLATEGRAY: Color = Color {
		red: 119,
		green: 136,
		blue: 153,
		alpha: 255,
	};
	/// Web color: light slate grey (119, 136, 153, 255)
	pub const LIGHTSLATEGREY: Color = Color {
		red: 119,
		green: 136,
		blue: 153,
		alpha: 255,
	};
	/// Web color: medium slate blue (123, 104, 238, 255)
	pub const MEDIUMSLATEBLUE: Color = Color {
		red: 123,
		green: 104,
		blue: 238,
		alpha: 255,
	};
	/// Web color: lawn green (124, 252, 0, 255)
	pub const LAWNGREEN: Color = Color {
		red: 124,
		green: 252,
		blue: 0,
		alpha: 255,
	};
	/// Web color: chartreuse (127, 255, 0, 255)
	pub const CHARTREUSE: Color = Color {
		red: 127,
		green: 255,
		blue: 0,
		alpha: 255,
	};
	/// Web color: aquamarine (127, 255, 212, 255)
	pub const AQUAMARINE: Color = Color {
		red: 127,
		green: 255,
		blue: 212,
		alpha: 255,
	};
	/// Web color: maroon (128, 0, 0, 255)
	pub const MAROON: Color = Color {
		red: 128,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// Web color: purple (128, 0, 128, 255)
	pub const PURPLE: Color = Color {
		red: 128,
		green: 0,
		blue: 128,
		alpha: 255,
	};
	/// Web color: olive (128, 128, 0, 255)
	pub const OLIVE: Color = Color {
		red: 128,
		green: 128,
		blue: 0,
		alpha: 255,
	};
	/// Web color: gray (128, 128, 128, 255)
	pub const GRAY: Color = Color {
		red: 128,
		green: 128,
		blue: 128,
		alpha: 255,
	};
	/// Web color: grey (128, 128, 128, 255)
	pub const GREY: Color = Color {
		red: 128,
		green: 128,
		blue: 128,
		alpha: 255,
	};
	/// Web color: sky blue (135, 206, 235, 255)
	pub const SKYBLUE: Color = Color {
		red: 135,
		green: 206,
		blue: 235,
		alpha: 255,
	};
	/// Web color: light sky blue (135, 206, 250, 255)
	pub const LIGHTSKYBLUE: Color = Color {
		red: 135,
		green: 206,
		blue: 250,
		alpha: 255,
	};
	/// Web color: blue violet (138, 43, 226, 255)
	pub const BLUEVIOLET: Color = Color {
		red: 138,
		green: 43,
		blue: 226,
		alpha: 255,
	};
	/// Web color: dark red (139, 0, 0, 255)
	pub const DARKRED: Color = Color {
		red: 139,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// Web color: dark magenta (139, 0, 139, 255)
	pub const DARKMAGENTA: Color = Color {
		red: 139,
		green: 0,
		blue: 139,
		alpha: 255,
	};
	/// Web color: saddle brown (139, 69, 19, 255)
	pub const SADDLEBROWN: Color = Color {
		red: 139,
		green: 69,
		blue: 19,
		alpha: 255,
	};
	/// Web color: dark sea green (143, 188, 143, 255)
	pub const DARKSEAGREEN: Color = Color {
		red: 143,
		green: 188,
		blue: 143,
		alpha: 255,
	};
	/// Web color: light green (144, 238, 144, 255)
	pub const LIGHTGREEN: Color = Color {
		red: 144,
		green: 238,
		blue: 144,
		alpha: 255,
	};
	/// Web color: medium purple (147, 112, 219, 255)
	pub const MEDIUMPURPLE: Color = Color {
		red: 147,
		green: 112,
		blue: 219,
		alpha: 255,
	};
	/// Web color: dark violet (148, 0, 211, 255)
	pub const DARKVIOLET: Color = Color {
		red: 148,
		green: 0,
		blue: 211,
		alpha: 255,
	};
	/// Web color: pale green (152, 251, 152, 255)
	pub const PALEGREEN: Color = Color {
		red: 152,
		green: 251,
		blue: 152,
		alpha: 255,
	};
	/// Web color: dark orchid (153, 50, 204, 255)
	pub const DARKORCHID: Color = Color {
		red: 153,
		green: 50,
		blue: 204,
		alpha: 255,
	};
	/// Web color: yellow green (154, 205, 50, 255)
	pub const YELLOWGREEN: Color = Color {
		red: 154,
		green: 205,
		blue: 50,
		alpha: 255,
	};
	/// Web color: sienna (160, 82, 45, 255)
	pub const SIENNA: Color = Color {
		red: 160,
		green: 82,
		blue: 45,
		alpha: 255,
	};
	/// Web color: brown (165, 42, 42, 255)
	pub const BROWN: Color = Color {
		red: 165,
		green: 42,
		blue: 42,
		alpha: 255,
	};
	/// Web color: dark gray (169, 169, 169, 255)
	pub const DARKGRAY: Color = Color {
		red: 169,
		green: 169,
		blue: 169,
		alpha: 255,
	};
	/// Web color: dark grey (169, 169, 169, 255)
	pub const DARKGREY: Color = Color {
		red: 169,
		green: 169,
		blue: 169,
		alpha: 255,
	};
	/// Web color: light blue (173, 216, 230, 255)
	pub const LIGHTBLUE: Color = Color {
		red: 173,
		green: 216,
		blue: 230,
		alpha: 255,
	};
	/// Web color: green yellow (173, 255, 47, 255)
	pub const GREENYELLOW: Color = Color {
		red: 173,
		green: 255,
		blue: 47,
		alpha: 255,
	};
	/// Web color: pale turquoise (175, 238, 238, 255)
	pub const PALETURQUOISE: Color = Color {
		red: 175,
		green: 238,
		blue: 238,
		alpha: 255,
	};
	/// Web color: light steel blue (176, 196, 222, 255)
	pub const LIGHTSTEELBLUE: Color = Color {
		red: 176,
		green: 196,
		blue: 222,
		alpha: 255,
	};
	/// Web color: powder blue (176, 224, 230, 255)
	pub const POWDERBLUE: Color = Color {
		red: 176,
		green: 224,
		blue: 230,
		alpha: 255,
	};
	/// Web color: fire brick (178, 34, 34, 255)
	pub const FIREBRICK: Color = Color {
		red: 178,
		green: 34,
		blue: 34,
		alpha: 255,
	};
	/// Web color: dark golden rod (184, 134, 11, 255)
	pub const DARKGOLDENROD: Color = Color {
		red: 184,
		green: 134,
		blue: 11,
		alpha: 255,
	};
	/// Web color: medium orchid (186, 85, 211, 255)
	pub const MEDIUMORCHID: Color = Color {
		red: 186,
		green: 85,
		blue: 211,
		alpha: 255,
	};
	/// Web color: rosy brown (188, 143, 143, 255)
	pub const ROSYBROWN: Color = Color {
		red: 188,
		green: 143,
		blue: 143,
		alpha: 255,
	};
	/// Web color: dark khaki (189, 183, 107, 255)
	pub const DARKKHAKI: Color = Color {
		red: 189,
		green: 183,
		blue: 107,
		alpha: 255,
	};
	/// Web color: silver (192, 192, 192, 255)
	pub const SILVER: Color = Color {
		red: 192,
		green: 192,
		blue: 192,
		alpha: 255,
	};
	/// Web color: medium violet red (199, 21, 133, 255)
	pub const MEDIUMVIOLETRED: Color = Color {
		red: 199,
		green: 21,
		blue: 133,
		alpha: 255,
	};
	/// Web color: indian red (205, 92, 92, 255)
	pub const INDIANRED: Color = Color {
		red: 205,
		green: 92,
		blue: 92,
		alpha: 255,
	};
	/// Web color: peru (205, 133, 63, 255)
	pub const PERU: Color = Color {
		red: 205,
		green: 133,
		blue: 63,
		alpha: 255,
	};
	/// Web color: chocolate (210, 105, 30, 255)
	pub const CHOCOLATE: Color = Color {
		red: 210,
		green: 105,
		blue: 30,
		alpha: 255,
	};
	/// Web color: tan (210, 180, 140, 255)
	pub const TAN: Color = Color {
		red: 210,
		green: 180,
		blue: 140,
		alpha: 255,
	};
	/// Web color: light gray (211, 211, 211, 255)
	pub const LIGHTGRAY: Color = Color {
		red: 211,
		green: 211,
		blue: 211,
		alpha: 255,
	};
	/// Web color: light grey (211, 211, 211, 255)
	pub const LIGHTGREY: Color = Color {
		red: 211,
		green: 211,
		blue: 211,
		alpha: 255,
	};
	/// Web color: thistle (216, 191, 216, 255)
	pub const THISTLE: Color = Color {
		red: 216,
		green: 191,
		blue: 216,
		alpha: 255,
	};
	/// Web color: orchid (218, 112, 214, 255)
	pub const ORCHID: Color = Color {
		red: 218,
		green: 112,
		blue: 214,
		alpha: 255,
	};
	/// Web color: golden rod (218, 165, 32, 255)
	pub const GOLDENROD: Color = Color {
		red: 218,
		green: 165,
		blue: 32,
		alpha: 255,
	};
	/// Web color: pale violet red (219, 112, 147, 255)
	pub const PALEVIOLETRED: Color = Color {
		red: 219,
		green: 112,
		blue: 147,
		alpha: 255,
	};
	/// Web color: crimson (220, 20, 60, 255)
	pub const CRIMSON: Color = Color {
		red: 220,
		green: 20,
		blue: 60,
		alpha: 255,
	};
	/// Web color: gainsboro (220, 220, 220, 255)
	pub const GAINSBORO: Color = Color {
		red: 220,
		green: 220,
		blue: 220,
		alpha: 255,
	};
	/// Web color: plum (221, 160, 221, 255)
	pub const PLUM: Color = Color {
		red: 221,
		green: 160,
		blue: 221,
		alpha: 255,
	};
	/// Web color: burlywood (222, 184, 135, 255)
	pub const BURLYWOOD: Color = Color {
		red: 222,
		green: 184,
		blue: 135,
		alpha: 255,
	};
	/// Web color: light cyan (224, 255, 255, 255)
	pub const LIGHTCYAN: Color = Color {
		red: 224,
		green: 255,
		blue: 255,
		alpha: 255,
	};
	/// Web color: lavender (230, 230, 250, 255)
	pub const LAVENDER: Color = Color {
		red: 230,
		green: 230,
		blue: 250,
		alpha: 255,
	};
	/// Web color: dark salmon (233, 150, 122, 255)
	pub const DARKSALMON: Color = Color {
		red: 233,
		green: 150,
		blue: 122,
		alpha: 255,
	};
	/// Web color: violet (238, 130, 238, 255)
	pub const VIOLET: Color = Color {
		red: 238,
		green: 130,
		blue: 238,
		alpha: 255,
	};
	/// Web color: pale golden rod (238, 232, 170, 255)
	pub const PALEGOLDENROD: Color = Color {
		red: 238,
		green: 232,
		blue: 170,
		alpha: 255,
	};
	/// Web color: light coral (240, 128, 128, 255)
	pub const LIGHTCORAL: Color = Color {
		red: 240,
		green: 128,
		blue: 128,
		alpha: 255,
	};
	/// Web color: khaki (240, 230, 140, 255)
	pub const KHAKI: Color = Color {
		red: 240,
		green: 230,
		blue: 140,
		alpha: 255,
	};
	/// Web color: alice blue (240, 248, 255, 255)
	pub const ALICEBLUE: Color = Color {
		red: 240,
		green: 248,
		blue: 255,
		alpha: 255,
	};
	/// Web color: honey dew (240, 255, 240, 255)
	pub const HONEYDEW: Color = Color {
		red: 240,
		green: 255,
		blue: 240,
		alpha: 255,
	};
	/// Web color: azure (240, 255, 255, 255)
	pub const AZURE: Color = Color {
		red: 240,
		green: 255,
		blue: 255,
		alpha: 255,
	};
	/// Web color: sandy brown (244, 164, 96, 255)
	pub const SANDYBROWN: Color = Color {
		red: 244,
		green: 164,
		blue: 96,
		alpha: 255,
	};
	/// Web color: wheat (245, 222, 179, 255)
	pub const WHEAT: Color = Color {
		red: 245,
		green: 222,
		blue: 179,
		alpha: 255,
	};
	/// Web color: beige (245, 245, 220, 255)
	pub const BEIGE: Color = Color {
		red: 245,
		green: 245,
		blue: 220,
		alpha: 255,
	};
	/// Web color: white smoke (245, 245, 245, 255)
	pub const WHITESMOKE: Color = Color {
		red: 245,
		green: 245,
		blue: 245,
		alpha: 255,
	};
	/// Web color: mint cream (245, 255, 250, 255)
	pub const MINTCREAM: Color = Color {
		red: 245,
		green: 255,
		blue: 250,
		alpha: 255,
	};
	/// Web color: ghost white (248, 248, 255, 255)
	pub const GHOSTWHITE: Color = Color {
		red: 248,
		green: 248,
		blue: 255,
		alpha: 255,
	};
	/// Web color: salmon (250, 128, 114, 255)
	pub const SALMON: Color = Color {
		red: 250,
		green: 128,
		blue: 114,
		alpha: 255,
	};
	/// Web color: antique white (250, 235, 215, 255)
	pub const ANTIQUEWHITE: Color = Color {
		red: 250,
		green: 235,
		blue: 215,
		alpha: 255,
	};
	/// Web color: linen (250, 240, 230, 255)
	pub const LINEN: Color = Color {
		red: 250,
		green: 240,
		blue: 230,
		alpha: 255,
	};
	/// Web color: light golden rod yellow (250, 250, 210, 255)
	pub const LIGHTGOLDENRODYELLOW: Color = Color {
		red: 250,
		green: 250,
		blue: 210,
		alpha: 255,
	};
	/// Web color: old lace (253, 245, 230, 255)
	pub const OLDLACE: Color = Color {
		red: 253,
		green: 245,
		blue: 230,
		alpha: 255,
	};
	/// Web color: red (255, 0, 0, 255)
	pub const RED: Color = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 255,
	};
	/// Web color: fuchsia (255, 0, 255, 255)
	pub const FUCHSIA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};
	/// Web color: magenta (255, 0, 255, 255)
	pub const MAGENTA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};
	/// Web color: deep pink (255, 20, 147, 255)
	pub const DEEPPINK: Color = Color {
		red: 255,
		green: 20,
		blue: 147,
		alpha: 255,
	};
	/// Web color: orange red (255, 69, 0, 255)
	pub const ORANGERED: Color = Color {
		red: 255,
		green: 69,
		blue: 0,
		alpha: 255,
	};
	/// Web color: tomato (255, 99, 71, 255)
	pub const TOMATO: Color = Color {
		red: 255,
		green: 99,
		blue: 71,
		alpha: 255,
	};
	/// Web color: hot pink (255, 105, 180, 255)
	pub const HOTPINK: Color = Color {
		red: 255,
		green: 105,
		blue: 180,
		alpha: 255,
	};
	/// Web color: coral (255, 127, 80, 255)
	pub const CORAL: Color = Color {
		red: 255,
		green: 127,
		blue: 80,
		alpha: 255,
	};
	/// Web color: dark orange (255, 140, 0, 255)
	pub const DARKORANGE: Color = Color {
		red: 255,
		green: 140,
		blue: 0,
		alpha: 255,
	};
	/// Web color: light salmon (255, 160, 122, 255)
	pub const LIGHTSALMON: Color = Color {
		red: 255,
		green: 160,
		blue: 122,
		alpha: 255,
	};
	/// Web color: orange (255, 165, 0, 255)
	pub const ORANGE: Color = Color {
		red: 255,
		green: 165,
		blue: 0,
		alpha: 255,
	};
	/// Web color: light pink (255, 182, 193, 255)
	pub const LIGHTPINK: Color = Color {
		red: 255,
		green: 182,
		blue: 193,
		alpha: 255,
	};
	/// Web color: pink (255, 192, 203, 255)
	pub const PINK: Color = Color {
		red: 255,
		green: 192,
		blue: 203,
		alpha: 255,
	};
	/// Web color: gold (255, 215, 0, 255)
	pub const GOLD: Color = Color {
		red: 255,
		green: 215,
		blue: 0,
		alpha: 255,
	};
	/// Web color: peach puff (255, 218, 185, 255)
	pub const PEACHPUFF: Color = Color {
		red: 255,
		green: 218,
		blue: 185,
		alpha: 255,
	};
	/// Web color: navajo white (255, 222, 173, 255)
	pub const NAVAJOWHITE: Color = Color {
		red: 255,
		green: 222,
		blue: 173,
		alpha: 255,
	};
	/// Web color: moccasin (255, 228, 181, 255)
	pub const MOCCASIN: Color = Color {
		red: 255,
		green: 228,
		blue: 181,
		alpha: 255,
	};
	/// Web color: bisque (255, 228, 196, 255)
	pub const BISQUE: Color = Color {
		red: 255,
		green: 228,
		blue: 196,
		alpha: 255,
	};
	/// Web color: misty rose (255, 228, 225, 255)
	pub const MISTYROSE: Color = Color {
		red: 255,
		green: 228,
		blue: 225,
		alpha: 255,
	};
	/// Web color: blanched almond (255, 235, 205, 255)
	pub const BLANCHEDALMOND: Color = Color {
		red: 255,
		green: 235,
		blue: 205,
		alpha: 255,
	};
	/// Web color: papaya whip (255, 239, 213, 255)
	pub const PAPAYAWHIP: Color = Color {
		red: 255,
		green: 239,
		blue: 213,
		alpha: 255,
	};
	/// Web color: lavender blush (255, 240, 245, 255)
	pub const LAVENDERBLUSH: Color = Color {
		red: 255,
		green: 240,
		blue: 245,
		alpha: 255,
	};
	/// Web color: seashell (255, 245, 238, 255)
	pub const SEASHELL: Color = Color {
		red: 255,
		green: 245,
		blue: 238,
		alpha: 255,
	};
	/// Web color: corn silk (255, 248, 220, 255)
	pub const CORNSILK: Color = Color {
		red: 255,
		green: 248,
		blue: 220,
		alpha: 255,
	};
	/// Web color: lemon chiffon (255, 250, 205, 255)
	pub const LEMONCHIFFON: Color = Color {
		red: 255,
		green: 250,
		blue: 205,
		alpha: 255,
	};
	/// Web color: floral white (255, 250, 240, 255)
	pub const FLORALWHITE: Color = Color {
		red: 255,
		green: 250,
		blue: 240,
		alpha: 255,
	};
	/// Web color: snow (255, 250, 250, 255)
	pub const SNOW: Color = Color {
		red: 255,
		green: 250,
		blue: 250,
		alpha: 255,
	};
	/// Web color: yellow (255, 255, 0, 255)
	pub const YELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 255,
	};
	/// Web color: light yellow (255, 255, 224, 255)
	pub const LIGHTYELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 224,
		alpha: 255,
	};
	/// Web color: ivory (255, 255, 240, 255)
	pub const IVORY: Color = Color {
		red: 255,
		green: 255,
		blue: 240,
		alpha: 255,
	};
	/// Web color: white (255, 255, 255, 255)
	pub const WHITE: Color = Color {
		red: 255,
		green: 255,
		blue: 255,
		alpha: 255,
	};
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
		let black = Color::BLACK;
		let white = Color::WHITE;
		assert!((black.relative_luminance() - 0.0).abs() < 0.01);
		assert!((white.relative_luminance() - 1.0).abs() < 0.01);
	}

	#[test]
	fn contrast_ratio() {
		let black = Color::BLACK;
		let white = Color::WHITE;
		let ratio = black.contrast_ratio(&white);
		assert!((ratio - 21.0).abs() < 0.1);
		let red = Color::RED;
		let blue = Color::BLUE;
		let green = Color::GREEN;
		let yellow = Color::YELLOW;

		let ratio_red_blue = red.contrast_ratio(&blue);
		assert!(ratio_red_blue > 2.0 && ratio_red_blue < 3.0);

		let ratio_red_green = red.contrast_ratio(&green);
		assert!(ratio_red_green > 2.0 && ratio_red_green < 3.0);

		let ratio_yellow_blue = yellow.contrast_ratio(&blue);
		assert!(ratio_yellow_blue > 7.0 && ratio_yellow_blue < 9.0);

		let ratio_white_yellow = Color::WHITE.contrast_ratio(&yellow);
		assert!(ratio_white_yellow > 1.0 && ratio_white_yellow < 2.0);

		let gray = Color::from_rgb(128, 128, 128);
		let ratio2 = black.contrast_ratio(&gray);
		assert!(ratio2 > 5.0 && ratio2 < 6.0);
	}

	#[test]
	fn contrast_black_or_white() {
		let dark = Color::from_rgb(10, 10, 10);
		assert_eq!(dark.contrast_black_or_white(), Color::WHITE);

		let light = Color::from_rgb(250, 250, 250);
		assert_eq!(light.contrast_black_or_white(), Color::BLACK);
	}

	#[test]
	fn contrast_color() {
		let c = Color::from_rgb(255, 0, 0); // red
		let contrast = c.contrast_color();
		// Should be a color with hue roughly opposite to red (cyan/greenish)
		let (h, _, _, _) = contrast.to_hsla();
		assert!((h > 170.0 && h < 200.0) || (h > 350.0 || h < 10.0));
	}

	#[test]
	fn default() {
		let c = Color::default();
		assert_eq!(c, Color::BLACK);
	}

	#[test]
	fn constants() {
		assert_eq!(Color::RED, Color::from_rgb(255, 0, 0));
		assert_eq!(Color::WHITE, Color::from_rgb(255, 255, 255));
		assert_eq!(Color::BLACK, Color::from_rgb(0, 0, 0));
		assert_eq!(Color::BLUE, Color::from_rgb(0, 0, 255));
	}
}
