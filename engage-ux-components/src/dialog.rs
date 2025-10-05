//! Dialog components (alert, modal, file dialogs)

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Dialog result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogResult {
	Ok,
	Cancel,
	Yes,
	No,
	Custom(u32),
}

/// Alert dialog type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
	Info,
	Warning,
	Error,
	Success,
	Question,
}

/// Alert dialog
#[derive(Clone, Serialize, Deserialize)]
pub struct AlertDialog {
	properties: ComponentProperties,
	title: String,
	message: String,
	alert_type: AlertType,
	visible: bool,
	icon: Option<String>,
	#[serde(skip)]
	on_ok: Option<EventCallback>,
}

impl AlertDialog {
	/// Create a new alert dialog
	pub fn new(id: ComponentId, title: impl Into<String>, message: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: title.into(),
			message: message.into(),
			alert_type: AlertType::Info,
			visible: false,
			icon: None,
			on_ok: None,
		}
	}

	/// Create an info alert
	pub fn info(id: ComponentId, title: impl Into<String>, message: impl Into<String>) -> Self {
		let mut dialog = Self::new(id, title, message);
		dialog.alert_type = AlertType::Info;
		dialog
	}

	/// Create a warning alert
	pub fn warning(id: ComponentId, title: impl Into<String>, message: impl Into<String>) -> Self {
		let mut dialog = Self::new(id, title, message);
		dialog.alert_type = AlertType::Warning;
		dialog
	}

	/// Create an error alert
	pub fn error(id: ComponentId, title: impl Into<String>, message: impl Into<String>) -> Self {
		let mut dialog = Self::new(id, title, message);
		dialog.alert_type = AlertType::Error;
		dialog
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Get message
	pub fn message(&self) -> &str {
		&self.message
	}

	/// Set message
	pub fn set_message(&mut self, message: impl Into<String>) {
		self.message = message.into();
	}

	/// Get alert type
	pub fn alert_type(&self) -> AlertType {
		self.alert_type
	}

	/// Set alert type
	pub fn set_alert_type(&mut self, alert_type: AlertType) {
		self.alert_type = alert_type;
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show dialog
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide dialog
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Set OK callback
	pub fn set_on_ok(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_ok = Some(std::sync::Arc::new(callback));
	}

	/// Handle OK event
	pub fn handle_ok(&self, event: &Event) {
		if let Some(ref callback) = self.on_ok {
			callback(event);
		}
	}
}

impl Component for AlertDialog {
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

/// Confirm dialog with Yes/No or OK/Cancel buttons
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfirmDialog {
	properties: ComponentProperties,
	title: String,
	message: String,
	visible: bool,
	use_yes_no: bool,
	color: Color,
	background_color: Color,
	#[serde(skip)]
	on_confirm: Option<EventCallback>,
	#[serde(skip)]
	on_cancel: Option<EventCallback>,
}

impl ConfirmDialog {
	/// Create a new confirm dialog
	pub fn new(id: ComponentId, title: impl Into<String>, message: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: title.into(),
			message: message.into(),
			visible: false,
			use_yes_no: false,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			on_confirm: None,
			on_cancel: None,
		}
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Get message
	pub fn message(&self) -> &str {
		&self.message
	}

	/// Set message
	pub fn set_message(&mut self, message: impl Into<String>) {
		self.message = message.into();
	}

	/// Check if using Yes/No buttons (vs OK/Cancel)
	pub fn uses_yes_no(&self) -> bool {
		self.use_yes_no
	}

	/// Set whether to use Yes/No buttons
	pub fn set_use_yes_no(&mut self, use_yes_no: bool) {
		self.use_yes_no = use_yes_no;
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show dialog
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide dialog
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Set confirm callback
	pub fn set_on_confirm(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_confirm = Some(std::sync::Arc::new(callback));
	}

	/// Set cancel callback
	pub fn set_on_cancel(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_cancel = Some(std::sync::Arc::new(callback));
	}

	/// Handle confirm event
	pub fn handle_confirm(&self, event: &Event) {
		if let Some(ref callback) = self.on_confirm {
			callback(event);
		}
	}

	/// Handle cancel event
	pub fn handle_cancel(&self, event: &Event) {
		if let Some(ref callback) = self.on_cancel {
			callback(event);
		}
	}
}

impl Component for ConfirmDialog {
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

/// Custom modal dialog
#[derive(Clone, Serialize, Deserialize)]
pub struct Modal {
	properties: ComponentProperties,
	title: String,
	visible: bool,
	closable: bool,
	modal: bool,
	width: Option<f32>,
	height: Option<f32>,
	color: Color,
	background_color: Color,
	overlay_color: Color,
	#[serde(skip)]
	on_close: Option<EventCallback>,
}

impl Modal {
	/// Create a new modal
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			title: String::new(),
			visible: false,
			closable: true,
			modal: true,
			width: None,
			height: None,
			color: Color::from_hex("#000000").unwrap(),
			background_color: Color::from_hex("#FFFFFF").unwrap(),
			overlay_color: Color::from_hex("#00000080").unwrap(), // 50% transparent black
			on_close: None,
		}
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Check if visible
	pub fn is_visible(&self) -> bool {
		self.visible
	}

	/// Show modal
	pub fn show(&mut self) {
		self.visible = true;
	}

	/// Hide modal
	pub fn hide(&mut self) {
		self.visible = false;
	}

	/// Toggle visibility
	pub fn toggle(&mut self) {
		self.visible = !self.visible;
	}

	/// Check if closable
	pub fn is_closable(&self) -> bool {
		self.closable
	}

	/// Set closable
	pub fn set_closable(&mut self, closable: bool) {
		self.closable = closable;
	}

	/// Check if modal (blocks interaction with background)
	pub fn is_modal(&self) -> bool {
		self.modal
	}

	/// Set modal
	pub fn set_modal(&mut self, modal: bool) {
		self.modal = modal;
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

	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set overlay color
	pub fn set_overlay_color(&mut self, color: Color) {
		self.overlay_color = color;
	}

	/// Set close callback
	pub fn set_on_close(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_close = Some(std::sync::Arc::new(callback));
	}

	/// Handle close event
	pub fn handle_close(&self, event: &Event) {
		if let Some(ref callback) = self.on_close {
			callback(event);
		}
	}
}

impl Component for Modal {
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

/// File dialog type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileDialogType {
	Open,
	Save,
	SelectFolder,
}

/// File filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFilter {
	pub name: String,
	pub extensions: Vec<String>,
}

impl FileFilter {
	pub fn new(name: impl Into<String>, extensions: Vec<String>) -> Self {
		Self {
			name: name.into(),
			extensions,
		}
	}
}

/// File dialog
#[derive(Clone, Serialize, Deserialize)]
pub struct FileDialog {
	properties: ComponentProperties,
	dialog_type: FileDialogType,
	title: String,
	filters: Vec<FileFilter>,
	default_path: Option<String>,
	multiple: bool,
	selected_paths: Vec<String>,
	#[serde(skip)]
	on_select: Option<EventCallback>,
	#[serde(skip)]
	on_cancel: Option<EventCallback>,
}

impl FileDialog {
	/// Create a new file dialog
	pub fn new(id: ComponentId, dialog_type: FileDialogType) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			dialog_type,
			title: match dialog_type {
				FileDialogType::Open => "Open File".to_string(),
				FileDialogType::Save => "Save File".to_string(),
				FileDialogType::SelectFolder => "Select Folder".to_string(),
			},
			filters: Vec::new(),
			default_path: None,
			multiple: false,
			selected_paths: Vec::new(),
			on_select: None,
			on_cancel: None,
		}
	}

	/// Create an open file dialog
	pub fn open(id: ComponentId) -> Self {
		Self::new(id, FileDialogType::Open)
	}

	/// Create a save file dialog
	pub fn save(id: ComponentId) -> Self {
		Self::new(id, FileDialogType::Save)
	}

	/// Create a select folder dialog
	pub fn select_folder(id: ComponentId) -> Self {
		Self::new(id, FileDialogType::SelectFolder)
	}

	/// Get dialog type
	pub fn dialog_type(&self) -> FileDialogType {
		self.dialog_type
	}

	/// Get title
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Set title
	pub fn set_title(&mut self, title: impl Into<String>) {
		self.title = title.into();
	}

	/// Add file filter
	pub fn add_filter(&mut self, filter: FileFilter) {
		self.filters.push(filter);
	}

	/// Get filters
	pub fn filters(&self) -> &[FileFilter] {
		&self.filters
	}

	/// Get default path
	pub fn default_path(&self) -> Option<&str> {
		self.default_path.as_deref()
	}

	/// Set default path
	pub fn set_default_path(&mut self, path: Option<String>) {
		self.default_path = path;
	}

	/// Check if multiple selection is enabled
	pub fn allows_multiple(&self) -> bool {
		self.multiple
	}

	/// Set multiple selection
	pub fn set_multiple(&mut self, multiple: bool) {
		self.multiple = multiple;
	}

	/// Get selected paths
	pub fn selected_paths(&self) -> &[String] {
		&self.selected_paths
	}

	/// Set selected paths
	pub fn set_selected_paths(&mut self, paths: Vec<String>) {
		self.selected_paths = paths;
	}

	/// Set select callback
	pub fn set_on_select(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_select = Some(std::sync::Arc::new(callback));
	}

	/// Set cancel callback
	pub fn set_on_cancel(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_cancel = Some(std::sync::Arc::new(callback));
	}

	/// Handle select event
	pub fn handle_select(&self, event: &Event) {
		if let Some(ref callback) = self.on_select {
			callback(event);
		}
	}

	/// Handle cancel event
	pub fn handle_cancel(&self, event: &Event) {
		if let Some(ref callback) = self.on_cancel {
			callback(event);
		}
	}
}

impl Component for FileDialog {
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
	fn test_alert_dialog_creation() {
		let alert = AlertDialog::new(1, "Title", "Message");
		assert_eq!(alert.id(), 1);
		assert_eq!(alert.title(), "Title");
		assert_eq!(alert.message(), "Message");
		assert!(!alert.is_visible());
	}

	#[test]
	fn test_alert_dialog_types() {
		let info = AlertDialog::info(1, "Info", "Message");
		assert_eq!(info.alert_type(), AlertType::Info);

		let warning = AlertDialog::warning(2, "Warning", "Message");
		assert_eq!(warning.alert_type(), AlertType::Warning);

		let error = AlertDialog::error(3, "Error", "Message");
		assert_eq!(error.alert_type(), AlertType::Error);
	}

	#[test]
	fn test_confirm_dialog_creation() {
		let confirm = ConfirmDialog::new(1, "Confirm", "Are you sure?");
		assert_eq!(confirm.id(), 1);
		assert_eq!(confirm.title(), "Confirm");
		assert!(!confirm.uses_yes_no());
	}

	#[test]
	fn test_modal_creation() {
		let modal = Modal::new(1);
		assert_eq!(modal.id(), 1);
		assert!(!modal.is_visible());
		assert!(modal.is_closable());
		assert!(modal.is_modal());
	}

	#[test]
	fn test_modal_visibility() {
		let mut modal = Modal::new(1);
		modal.show();
		assert!(modal.is_visible());
		modal.hide();
		assert!(!modal.is_visible());
	}

	#[test]
	fn test_file_dialog_types() {
		let open = FileDialog::open(1);
		assert_eq!(open.dialog_type(), FileDialogType::Open);

		let save = FileDialog::save(2);
		assert_eq!(save.dialog_type(), FileDialogType::Save);

		let folder = FileDialog::select_folder(3);
		assert_eq!(folder.dialog_type(), FileDialogType::SelectFolder);
	}

	#[test]
	fn test_file_dialog_filters() {
		let mut dialog = FileDialog::open(1);
		dialog.add_filter(FileFilter::new(
			"Images",
			vec!["png".to_string(), "jpg".to_string()],
		));
		assert_eq!(dialog.filters().len(), 1);
	}

	#[test]
	fn test_file_dialog_multiple() {
		let mut dialog = FileDialog::open(1);
		assert!(!dialog.allows_multiple());
		dialog.set_multiple(true);
		assert!(dialog.allows_multiple());
	}
}
