# Color

`Color` is the central color type used throughout Engage UX. It supports RGBA and HSL representations, conversions, and parsing from common formats.

Key APIs:

- `Color::rgb(r, g, b, a)` — construct from floats in 0.0..=1.0
- `Color::from_hex("#RRGGBB")`, `Color::from_hex("#RRGGBBAA")` — parse from hex
- `to_rgb()`, `to_hsl()` — conversion helpers

Notes:

- Tests now include negative cases for invalid hex lengths and malformed deserialization.
