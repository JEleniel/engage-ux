use crate::Color;

/// Common palette: all "pure" mixes where each RGB component is one of
/// {0, 128, 255}. Also includes a TRANSPARENT entry.
///
/// Use as `engage_ux::color::Common::RGB_255_128_0` or
/// `engage_ux::color::Common::TRANSPARENT`.
pub struct Common;

impl Common {
	/// Common black (0, 0, 0, 255)
	pub const BLACK: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// Common navy (0, 0, 128, 255)
	pub const NAVY: Color = Color {
		red: 0,
		green: 0,
		blue: 128,
		alpha: 255,
	};

	/// Common blue (0, 0, 255, 255)
	pub const BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// Common green (0, 128, 0, 255)
	pub const GREEN: Color = Color {
		red: 0,
		green: 128,
		blue: 0,
		alpha: 255,
	};

	/// Common teal (0, 128, 128, 255)
	pub const TEAL: Color = Color {
		red: 0,
		green: 128,
		blue: 128,
		alpha: 255,
	};

	/// Common sky blue (0, 128, 255, 255)
	pub const SKYBLUE: Color = Color {
		red: 0,
		green: 128,
		blue: 255,
		alpha: 255,
	};

	/// Common lime (0, 255, 0, 255)
	pub const LIME: Color = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// Common spring green (0, 255, 128, 255)
	pub const SPRINGGREEN: Color = Color {
		red: 0,
		green: 255,
		blue: 128,
		alpha: 255,
	};

	/// Common aqua (0, 255, 255, 255)
	pub const AQUA: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Common maroon (128, 0, 0, 255)
	pub const MAROON: Color = Color {
		red: 128,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// Common purple (128, 0, 128, 255)
	pub const PURPLE: Color = Color {
		red: 128,
		green: 0,
		blue: 128,
		alpha: 255,
	};

	/// Common violet (128, 0, 255, 255)
	pub const VIOLET: Color = Color {
		red: 128,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// Common olive (128, 128, 0, 255)
	pub const OLIVE: Color = Color {
		red: 128,
		green: 128,
		blue: 0,
		alpha: 255,
	};

	/// Common gray (128, 128, 128, 255)
	pub const GRAY: Color = Color {
		red: 128,
		green: 128,
		blue: 128,
		alpha: 255,
	};

	/// Common light blue (128, 128, 255, 255)
	pub const LIGHTBLUE: Color = Color {
		red: 128,
		green: 128,
		blue: 255,
		alpha: 255,
	};

	/// Common chartreuse (128, 255, 0, 255)
	pub const CHARTREUSE: Color = Color {
		red: 128,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// Common light green (128, 255, 128, 255)
	pub const LIGHT_GREEN: Color = Color {
		red: 128,
		green: 255,
		blue: 128,
		alpha: 255,
	};

	/// Common pale turquoise (128, 255, 255, 255)
	pub const PALETURQUOISE: Color = Color {
		red: 128,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Common red (255, 0, 0, 255)
	pub const RED: Color = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// Common hot pink (255, 0, 128, 255)
	pub const HOTPINK: Color = Color {
		red: 255,
		green: 0,
		blue: 128,
		alpha: 255,
	};

	/// Common fuchsia (255, 0, 255, 255)
	pub const FUCHSIA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// Common orange (255, 128, 0, 255)
	pub const ORANGE: Color = Color {
		red: 255,
		green: 128,
		blue: 0,
		alpha: 255,
	};

	/// Common salmon (255, 128, 128, 255)
	pub const SALMON: Color = Color {
		red: 255,
		green: 128,
		blue: 128,
		alpha: 255,
	};

	/// Common pink (255, 128, 255, 255)
	pub const PINK: Color = Color {
		red: 255,
		green: 128,
		blue: 255,
		alpha: 255,
	};

	/// Common yellow (255, 255, 0, 255)
	pub const YELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// Common light yellow (255, 255, 128, 255)
	pub const LIGHTYELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 128,
		alpha: 255,
	};

	/// Common white (255, 255, 255, 255)
	pub const WHITE: Color = Color {
		red: 255,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Transparent (alpha = 0)
	pub const TRANSPARENT: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 0,
	};
}
