# Documentation Assets

This directory contains visual assets for the Engage UX documentation.

## Directory Structure

```text
assets/
├── images/       # General images (logos, icons, etc.)
├── diagrams/     # Architecture and flow diagrams
└── screenshots/  # Component screenshots and examples
```

## Asset Guidelines

### Images

- **Format**: PNG (preferred), SVG, or JPEG
- **Resolution**: Minimum 2x for screenshots (HiDPI)
- **Size**: Optimize for web (<500KB when possible)
- **Naming**: Use kebab-case (e.g., `button-example.png`)

### Diagrams

- **Format**: SVG (preferred) or PNG
- **Tools**: Draw.io, Mermaid, PlantUML, or similar
- **Style**: Match documentation theme
- **Text**: Use readable fonts (minimum 12pt)

### Screenshots

- **Format**: PNG with transparency when appropriate
- **Resolution**: 2x or 3x for Retina displays
- **Content**: Show relevant UI elements clearly
- **Background**: Clean, minimal distractions

## Adding Assets

### 1. Create the Asset

Use appropriate tools for your asset type:

- **Screenshots**: Use OS screenshot tools or browser dev tools
- **Diagrams**: Use Draw.io, Figma, or code-based tools (Mermaid, PlantUML)
- **Icons**: Use SVG format for scalability

### 2. Optimize

Optimize assets before committing:

```bash
# PNG optimization
pngquant --quality=65-80 input.png -o output.png

# SVG optimization
svgo input.svg -o output.svg

# JPEG optimization
jpegoptim --max=85 input.jpg
```

### 3. Place in Correct Directory

```text
assets/
├── images/
│   ├── logo.svg
│   ├── icon-button.png
│   └── icon-input.png
├── diagrams/
│   ├── architecture-overview.svg
│   ├── component-hierarchy.svg
│   └── data-flow.svg
└── screenshots/
    ├── button-variants.png
    ├── theme-demo-light.png
    └── theme-demo-dark.png
```

### 4. Reference in Documentation

Use relative paths in markdown:

```text
![Button Variants](../assets/screenshots/button-variants.png)

![Architecture](../assets/diagrams/architecture-overview.svg)
```

## Asset Inventory

### Images

- `logo.svg` - Engage UX logo (placeholder - add optimized SVG in `assets/images/` when available)
- `icon-*.png` - Component icons (placeholder - add icons in `assets/images/` when available)

### Diagrams

- `architecture-overview.svg` - System architecture diagram (placeholder - add optimized SVG in `assets/diagrams/` when available)
- `component-hierarchy.svg` - Component inheritance tree (placeholder)
- `data-flow.svg` - Event and data flow diagram (placeholder)
- `theme-structure.svg` - Theme system structure (placeholder)
- `input-system.svg` - Input handling flow (placeholder)

### Screenshots

- `button-variants.png` - All button variants (placeholder)
- `theme-demo-light.png` - Light theme showcase (placeholder)
- `theme-demo-dark.png` - Dark theme showcase (placeholder)
- `component-gallery.png` - All components overview (placeholder)
- `form-example.png` - Form components example (placeholder)

## Tools and Resources

### Diagram Tools

- [Draw.io](https://www.drawio.com/) - Free diagram editor
- [Mermaid](https://mermaid.js.org/) - Text-based diagrams
- [PlantUML](https://plantuml.com/) - UML diagrams from text
- [Excalidraw](https://excalidraw.com/) - Hand-drawn style diagrams

### Screenshot Tools

- **macOS**: Cmd+Shift+4, Screenshot.app
- **Windows**: Win+Shift+S, Snipping Tool
- **Linux**: Flameshot, GNOME Screenshot
- **Browser**: DevTools device emulation

### Optimization Tools

- [TinyPNG](https://tinypng.com/) - Online PNG/JPEG compression
- [SVGOMG](https://jakearchibald.github.io/svgomg/) - Online SVG optimizer
- [ImageOptim](https://imageoptim.com/) - macOS image optimizer
- [Squoosh](https://squoosh.app/) - Browser-based image compressor

## Contributing Assets

When contributing documentation assets:

1. **Follow guidelines** - Match the style and format guidelines
2. **Optimize first** - Reduce file size before committing
3. **Use descriptive names** - Clear, kebab-case filenames
4. **Update inventory** - Add to the asset inventory list
5. **Test in docs** - Verify assets display correctly

## License

All assets in this directory are licensed under the same terms as the Engage UX project (dual Apache-2.0/MIT).

---

[← Back to Documentation](../index.md)
