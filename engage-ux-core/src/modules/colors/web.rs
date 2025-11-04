use crate::Color;

/// Web (CSS) named color group. Use as `engage_ux::color::Web::RED`.
pub struct Web;

impl Web {
	/// Web black (0, 0, 0, 255)
	pub const BLACK: Color = Color {
		red: 0,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// Web navy (0, 0, 128, 255)
	pub const NAVY: Color = Color {
		red: 0,
		green: 0,
		blue: 128,
		alpha: 255,
	};

	/// Web dark blue (0, 0, 139, 255)
	pub const DARKBLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 139,
		alpha: 255,
	};

	/// Web medium blue (0, 0, 205, 255)
	pub const MEDIUMBLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 205,
		alpha: 255,
	};

	/// Web blue (0, 0, 255, 255)
	pub const BLUE: Color = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// Web dark green (0, 100, 0, 255)
	pub const DARKGREEN: Color = Color {
		red: 0,
		green: 100,
		blue: 0,
		alpha: 255,
	};

	/// Web green (0, 128, 0, 255)
	pub const GREEN: Color = Color {
		red: 0,
		green: 128,
		blue: 0,
		alpha: 255,
	};

	/// Web teal (0, 128, 128, 255)
	pub const TEAL: Color = Color {
		red: 0,
		green: 128,
		blue: 128,
		alpha: 255,
	};

	/// Web dark cyan (0, 139, 139, 255)
	pub const DARKCYAN: Color = Color {
		red: 0,
		green: 139,
		blue: 139,
		alpha: 255,
	};

	/// Web deepskyblue (0, 191, 255, 255)
	pub const DEEPSKYBLUE: Color = Color {
		red: 0,
		green: 191,
		blue: 255,
		alpha: 255,
	};

	/// Web dark turquoise (0, 206, 209, 255)
	pub const DARKTURQUOISE: Color = Color {
		red: 0,
		green: 206,
		blue: 209,
		alpha: 255,
	};

	/// Web medium spring green (0, 250, 154, 255)
	pub const MEDIUMSPRINGGREEN: Color = Color {
		red: 0,
		green: 250,
		blue: 154,
		alpha: 255,
	};

	/// Web lime (0, 255, 0, 255)
	pub const LIME: Color = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// Web spring green (0, 255, 127, 255)
	pub const SPRINGGREEN: Color = Color {
		red: 0,
		green: 255,
		blue: 127,
		alpha: 255,
	};

	/// Web aqua (0, 255, 255, 255)
	pub const AQUA: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Web cyan (0, 255, 255, 255)
	pub const CYAN: Color = Color {
		red: 0,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Web midnight blue (25, 25, 112, 255)
	pub const MIDNIGHTBLUE: Color = Color {
		red: 25,
		green: 25,
		blue: 112,
		alpha: 255,
	};

	/// Web dodger blue (30, 144, 255, 255)
	pub const DODGERBLUE: Color = Color {
		red: 30,
		green: 144,
		blue: 255,
		alpha: 255,
	};

	/// Web light sea green (32, 178, 170, 255)
	pub const LIGHTSEAGREEN: Color = Color {
		red: 32,
		green: 178,
		blue: 170,
		alpha: 255,
	};

	/// Web forest green (34, 139, 34, 255)
	pub const FORESTGREEN: Color = Color {
		red: 34,
		green: 139,
		blue: 34,
		alpha: 255,
	};

	/// Web sea green (46, 139, 87, 255)
	pub const SEAGREEN: Color = Color {
		red: 46,
		green: 139,
		blue: 87,
		alpha: 255,
	};

	/// Web dark slate gray (47, 79, 79, 255)
	pub const DARKSLATEGRAY: Color = Color {
		red: 47,
		green: 79,
		blue: 79,
		alpha: 255,
	};

	/// Web lime green (50, 205, 50, 255)
	pub const LIMEGREEN: Color = Color {
		red: 50,
		green: 205,
		blue: 50,
		alpha: 255,
	};

	/// Web medium sea green (60, 179, 113, 255)
	pub const MEDIUMSEAGREEN: Color = Color {
		red: 60,
		green: 179,
		blue: 113,
		alpha: 255,
	};

	/// Web turquoise (64, 224, 208, 255)
	pub const TURQUOISE: Color = Color {
		red: 64,
		green: 224,
		blue: 208,
		alpha: 255,
	};

	/// Web royal blue (65, 105, 225, 255)
	pub const ROYALBLUE: Color = Color {
		red: 65,
		green: 105,
		blue: 225,
		alpha: 255,
	};

	/// Web steel blue (70, 130, 180, 255)
	pub const STEELBLUE: Color = Color {
		red: 70,
		green: 130,
		blue: 180,
		alpha: 255,
	};

	/// Web indigo (75, 0, 130, 255)
	pub const INDIGO: Color = Color {
		red: 75,
		green: 0,
		blue: 130,
		alpha: 255,
	};

	/// Web olive drab (107, 142, 35, 255)
	pub const OLIVEDRAB: Color = Color {
		red: 107,
		green: 142,
		blue: 35,
		alpha: 255,
	};

	/// Web slate gray (112, 128, 144, 255)
	pub const SLATEGRAY: Color = Color {
		red: 112,
		green: 128,
		blue: 144,
		alpha: 255,
	};

	/// Web dark violet (148, 0, 211, 255)
	pub const DARKVIOLET: Color = Color {
		red: 148,
		green: 0,
		blue: 211,
		alpha: 255,
	};

	/// Web pale green (152, 251, 152, 255)
	pub const PALEGREEN: Color = Color {
		red: 152,
		green: 251,
		blue: 152,
		alpha: 255,
	};

	/// Web saddle brown (139, 69, 19, 255)
	pub const SADDLEBROWN: Color = Color {
		red: 139,
		green: 69,
		blue: 19,
		alpha: 255,
	};

	/// Web dark sea green (143, 188, 143, 255)
	pub const DARKSEAGREEN: Color = Color {
		red: 143,
		green: 188,
		blue: 143,
		alpha: 255,
	};

	/// Web light green (144, 238, 144, 255)
	pub const LIGHTGREEN: Color = Color {
		red: 144,
		green: 238,
		blue: 144,
		alpha: 255,
	};

	/// Web medium purple (147, 112, 219, 255)
	pub const MEDIUMPURPLE: Color = Color {
		red: 147,
		green: 112,
		blue: 219,
		alpha: 255,
	};

	/// Web sienna (160, 82, 45, 255)
	pub const SIENNA: Color = Color {
		red: 160,
		green: 82,
		blue: 45,
		alpha: 255,
	};

	/// Web brown (165, 42, 42, 255)
	pub const BROWN: Color = Color {
		red: 165,
		green: 42,
		blue: 42,
		alpha: 255,
	};

	/// Web dark gray (169, 169, 169, 255)
	pub const DARKGRAY: Color = Color {
		red: 169,
		green: 169,
		blue: 169,
		alpha: 255,
	};

	/// Web light blue (173, 216, 230, 255)
	pub const LIGHTBLUE: Color = Color {
		red: 173,
		green: 216,
		blue: 230,
		alpha: 255,
	};

	/// Web pale turquoise (175, 238, 238, 255)
	pub const PALETURQUOISE: Color = Color {
		red: 175,
		green: 238,
		blue: 238,
		alpha: 255,
	};

	/// Web light cyan (224, 255, 255, 255)
	pub const LIGHTCYAN: Color = Color {
		red: 224,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Web lavender (230, 230, 250, 255)
	pub const LAVENDER: Color = Color {
		red: 230,
		green: 230,
		blue: 250,
		alpha: 255,
	};

	/// Web red (255, 0, 0, 255)
	pub const RED: Color = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 255,
	};

	/// Web fuchsia (255, 0, 255, 255)
	pub const FUCHSIA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// Web magenta (255, 0, 255, 255)
	pub const MAGENTA: Color = Color {
		red: 255,
		green: 0,
		blue: 255,
		alpha: 255,
	};

	/// Web deep pink (255, 20, 147, 255)
	pub const DEEPPINK: Color = Color {
		red: 255,
		green: 20,
		blue: 147,
		alpha: 255,
	};

	/// Web orange red (255, 69, 0, 255)
	pub const ORANGERED: Color = Color {
		red: 255,
		green: 69,
		blue: 0,
		alpha: 255,
	};

	/// Web tomato (255, 99, 71, 255)
	pub const TOMATO: Color = Color {
		red: 255,
		green: 99,
		blue: 71,
		alpha: 255,
	};

	/// Web hot pink (255, 105, 180, 255)
	pub const HOTPINK: Color = Color {
		red: 255,
		green: 105,
		blue: 180,
		alpha: 255,
	};

	/// Web coral (255, 127, 80, 255)
	pub const CORAL: Color = Color {
		red: 255,
		green: 127,
		blue: 80,
		alpha: 255,
	};

	/// Web dark orange (255, 140, 0, 255)
	pub const DARKORANGE: Color = Color {
		red: 255,
		green: 140,
		blue: 0,
		alpha: 255,
	};

	/// Web orange (255, 165, 0, 255)
	pub const ORANGE: Color = Color {
		red: 255,
		green: 165,
		blue: 0,
		alpha: 255,
	};

	/// Web pink (255, 192, 203, 255)
	pub const PINK: Color = Color {
		red: 255,
		green: 192,
		blue: 203,
		alpha: 255,
	};

	/// Web gold (255, 215, 0, 255)
	pub const GOLD: Color = Color {
		red: 255,
		green: 215,
		blue: 0,
		alpha: 255,
	};

	/// Web white (255, 255, 255, 255)
	pub const WHITE: Color = Color {
		red: 255,
		green: 255,
		blue: 255,
		alpha: 255,
	};

	/// Web ivory (255, 255, 240, 255)
	pub const IVORY: Color = Color {
		red: 255,
		green: 255,
		blue: 240,
		alpha: 255,
	};

	/// Web yellow (255, 255, 0, 255)
	pub const YELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 255,
	};

	/// Web light yellow (255, 255, 224, 255)
	pub const LIGHTYELLOW: Color = Color {
		red: 255,
		green: 255,
		blue: 224,
		alpha: 255,
	};

	/// Web silver (192, 192, 192, 255)
	pub const SILVER: Color = Color {
		red: 192,
		green: 192,
		blue: 192,
		alpha: 255,
	};

	/// Web gainsboro (220, 220, 220, 255)
	pub const GAINSBORO: Color = Color {
		red: 220,
		green: 220,
		blue: 220,
		alpha: 255,
	};

	/// Web snow (255, 250, 250, 255)
	pub const SNOW: Color = Color {
		red: 255,
		green: 250,
		blue: 250,
		alpha: 255,
	};
}
