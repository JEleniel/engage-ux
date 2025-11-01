//! Rendering utilities and SVG support
//!
//! Provides SVG parsing and rendering without script execution for security.

pub mod svg;
pub mod error;

pub use svg::{SvgDocument, SvgElement, SvgParser};

pub use error::RenderError;

