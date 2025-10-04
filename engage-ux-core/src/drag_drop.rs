//! Drag and drop system for Engage UX
//!
//! Provides a comprehensive drag and drop API supporting drag sources,
//! drop targets, drag data, and drag events.

use crate::component::ComponentId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Drag and drop data type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragDataType {
	/// Plain text data
	Text,
	/// HTML data
	Html,
	/// File paths
	Files,
	/// Image data
	Image,
	/// Custom data type
	Custom(String),
}

/// Drag data container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragData {
	/// Data type
	data_type: DragDataType,
	/// Data content as bytes
	data: Vec<u8>,
	/// Metadata
	metadata: HashMap<String, String>,
}

impl DragData {
	/// Create new drag data
	pub fn new(data_type: DragDataType, data: Vec<u8>) -> Self {
		Self {
			data_type,
			data,
			metadata: HashMap::new(),
		}
	}

	/// Create text drag data
	pub fn text(text: impl Into<String>) -> Self {
		Self::new(DragDataType::Text, text.into().into_bytes())
	}

	/// Create file drag data
	pub fn files(paths: Vec<String>) -> Self {
		let data = serde_json::to_vec(&paths).unwrap_or_default();
		Self::new(DragDataType::Files, data)
	}

	/// Create custom drag data
	pub fn custom(type_name: impl Into<String>, data: Vec<u8>) -> Self {
		Self::new(DragDataType::Custom(type_name.into()), data)
	}

	/// Get data type
	pub fn data_type(&self) -> &DragDataType {
		&self.data_type
	}

	/// Get data as bytes
	pub fn data(&self) -> &[u8] {
		&self.data
	}

	/// Get data as text (if text type)
	pub fn as_text(&self) -> Option<String> {
		if self.data_type == DragDataType::Text {
			String::from_utf8(self.data.clone()).ok()
		} else {
			None
		}
	}

	/// Get data as file paths (if files type)
	pub fn as_files(&self) -> Option<Vec<String>> {
		if self.data_type == DragDataType::Files {
			serde_json::from_slice(&self.data).ok()
		} else {
			None
		}
	}

	/// Add metadata
	pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
		self.metadata.insert(key.into(), value.into());
		self
	}

	/// Get metadata
	pub fn metadata(&self, key: &str) -> Option<&String> {
		self.metadata.get(key)
	}
}

/// Drag operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragOperation {
	/// Copy operation
	Copy,
	/// Move operation
	Move,
	/// Link operation
	Link,
	/// No operation allowed
	None,
}

/// Drag event type
#[derive(Debug, Clone, PartialEq)]
pub enum DragEvent {
	/// Drag operation started
	DragStart {
		source: ComponentId,
		data: DragData,
		x: f32,
		y: f32,
	},
	/// Drag is moving
	DragMove {
		source: ComponentId,
		x: f32,
		y: f32,
	},
	/// Drag entered a drop target
	DragEnter {
		source: ComponentId,
		target: ComponentId,
		x: f32,
		y: f32,
	},
	/// Drag is over a drop target
	DragOver {
		source: ComponentId,
		target: ComponentId,
		x: f32,
		y: f32,
	},
	/// Drag left a drop target
	DragLeave {
		source: ComponentId,
		target: ComponentId,
	},
	/// Item dropped on target
	Drop {
		source: ComponentId,
		target: ComponentId,
		data: DragData,
		operation: DragOperation,
		x: f32,
		y: f32,
	},
	/// Drag operation ended (dropped or cancelled)
	DragEnd {
		source: ComponentId,
		success: bool,
	},
}

/// Drag source trait for components that can be dragged
pub trait DragSource {
	/// Check if component can be dragged
	fn can_drag(&self) -> bool {
		true
	}

	/// Get drag data when drag starts
	fn get_drag_data(&self) -> Option<DragData>;

	/// Get allowed drag operations
	fn allowed_operations(&self) -> Vec<DragOperation> {
		vec![DragOperation::Copy, DragOperation::Move]
	}

	/// Called when drag starts
	fn on_drag_start(&mut self, _x: f32, _y: f32) {}

	/// Called when drag ends
	fn on_drag_end(&mut self, _success: bool) {}
}

/// Drop target trait for components that can receive drops
pub trait DropTarget {
	/// Check if component can accept the drop
	fn can_drop(&self, data: &DragData) -> bool;

	/// Get accepted data types
	fn accepted_types(&self) -> Vec<DragDataType> {
		vec![DragDataType::Text, DragDataType::Files]
	}

	/// Get preferred drop operation
	fn preferred_operation(&self, _data: &DragData) -> DragOperation {
		DragOperation::Copy
	}

	/// Called when drag enters the target
	fn on_drag_enter(&mut self, _data: &DragData, _x: f32, _y: f32) {}

	/// Called when drag is over the target
	fn on_drag_over(&mut self, _data: &DragData, _x: f32, _y: f32) {}

	/// Called when drag leaves the target
	fn on_drag_leave(&mut self) {}

	/// Called when item is dropped on target
	fn on_drop(&mut self, data: DragData, operation: DragOperation, x: f32, y: f32) -> bool;
}

/// Drag state information
#[derive(Debug, Clone)]
struct DragState {
	source: ComponentId,
	data: DragData,
	current_target: Option<ComponentId>,
	operation: DragOperation,
}

/// Drag and drop manager
pub struct DragManager {
	/// Current drag state
	current_drag: Option<DragState>,
	/// Registered drop targets
	drop_targets: HashMap<ComponentId, Arc<RwLock<dyn DropTarget + Send + Sync>>>,
}

impl DragManager {
	/// Create a new drag manager
	pub fn new() -> Self {
		Self {
			current_drag: None,
			drop_targets: HashMap::new(),
		}
	}

	/// Register a drop target
	pub fn register_target(
		&mut self,
		id: ComponentId,
		target: Arc<RwLock<dyn DropTarget + Send + Sync>>,
	) {
		self.drop_targets.insert(id, target);
	}

	/// Unregister a drop target
	pub fn unregister_target(&mut self, id: ComponentId) {
		self.drop_targets.remove(&id);
	}

	/// Start a drag operation
	pub fn start_drag(
		&mut self,
		source: ComponentId,
		data: DragData,
		operation: DragOperation,
	) -> DragEvent {
		self.current_drag = Some(DragState {
			source,
			data: data.clone(),
			current_target: None,
			operation,
		});

		DragEvent::DragStart {
			source,
			data,
			x: 0.0,
			y: 0.0,
		}
	}

	/// Update drag position and check for target
	pub async fn update_drag(&mut self, x: f32, y: f32, target: Option<ComponentId>) -> Option<DragEvent> {
		let drag_state = self.current_drag.as_mut()?;

		// Check if we entered a new target
		if target != drag_state.current_target {
			// Leave old target
			if let Some(old_target) = drag_state.current_target {
				if let Some(target_ref) = self.drop_targets.get(&old_target) {
					if let Ok(mut target) = target_ref.try_write() {
						target.on_drag_leave();
					}
				}

				// Emit leave event
				let event = DragEvent::DragLeave {
					source: drag_state.source,
					target: old_target,
				};

				drag_state.current_target = None;
				return Some(event);
			}

			// Enter new target
			if let Some(new_target) = target {
				if let Some(target_ref) = self.drop_targets.get(&new_target) {
					if let Ok(mut target) = target_ref.try_write() {
						if target.can_drop(&drag_state.data) {
							target.on_drag_enter(&drag_state.data, x, y);
							drag_state.current_target = Some(new_target);

							return Some(DragEvent::DragEnter {
								source: drag_state.source,
								target: new_target,
								x,
								y,
							});
						}
					}
				}
			}
		}

		// Over target
		if let Some(current_target) = drag_state.current_target {
			if let Some(target_ref) = self.drop_targets.get(&current_target) {
				if let Ok(mut target) = target_ref.try_write() {
					target.on_drag_over(&drag_state.data, x, y);
				}
			}

			return Some(DragEvent::DragOver {
				source: drag_state.source,
				target: current_target,
				x,
				y,
			});
		}

		// Just moving
		Some(DragEvent::DragMove {
			source: drag_state.source,
			x,
			y,
		})
	}

	/// Complete drop operation
	pub async fn drop(&mut self, x: f32, y: f32) -> Option<DragEvent> {
		let drag_state = self.current_drag.take()?;

		if let Some(target_id) = drag_state.current_target {
			if let Some(target_ref) = self.drop_targets.get(&target_id) {
				if let Ok(mut target) = target_ref.try_write() {
					let _success =
						target.on_drop(drag_state.data.clone(), drag_state.operation, x, y);

					return Some(DragEvent::Drop {
						source: drag_state.source,
						target: target_id,
						data: drag_state.data,
						operation: drag_state.operation,
						x,
						y,
					});
				}
			}
		}

		Some(DragEvent::DragEnd {
			source: drag_state.source,
			success: false,
		})
	}

	/// Cancel current drag
	pub fn cancel_drag(&mut self) -> Option<DragEvent> {
		let drag_state = self.current_drag.take()?;

		// Leave current target if any
		if let Some(target_id) = drag_state.current_target {
			if let Some(target_ref) = self.drop_targets.get(&target_id) {
				if let Ok(mut target) = target_ref.try_write() {
					target.on_drag_leave();
				}
			}
		}

		Some(DragEvent::DragEnd {
			source: drag_state.source,
			success: false,
		})
	}

	/// Check if drag is in progress
	pub fn is_dragging(&self) -> bool {
		self.current_drag.is_some()
	}

	/// Get current drag source
	pub fn current_source(&self) -> Option<ComponentId> {
		self.current_drag.as_ref().map(|s| s.source)
	}
}

impl Default for DragManager {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_drag_data_text() {
		let data = DragData::text("Hello");
		assert_eq!(data.data_type(), &DragDataType::Text);
		assert_eq!(data.as_text(), Some("Hello".to_string()));
	}

	#[test]
	fn test_drag_data_files() {
		let files = vec!["file1.txt".to_string(), "file2.txt".to_string()];
		let data = DragData::files(files.clone());
		assert_eq!(data.data_type(), &DragDataType::Files);
		assert_eq!(data.as_files(), Some(files));
	}

	#[test]
	fn test_drag_data_metadata() {
		let data = DragData::text("Hello")
			.with_metadata("format", "plain")
			.with_metadata("encoding", "utf-8");
		assert_eq!(data.metadata("format"), Some(&"plain".to_string()));
		assert_eq!(data.metadata("encoding"), Some(&"utf-8".to_string()));
	}

	#[test]
	fn test_drag_manager_creation() {
		let manager = DragManager::new();
		assert!(!manager.is_dragging());
		assert_eq!(manager.current_source(), None);
	}

	#[test]
	fn test_drag_manager_start() {
		let mut manager = DragManager::new();
		let data = DragData::text("Test");
		let event = manager.start_drag(1, data, DragOperation::Copy);

		assert!(manager.is_dragging());
		assert_eq!(manager.current_source(), Some(1));

		match event {
			DragEvent::DragStart { source, .. } => {
				assert_eq!(source, 1);
			}
			_ => panic!("Expected DragStart event"),
		}
	}

	#[test]
	fn test_drag_manager_cancel() {
		let mut manager = DragManager::new();
		let data = DragData::text("Test");
		manager.start_drag(1, data, DragOperation::Copy);

		let event = manager.cancel_drag();
		assert!(!manager.is_dragging());

		match event {
			Some(DragEvent::DragEnd { source, success }) => {
				assert_eq!(source, 1);
				assert!(!success);
			}
			_ => panic!("Expected DragEnd event"),
		}
	}

	struct TestDropTarget {
		can_accept: bool,
		dropped_data: Option<String>,
	}

	impl DropTarget for TestDropTarget {
		fn can_drop(&self, _data: &DragData) -> bool {
			self.can_accept
		}

		fn on_drop(
			&mut self,
			data: DragData,
			_operation: DragOperation,
			_x: f32,
			_y: f32,
		) -> bool {
			self.dropped_data = data.as_text();
			true
		}
	}

	#[tokio::test]
	async fn test_drag_manager_drop() {
		let mut manager = DragManager::new();
		let target = Arc::new(RwLock::new(TestDropTarget {
			can_accept: true,
			dropped_data: None,
		}));

		manager.register_target(2, target.clone());

		let data = DragData::text("Test");
		manager.start_drag(1, data, DragOperation::Copy);

		// Update to target
		let event = manager.update_drag(10.0, 10.0, Some(2)).await;
		assert!(matches!(event, Some(DragEvent::DragEnter { .. })));

		// Drop
		let event = manager.drop(10.0, 10.0).await;
		assert!(matches!(event, Some(DragEvent::Drop { .. })));

		// Check target received data
		let target_lock = target.read().await;
		assert_eq!(target_lock.dropped_data, Some("Test".to_string()));
	}

	#[test]
	fn test_drag_operations() {
		let operations = vec![
			DragOperation::Copy,
			DragOperation::Move,
			DragOperation::Link,
			DragOperation::None,
		];
		assert_eq!(operations.len(), 4);
	}

	#[test]
	fn test_drag_data_types() {
		let types = vec![
			DragDataType::Text,
			DragDataType::Html,
			DragDataType::Files,
			DragDataType::Image,
			DragDataType::Custom("custom".to_string()),
		];
		assert_eq!(types.len(), 5);
	}
}
