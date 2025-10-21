//! Carousel component for cycling through items

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Carousel item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarouselItem {
	pub id: String,
	pub image_url: Option<String>,
	pub title: Option<String>,
	pub description: Option<String>,
}

impl CarouselItem {
	pub fn new(id: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			image_url: None,
			title: None,
			description: None,
		}
	}
}

/// Carousel transition effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CarouselTransition {
	Slide,
	Fade,
	None,
}

/// Carousel component
#[derive(Clone, Serialize, Deserialize)]
pub struct Carousel {
	properties: ComponentProperties,
	items: Vec<CarouselItem>,
	current_index: usize,
	autoplay: bool,
	autoplay_interval_ms: u64,
	loop_enabled: bool,
	show_indicators: bool,
	show_controls: bool,
	transition: CarouselTransition,
	color: Color,
	background_color: Color,
	indicator_color: Color,
	active_indicator_color: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl Carousel {
	/// Create a new carousel
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			items: Vec::new(),
			current_index: 0,
			autoplay: false,
			autoplay_interval_ms: 3000,
			loop_enabled: true,
			show_indicators: true,
			show_controls: true,
			transition: CarouselTransition::Slide,
			color: Color::from_hex("#FFFFFF").unwrap(),
			background_color: Color::from_hex("#000000").unwrap(),
			indicator_color: Color::from_hex("#FFFFFF80").unwrap(),
			active_indicator_color: Color::from_hex("#FFFFFF").unwrap(),
			on_change: None,
		}
	}

	/// Add an item
	pub fn add_item(&mut self, item: CarouselItem) {
		self.items.push(item);
	}

	/// Get items
	pub fn items(&self) -> &[CarouselItem] {
		&self.items
	}

	/// Get items (mutable)
	pub fn items_mut(&mut self) -> &mut Vec<CarouselItem> {
		&mut self.items
	}

	/// Get current index
	pub fn current_index(&self) -> usize {
		self.current_index
	}

	/// Set current index
	pub fn set_current_index(&mut self, index: usize) {
		if index < self.items.len() {
			self.current_index = index;
		}
	}

	/// Go to next item
	pub fn next_item(&mut self) -> bool {
		if self.items.is_empty() {
			return false;
		}

		if self.current_index < self.items.len() - 1 {
			self.current_index += 1;
			true
		} else if self.loop_enabled {
			self.current_index = 0;
			true
		} else {
			false
		}
	}

	/// Go to previous item
	pub fn previous_item(&mut self) -> bool {
		if self.items.is_empty() {
			return false;
		}

		if self.current_index > 0 {
			self.current_index -= 1;
			true
		} else if self.loop_enabled {
			self.current_index = self.items.len() - 1;
			true
		} else {
			false
		}
	}

	/// Check if autoplay is enabled
	pub fn is_autoplay(&self) -> bool {
		self.autoplay
	}

	/// Set autoplay
	pub fn set_autoplay(&mut self, autoplay: bool) {
		self.autoplay = autoplay;
	}

	/// Get autoplay interval in milliseconds
	pub fn autoplay_interval_ms(&self) -> u64 {
		self.autoplay_interval_ms
	}

	/// Set autoplay interval in milliseconds
	pub fn set_autoplay_interval_ms(&mut self, interval_ms: u64) {
		self.autoplay_interval_ms = interval_ms.max(100);
	}

	/// Check if loop is enabled
	pub fn is_loop_enabled(&self) -> bool {
		self.loop_enabled
	}

	/// Set loop enabled
	pub fn set_loop_enabled(&mut self, enabled: bool) {
		self.loop_enabled = enabled;
	}

	/// Check if indicators are shown
	pub fn shows_indicators(&self) -> bool {
		self.show_indicators
	}

	/// Set whether to show indicators
	pub fn set_show_indicators(&mut self, show: bool) {
		self.show_indicators = show;
	}

	/// Check if controls are shown
	pub fn shows_controls(&self) -> bool {
		self.show_controls
	}

	/// Set whether to show controls
	pub fn set_show_controls(&mut self, show: bool) {
		self.show_controls = show;
	}

	/// Get transition effect
	pub fn transition(&self) -> CarouselTransition {
		self.transition
	}

	/// Set transition effect
	pub fn set_transition(&mut self, transition: CarouselTransition) {
		self.transition = transition;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set indicator color
	pub fn set_indicator_color(&mut self, color: Color) {
		self.indicator_color = color;
	}

	/// Set active indicator color
	pub fn set_active_indicator_color(&mut self, color: Color) {
		self.active_indicator_color = color;
	}

	/// Set change callback
	pub fn set_on_change(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_change = Some(std::sync::Arc::new(callback));
	}

	/// Handle change event
	pub fn handle_change(&self, event: &Event) {
		if let Some(ref callback) = self.on_change {
			callback(event);
		}
	}
}

impl Component for Carousel {
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
	fn test_carousel_creation() {
		let carousel = Carousel::new(1);
		assert_eq!(carousel.id(), 1);
		assert_eq!(carousel.items().len(), 0);
		assert_eq!(carousel.current_index(), 0);
	}

	#[test]
	fn test_carousel_add_items() {
		let mut carousel = Carousel::new(1);
		carousel.add_item(CarouselItem::new("item1"));
		carousel.add_item(CarouselItem::new("item2"));
		carousel.add_item(CarouselItem::new("item3"));
		assert_eq!(carousel.items().len(), 3);
	}

	#[test]
	fn test_carousel_navigation() {
		let mut carousel = Carousel::new(1);
		carousel.add_item(CarouselItem::new("item1"));
		carousel.add_item(CarouselItem::new("item2"));
		carousel.add_item(CarouselItem::new("item3"));

		assert!(carousel.next_item());
		assert_eq!(carousel.current_index(), 1);

		assert!(carousel.next_item());
		assert_eq!(carousel.current_index(), 2);

		// Should loop back to start
		assert!(carousel.next_item());
		assert_eq!(carousel.current_index(), 0);
	}

	#[test]
	fn test_carousel_navigation_no_loop() {
		let mut carousel = Carousel::new(1);
		carousel.set_loop_enabled(false);
		carousel.add_item(CarouselItem::new("item1"));
		carousel.add_item(CarouselItem::new("item2"));

		assert!(carousel.next_item());
		assert_eq!(carousel.current_index(), 1);

		// Should not advance past end
		assert!(!carousel.next_item());
		assert_eq!(carousel.current_index(), 1);
	}

	#[test]
	fn test_carousel_previous() {
		let mut carousel = Carousel::new(1);
		carousel.add_item(CarouselItem::new("item1"));
		carousel.add_item(CarouselItem::new("item2"));
		carousel.add_item(CarouselItem::new("item3"));

		// Should loop to end
		assert!(carousel.previous_item());
		assert_eq!(carousel.current_index(), 2);
	}

	#[test]
	fn test_carousel_autoplay() {
		let mut carousel = Carousel::new(1);
		assert!(!carousel.is_autoplay());

		carousel.set_autoplay(true);
		assert!(carousel.is_autoplay());

		carousel.set_autoplay_interval_ms(5000);
		assert_eq!(carousel.autoplay_interval_ms(), 5000);
	}
}
