use crate::Color;

/// ANSI palette group. Use as `engage_ux::color::Ansi::RED`.
pub struct Ansi;

impl Ansi {
	/// ANSI black (0, 0, 0, 255)
	pub const BLACK: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// ANSI red (128, 0, 0, 255)
	pub const RED: Color = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// ANSI green (0, 128, 0, 255)
	pub const GREEN: Color = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// ANSI yellow (128, 128, 0, 255)
	pub const YELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// ANSI blue (0, 0, 128, 255)
	pub const BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// ANSI magenta (128, 0, 128, 255)
	pub const MAGENTA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// ANSI cyan (0, 128, 128, 255)
	pub const CYAN: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// ANSI white (192, 192, 192, 255)
	pub const WHITE: Color = Color {
		red: 255,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// ANSI bright black / gray (128, 128, 128, 255)
	pub const BRIGHT_BLACK: Color = Color {
		red: 128,
		green: 128,
		blue: 128,
		alpha: 255,
	};

	/// ANSI bright red (255, 0, 0, 255)
	pub const BRIGHT_RED: Color = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// ANSI bright green (0, 255, 0, 255)
	pub const BRIGHT_GREEN: Color = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// ANSI bright yellow (255, 255, 0, 255)
	pub const BRIGHT_YELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// ANSI bright blue (0, 0, 255, 255)
	pub const BRIGHT_BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// ANSI bright magenta (255, 0, 255, 255)
	pub const BRIGHT_MAGENTA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// ANSI bright cyan (0, 255, 255, 255)
	pub const BRIGHT_CYAN: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// ANSI bright white (255, 255, 255, 255)
	pub const BRIGHT_WHITE: Color = Color {
		red: 255,
		green: 255,
		blue: 255,
		alpha: 255,
	};
}
