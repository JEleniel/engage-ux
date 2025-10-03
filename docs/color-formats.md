# Color Format Specification

Engage UX supports multiple color formats for maximum convenience and flexibility. All formats can be used interchangeably in theme JSON files, allowing you to choose the most appropriate format for each color in your theme.

## Supported Formats

### 1. Hex Format

The most common format for web colors. Perfect for copying colors directly from design tools like Figma, Adobe XD, or Sketch.

**Without Alpha (fully opaque):**

```json
{
	"primary": {
		"hex": "#1976D2"
	}
}
```

**With Alpha (transparency):**

```json
{
	"primary": {
		"hex": "#1976D280"
	}
}
```

**Format details:**
- 6 characters for RGB: `#RRGGBB` where each pair is a hexadecimal value (00-FF)
- 8 characters for RGBA: `#RRGGBBAA` where the last pair is alpha
- Alpha values: `00` (fully transparent) to `FF` (fully opaque)
- Case-insensitive: `#FFFFFF` and `#ffffff` are equivalent

### 2. RGB Array Format

Standard RGB format using familiar 0-255 values. Ideal for programmatic color generation and working with image processing tools.

**Without Alpha (defaults to fully opaque):**

```json
{
	"primary": {
		"rgb": [25, 118, 210]
	}
}
```

**With Alpha (transparency):**

```json
{
	"primary": {
		"rgb": [25, 118, 210, 0.5]
	}
}
```

**Format details:**
- Red, Green, Blue: Integer values from `0` to `255`
- Alpha: Decimal value from `0.0` (fully transparent) to `1.0` (fully opaque)
- Alpha is optional; omitting it defaults to `1.0` (fully opaque)

### 3. HSL Array Format

Hue-Saturation-Lightness format. Perfect for color manipulation, creating color schemes, and dynamic theming.

**Without Alpha (defaults to fully opaque):**

```json
{
	"primary": {
		"hsl": [207, 0.79, 0.46]
	}
}
```

**With Alpha (transparency):**

```json
{
	"primary": {
		"hsl": [207, 0.79, 0.46, 0.5]
	}
}
```

**Format details:**
- Hue: Degrees from `0` to `360` on the color wheel (0=red, 120=green, 240=blue)
- Saturation: Decimal from `0.0` (grayscale) to `1.0` (full color) representing 0% to 100%
- Lightness: Decimal from `0.0` (black) to `1.0` (white) representing 0% to 100%
- Alpha: Decimal from `0.0` (fully transparent) to `1.0` (fully opaque)
- Alpha is optional; omitting it defaults to `1.0` (fully opaque)

**Why use HSL?**
- Easy to create lighter/darker variants by adjusting lightness
- Simple to create complementary colors by adjusting hue
- Intuitive for creating color schemes (monochromatic, analogous, triadic, etc.)

### 4. Legacy Format (Backward Compatible)

The original internal format, still fully supported for backward compatibility with existing themes.

```json
{
	"primary": {
		"space": "RGB",
		"components": [0.098, 0.463, 0.824, 1.0]
	}
}
```

**Format details:**
- RGB components: Decimal values from `0.0` to `1.0` (normalized)
- Alpha: Decimal value from `0.0` to `1.0`
- All four components are required (Red, Green, Blue, Alpha)

**Note:** While this format is still supported, the new user-friendly formats (hex, RGB, HSL) are recommended for better readability and easier theme maintenance.

## Format Mixing

You can mix different formats within the same theme file:

```json
{
	"colors": {
		"primary": {
			"hex": "#1976D2"
		},
		"secondary": {
			"rgb": [66, 66, 66]
		},
		"background": {
			"hsl": [0, 0, 1.0]
		},
		"error": {
			"space": "RGB",
			"components": [0.827, 0.184, 0.184, 1.0]
		}
	}
}
```

## Examples

### Complete Theme with User-Friendly Formats

```json
{
	"name": "My Theme",
	"colors": {
		"primary": {
			"hex": "#1976D2"
		},
		"secondary": {
			"hex": "#424242"
		},
		"background": {
			"hex": "#FFFFFF"
		},
		"surface": {
			"hex": "#F5F5F5"
		},
		"error": {
			"hex": "#D32F2F"
		},
		"warning": {
			"hex": "#FFA000"
		},
		"success": {
			"hex": "#388E3C"
		},
		"info": {
			"hex": "#0288D1"
		},
		"text_primary": {
			"hex": "#212121"
		},
		"text_secondary": {
			"hex": "#757575"
		},
		"text_disabled": {
			"hex": "#BDBDBD"
		},
		"custom": {}
	},
	"typography": {
		"font_family": "system-ui, -apple-system, sans-serif",
		"font_size_base": 16.0,
		"font_size_small": 14.0,
		"font_size_large": 20.0,
		"line_height": 1.5
	},
	"spacing": {
		"unit": 8.0,
		"small": 8.0,
		"medium": 16.0,
		"large": 24.0
	},
	"borders": {
		"width": 1.0,
		"radius": 4.0,
		"color": {
			"hex": "#E0E0E0"
		}
	},
	"shadows": {
		"enabled": true,
		"blur_radius": 4.0,
		"offset_x": 0.0,
		"offset_y": 2.0,
		"color": {
			"rgb": [0, 0, 0, 0.2]
		}
	}
}
```

## Conversion Reference

### Common Color Conversions

| Hex       | RGB               | HSL                 | Description      |
| --------- | ----------------- | ------------------- | ---------------- |
| `#FFFFFF` | `[255, 255, 255]` | `[0, 0, 1.0]`       | White            |
| `#000000` | `[0, 0, 0]`       | `[0, 0, 0]`         | Black            |
| `#FF0000` | `[255, 0, 0]`     | `[0, 1.0, 0.5]`     | Red              |
| `#00FF00` | `[0, 255, 0]`     | `[120, 1.0, 0.5]`   | Green            |
| `#0000FF` | `[0, 0, 255]`     | `[240, 1.0, 0.5]`   | Blue             |
| `#FFFF00` | `[255, 255, 0]`   | `[60, 1.0, 0.5]`    | Yellow           |
| `#FF00FF` | `[255, 0, 255]`   | `[300, 1.0, 0.5]`   | Magenta          |
| `#00FFFF` | `[0, 255, 255]`   | `[180, 1.0, 0.5]`   | Cyan             |
| `#808080` | `[128, 128, 128]` | `[0, 0, 0.502]`     | Gray             |
| `#1976D2` | `[25, 118, 210]`  | `[207, 0.79, 0.46]` | Material Blue    |

## Implementation Details

### Automatic Format Detection

Engage UX automatically detects which format you're using based on the JSON structure:
- If it has a `"hex"` key, it's parsed as hex format
- If it has an `"rgb"` key, it's parsed as RGB array format
- If it has an `"hsl"` key, it's parsed as HSL array format
- If it has `"space"` and `"components"` keys, it's parsed as legacy format

### Internal Processing

- All color formats are converted to an internal representation for processing
- Serialization uses the legacy format for consistency in the codebase
- Deserialization accepts all formats automatically without configuration
- RGB values (0-255) are normalized to `0.0-1.0` internally
- HSL hue values are normalized using modulo 360 (e.g., 380° becomes 20°)
- Alpha values are clamped to `0.0-1.0` range
- Invalid color formats return descriptive error messages

### Performance

- Format detection and conversion happens once during theme loading
- No performance overhead during runtime
- All conversions are validated for correctness
