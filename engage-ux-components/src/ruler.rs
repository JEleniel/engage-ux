//! Ruler component for measurement and alignment

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Ruler orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RulerOrientation {
	Horizontal,
	Vertical,
}

/// Ruler unit system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RulerUnit {
	Pixels,
	Centimeters,
	Inches,
	Points,
}

impl RulerUnit {
	/// Get the pixels per unit
	pub fn pixels_per_unit(&self) -> f32 {
		match self {
			RulerUnit::Pixels => 1.0,
			RulerUnit::Centimeters => 37.8, // Approximate 96 DPI
			RulerUnit::Inches => 96.0,      // Standard 96 DPI
			RulerUnit::Points => 1.33,      // 1 point = 1/72 inch
		}
	}

	/// Get unit label
	pub fn label(&self) -> &str {
		match self {
			RulerUnit::Pixels => "px",
			RulerUnit::Centimeters => "cm",
			RulerUnit::Inches => "in",
			RulerUnit::Points => "pt",
		}
	}
}

/// Ruler component for measurement guides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ruler {
	properties: ComponentProperties,
	orientation: RulerOrientation,
	unit: RulerUnit,
	start: f32,
	end: f32,
	major_tick_interval: f32,
	minor_tick_interval: f32,
	show_labels: bool,
	color: Color,
	background_color: Color,
	tick_color: Color,
	label_color: Color,
	thickness: f32,
}

impl Ruler {
	/// Create a new ruler
	pub fn new(id: ComponentId, orientation: RulerOrientation) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			orientation,
			unit: RulerUnit::Pixels,
			start: 0.0,
			end: 1000.0,
			major_tick_interval: 100.0,
			minor_tick_interval: 10.0,
			show_labels: true,
			color: Color::from_hex("#CCCCCC").unwrap(),
			background_color: Color::from_hex("#F5F5F5").unwrap(),
			tick_color: Color::from_hex("#666666").unwrap(),
			label_color: Color::from_hex("#000000").unwrap(),
			thickness: 30.0,
		}
	}

	/// Get orientation
	pub fn orientation(&self) -> RulerOrientation {
		self.orientation
	}

	/// Set orientation
	pub fn set_orientation(&mut self, orientation: RulerOrientation) {
		self.orientation = orientation;
	}

	/// Get unit
	pub fn unit(&self) -> RulerUnit {
		self.unit
	}

	/// Set unit
	pub fn set_unit(&mut self, unit: RulerUnit) {
		self.unit = unit;
	}

	/// Get start position
	pub fn start(&self) -> f32 {
		self.start
	}

	/// Set start position
	pub fn set_start(&mut self, start: f32) {
		self.start = start;
	}

	/// Get end position
	pub fn end(&self) -> f32 {
		self.end
	}

	/// Set end position
	pub fn set_end(&mut self, end: f32) {
		self.end = end;
	}

	/// Get range
	pub fn range(&self) -> f32 {
		self.end - self.start
	}

	/// Get major tick interval
	pub fn major_tick_interval(&self) -> f32 {
		self.major_tick_interval
	}

	/// Set major tick interval
	pub fn set_major_tick_interval(&mut self, interval: f32) {
		self.major_tick_interval = interval.max(1.0);
	}

	/// Get minor tick interval
	pub fn minor_tick_interval(&self) -> f32 {
		self.minor_tick_interval
	}

	/// Set minor tick interval
	pub fn set_minor_tick_interval(&mut self, interval: f32) {
		self.minor_tick_interval = interval.max(1.0);
	}

	/// Check if labels are shown
	pub fn shows_labels(&self) -> bool {
		self.show_labels
	}

	/// Set whether to show labels
	pub fn set_show_labels(&mut self, show: bool) {
		self.show_labels = show;
	}

	/// Set ruler color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set tick color
	pub fn set_tick_color(&mut self, color: Color) {
		self.tick_color = color;
	}

	/// Set label color
	pub fn set_label_color(&mut self, color: Color) {
		self.label_color = color;
	}

	/// Get thickness
	pub fn thickness(&self) -> f32 {
		self.thickness
	}

	/// Set thickness
	pub fn set_thickness(&mut self, thickness: f32) {
		self.thickness = thickness.max(20.0);
	}
}

impl Component for Ruler {
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
	fn test_ruler_creation() {
		let ruler = Ruler::new(1, RulerOrientation::Horizontal);
		assert_eq!(ruler.id(), 1);
		assert_eq!(ruler.orientation(), RulerOrientation::Horizontal);
	}

	#[test]
	fn test_ruler_range() {
		let mut ruler = Ruler::new(1, RulerOrientation::Horizontal);
		ruler.set_start(100.0);
		ruler.set_end(500.0);
		assert_eq!(ruler.range(), 400.0);
	}

	#[test]
	fn test_ruler_units() {
		let mut ruler = Ruler::new(1, RulerOrientation::Horizontal);
		ruler.set_unit(RulerUnit::Inches);
		assert_eq!(ruler.unit(), RulerUnit::Inches);
		assert_eq!(ruler.unit().label(), "in");
	}

	#[test]
	fn test_ruler_ticks() {
		let mut ruler = Ruler::new(1, RulerOrientation::Horizontal);
		ruler.set_major_tick_interval(50.0);
		ruler.set_minor_tick_interval(5.0);
		assert_eq!(ruler.major_tick_interval(), 50.0);
		assert_eq!(ruler.minor_tick_interval(), 5.0);
	}

	#[test]
	fn test_ruler_orientation() {
		let mut ruler = Ruler::new(1, RulerOrientation::Horizontal);
		ruler.set_orientation(RulerOrientation::Vertical);
		assert_eq!(ruler.orientation(), RulerOrientation::Vertical);
	}
}
