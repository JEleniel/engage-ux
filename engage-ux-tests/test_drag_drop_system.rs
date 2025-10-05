//! Integration tests for the drag and drop system

use engage_ux_core::drag_drop::{
	DragData, DragDataType, DragEvent, DragManager, DragOperation, DragSource, DropTarget,
};
use std::sync::Arc;
use tokio::sync::RwLock;

// Test drag source
struct TestSource {
	id: u64,
	data: String,
	drag_started: bool,
	drag_ended: bool,
	drag_success: bool,
}

impl TestSource {
	fn new(id: u64, data: impl Into<String>) -> Self {
		Self {
			id,
			data: data.into(),
			drag_started: false,
			drag_ended: false,
			drag_success: false,
		}
	}
}

impl DragSource for TestSource {
	fn get_drag_data(&self) -> Option<DragData> {
		Some(DragData::text(&self.data))
	}

	fn on_drag_start(&mut self, _x: f32, _y: f32) {
		self.drag_started = true;
	}

	fn on_drag_end(&mut self, success: bool) {
		self.drag_ended = true;
		self.drag_success = success;
	}
}

// Test drop target
struct TestTarget {
	id: u64,
	accepted_types: Vec<DragDataType>,
	enter_count: usize,
	over_count: usize,
	leave_count: usize,
	drop_count: usize,
	last_dropped: Option<String>,
}

impl TestTarget {
	fn new(id: u64) -> Self {
		Self {
			id,
			accepted_types: vec![DragDataType::Text],
			enter_count: 0,
			over_count: 0,
			leave_count: 0,
			drop_count: 0,
			last_dropped: None,
		}
	}
}

impl DropTarget for TestTarget {
	fn can_drop(&self, data: &DragData) -> bool {
		self.accepted_types.contains(data.data_type())
	}

	fn on_drag_enter(&mut self, _data: &DragData, _x: f32, _y: f32) {
		self.enter_count += 1;
	}

	fn on_drag_over(&mut self, _data: &DragData, _x: f32, _y: f32) {
		self.over_count += 1;
	}

	fn on_drag_leave(&mut self) {
		self.leave_count += 1;
	}

	fn on_drop(&mut self, data: DragData, _operation: DragOperation, _x: f32, _y: f32) -> bool {
		self.drop_count += 1;
		self.last_dropped = data.as_text();
		true
	}
}

#[tokio::test]
async fn test_basic_drag_and_drop() {
	let mut manager = DragManager::new();
	let mut source = TestSource::new(1, "test data");
	let target = Arc::new(RwLock::new(TestTarget::new(2)));

	// Register target
	manager.register_target(2, target.clone());

	// Start drag
	let data = source.get_drag_data().unwrap();
	source.on_drag_start(0.0, 0.0);
	let event = manager.start_drag(1, data, DragOperation::Copy);

	assert!(matches!(event, DragEvent::DragStart { .. }));
	assert!(manager.is_dragging());
	assert_eq!(manager.current_source(), Some(1));
	assert!(source.drag_started);

	// Move over target
	let event = manager.update_drag(10.0, 10.0, Some(2)).await;
	assert!(matches!(event, Some(DragEvent::DragEnter { .. })));

	// Verify target received enter event
	{
		let target_ref = target.read().await;
		assert_eq!(target_ref.enter_count, 1);
	}

	// Drop
	let event = manager.drop(10.0, 10.0).await;
	assert!(matches!(event, Some(DragEvent::Drop { .. })));

	// Verify drop was received
	{
		let target_ref = target.read().await;
		assert_eq!(target_ref.drop_count, 1);
		assert_eq!(target_ref.last_dropped, Some("test data".to_string()));
	}

	assert!(!manager.is_dragging());
}

#[tokio::test]
async fn test_drag_between_targets() {
	let mut manager = DragManager::new();
	let target1 = Arc::new(RwLock::new(TestTarget::new(1)));
	let target2 = Arc::new(RwLock::new(TestTarget::new(2)));

	manager.register_target(1, target1.clone());
	manager.register_target(2, target2.clone());

	// Start drag
	let data = DragData::text("test");
	manager.start_drag(99, data, DragOperation::Copy);

	// Enter first target
	let event = manager.update_drag(10.0, 10.0, Some(1)).await;
	assert!(matches!(
		event,
		Some(DragEvent::DragEnter { target: 1, .. })
	));

	{
		let t1 = target1.read().await;
		assert_eq!(t1.enter_count, 1);
	}

	// Move to second target
	let event = manager.update_drag(20.0, 10.0, Some(2)).await;
	assert!(matches!(
		event,
		Some(DragEvent::DragLeave { target: 1, .. })
	));

	// Verify first target received leave
	{
		let t1 = target1.read().await;
		assert_eq!(t1.leave_count, 1);
	}

	// Next update should be enter on second target
	let event = manager.update_drag(20.0, 10.0, Some(2)).await;
	assert!(matches!(
		event,
		Some(DragEvent::DragEnter { target: 2, .. })
	));

	{
		let t2 = target2.read().await;
		assert_eq!(t2.enter_count, 1);
	}
}

#[tokio::test]
async fn test_drag_cancel() {
	let mut manager = DragManager::new();
	let target = Arc::new(RwLock::new(TestTarget::new(1)));

	manager.register_target(1, target.clone());

	// Start drag
	let data = DragData::text("test");
	manager.start_drag(99, data, DragOperation::Copy);

	// Enter target
	manager.update_drag(10.0, 10.0, Some(1)).await;

	// Cancel
	let event = manager.cancel_drag();
	assert!(matches!(
		event,
		Some(DragEvent::DragEnd { success: false, .. })
	));

	// Verify target received leave
	{
		let target_ref = target.read().await;
		assert_eq!(target_ref.leave_count, 1);
		assert_eq!(target_ref.drop_count, 0); // No drop
	}

	assert!(!manager.is_dragging());
}

#[test]
fn test_drag_data_types() {
	// Text data
	let text_data = DragData::text("Hello");
	assert_eq!(text_data.data_type(), &DragDataType::Text);
	assert_eq!(text_data.as_text(), Some("Hello".to_string()));

	// File data
	let files = vec!["file1.txt".to_string(), "file2.txt".to_string()];
	let file_data = DragData::files(files.clone());
	assert_eq!(file_data.data_type(), &DragDataType::Files);
	assert_eq!(file_data.as_files(), Some(files));

	// Custom data
	let custom_data = DragData::custom("widget", vec![1, 2, 3]);
	match custom_data.data_type() {
		DragDataType::Custom(name) => assert_eq!(name, "widget"),
		_ => panic!("Expected Custom type"),
	}
}

#[test]
fn test_drag_data_metadata() {
	let data = DragData::text("test")
		.with_metadata("source", "component_1")
		.with_metadata("timestamp", "1234567890");

	assert_eq!(data.metadata("source"), Some(&"component_1".to_string()));
	assert_eq!(data.metadata("timestamp"), Some(&"1234567890".to_string()));
	assert_eq!(data.metadata("nonexistent"), None);
}

#[tokio::test]
async fn test_drag_over_events() {
	let mut manager = DragManager::new();
	let target = Arc::new(RwLock::new(TestTarget::new(1)));

	manager.register_target(1, target.clone());

	// Start drag
	let data = DragData::text("test");
	manager.start_drag(99, data, DragOperation::Copy);

	// Enter target
	manager.update_drag(10.0, 10.0, Some(1)).await;

	// Multiple over events
	for _ in 0..5 {
		let event = manager.update_drag(10.0, 10.0, Some(1)).await;
		assert!(matches!(event, Some(DragEvent::DragOver { .. })));
	}

	// Verify over count
	{
		let target_ref = target.read().await;
		assert_eq!(target_ref.over_count, 5);
	}
}

#[tokio::test]
async fn test_drop_target_filtering() {
	let mut manager = DragManager::new();

	// Target that only accepts files
	struct FileOnlyTarget {
		drop_count: usize,
	}

	impl DropTarget for FileOnlyTarget {
		fn can_drop(&self, data: &DragData) -> bool {
			matches!(data.data_type(), DragDataType::Files)
		}

		fn accepted_types(&self) -> Vec<DragDataType> {
			vec![DragDataType::Files]
		}

		fn on_drop(
			&mut self,
			_data: DragData,
			_operation: DragOperation,
			_x: f32,
			_y: f32,
		) -> bool {
			self.drop_count += 1;
			true
		}
	}

	let target = Arc::new(RwLock::new(FileOnlyTarget { drop_count: 0 }));
	manager.register_target(1, target.clone());

	// Try to drop text (should be rejected)
	let text_data = DragData::text("test");
	manager.start_drag(99, text_data, DragOperation::Copy);

	let event = manager.update_drag(10.0, 10.0, Some(1)).await;
	// Should not enter because can_drop returns false
	assert!(matches!(event, Some(DragEvent::DragMove { .. })));

	manager.cancel_drag();

	// Try to drop files (should be accepted)
	let file_data = DragData::files(vec!["test.txt".to_string()]);
	manager.start_drag(99, file_data, DragOperation::Copy);

	let event = manager.update_drag(10.0, 10.0, Some(1)).await;
	assert!(matches!(event, Some(DragEvent::DragEnter { .. })));

	manager.drop(10.0, 10.0).await;

	{
		let target_ref = target.read().await;
		assert_eq!(target_ref.drop_count, 1);
	}
}

#[test]
fn test_drag_operations() {
	// Test different drag operations
	let operations = vec![
		DragOperation::Copy,
		DragOperation::Move,
		DragOperation::Link,
		DragOperation::None,
	];

	for op in operations {
		let data = DragData::text("test");
		let mut manager = DragManager::new();
		manager.start_drag(1, data, op);
		assert!(manager.is_dragging());
	}
}
