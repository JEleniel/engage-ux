//! Image component for displaying images

use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Image fit mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFit {
	/// Image fills the container, may be cropped
	Cover,
	/// Image fits within container, may have empty space
	Contain,
	/// Image is stretched to fill container
	Fill,
	/// Image maintains original size
	None,
	/// Image scales down if larger than container
	ScaleDown,
}

/// Image component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
	properties: ComponentProperties,
	source: String,
	alt_text: String,
	width: Option<f32>,
	height: Option<f32>,
	fit: ImageFit,
	lazy: bool,
}

impl Image {
	/// Create a new image
	pub fn new(id: ComponentId, source: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			source: source.into(),
			alt_text: String::new(),
			width: None,
			height: None,
			fit: ImageFit::Cover,
			lazy: false,
		}
	}

	/// Get image source URL
	pub fn source(&self) -> &str {
		&self.source
	}

	/// Set image source URL
	pub fn set_source(&mut self, source: impl Into<String>) {
		self.source = source.into();
	}

	/// Get alt text
	pub fn alt_text(&self) -> &str {
		&self.alt_text
	}

	/// Set alt text (for accessibility)
	pub fn set_alt_text(&mut self, alt_text: impl Into<String>) {
		self.alt_text = alt_text.into();
	}

	/// Get width
	pub fn width(&self) -> Option<f32> {
		self.width
	}

	/// Set width
	pub fn set_width(&mut self, width: Option<f32>) {
		self.width = width;
	}

	/// Get height
	pub fn height(&self) -> Option<f32> {
		self.height
	}

	/// Set height
	pub fn set_height(&mut self, height: Option<f32>) {
		self.height = height;
	}

	/// Get fit mode
	pub fn fit(&self) -> ImageFit {
		self.fit
	}

	/// Set fit mode
	pub fn set_fit(&mut self, fit: ImageFit) {
		self.fit = fit;
	}

	/// Check if lazy loading is enabled
	pub fn is_lazy(&self) -> bool {
		self.lazy
	}

	/// Set lazy loading
	pub fn set_lazy(&mut self, lazy: bool) {
		self.lazy = lazy;
	}
}

impl Component for Image {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn properties(&self) -> &ComponentProperties {
		&self.properties
	}

	fn properties_mut(&mut self) -> &mut ComponentProperties {
		&mut self.properties
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_image_creation() {
		let image = Image::new(1, "https://example.com/image.jpg");
		assert_eq!(image.id(), 1);
		assert_eq!(image.source(), "https://example.com/image.jpg");
	}

	#[test]
	fn test_image_alt_text() {
		let mut image = Image::new(1, "image.jpg");
		image.set_alt_text("A beautiful image");
		assert_eq!(image.alt_text(), "A beautiful image");
	}

	#[test]
	fn test_image_dimensions() {
		let mut image = Image::new(1, "image.jpg");
		image.set_width(Some(800.0));
		image.set_height(Some(600.0));
		assert_eq!(image.width(), Some(800.0));
		assert_eq!(image.height(), Some(600.0));
	}

	#[test]
	fn test_image_fit() {
		let mut image = Image::new(1, "image.jpg");
		image.set_fit(ImageFit::Contain);
		assert_eq!(image.fit(), ImageFit::Contain);
	}

	#[test]
	fn test_image_lazy_loading() {
		let mut image = Image::new(1, "image.jpg");
		assert!(!image.is_lazy());
		image.set_lazy(true);
		assert!(image.is_lazy());
	}
}
