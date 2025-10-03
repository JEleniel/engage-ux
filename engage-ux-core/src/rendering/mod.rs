//! Rendering utilities and SVG support
//!
//! Provides SVG parsing and rendering without script execution for security.

pub mod svg;

pub use svg::{SvgDocument, SvgElement, SvgParser};

/// Rendering error types
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
	/// Invalid SVG syntax
	InvalidSvg(String),
	/// Unsupported SVG feature
	UnsupportedFeature(String),
	/// Script execution attempted (security violation)
	ScriptDetected,
	/// External resource loading not allowed
	ExternalResourceBlocked(String),
}

impl std::fmt::Display for RenderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RenderError::InvalidSvg(msg) => write!(f, "Invalid SVG: {}", msg),
			RenderError::UnsupportedFeature(feature) => {
				write!(f, "Unsupported SVG feature: {}", feature)
			}
			RenderError::ScriptDetected => {
				write!(f, "Script execution is not allowed for security reasons")
			}
			RenderError::ExternalResourceBlocked(url) => {
				write!(f, "External resource blocked: {}", url)
			}
		}
	}
}

impl std::error::Error for RenderError {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_render_error_display() {
		let error = RenderError::InvalidSvg("Missing closing tag".to_string());
		assert_eq!(error.to_string(), "Invalid SVG: Missing closing tag");

		let script_error = RenderError::ScriptDetected;
		assert_eq!(
			script_error.to_string(),
			"Script execution is not allowed for security reasons"
		);
	}
}
