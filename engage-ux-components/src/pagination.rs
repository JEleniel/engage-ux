//! Pagination component for navigating through pages

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Pagination variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaginationVariant {
	Default,
	Rounded,
	Outlined,
}

/// Pagination size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaginationSize {
	Small,
	Medium,
	Large,
}

/// Pagination component
#[derive(Clone, Serialize, Deserialize)]
pub struct Pagination {
	properties: ComponentProperties,
	current_page: usize,
	total_pages: usize,
	sibling_count: usize,
	show_first_last: bool,
	show_prev_next: bool,
	variant: PaginationVariant,
	size: PaginationSize,
	color: Color,
	active_color: Color,
	background_color: Color,
	active_background: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl Pagination {
	/// Create a new pagination component
	pub fn new(id: ComponentId, total_pages: usize) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			current_page: 1,
			total_pages: total_pages.max(1),
			sibling_count: 1,
			show_first_last: true,
			show_prev_next: true,
			variant: PaginationVariant::Default,
			size: PaginationSize::Medium,
			color: Color::from_hex("#000000").unwrap(),
			active_color: Color::from_hex("#FFFFFF").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			active_background: Color::from_hex("#1976D2").unwrap(),
			on_change: None,
		}
	}

	/// Get current page (1-indexed)
	pub fn current_page(&self) -> usize {
		self.current_page
	}

	/// Set current page (1-indexed)
	pub fn set_current_page(&mut self, page: usize) {
		if page >= 1 && page <= self.total_pages {
			self.current_page = page;
		}
	}

	/// Go to next page
	pub fn next_page(&mut self) -> bool {
		if self.current_page < self.total_pages {
			self.current_page += 1;
			true
		} else {
			false
		}
	}

	/// Go to previous page
	pub fn prev_page(&mut self) -> bool {
		if self.current_page > 1 {
			self.current_page -= 1;
			true
		} else {
			false
		}
	}

	/// Go to first page
	pub fn first(&mut self) {
		self.current_page = 1;
	}

	/// Go to last page
	pub fn last(&mut self) {
		self.current_page = self.total_pages;
	}

	/// Get total pages
	pub fn total_pages(&self) -> usize {
		self.total_pages
	}

	/// Set total pages
	pub fn set_total_pages(&mut self, total_pages: usize) {
		self.total_pages = total_pages.max(1);
		if self.current_page > self.total_pages {
			self.current_page = self.total_pages;
		}
	}

	/// Get sibling count (number of page buttons on each side of current)
	pub fn sibling_count(&self) -> usize {
		self.sibling_count
	}

	/// Set sibling count
	pub fn set_sibling_count(&mut self, count: usize) {
		self.sibling_count = count;
	}

	/// Check if first/last buttons are shown
	pub fn shows_first_last(&self) -> bool {
		self.show_first_last
	}

	/// Set whether to show first/last buttons
	pub fn set_show_first_last(&mut self, show: bool) {
		self.show_first_last = show;
	}

	/// Check if prev/next buttons are shown
	pub fn shows_prev_next(&self) -> bool {
		self.show_prev_next
	}

	/// Set whether to show prev/next buttons
	pub fn set_show_prev_next(&mut self, show: bool) {
		self.show_prev_next = show;
	}

	/// Get variant
	pub fn variant(&self) -> PaginationVariant {
		self.variant
	}

	/// Set variant
	pub fn set_variant(&mut self, variant: PaginationVariant) {
		self.variant = variant;
	}

	/// Get size
	pub fn size(&self) -> PaginationSize {
		self.size
	}

	/// Set size
	pub fn set_size(&mut self, size: PaginationSize) {
		self.size = size;
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set active page color
	pub fn set_active_color(&mut self, color: Color) {
		self.active_color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set active page background
	pub fn set_active_background(&mut self, color: Color) {
		self.active_background = color;
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

impl Component for Pagination {
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
	fn test_pagination_creation() {
		let pagination = Pagination::new(1, 10);
		assert_eq!(pagination.id(), 1);
		assert_eq!(pagination.current_page(), 1);
		assert_eq!(pagination.total_pages(), 10);
	}

	#[test]
	fn test_pagination_navigation() {
		let mut pagination = Pagination::new(1, 10);

		assert!(pagination.next_page());
		assert_eq!(pagination.current_page(), 2);

		assert!(pagination.prev_page());
		assert_eq!(pagination.current_page(), 1);

		assert!(!pagination.prev_page()); // Can't go below 1
		assert_eq!(pagination.current_page(), 1);

		pagination.last();
		assert_eq!(pagination.current_page(), 10);

		assert!(!pagination.next_page()); // Can't go above total

		pagination.first();
		assert_eq!(pagination.current_page(), 1);
	}

	#[test]
	fn test_pagination_set_page() {
		let mut pagination = Pagination::new(1, 10);

		pagination.set_current_page(5);
		assert_eq!(pagination.current_page(), 5);

		pagination.set_current_page(15); // Out of range
		assert_eq!(pagination.current_page(), 5); // Should not change
	}

	#[test]
	fn test_pagination_total_pages_update() {
		let mut pagination = Pagination::new(1, 10);
		pagination.set_current_page(8);

		pagination.set_total_pages(5); // Reduce total pages
		assert_eq!(pagination.current_page(), 5); // Should adjust current
	}

	#[test]
	fn test_pagination_sibling_count() {
		let mut pagination = Pagination::new(1, 10);
		assert_eq!(pagination.sibling_count(), 1);

		pagination.set_sibling_count(2);
		assert_eq!(pagination.sibling_count(), 2);
	}

	#[test]
	fn test_pagination_visibility_options() {
		let mut pagination = Pagination::new(1, 10);
		assert!(pagination.shows_first_last());
		assert!(pagination.shows_prev_next());

		pagination.set_show_first_last(false);
		pagination.set_show_prev_next(false);
		assert!(!pagination.shows_first_last());
		assert!(!pagination.shows_prev_next());
	}
}
