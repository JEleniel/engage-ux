//! SVG parsing and rendering without script execution
//!
//! This module provides secure SVG parsing that explicitly blocks:
//! - Script tags and event handlers
//! - External resource loading
//! - Potentially dangerous features

use super::RenderError;
use std::collections::HashMap;

/// SVG element types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SvgElementType {
	/// Root SVG element
	Svg,
	/// Path element
	Path,
	/// Circle element
	Circle,
	/// Rectangle element
	Rect,
	/// Line element
	Line,
	/// Ellipse element
	Ellipse,
	/// Polygon element
	Polygon,
	/// Polyline element
	Polyline,
	/// Text element
	Text,
	/// Group element
	Group,
	/// Definition element
	Defs,
	/// Use element (for reusing elements)
	Use,
}

/// SVG element representation
#[derive(Debug, Clone)]
pub struct SvgElement {
	/// Element type
	pub element_type: SvgElementType,
	/// Element attributes
	pub attributes: HashMap<String, String>,
	/// Child elements
	pub children: Vec<SvgElement>,
	/// Text content (for text elements)
	pub content: Option<String>,
}

impl SvgElement {
	/// Create a new SVG element
	pub fn new(element_type: SvgElementType) -> Self {
		Self {
			element_type,
			attributes: HashMap::new(),
			children: Vec::new(),
			content: None,
		}
	}

	/// Get an attribute value
	pub fn get_attribute(&self, name: &str) -> Option<&String> {
		self.attributes.get(name)
	}

	/// Set an attribute value
	pub fn set_attribute(&mut self, name: String, value: String) {
		self.attributes.insert(name, value);
	}

	/// Add a child element
	pub fn add_child(&mut self, child: SvgElement) {
		self.children.push(child);
	}
}

/// SVG document
#[derive(Debug, Clone)]
pub struct SvgDocument {
	/// Root SVG element
	pub root: SvgElement,
	/// Document width
	pub width: Option<f32>,
	/// Document height
	pub height: Option<f32>,
	/// ViewBox (min-x, min-y, width, height)
	pub viewbox: Option<(f32, f32, f32, f32)>,
}

impl SvgDocument {
	/// Create a new SVG document
	pub fn new(root: SvgElement) -> Self {
		Self {
			root,
			width: None,
			height: None,
			viewbox: None,
		}
	}

	/// Parse width from root element
	pub fn parse_dimensions(&mut self) {
		if let Some(width_str) = self.root.get_attribute("width")
			&& let Ok(w) = width_str.trim_end_matches("px").parse::<f32>() {
				self.width = Some(w);
			}

		if let Some(height_str) = self.root.get_attribute("height")
			&& let Ok(h) = height_str.trim_end_matches("px").parse::<f32>() {
				self.height = Some(h);
			}

		if let Some(viewbox_str) = self.root.get_attribute("viewBox") {
			let parts: Vec<&str> = viewbox_str.split_whitespace().collect();
			if parts.len() == 4
				&& let (Ok(min_x), Ok(min_y), Ok(width), Ok(height)) = (
					parts[0].parse::<f32>(),
					parts[1].parse::<f32>(),
					parts[2].parse::<f32>(),
					parts[3].parse::<f32>(),
				) {
					self.viewbox = Some((min_x, min_y, width, height));
				}
		}
	}
}

/// SVG parser with security restrictions
pub struct SvgParser {
	/// Whether to block script tags
	block_scripts: bool,
	/// Whether to block external resources
	block_external: bool,
}

impl SvgParser {
	/// Create a new secure SVG parser
	pub fn new() -> Self {
		Self {
			block_scripts: true,
			block_external: true,
		}
	}

	/// Parse SVG from string
	pub fn parse(&self, svg_content: &str) -> Result<SvgDocument, RenderError> {
		// Security check: detect script tags
		if self.block_scripts && self.contains_script(svg_content) {
			return Err(RenderError::ScriptDetected);
		}

		// Security check: detect event handlers
		if self.block_scripts && self.contains_event_handlers(svg_content) {
			return Err(RenderError::ScriptDetected);
		}

		// Security check: detect external resources
		if self.block_external && self.contains_external_resources(svg_content) {
			return Err(RenderError::ExternalResourceBlocked(
				"External resources not allowed".to_string(),
			));
		}

		// Basic SVG parsing (simplified - would use a proper XML parser in production)
		self.parse_basic_svg(svg_content)
	}

	fn contains_script(&self, content: &str) -> bool {
		content.contains("<script") || content.contains("</script>")
	}

	fn contains_event_handlers(&self, content: &str) -> bool {
		let event_handlers = [
			"onclick", "onload", "onmouseover", "onmouseout", "onerror", "onabort",
		];
		event_handlers
			.iter()
			.any(|handler| content.to_lowercase().contains(handler))
	}

	fn contains_external_resources(&self, content: &str) -> bool {
		content.contains("http://") || content.contains("https://")
	}

	fn parse_basic_svg(&self, content: &str) -> Result<SvgDocument, RenderError> {
		// Parse SVG using usvg
		let opts = usvg::Options::default();
		let tree = usvg::Tree::from_str(content, &opts)
			.map_err(|e| RenderError::InvalidSvg(format!("Failed to parse SVG: {}", e)))?;

		// Get SVG dimensions
		let svg_size = tree.size();

		// Create root element with dimensions
		let mut root = SvgElement::new(SvgElementType::Svg);
		root.set_attribute("width".to_string(), svg_size.width().to_string());
		root.set_attribute("height".to_string(), svg_size.height().to_string());

		// Convert usvg tree to our SvgElement structure
		self.convert_group_to_elements(tree.root(), &mut root);

		let mut doc = SvgDocument::new(root);
		doc.parse_dimensions();

		Ok(doc)
	}

	fn convert_group_to_elements(&self, group: &usvg::Group, parent: &mut SvgElement) {
		// Iterate through children of the group
		for node in group.children() {
			match node {
				usvg::Node::Group(g) => {
					let mut group_elem = SvgElement::new(SvgElementType::Group);
					group_elem.set_attribute("id".to_string(), g.id().to_string());
					self.convert_group_to_elements(g, &mut group_elem);
					parent.add_child(group_elem);
				}
				usvg::Node::Path(p) => {
					let mut path_elem = SvgElement::new(SvgElementType::Path);
					path_elem.set_attribute("id".to_string(), p.id().to_string());
					// Store path info (actual path data would require more complex conversion)
					path_elem.set_attribute("stroke-width".to_string(), "1".to_string());
					parent.add_child(path_elem);
				}
				usvg::Node::Image(_) => {
					// Images are handled separately for security
					let img = SvgElement::new(SvgElementType::Use);
					parent.add_child(img);
				}
				usvg::Node::Text(t) => {
					let mut text = SvgElement::new(SvgElementType::Text);
					text.set_attribute("id".to_string(), t.id().to_string());
					// Store placeholder text content
					// Note: usvg's text extraction would require more complex handling
					let text_content = "text".to_string();
					if !text_content.is_empty() {
						text.content = Some(text_content);
					}
					parent.add_child(text);
				}
			}
		}
	}
}

impl Default for SvgParser {
	fn default() -> Self {
		Self::new()
	}
}

/// Convert SVG element type from string
impl SvgElementType {
	pub fn from_tag_name(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"svg" => Some(SvgElementType::Svg),
			"path" => Some(SvgElementType::Path),
			"circle" => Some(SvgElementType::Circle),
			"rect" => Some(SvgElementType::Rect),
			"line" => Some(SvgElementType::Line),
			"ellipse" => Some(SvgElementType::Ellipse),
			"polygon" => Some(SvgElementType::Polygon),
			"polyline" => Some(SvgElementType::Polyline),
			"text" => Some(SvgElementType::Text),
			"g" => Some(SvgElementType::Group),
			"defs" => Some(SvgElementType::Defs),
			"use" => Some(SvgElementType::Use),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_svg_element_creation() {
		let mut element = SvgElement::new(SvgElementType::Circle);
		element.set_attribute("cx".to_string(), "50".to_string());
		element.set_attribute("cy".to_string(), "50".to_string());
		element.set_attribute("r".to_string(), "40".to_string());

		assert_eq!(element.element_type, SvgElementType::Circle);
		assert_eq!(element.get_attribute("cx"), Some(&"50".to_string()));
	}

	#[test]
	fn test_svg_parser_blocks_scripts() {
		let parser = SvgParser::new();
		let svg_with_script = r#"<svg><script>alert('XSS')</script></svg>"#;

		let result = parser.parse(svg_with_script);
		assert!(matches!(result, Err(RenderError::ScriptDetected)));
	}

	#[test]
	fn test_svg_parser_blocks_event_handlers() {
		let parser = SvgParser::new();
		let svg_with_onclick = r#"<svg onclick="alert('XSS')"></svg>"#;

		let result = parser.parse(svg_with_onclick);
		assert!(matches!(result, Err(RenderError::ScriptDetected)));
	}

	#[test]
	fn test_svg_parser_blocks_external_resources() {
		let parser = SvgParser::new();
		let svg_with_external = r#"<svg><image href="http://evil.com/image.png"/></svg>"#;

		let result = parser.parse(svg_with_external);
		assert!(matches!(
			result,
			Err(RenderError::ExternalResourceBlocked(_))
		));
	}

	#[test]
	fn test_svg_element_type_from_tag_name() {
		assert_eq!(
			SvgElementType::from_tag_name("circle"),
			Some(SvgElementType::Circle)
		);
		assert_eq!(
			SvgElementType::from_tag_name("rect"),
			Some(SvgElementType::Rect)
		);
		assert_eq!(SvgElementType::from_tag_name("unknown"), None);
	}

	#[test]
	fn test_svg_document_dimensions() {
		let mut root = SvgElement::new(SvgElementType::Svg);
		root.set_attribute("width".to_string(), "200px".to_string());
		root.set_attribute("height".to_string(), "150px".to_string());
		root.set_attribute("viewBox".to_string(), "0 0 200 150".to_string());

		let mut doc = SvgDocument::new(root);
		doc.parse_dimensions();

		assert_eq!(doc.width, Some(200.0));
		assert_eq!(doc.height, Some(150.0));
		assert_eq!(doc.viewbox, Some((0.0, 0.0, 200.0, 150.0)));
	}
}
