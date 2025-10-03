//! Media handling for fonts and images
//!
//! Provides support for loading and managing fonts and image formats.

pub mod font;
pub mod image;

pub use font::{Font, FontFamily, FontStyle, FontWeight};
pub use image::{ImageData, ImageFormat};

/// Media loading error
#[derive(Debug, Clone, PartialEq)]
pub enum MediaError {
	/// Unsupported format
	UnsupportedFormat(String),
	/// Invalid data
	InvalidData(String),
	/// Loading failed
	LoadFailed(String),
}

impl std::fmt::Display for MediaError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MediaError::UnsupportedFormat(format) => {
				write!(f, "Unsupported format: {}", format)
			}
			MediaError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
			MediaError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
		}
	}
}

impl std::error::Error for MediaError {}
