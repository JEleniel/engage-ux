//! Table component for structured data display

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};

/// Table cell data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
	pub value: String,
	pub align: TextAlign,
	pub color: Option<Color>,
}

impl TableCell {
	pub fn new(value: impl Into<String>) -> Self {
		Self {
			value: value.into(),
			align: TextAlign::Left,
			color: None,
		}
	}
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
	Left,
	Center,
	Right,
}

/// Table column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
	pub key: String,
	pub header: String,
	pub width: Option<f32>,
	pub sortable: bool,
	pub align: TextAlign,
}

impl TableColumn {
	pub fn new(key: impl Into<String>, header: impl Into<String>) -> Self {
		Self {
			key: key.into(),
			header: header.into(),
			width: None,
			sortable: false,
			align: TextAlign::Left,
		}
	}
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortDirection {
	Ascending,
	Descending,
}

/// Table component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
	properties: ComponentProperties,
	columns: Vec<TableColumn>,
	rows: Vec<Vec<TableCell>>,
	sortable: bool,
	sort_column: Option<usize>,
	sort_direction: Option<SortDirection>,
	selectable: bool,
	selected_rows: Vec<usize>,
	striped: bool,
	bordered: bool,
	hover: bool,
	color: Color,
	background_color: Color,
	header_background: Color,
	stripe_color: Color,
	border_color: Color,
}

impl Table {
	/// Create a new table
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			columns: Vec::new(),
			rows: Vec::new(),
			sortable: false,
			sort_column: None,
			sort_direction: None,
			selectable: false,
			selected_rows: Vec::new(),
			striped: true,
			bordered: true,
			hover: true,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			header_background: Color::from_hex("#F5F5F5").unwrap(),
			stripe_color: Color::from_hex("#FAFAFA").unwrap(),
			border_color: Color::from_hex("#E0E0E0").unwrap(),
		}
	}

	/// Add a column
	pub fn add_column(&mut self, column: TableColumn) {
		self.columns.push(column);
	}

	/// Set columns
	pub fn set_columns(&mut self, columns: Vec<TableColumn>) {
		self.columns = columns;
	}

	/// Get columns
	pub fn columns(&self) -> &[TableColumn] {
		&self.columns
	}

	/// Add a row
	pub fn add_row(&mut self, row: Vec<TableCell>) {
		self.rows.push(row);
	}

	/// Set rows
	pub fn set_rows(&mut self, rows: Vec<Vec<TableCell>>) {
		self.rows = rows;
	}

	/// Get rows
	pub fn rows(&self) -> &[Vec<TableCell>] {
		&self.rows
	}

	/// Get cell at position
	pub fn cell_at(&self, row: usize, col: usize) -> Option<&TableCell> {
		self.rows.get(row).and_then(|r| r.get(col))
	}

	/// Set sortable
	pub fn set_sortable(&mut self, sortable: bool) {
		self.sortable = sortable;
	}

	/// Check if sortable
	pub fn is_sortable(&self) -> bool {
		self.sortable
	}

	/// Sort by column
	pub fn sort_by(&mut self, column: usize, direction: SortDirection) {
		if column < self.columns.len() && self.columns[column].sortable {
			self.sort_column = Some(column);
			self.sort_direction = Some(direction);
		}
	}

	/// Get sort column
	pub fn sort_column(&self) -> Option<usize> {
		self.sort_column
	}

	/// Get sort direction
	pub fn sort_direction(&self) -> Option<SortDirection> {
		self.sort_direction
	}

	/// Set selectable
	pub fn set_selectable(&mut self, selectable: bool) {
		self.selectable = selectable;
	}

	/// Check if selectable
	pub fn is_selectable(&self) -> bool {
		self.selectable
	}

	/// Select row
	pub fn select_row(&mut self, row: usize) {
		if row < self.rows.len() && !self.selected_rows.contains(&row) {
			self.selected_rows.push(row);
		}
	}

	/// Deselect row
	pub fn deselect_row(&mut self, row: usize) {
		self.selected_rows.retain(|&r| r != row);
	}

	/// Get selected rows
	pub fn selected_rows(&self) -> &[usize] {
		&self.selected_rows
	}

	/// Set striped
	pub fn set_striped(&mut self, striped: bool) {
		self.striped = striped;
	}

	/// Check if striped
	pub fn is_striped(&self) -> bool {
		self.striped
	}

	/// Set bordered
	pub fn set_bordered(&mut self, bordered: bool) {
		self.bordered = bordered;
	}

	/// Check if bordered
	pub fn is_bordered(&self) -> bool {
		self.bordered
	}

	/// Set hover effect
	pub fn set_hover(&mut self, hover: bool) {
		self.hover = hover;
	}

	/// Check if hover effect enabled
	pub fn has_hover(&self) -> bool {
		self.hover
	}

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set header background color
	pub fn set_header_background(&mut self, color: Color) {
		self.header_background = color;
	}

	/// Set stripe color
	pub fn set_stripe_color(&mut self, color: Color) {
		self.stripe_color = color;
	}

	/// Set border color
	pub fn set_border_color(&mut self, color: Color) {
		self.border_color = color;
	}
}

impl Component for Table {
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
	fn test_table_creation() {
		let table = Table::new(1);
		assert_eq!(table.id(), 1);
		assert_eq!(table.columns().len(), 0);
		assert_eq!(table.rows().len(), 0);
	}

	#[test]
	fn test_table_columns() {
		let mut table = Table::new(1);
		table.add_column(TableColumn::new("id", "ID"));
		table.add_column(TableColumn::new("name", "Name"));
		assert_eq!(table.columns().len(), 2);
	}

	#[test]
	fn test_table_rows() {
		let mut table = Table::new(1);
		table.add_row(vec![TableCell::new("1"), TableCell::new("John")]);
		table.add_row(vec![TableCell::new("2"), TableCell::new("Jane")]);
		assert_eq!(table.rows().len(), 2);
	}

	#[test]
	fn test_table_cell_access() {
		let mut table = Table::new(1);
		table.add_row(vec![TableCell::new("1"), TableCell::new("John")]);
		
		let cell = table.cell_at(0, 1);
		assert!(cell.is_some());
		assert_eq!(cell.unwrap().value, "John");
	}

	#[test]
	fn test_table_selection() {
		let mut table = Table::new(1);
		table.set_selectable(true);
		table.add_row(vec![TableCell::new("1")]);
		table.add_row(vec![TableCell::new("2")]);
		
		table.select_row(0);
		assert_eq!(table.selected_rows().len(), 1);
		
		table.deselect_row(0);
		assert_eq!(table.selected_rows().len(), 0);
	}

	#[test]
	fn test_table_styling() {
		let mut table = Table::new(1);
		assert!(table.is_striped());
		assert!(table.is_bordered());
		
		table.set_striped(false);
		table.set_bordered(false);
		assert!(!table.is_striped());
		assert!(!table.is_bordered());
	}
}
