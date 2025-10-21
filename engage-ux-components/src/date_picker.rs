//! Date picker component

use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Date struct
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Date {
	pub year: i32,
	pub month: u8, // 1-12
	pub day: u8,   // 1-31
}

impl Date {
	pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
		if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
			return None;
		}
		Some(Self { year, month, day })
	}

	pub fn is_valid(&self) -> bool {
		if self.month < 1 || self.month > 12 || self.day < 1 {
			return false;
		}

		let days_in_month = match self.month {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			4 | 6 | 9 | 11 => 30,
			2 => {
				if self.is_leap_year() {
					29
				} else {
					28
				}
			}
			_ => return false,
		};

		self.day <= days_in_month
	}

	pub fn is_leap_year(&self) -> bool {
		(self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
	}
}

/// Date picker view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatePickerView {
	Days,
	Months,
	Years,
}

/// Date picker component
#[derive(Clone, Serialize, Deserialize)]
pub struct DatePicker {
	properties: ComponentProperties,
	selected_date: Option<Date>,
	min_date: Option<Date>,
	max_date: Option<Date>,
	view: DatePickerView,
	viewing_month: u8,
	viewing_year: i32,
	open: bool,
	show_week_numbers: bool,
	first_day_of_week: u8, // 0 = Sunday, 1 = Monday, etc.
	color: Color,
	background_color: Color,
	selected_color: Color,
	selected_background: Color,
	today_color: Color,
	disabled_color: Color,
	#[serde(skip)]
	on_change: Option<EventCallback>,
}

impl DatePicker {
	/// Create a new date picker
	pub fn new(id: ComponentId) -> Self {
		let now = Self::today();
		Self {
			properties: ComponentProperties::new(id),
			selected_date: None,
			min_date: None,
			max_date: None,
			view: DatePickerView::Days,
			viewing_month: now.month,
			viewing_year: now.year,
			open: false,
			show_week_numbers: false,
			first_day_of_week: 0,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			selected_color: Color::from_hex("#FFFFFF").unwrap(),
			selected_background: Color::from_hex("#1976D2").unwrap(),
			today_color: Color::from_hex("#1976D2").unwrap(),
			disabled_color: Color::from_hex("#CCCCCC").unwrap(),
			on_change: None,
		}
	}

	/// Get today's date (simplified - in real implementation would use system time)
	fn today() -> Date {
		Date {
			year: 2024,
			month: 1,
			day: 1,
		}
	}

	/// Get selected date
	pub fn selected_date(&self) -> Option<Date> {
		self.selected_date
	}

	/// Set selected date
	pub fn set_selected_date(&mut self, date: Option<Date>) {
		if let Some(d) = date {
			if !d.is_valid() {
				return;
			}
			if let Some(min) = self.min_date
				&& (d.year < min.year
					|| (d.year == min.year && d.month < min.month)
					|| (d.year == min.year && d.month == min.month && d.day < min.day))
			{
				return;
			}
			if let Some(max) = self.max_date
				&& (d.year > max.year
					|| (d.year == max.year && d.month > max.month)
					|| (d.year == max.year && d.month == max.month && d.day > max.day))
			{
				return;
			}
		}
		self.selected_date = date;
	}

	/// Get minimum date
	pub fn min_date(&self) -> Option<Date> {
		self.min_date
	}

	/// Set minimum date
	pub fn set_min_date(&mut self, date: Option<Date>) {
		self.min_date = date;
	}

	/// Get maximum date
	pub fn max_date(&self) -> Option<Date> {
		self.max_date
	}

	/// Set maximum date
	pub fn set_max_date(&mut self, date: Option<Date>) {
		self.max_date = date;
	}

	/// Get current view
	pub fn view(&self) -> DatePickerView {
		self.view
	}

	/// Set current view
	pub fn set_view(&mut self, view: DatePickerView) {
		self.view = view;
	}

	/// Get viewing month
	pub fn viewing_month(&self) -> u8 {
		self.viewing_month
	}

	/// Set viewing month
	pub fn set_viewing_month(&mut self, month: u8) {
		if (1..=12).contains(&month) {
			self.viewing_month = month;
		}
	}

	/// Get viewing year
	pub fn viewing_year(&self) -> i32 {
		self.viewing_year
	}

	/// Set viewing year
	pub fn set_viewing_year(&mut self, year: i32) {
		self.viewing_year = year;
	}

	/// Navigate to next month
	pub fn next_month(&mut self) {
		if self.viewing_month == 12 {
			self.viewing_month = 1;
			self.viewing_year += 1;
		} else {
			self.viewing_month += 1;
		}
	}

	/// Navigate to previous month
	pub fn previous_month(&mut self) {
		if self.viewing_month == 1 {
			self.viewing_month = 12;
			self.viewing_year -= 1;
		} else {
			self.viewing_month -= 1;
		}
	}

	/// Check if open
	pub fn is_open(&self) -> bool {
		self.open
	}

	/// Open picker
	pub fn open(&mut self) {
		self.open = true;
	}

	/// Close picker
	pub fn close(&mut self) {
		self.open = false;
	}

	/// Toggle picker
	pub fn toggle(&mut self) {
		self.open = !self.open;
	}

	/// Check if week numbers are shown
	pub fn shows_week_numbers(&self) -> bool {
		self.show_week_numbers
	}

	/// Set whether to show week numbers
	pub fn set_show_week_numbers(&mut self, show: bool) {
		self.show_week_numbers = show;
	}

	/// Get first day of week
	pub fn first_day_of_week(&self) -> u8 {
		self.first_day_of_week
	}

	/// Set first day of week (0 = Sunday, 1 = Monday, etc.)
	pub fn set_first_day_of_week(&mut self, day: u8) {
		if day <= 6 {
			self.first_day_of_week = day;
		}
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set selected date color
	pub fn set_selected_color(&mut self, color: Color) {
		self.selected_color = color;
	}

	/// Set selected date background
	pub fn set_selected_background(&mut self, color: Color) {
		self.selected_background = color;
	}

	/// Set today's date color
	pub fn set_today_color(&mut self, color: Color) {
		self.today_color = color;
	}

	/// Set disabled date color
	pub fn set_disabled_color(&mut self, color: Color) {
		self.disabled_color = color;
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

impl Component for DatePicker {
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
	fn test_date_creation() {
		let date = Date::new(2024, 1, 15).unwrap();
		assert_eq!(date.year, 2024);
		assert_eq!(date.month, 1);
		assert_eq!(date.day, 15);
		assert!(date.is_valid());
	}

	#[test]
	fn test_date_invalid() {
		assert!(Date::new(2024, 13, 1).is_none());
		assert!(Date::new(2024, 1, 32).is_none());
	}

	#[test]
	fn test_date_leap_year() {
		let date = Date::new(2024, 2, 29).unwrap();
		assert!(date.is_leap_year());
		assert!(date.is_valid());

		let date = Date::new(2023, 2, 29).unwrap();
		assert!(!date.is_leap_year());
		assert!(!date.is_valid());
	}

	#[test]
	fn test_date_picker_creation() {
		let picker = DatePicker::new(1);
		assert_eq!(picker.id(), 1);
		assert_eq!(picker.selected_date(), None);
	}

	#[test]
	fn test_date_picker_selection() {
		let mut picker = DatePicker::new(1);
		let date = Date::new(2024, 6, 15).unwrap();
		picker.set_selected_date(Some(date));
		assert_eq!(picker.selected_date(), Some(date));
	}

	#[test]
	fn test_date_picker_navigation() {
		let mut picker = DatePicker::new(1);
		picker.set_viewing_month(1);
		picker.set_viewing_year(2024);

		picker.next_month();
		assert_eq!(picker.viewing_month(), 2);
		assert_eq!(picker.viewing_year(), 2024);

		picker.previous_month();
		assert_eq!(picker.viewing_month(), 1);
		assert_eq!(picker.viewing_year(), 2024);
	}

	#[test]
	fn test_date_picker_year_wrap() {
		let mut picker = DatePicker::new(1);
		picker.set_viewing_month(12);
		picker.set_viewing_year(2024);

		picker.next_month();
		assert_eq!(picker.viewing_month(), 1);
		assert_eq!(picker.viewing_year(), 2025);
	}

	#[test]
	fn test_date_picker_min_max() {
		let mut picker = DatePicker::new(1);
		picker.set_min_date(Date::new(2024, 1, 1));
		picker.set_max_date(Date::new(2024, 12, 31));

		// Should accept valid date
		picker.set_selected_date(Date::new(2024, 6, 15));
		assert!(picker.selected_date().is_some());

		// Should reject date before min
		picker.set_selected_date(Date::new(2023, 12, 31));
		assert_eq!(picker.selected_date(), Date::new(2024, 6, 15));
	}
}
