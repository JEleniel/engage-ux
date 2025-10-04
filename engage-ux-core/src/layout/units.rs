//! Unit system for flexible sizing
//!
//! Supports:
//! - Pixels (px) - absolute units
//! - Relative to base (rb) - scales relative to theme base size  
//! - Relative to parent (rp) - scales relative to inherited size
//! - Percentage (%) - relative to parent dimension

use serde::{Deserialize, Serialize};
use std::fmt;

/// A measurement unit that can be absolute or relative
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Unit {
	/// Absolute pixels
	Pixels(f32),
	/// Relative unit
	Relative(RelativeUnit),
}

/// Relative unit types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RelativeUnit {
	/// Relative to base size (similar to em)
	#[serde(rename = "rb")]
	RelativeToBase(f32),
	/// Relative to parent/inherited size (similar to rem)
	#[serde(rename = "rp")]
	RelativeToParent(f32),
	/// Percentage of parent dimension
	#[serde(rename = "%")]
	Percentage(f32),
}

impl Unit {
	/// Create a pixel unit
	pub fn pixels(value: f32) -> Self {
		Unit::Pixels(value)
	}

	/// Create a relative-to-base unit
	pub fn rb(value: f32) -> Self {
		Unit::Relative(RelativeUnit::RelativeToBase(value))
	}

	/// Create a relative-to-parent unit
	pub fn rp(value: f32) -> Self {
		Unit::Relative(RelativeUnit::RelativeToParent(value))
	}

	/// Create a percentage unit
	pub fn percent(value: f32) -> Self {
		Unit::Relative(RelativeUnit::Percentage(value))
	}

	/// Convert to pixels given context
	///
	/// # Arguments
	/// * `parent_dimension` - The parent's dimension (width or height) in pixels
	/// * `base_size` - The theme's base size in pixels
	/// * `inherited_size` - The inherited size in pixels (for rp units)
	pub fn to_pixels(&self, parent_dimension: f32, base_size: f32, inherited_size: f32) -> f32 {
		match self {
			Unit::Pixels(px) => *px,
			Unit::Relative(rel) => rel.to_pixels(parent_dimension, base_size, inherited_size),
		}
	}

	/// Parse a unit from a string
	///
	/// Supported formats:
	/// - "100px" or "100" -> Pixels(100.0)
	/// - "2rb" -> RelativeToBase(2.0)
	/// - "1.5rp" -> RelativeToParent(1.5)
	/// - "50%" -> Percentage(50.0)
	pub fn parse(s: &str) -> Result<Self, String> {
		let s = s.trim();
		
		if s.is_empty() {
			return Err("Empty string".to_string());
		}

		// Try percentage first
		if let Some(stripped) = s.strip_suffix('%') {
			let value = stripped
				.trim()
				.parse::<f32>()
				.map_err(|e| format!("Invalid percentage value: {}", e))?;
			return Ok(Unit::percent(value));
		}

		// Try rb
		if let Some(stripped) = s.strip_suffix("rb") {
			let value = stripped
				.trim()
				.parse::<f32>()
				.map_err(|e| format!("Invalid rb value: {}", e))?;
			return Ok(Unit::rb(value));
		}

		// Try rp
		if let Some(stripped) = s.strip_suffix("rp") {
			let value = stripped
				.trim()
				.parse::<f32>()
				.map_err(|e| format!("Invalid rp value: {}", e))?;
			return Ok(Unit::rp(value));
		}

		// Try pixels (with or without "px" suffix)
		let value_str = s.strip_suffix("px").unwrap_or(s);

		let value = value_str
			.trim()
			.parse::<f32>()
			.map_err(|e| format!("Invalid pixel value: {}", e))?;
		Ok(Unit::pixels(value))
	}
}

impl RelativeUnit {
	/// Convert to pixels given context
	pub fn to_pixels(&self, parent_dimension: f32, base_size: f32, inherited_size: f32) -> f32 {
		match self {
			RelativeUnit::RelativeToBase(multiplier) => multiplier * base_size,
			RelativeUnit::RelativeToParent(multiplier) => multiplier * inherited_size,
			RelativeUnit::Percentage(percent) => (percent / 100.0) * parent_dimension,
		}
	}
}

impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Unit::Pixels(px) => write!(f, "{}px", px),
			Unit::Relative(rel) => write!(f, "{}", rel),
		}
	}
}

impl fmt::Display for RelativeUnit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			RelativeUnit::RelativeToBase(value) => write!(f, "{}rb", value),
			RelativeUnit::RelativeToParent(value) => write!(f, "{}rp", value),
			RelativeUnit::Percentage(value) => write!(f, "{}%", value),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pixel_unit() {
		let unit = Unit::pixels(100.0);
		assert_eq!(unit.to_pixels(800.0, 16.0, 20.0), 100.0);
	}

	#[test]
	fn test_relative_to_base() {
		let unit = Unit::rb(2.0);
		// 2 * base_size (16.0) = 32.0
		assert_eq!(unit.to_pixels(800.0, 16.0, 20.0), 32.0);
	}

	#[test]
	fn test_relative_to_parent() {
		let unit = Unit::rp(1.5);
		// 1.5 * inherited_size (20.0) = 30.0
		assert_eq!(unit.to_pixels(800.0, 16.0, 20.0), 30.0);
	}

	#[test]
	fn test_percentage() {
		let unit = Unit::percent(50.0);
		// 50% of parent_dimension (800.0) = 400.0
		assert_eq!(unit.to_pixels(800.0, 16.0, 20.0), 400.0);
	}

	#[test]
	fn test_parse_pixels() {
		assert_eq!(Unit::parse("100px").unwrap(), Unit::pixels(100.0));
		assert_eq!(Unit::parse("100").unwrap(), Unit::pixels(100.0));
		assert_eq!(Unit::parse("  50  ").unwrap(), Unit::pixels(50.0));
	}

	#[test]
	fn test_parse_rb() {
		assert_eq!(Unit::parse("2rb").unwrap(), Unit::rb(2.0));
		assert_eq!(Unit::parse("1.5rb").unwrap(), Unit::rb(1.5));
	}

	#[test]
	fn test_parse_rp() {
		assert_eq!(Unit::parse("1rp").unwrap(), Unit::rp(1.0));
		assert_eq!(Unit::parse("0.75rp").unwrap(), Unit::rp(0.75));
	}

	#[test]
	fn test_parse_percentage() {
		assert_eq!(Unit::parse("50%").unwrap(), Unit::percent(50.0));
		assert_eq!(Unit::parse("100%").unwrap(), Unit::percent(100.0));
	}

	#[test]
	fn test_parse_invalid() {
		assert!(Unit::parse("").is_err());
		assert!(Unit::parse("abc").is_err());
		assert!(Unit::parse("px").is_err());
	}

	#[test]
	fn test_display() {
		assert_eq!(Unit::pixels(100.0).to_string(), "100px");
		assert_eq!(Unit::rb(2.0).to_string(), "2rb");
		assert_eq!(Unit::rp(1.5).to_string(), "1.5rp");
		assert_eq!(Unit::percent(50.0).to_string(), "50%");
	}
}
