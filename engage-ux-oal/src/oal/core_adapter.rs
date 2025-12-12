use crate::oal::canvas::Canvas;
use engage_ux_core::Color;
use engage_ux_core::LineStyle as CoreLineStyle;
use engage_ux_core::geometry::{
	Circle as CoreCircle, Ellipse as CoreEllipse, Line as CoreLine, Point as CorePoint,
	Polygon as CorePolygon, Polyline as CorePolyline, Rectangle as CoreRectangle, Text as CoreText,
};
use std::sync::Arc;

/// Adapter helpers to record engage-ux-core geometry primitives into an OAL `Canvas`.
/// These functions take geometry-only types from `engage-ux-core` and accept
/// styling parameters (color, stroke, fill) from the caller so core stays
/// device-independent.
/// Record a point
pub fn emit_point(canvas: &Canvas, pos: &engage_ux_core::geometry::Point, color: Color) {
	canvas.point(pos.clone(), color);
}

/// Record a line
pub fn emit_line(
	canvas: &Canvas,
	line: &engage_ux_core::geometry::Line,
	color: Color,
	stroke: f32,
) {
	canvas.line(line.clone(), color, stroke);
}

/// Record a circle
pub fn emit_circle(
	canvas: &Canvas,
	c: &engage_ux_core::geometry::Circle,
	color: Color,
	stroke: f32,
	fill: bool,
) {
	canvas.circle(c.clone(), color, stroke, fill);
}

/// Record a rectangle. The adapter converts per-corner radii conservatively
/// into a single radius pair (min of the x/y radii) because the OAL's
/// RoundedRect primitive currently carries a single radii pair.
pub fn emit_rectangle(
	canvas: &Canvas,
	r: &engage_ux_core::geometry::Rectangle,
	color: Color,
	stroke: f32,
	fill: bool,
) {
	// Core Rectangle already includes corner radii. We forward it directly.
	canvas.rounded_rect(r.clone(), color, stroke, fill);
}

/// Record polygon (closed)
pub fn emit_polygon(
	canvas: &Canvas,
	p: &engage_ux_core::geometry::Polygon,
	color: Color,
	stroke: f32,
	fill: bool,
) {
	canvas.polygon(p.clone(), color, stroke, fill);
}

/// Record polyline (open)
pub fn emit_polyline(
	canvas: &Canvas,
	pl: &engage_ux_core::geometry::Polyline,
	color: Color,
	stroke: f32,
) {
	canvas.polyline(pl.clone(), color, stroke);
}

/// Record text. The `font_size` is taken from the core `Text` primitive which is
/// specified in OAL units; renderers are expected to convert units -> pixels
/// using `DeviceMetrics` when rasterizing.
pub fn emit_text(canvas: &Canvas, t: &engage_ux_core::geometry::Text, color: Color) {
	canvas.text(t.clone(), color);
}

/// Record an ellipse
pub fn emit_ellipse(
	canvas: &Canvas,
	e: &engage_ux_core::geometry::Ellipse,
	color: Color,
	stroke: f32,
	fill: bool,
) {
	canvas.ellipse(e.clone(), color, stroke, fill);
}

/// Adapter that implements `engage_ux_core::DrawableContext` by recording
/// draw calls into an OAL `Canvas`.
pub struct DrawableContextAdapter {
	canvas: Arc<Canvas>,
}

impl DrawableContextAdapter {
	/// Create a new adapter that records into the provided `Canvas`.
	pub fn new(canvas: Arc<Canvas>) -> Self {
		Self { canvas }
	}

	fn resolve_color_from_line_style(ls: &Option<CoreLineStyle>) -> Color {
		if let Some(ls) = ls
			&& let Some(c) = ls.color.as_ref() {
				return *c;
			}
		Color::from_rgb(0, 0, 0)
	}

	fn resolve_stroke_from_line_style(ls: &Option<CoreLineStyle>) -> f32 {
		if let Some(ls) = ls {
			return ls.stroke_width.unwrap_or(1.0);
		}
		1.0
	}
}

impl engage_ux_core::component::DrawableContext for DrawableContextAdapter {
	fn draw_point(&mut self, p: CorePoint) {
		let color = Self::resolve_color_from_line_style(&p.style);
		self.canvas.point(p, color);
	}

	fn draw_line(&mut self, l: CoreLine) {
		let color = Self::resolve_color_from_line_style(&l.line_style);
		let stroke = Self::resolve_stroke_from_line_style(&l.line_style);
		self.canvas.line(l, color, stroke);
	}

	fn draw_rectangle(&mut self, r: CoreRectangle) {
		let mut fill = false;
		let mut color = Color::from_rgb(0, 0, 0);
		if let Some(fs) = r.fill_style.as_ref()
			&& let Some(c) = fs.color.as_ref() {
				color = *c;
				fill = true;
			}
		if !fill {
			// fallback to line style
			color = Self::resolve_color_from_line_style(&r.line_style);
		}
		let stroke = Self::resolve_stroke_from_line_style(&r.line_style);
		self.canvas.rounded_rect(r, color, stroke, fill);
	}

	fn draw_circle(&mut self, c: CoreCircle) {
		let mut fill = false;
		let mut color = Color::from_rgb(0, 0, 0);
		if let Some(fs) = c.fill_style.as_ref()
			&& let Some(col) = fs.color.as_ref() {
				color = *col;
				fill = true;
			}
		if !fill {
			color = Self::resolve_color_from_line_style(&c.line_style);
		}
		let stroke = Self::resolve_stroke_from_line_style(&c.line_style);
		self.canvas.circle(c, color, stroke, fill);
	}

	fn draw_ellipse(&mut self, e: CoreEllipse) {
		let mut fill = false;
		let mut color = Color::from_rgb(0, 0, 0);
		if let Some(fs) = e.fill_style.as_ref()
			&& let Some(col) = fs.color.as_ref() {
				color = *col;
				fill = true;
			}
		if !fill {
			color = Self::resolve_color_from_line_style(&e.line_style);
		}
		let stroke = Self::resolve_stroke_from_line_style(&e.line_style);
		self.canvas.ellipse(e, color, stroke, fill);
	}

	fn draw_polygon(&mut self, p: CorePolygon) {
		// Use polygon.fill_style to determine fill and color
		let mut fill = false;
		let mut color = Color::from_rgb(0, 0, 0);
		if let Some(fs) = p.fill_style.as_ref()
			&& let Some(col) = fs.color.as_ref() {
				color = *col;
				fill = true;
			}
		if !fill {
			color = Self::resolve_color_from_line_style(&p.line_style);
		}
		let stroke = Self::resolve_stroke_from_line_style(&p.line_style);
		self.canvas.polygon(p, color, stroke, fill);
	}

	fn draw_polyline(&mut self, p: CorePolyline) {
		let color = p
			.style
			.as_ref()
			.and_then(|s| s.color)
			.unwrap_or_else(|| Color::from_rgb(0, 0, 0));
		let stroke = p.style.as_ref().and_then(|s| s.stroke_width).unwrap_or(1.0);
		self.canvas.polyline(p, color, stroke);
	}

	fn draw_text(&mut self, t: CoreText) {
		let mut color = Color::from_rgb(0, 0, 0);
		if let Some(fs) = t.fill_style.as_ref()
			&& let Some(c) = fs.color.as_ref() {
				color = *c;
			}
		if let Some(ls) = t.line_style.as_ref()
			&& let Some(c) = ls.color.as_ref() {
				color = *c;
			}
		self.canvas.text(t, color);
	}
}
