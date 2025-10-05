//! Drag and drop system demonstration
//!
//! Shows drag sources, drop targets, and drag events.

use engage_ux_core::drag_drop::{
	DragData, DragEvent, DragManager, DragOperation, DragSource, DropTarget,
};
use std::sync::Arc;
use tokio::sync::RwLock;

// Example drag source component
struct DraggableItem {
	id: u64,
	name: String,
}

impl DragSource for DraggableItem {
	fn get_drag_data(&self) -> Option<DragData> {
		Some(DragData::text(&self.name).with_metadata("item_id", self.id.to_string()))
	}

	fn on_drag_start(&mut self, x: f32, y: f32) {
		println!(
			"[DragSource] {} started dragging at ({}, {})",
			self.name, x, y
		);
	}

	fn on_drag_end(&mut self, success: bool) {
		println!(
			"[DragSource] {} drag ended - {}",
			self.name,
			if success { "SUCCESS" } else { "CANCELLED" }
		);
	}
}

// Example drop target component
struct DropZone {
	id: u64,
	name: String,
	accepted_items: Vec<String>,
}

impl DropTarget for DropZone {
	fn can_drop(&self, data: &DragData) -> bool {
		if let Some(text) = data.as_text() {
			self.accepted_items.iter().any(|item| item == &text)
		} else {
			false
		}
	}

	fn on_drag_enter(&mut self, data: &DragData, x: f32, y: f32) {
		println!(
			"[DropTarget] {} - Drag entered at ({}, {}): {:?}",
			self.name,
			x,
			y,
			data.as_text()
		);
	}

	fn on_drag_over(&mut self, _data: &DragData, x: f32, y: f32) {
		println!("[DropTarget] {} - Drag over at ({}, {})", self.name, x, y);
	}

	fn on_drag_leave(&mut self) {
		println!("[DropTarget] {} - Drag left", self.name);
	}

	fn on_drop(&mut self, data: DragData, operation: DragOperation, x: f32, y: f32) -> bool {
		println!(
			"[DropTarget] {} - Dropped at ({}, {}): {:?} with {:?}",
			self.name,
			x,
			y,
			data.as_text(),
			operation
		);
		true
	}
}

#[tokio::main]
async fn main() {
	println!("Engage UX - Drag and Drop Demo\n");
	println!("===============================\n");

	// Create drag manager
	let mut manager = DragManager::new();

	// Create drop zones
	let zone1 = Arc::new(RwLock::new(DropZone {
		id: 1,
		name: "Zone 1".to_string(),
		accepted_items: vec!["Item A".to_string(), "Item B".to_string()],
	}));

	let zone2 = Arc::new(RwLock::new(DropZone {
		id: 2,
		name: "Zone 2".to_string(),
		accepted_items: vec!["Item B".to_string(), "Item C".to_string()],
	}));

	// Register drop targets
	manager.register_target(1, zone1.clone());
	manager.register_target(2, zone2.clone());

	println!("Setup complete: 2 drop zones registered\n");

	// Demo 1: Successful drag and drop to Zone 1
	println!("Demo 1: Drag 'Item A' to Zone 1");
	println!("--------------------------------");

	let mut item_a = DraggableItem {
		id: 101,
		name: "Item A".to_string(),
	};

	// Start drag
	if let Some(data) = item_a.get_drag_data() {
		let event = manager.start_drag(101, data, DragOperation::Copy);
		println!("Event: {:?}", event);
		item_a.on_drag_start(10.0, 10.0);

		// Move to zone 1
		if let Some(event) = manager.update_drag(50.0, 50.0, Some(1)).await {
			println!("Event: {:?}", event);
		}

		// Drop
		if let Some(event) = manager.drop(50.0, 50.0).await {
			println!("Event: {:?}", event);
			item_a.on_drag_end(true);
		}
	}
	println!();

	// Demo 2: Drag to incompatible zone
	println!("Demo 2: Drag 'Item A' to Zone 2 (incompatible)");
	println!("-----------------------------------------------");

	// Start drag
	if let Some(data) = item_a.get_drag_data() {
		let event = manager.start_drag(101, data, DragOperation::Copy);
		println!("Event: {:?}", event);
		item_a.on_drag_start(10.0, 10.0);

		// Try to move to zone 2 (should be rejected)
		if let Some(event) = manager.update_drag(150.0, 50.0, Some(2)).await {
			println!("Event: {:?}", event);
		}

		// Cancel (no valid target)
		if let Some(event) = manager.cancel_drag() {
			println!("Event: {:?}", event);
			item_a.on_drag_end(false);
		}
	}
	println!();

	// Demo 3: Drag to compatible zone
	println!("Demo 3: Drag 'Item B' to Zone 2");
	println!("--------------------------------");

	let mut item_b = DraggableItem {
		id: 102,
		name: "Item B".to_string(),
	};

	if let Some(data) = item_b.get_drag_data() {
		let event = manager.start_drag(102, data, DragOperation::Move);
		println!("Event: {:?}", event);
		item_b.on_drag_start(10.0, 10.0);

		// Move to zone 2
		if let Some(event) = manager.update_drag(150.0, 50.0, Some(2)).await {
			println!("Event: {:?}", event);
		}

		// Drop
		if let Some(event) = manager.drop(150.0, 50.0).await {
			println!("Event: {:?}", event);
			item_b.on_drag_end(true);
		}
	}
	println!();

	// Demo 4: Drag with zone switching
	println!("Demo 4: Drag 'Item B' between zones");
	println!("------------------------------------");

	if let Some(data) = item_b.get_drag_data() {
		let event = manager.start_drag(102, data, DragOperation::Copy);
		println!("Event: {:?}", event);
		item_b.on_drag_start(10.0, 10.0);

		// Enter zone 1
		if let Some(event) = manager.update_drag(50.0, 50.0, Some(1)).await {
			println!("Event: {:?}", event);
		}

		// Leave zone 1, enter zone 2
		if let Some(event) = manager.update_drag(150.0, 50.0, Some(2)).await {
			println!("Event: {:?}", event);
		}

		// Drop in zone 2
		if let Some(event) = manager.drop(150.0, 50.0).await {
			println!("Event: {:?}", event);
			item_b.on_drag_end(true);
		}
	}
	println!();

	// Demo 5: File drag
	println!("Demo 5: Drag files");
	println!("------------------");

	let files = vec![
		"/path/to/file1.txt".to_string(),
		"/path/to/file2.txt".to_string(),
	];
	let file_data = DragData::files(files);

	let event = manager.start_drag(103, file_data, DragOperation::Copy);
	println!("Event: {:?}", event);

	if let Some(event) = manager.cancel_drag() {
		println!("Event: {:?}", event);
	}
	println!();

	// Demo 6: Custom data
	println!("Demo 6: Custom drag data");
	println!("------------------------");

	let custom_data = DragData::custom("widget", vec![1, 2, 3, 4])
		.with_metadata("widget_type", "button")
		.with_metadata("widget_id", "btn_123");

	println!("Custom data type: {:?}", custom_data.data_type());
	println!("Metadata:");
	println!("  widget_type: {:?}", custom_data.metadata("widget_type"));
	println!("  widget_id: {:?}", custom_data.metadata("widget_id"));
	println!();

	println!("Drag and drop demo complete!");
}
