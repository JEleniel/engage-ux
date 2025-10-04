//! Multi-monitor support for Engage UX
//!
//! Provides types and configuration for managing multiple monitors.

use serde::{Deserialize, Serialize};

/// Monitor identification and properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Monitor {
	/// Unique monitor identifier
	pub id: u32,
	/// Monitor name/description
	pub name: String,
	/// Physical position (x, y) in virtual screen space
	pub position: (i32, i32),
	/// Resolution (width, height) in pixels
	pub resolution: (u32, u32),
	/// DPI scaling factor
	pub scale_factor: f32,
	/// Whether this is the primary monitor
	pub is_primary: bool,
	/// Monitor refresh rate in Hz
	pub refresh_rate: Option<u32>,
}

impl Monitor {
	/// Create a new monitor
	pub fn new(id: u32, name: String, resolution: (u32, u32)) -> Self {
		Self {
			id,
			name,
			position: (0, 0),
			resolution,
			scale_factor: 1.0,
			is_primary: false,
			refresh_rate: None,
		}
	}

	/// Set monitor position
	pub fn with_position(mut self, x: i32, y: i32) -> Self {
		self.position = (x, y);
		self
	}

	/// Set DPI scale factor
	pub fn with_scale_factor(mut self, scale: f32) -> Self {
		self.scale_factor = scale;
		self
	}

	/// Mark as primary monitor
	pub fn as_primary(mut self) -> Self {
		self.is_primary = true;
		self
	}

	/// Set refresh rate
	pub fn with_refresh_rate(mut self, hz: u32) -> Self {
		self.refresh_rate = Some(hz);
		self
	}

	/// Get the bounding rectangle of this monitor
	pub fn bounds(&self) -> MonitorBounds {
		MonitorBounds {
			x: self.position.0,
			y: self.position.1,
			width: self.resolution.0,
			height: self.resolution.1,
		}
	}
}

/// Monitor bounding rectangle in virtual screen coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorBounds {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

impl MonitorBounds {
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
		Self { x, y, width, height }
	}

	/// Check if this bounds contains a point
	pub fn contains_point(&self, x: i32, y: i32) -> bool {
		x >= self.x
			&& x < self.x + self.width as i32
			&& y >= self.y
			&& y < self.y + self.height as i32
	}

	/// Check if this bounds intersects another
	pub fn intersects(&self, other: &MonitorBounds) -> bool {
		!(self.x + self.width as i32 <= other.x
			|| other.x + other.width as i32 <= self.x
			|| self.y + self.height as i32 <= other.y
			|| other.y + other.height as i32 <= self.y)
	}
}

/// Layout mode for multiple monitors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitorLayoutMode {
	/// Treat all monitors as one unified surface
	Unified,
	/// Treat each monitor as a separate surface
	Separate,
	/// Mix of unified and separate (custom configuration)
	Mixed,
}

/// Configuration for multi-monitor setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfiguration {
	/// List of available monitors
	pub monitors: Vec<Monitor>,
	/// Layout mode
	pub layout_mode: MonitorLayoutMode,
	/// Primary monitor ID (if any)
	pub primary_monitor: Option<u32>,
	/// Custom groupings for Mixed mode (monitor IDs that should be unified)
	pub monitor_groups: Vec<Vec<u32>>,
}

impl MonitorConfiguration {
	/// Create a new monitor configuration
	pub fn new(layout_mode: MonitorLayoutMode) -> Self {
		Self {
			monitors: Vec::new(),
			layout_mode,
			primary_monitor: None,
			monitor_groups: Vec::new(),
		}
	}

	/// Add a monitor to the configuration
	pub fn add_monitor(&mut self, monitor: Monitor) {
		if monitor.is_primary {
			self.primary_monitor = Some(monitor.id);
		}
		self.monitors.push(monitor);
	}

	/// Get monitor by ID
	pub fn get_monitor(&self, id: u32) -> Option<&Monitor> {
		self.monitors.iter().find(|m| m.id == id)
	}

	/// Get primary monitor
	pub fn primary_monitor(&self) -> Option<&Monitor> {
		if let Some(id) = self.primary_monitor {
			self.get_monitor(id)
		} else {
			self.monitors.first()
		}
	}

	/// Calculate total virtual screen bounds for Unified mode
	pub fn virtual_bounds(&self) -> Option<MonitorBounds> {
		if self.monitors.is_empty() {
			return None;
		}

		let mut min_x = i32::MAX;
		let mut min_y = i32::MAX;
		let mut max_x = i32::MIN;
		let mut max_y = i32::MIN;

		for monitor in &self.monitors {
			let bounds = monitor.bounds();
			min_x = min_x.min(bounds.x);
			min_y = min_y.min(bounds.y);
			max_x = max_x.max(bounds.x + bounds.width as i32);
			max_y = max_y.max(bounds.y + bounds.height as i32);
		}

		Some(MonitorBounds {
			x: min_x,
			y: min_y,
			width: (max_x - min_x) as u32,
			height: (max_y - min_y) as u32,
		})
	}

	/// Find which monitor contains a given point
	pub fn monitor_at_point(&self, x: i32, y: i32) -> Option<&Monitor> {
		self.monitors
			.iter()
			.find(|m| m.bounds().contains_point(x, y))
	}

	/// Get all monitors in a group (for Mixed mode)
	pub fn get_monitor_group(&self, group_index: usize) -> Vec<&Monitor> {
		if group_index >= self.monitor_groups.len() {
			return Vec::new();
		}

		let group_ids = &self.monitor_groups[group_index];
		self.monitors
			.iter()
			.filter(|m| group_ids.contains(&m.id))
			.collect()
	}

	/// Add a monitor group for Mixed mode
	pub fn add_monitor_group(&mut self, monitor_ids: Vec<u32>) {
		self.monitor_groups.push(monitor_ids);
	}
}

impl Default for MonitorConfiguration {
	fn default() -> Self {
		Self::new(MonitorLayoutMode::Unified)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_monitor_creation() {
		let monitor = Monitor::new(1, "Monitor 1".to_string(), (1920, 1080));
		assert_eq!(monitor.id, 1);
		assert_eq!(monitor.name, "Monitor 1");
		assert_eq!(monitor.resolution, (1920, 1080));
		assert_eq!(monitor.scale_factor, 1.0);
		assert!(!monitor.is_primary);
	}

	#[test]
	fn test_monitor_builder() {
		let monitor = Monitor::new(1, "Main".to_string(), (2560, 1440))
			.with_position(0, 0)
			.with_scale_factor(1.5)
			.as_primary()
			.with_refresh_rate(144);

		assert_eq!(monitor.position, (0, 0));
		assert_eq!(monitor.scale_factor, 1.5);
		assert!(monitor.is_primary);
		assert_eq!(monitor.refresh_rate, Some(144));
	}

	#[test]
	fn test_monitor_bounds() {
		let monitor = Monitor::new(1, "Test".to_string(), (1920, 1080))
			.with_position(100, 50);

		let bounds = monitor.bounds();
		assert_eq!(bounds.x, 100);
		assert_eq!(bounds.y, 50);
		assert_eq!(bounds.width, 1920);
		assert_eq!(bounds.height, 1080);
	}

	#[test]
	fn test_bounds_contains_point() {
		let bounds = MonitorBounds::new(0, 0, 1920, 1080);
		assert!(bounds.contains_point(100, 100));
		assert!(bounds.contains_point(0, 0));
		assert!(bounds.contains_point(1919, 1079));
		assert!(!bounds.contains_point(-1, 0));
		assert!(!bounds.contains_point(1920, 1080));
	}

	#[test]
	fn test_bounds_intersects() {
		let bounds1 = MonitorBounds::new(0, 0, 1920, 1080);
		let bounds2 = MonitorBounds::new(1920, 0, 1920, 1080);
		let bounds3 = MonitorBounds::new(100, 100, 500, 500);

		// Adjacent monitors don't intersect
		assert!(!bounds1.intersects(&bounds2));

		// Overlapping monitors intersect
		assert!(bounds1.intersects(&bounds3));
		assert!(bounds3.intersects(&bounds1));
	}

	#[test]
	fn test_monitor_configuration() {
		let mut config = MonitorConfiguration::new(MonitorLayoutMode::Unified);

		let monitor1 = Monitor::new(1, "Monitor 1".to_string(), (1920, 1080))
			.with_position(0, 0)
			.as_primary();

		let monitor2 = Monitor::new(2, "Monitor 2".to_string(), (1920, 1080))
			.with_position(1920, 0);

		config.add_monitor(monitor1);
		config.add_monitor(monitor2);

		assert_eq!(config.monitors.len(), 2);
		assert_eq!(config.primary_monitor, Some(1));
	}

	#[test]
	fn test_virtual_bounds() {
		let mut config = MonitorConfiguration::new(MonitorLayoutMode::Unified);

		config.add_monitor(
			Monitor::new(1, "Left".to_string(), (1920, 1080)).with_position(0, 0),
		);
		config.add_monitor(
			Monitor::new(2, "Right".to_string(), (1920, 1080)).with_position(1920, 0),
		);

		let bounds = config.virtual_bounds().unwrap();
		assert_eq!(bounds.x, 0);
		assert_eq!(bounds.y, 0);
		assert_eq!(bounds.width, 3840); // Two 1920px monitors
		assert_eq!(bounds.height, 1080);
	}

	#[test]
	fn test_monitor_at_point() {
		let mut config = MonitorConfiguration::new(MonitorLayoutMode::Separate);

		config.add_monitor(
			Monitor::new(1, "Left".to_string(), (1920, 1080)).with_position(0, 0),
		);
		config.add_monitor(
			Monitor::new(2, "Right".to_string(), (1920, 1080)).with_position(1920, 0),
		);

		let monitor = config.monitor_at_point(100, 100);
		assert!(monitor.is_some());
		assert_eq!(monitor.unwrap().id, 1);

		let monitor2 = config.monitor_at_point(2000, 100);
		assert!(monitor2.is_some());
		assert_eq!(monitor2.unwrap().id, 2);

		let no_monitor = config.monitor_at_point(5000, 5000);
		assert!(no_monitor.is_none());
	}

	#[test]
	fn test_monitor_groups() {
		let mut config = MonitorConfiguration::new(MonitorLayoutMode::Mixed);

		config.add_monitor(Monitor::new(1, "M1".to_string(), (1920, 1080)));
		config.add_monitor(Monitor::new(2, "M2".to_string(), (1920, 1080)));
		config.add_monitor(Monitor::new(3, "M3".to_string(), (1920, 1080)));

		// Group monitors 1 and 2 together
		config.add_monitor_group(vec![1, 2]);

		let group = config.get_monitor_group(0);
		assert_eq!(group.len(), 2);
		assert!(group.iter().any(|m| m.id == 1));
		assert!(group.iter().any(|m| m.id == 2));
	}

	#[test]
	fn test_primary_monitor() {
		let mut config = MonitorConfiguration::new(MonitorLayoutMode::Unified);

		config.add_monitor(Monitor::new(1, "M1".to_string(), (1920, 1080)));
		config.add_monitor(
			Monitor::new(2, "M2".to_string(), (1920, 1080)).as_primary(),
		);

		let primary = config.primary_monitor();
		assert!(primary.is_some());
		assert_eq!(primary.unwrap().id, 2);
	}
}
