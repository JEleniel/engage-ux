use crate::Color;

/// 'Engage' palette — high-contrast interface accent colors inspired by
/// futuristic control-panel palettes. Names use common color words so
/// they remain descriptive and avoid any potentially protected naming.
///
/// Use as `engage_ux::color::Engage::ORANGE`.
pub struct Engage;

impl Engage {
	/// Interface accent orange (255, 153, 51, 255)
	pub const ORANGE: Color = Color {
		red: 255,
		green: 153,
		blue: 51,
		alpha: 255,
	};

	/// Accent gold / mustard (255, 204, 0, 255)
	pub const GOLD: Color = Color {
		red: 255,
		green: 204,
		blue: 0,
		alpha: 255,
	};

	/// Deep blue accent (0, 102, 204, 255)
	pub const BLUE: Color = Color {
		red: 0,
		green: 102,
		blue: 204,
		alpha: 255,
	};

	/// Cyan / aqua accent (0, 204, 204, 255)
	pub const CYAN: Color = Color {
		red: 0,
		green: 204,
		blue: 204,
		alpha: 255,
	};

	/// Teal / sea accent (0, 153, 153, 255)
	pub const TEAL: Color = Color {
		red: 0,
		green: 153,
		blue: 153,
		alpha: 255,
	};

	/// Lavender / purple accent (153, 51, 204, 255)
	pub const PURPLE: Color = Color {
		red: 153,
		green: 51,
		blue: 204,
		alpha: 255,
	};

	/// Magenta / pink accent (255, 102, 178, 255)
	pub const MAGENTA: Color = Color {
		red: 255,
		green: 102,
		blue: 178,
		alpha: 255,
	};

	/// Muted beige / tan (204, 153, 102, 255)
	pub const TAN: Color = Color {
		red: 204,
		green: 153,
		blue: 102,
		alpha: 255,
	};

	/// Olive / lime accent (153, 204, 0, 255)
	pub const LIME: Color = Color {
		red: 153,
		green: 204,
		blue: 0,
		alpha: 255,
	};

	/// Dark charcoal / interface background (48, 48, 48, 255)
	pub const CHARCOAL: Color = Color {
		red: 48,
		green: 48,
		blue: 48,
		alpha: 255,
	};

	/// Standard black (0, 0, 0, 255)
	pub const BLACK: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// Standard white (255, 255, 255, 255)
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
