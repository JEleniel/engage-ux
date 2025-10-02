# Color Format Specification

Engage UX supports multiple color formats for user convenience and flexibility. All formats can be used in theme JSON files.

## Supported Formats

### 1. Hex Format

The most common format for web colors.

**Without Alpha:**

```json
{
	"primary": {
		"hex": "#1976D2"
	}
}
```

**With Alpha:**

```json
{
	"primary": {
		"hex": "#1976D280"
	}
}
```

-	6 characters for RGB (`#RRGGBB`)
-	8 characters for RGBA (`#RRGGBBAA`)
-	Alpha values: `00` (transparent) to `FF` (opaque)

### 2. RGB Array Format

Convenient for programmatic color generation.

**Without Alpha (defaults to 1.0):**

```json
{
	"primary": {
		"rgb": [25, 118, 210]
	}
}
```

**With Alpha:**

```json
{
	"primary": {
		"rgb": [25, 118, 210, 0.5]
	}
}
```

-	Red, Green, Blue: `0-255`
-	Alpha: `0.0` (transparent) to `1.0` (opaque)

### 3. HSL Array Format

Useful for color manipulation and theming.

**Without Alpha (defaults to 1.0):**

```json
{
	"primary": {
		"hsl": [207, 0.79, 0.46]
	}
}
```

**With Alpha:**

```json
{
	"primary": {
		"hsl": [207, 0.79, 0.46, 0.5]
	}
}
```

-	Hue: `0-360` degrees
-	Saturation: `0.0-1.0` (0% to 100%)
-	Lightness: `0.0-1.0` (0% to 100%)
-	Alpha: `0.0-1.0` (transparent to opaque)

### 4. Legacy Format (Backward Compatible)

The original internal format, still supported for backward compatibility.

```json
{
	"primary": {
		"space": "RGB",
		"components": [0.098, 0.463, 0.824, 1.0]
	}
}
```

-	RGB components: `0.0-1.0`
-	Alpha: `0.0-1.0`

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

-	All color formats are converted to an internal representation for processing
-	Serialization uses the legacy format for consistency
-	Deserialization accepts all formats automatically
-	RGB values are normalized to `0.0-1.0` internally
-	HSL hue values are normalized using modulo 360
-	Alpha values are clamped to `0.0-1.0`
-	Invalid color formats return descriptive error messages
