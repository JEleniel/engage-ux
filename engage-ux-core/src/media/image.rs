//! Image format support (PNG, JPEG, WebP, etc.)

use super::MediaError;

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
	/// PNG format
	Png,
	/// JPEG format
	Jpeg,
	/// WebP format
	WebP,
	/// GIF format (no animation support)
	Gif,
	/// BMP format
	Bmp,
	/// TIFF format
	Tiff,
}

impl ImageFormat {
	/// Detect format from file extension
	pub fn from_extension(ext: &str) -> Option<Self> {
		match ext.to_lowercase().as_str() {
			"png" => Some(ImageFormat::Png),
			"jpg" | "jpeg" => Some(ImageFormat::Jpeg),
			"webp" => Some(ImageFormat::WebP),
			"gif" => Some(ImageFormat::Gif),
			"bmp" => Some(ImageFormat::Bmp),
			"tif" | "tiff" => Some(ImageFormat::Tiff),
			_ => None,
		}
	}

	/// Detect format from magic bytes
	pub fn from_bytes(data: &[u8]) -> Option<Self> {
		if data.len() < 4 {
			return None;
		}

		// PNG: 89 50 4E 47
		if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
			return Some(ImageFormat::Png);
		}

		// JPEG: FF D8 FF
		if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
			return Some(ImageFormat::Jpeg);
		}

		// WebP: RIFF....WEBP
		if data.starts_with(b"RIFF") && data.len() >= 12 && &data[8..12] == b"WEBP" {
			return Some(ImageFormat::WebP);
		}

		// GIF: GIF87a or GIF89a
		if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
			return Some(ImageFormat::Gif);
		}

		// BMP: BM
		if data.starts_with(b"BM") {
			return Some(ImageFormat::Bmp);
		}

		// TIFF: II or MM
		if data.starts_with(b"II") || data.starts_with(b"MM") {
			return Some(ImageFormat::Tiff);
		}

		None
	}

	/// Get MIME type
	pub fn mime_type(&self) -> &'static str {
		match self {
			ImageFormat::Png => "image/png",
			ImageFormat::Jpeg => "image/jpeg",
			ImageFormat::WebP => "image/webp",
			ImageFormat::Gif => "image/gif",
			ImageFormat::Bmp => "image/bmp",
			ImageFormat::Tiff => "image/tiff",
		}
	}
}

/// Color type for images
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
	/// Grayscale
	Grayscale,
	/// RGB
	Rgb,
	/// RGBA (with alpha channel)
	Rgba,
}

/// Image data representation
#[derive(Debug, Clone)]
pub struct ImageData {
	/// Image width in pixels
	pub width: u32,
	/// Image height in pixels
	pub height: u32,
	/// Image format
	pub format: ImageFormat,
	/// Color type
	pub color_type: ColorType,
	/// Raw pixel data
	pub data: Vec<u8>,
}

impl ImageData {
	/// Create new image data
	pub fn new(
		width: u32,
		height: u32,
		format: ImageFormat,
		color_type: ColorType,
		data: Vec<u8>,
	) -> Self {
		Self {
			width,
			height,
			format,
			color_type,
			data,
		}
	}

	/// Load from file path (stub for now)
	pub fn load_from_file(_path: &str) -> Result<Self, MediaError> {
		// TODO: Implement actual image loading
		Err(MediaError::UnsupportedFormat(
			"Image loading not yet implemented".to_string(),
		))
	}

	/// Load from bytes
	pub fn load_from_bytes(data: Vec<u8>) -> Result<Self, MediaError> {
		// Detect format
		let format = ImageFormat::from_bytes(&data).ok_or_else(|| {
			MediaError::UnsupportedFormat("Unknown image format".to_string())
		})?;

		// Create placeholder image data
		// TODO: Implement actual image decoding
		Ok(Self {
			width: 1,
			height: 1,
			format,
			color_type: ColorType::Rgba,
			data: vec![0; 4], // Placeholder
		})
	}

	/// Get bytes per pixel
	pub fn bytes_per_pixel(&self) -> usize {
		match self.color_type {
			ColorType::Grayscale => 1,
			ColorType::Rgb => 3,
			ColorType::Rgba => 4,
		}
	}

	/// Get total size in bytes
	pub fn size_bytes(&self) -> usize {
		(self.width * self.height) as usize * self.bytes_per_pixel()
	}

	/// Get pixel at coordinates
	pub fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
		if x >= self.width || y >= self.height {
			return None;
		}

		let index = ((y * self.width + x) as usize) * self.bytes_per_pixel();
		let end = index + self.bytes_per_pixel();

		self.data.get(index..end)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_image_format_from_extension() {
		assert_eq!(
			ImageFormat::from_extension("png"),
			Some(ImageFormat::Png)
		);
		assert_eq!(
			ImageFormat::from_extension("jpg"),
			Some(ImageFormat::Jpeg)
		);
		assert_eq!(
			ImageFormat::from_extension("webp"),
			Some(ImageFormat::WebP)
		);
		assert_eq!(ImageFormat::from_extension("unknown"), None);
	}

	#[test]
	fn test_image_format_from_bytes() {
		// PNG magic bytes
		let png_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
		assert_eq!(ImageFormat::from_bytes(&png_bytes), Some(ImageFormat::Png));

		// JPEG magic bytes
		let jpeg_bytes = vec![0xFF, 0xD8, 0xFF, 0xE0];
		assert_eq!(
			ImageFormat::from_bytes(&jpeg_bytes),
			Some(ImageFormat::Jpeg)
		);

		// Unknown format
		let unknown_bytes = vec![0x00, 0x01, 0x02, 0x03];
		assert_eq!(ImageFormat::from_bytes(&unknown_bytes), None);
	}

	#[test]
	fn test_image_format_mime_type() {
		assert_eq!(ImageFormat::Png.mime_type(), "image/png");
		assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
		assert_eq!(ImageFormat::WebP.mime_type(), "image/webp");
	}

	#[test]
	fn test_image_data_creation() {
		let data = vec![255, 0, 0, 255]; // Red pixel
		let image = ImageData::new(1, 1, ImageFormat::Png, ColorType::Rgba, data.clone());

		assert_eq!(image.width, 1);
		assert_eq!(image.height, 1);
		assert_eq!(image.format, ImageFormat::Png);
		assert_eq!(image.data, data);
	}

	#[test]
	fn test_bytes_per_pixel() {
		let grayscale = ImageData::new(
			1,
			1,
			ImageFormat::Png,
			ColorType::Grayscale,
			vec![128],
		);
		assert_eq!(grayscale.bytes_per_pixel(), 1);

		let rgb = ImageData::new(1, 1, ImageFormat::Png, ColorType::Rgb, vec![255, 0, 0]);
		assert_eq!(rgb.bytes_per_pixel(), 3);

		let rgba = ImageData::new(
			1,
			1,
			ImageFormat::Png,
			ColorType::Rgba,
			vec![255, 0, 0, 255],
		);
		assert_eq!(rgba.bytes_per_pixel(), 4);
	}

	#[test]
	fn test_get_pixel() {
		let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2x1 image: red, green
		let image = ImageData::new(2, 1, ImageFormat::Png, ColorType::Rgba, data);

		let pixel1 = image.get_pixel(0, 0).unwrap();
		assert_eq!(pixel1, &[255, 0, 0, 255]);

		let pixel2 = image.get_pixel(1, 0).unwrap();
		assert_eq!(pixel2, &[0, 255, 0, 255]);

		assert!(image.get_pixel(2, 0).is_none()); // Out of bounds
	}

	#[test]
	fn test_load_from_bytes() {
		let png_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
		let result = ImageData::load_from_bytes(png_bytes);
		assert!(result.is_ok());

		let image = result.unwrap();
		assert_eq!(image.format, ImageFormat::Png);
	}
}
