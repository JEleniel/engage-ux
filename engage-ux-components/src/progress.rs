//! Progress indicator component

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Progress indicator type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressType {
	/// Linear progress bar
	Linear,
	/// Circular progress indicator
	Circular,
	/// Indeterminate (loading spinner)
	Indeterminate,
}

/// Progress indicator component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
	properties: ComponentProperties,
	value: f32,
	max_value: f32,
	progress_type: ProgressType,
	show_label: bool,
	label: String,
	color: Color,
	background_color: Color,
	height: f32,
	thickness: f32,
}

impl Progress {
	/// Create a new progress indicator
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			value: 0.0,
			max_value: 100.0,
			progress_type: ProgressType::Linear,
			show_label: false,
			label: String::new(),
			color: Color::from_hex("#1976D2").unwrap(),
			background_color: Color::from_hex("#E0E0E0").unwrap(),
			height: 8.0,
			thickness: 4.0,
		}
	}

	/// Get current value
	pub fn value(&self) -> f32 {
		self.value
	}

	/// Set current value
	pub fn set_value(&mut self, value: f32) {
		self.value = value.clamp(0.0, self.max_value);
	}

	/// Get maximum value
	pub fn max_value(&self) -> f32 {
		self.max_value
	}

	/// Set maximum value
	pub fn set_max_value(&mut self, max_value: f32) {
		self.max_value = max_value.max(0.0);
		self.value = self.value.clamp(0.0, self.max_value);
	}

	/// Get progress percentage (0.0 to 1.0)
	pub fn percentage(&self) -> f32 {
		if self.max_value == 0.0 {
			0.0
		} else {
			self.value / self.max_value
		}
	}

	/// Get progress type
	pub fn progress_type(&self) -> ProgressType {
		self.progress_type
	}

	/// Set progress type
	pub fn set_progress_type(&mut self, progress_type: ProgressType) {
		self.progress_type = progress_type;
	}

	/// Check if label is shown
	pub fn shows_label(&self) -> bool {
		self.show_label
	}

	/// Set whether to show label
	pub fn set_show_label(&mut self, show_label: bool) {
		self.show_label = show_label;
	}

	/// Get label text
	pub fn label(&self) -> &str {
		&self.label
	}

	/// Set label text
	pub fn set_label(&mut self, label: impl Into<String>) {
		self.label = label.into();
	}

	/// Set progress color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get progress color
	pub fn color(&self) -> &Color {
		&self.color
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Get background color
	pub fn background_color(&self) -> &Color {
		&self.background_color
	}

	/// Set height (for linear progress)
	pub fn set_height(&mut self, height: f32) {
		self.height = height.max(1.0);
	}

	/// Get height
	pub fn height(&self) -> f32 {
		self.height
	}

	/// Set thickness (for circular progress)
	pub fn set_thickness(&mut self, thickness: f32) {
		self.thickness = thickness.max(1.0);
	}

	/// Get thickness
	pub fn thickness(&self) -> f32 {
		self.thickness
	}
}

impl Component for Progress {
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
	fn test_progress_creation() {
		let progress = Progress::new(1);
		assert_eq!(progress.id(), 1);
		assert_eq!(progress.value(), 0.0);
		assert_eq!(progress.max_value(), 100.0);
	}

	#[test]
	fn test_progress_value() {
		let mut progress = Progress::new(1);
		progress.set_value(50.0);
		assert_eq!(progress.value(), 50.0);
		assert_eq!(progress.percentage(), 0.5);
	}

	#[test]
	fn test_progress_clamping() {
		let mut progress = Progress::new(1);
		progress.set_value(150.0); // Over max
		assert_eq!(progress.value(), 100.0);

		progress.set_value(-10.0); // Under min
		assert_eq!(progress.value(), 0.0);
	}

	#[test]
	fn test_progress_percentage() {
		let mut progress = Progress::new(1);
		progress.set_max_value(200.0);
		progress.set_value(50.0);
		assert_eq!(progress.percentage(), 0.25);
	}

	#[test]
	fn test_progress_type() {
		let mut progress = Progress::new(1);
		assert_eq!(progress.progress_type(), ProgressType::Linear);

		progress.set_progress_type(ProgressType::Circular);
		assert_eq!(progress.progress_type(), ProgressType::Circular);
	}

	#[test]
	fn test_progress_label() {
		let mut progress = Progress::new(1);
		assert!(!progress.shows_label());

		progress.set_show_label(true);
		progress.set_label("Loading...");
		assert!(progress.shows_label());
		assert_eq!(progress.label(), "Loading...");
	}
}
