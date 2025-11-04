use serde::{Deserialize, Serialize};

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnimationState {
	/// Animation is idle (not started).
	#[default]
	Idle,
	/// Animation is running.
	Running,
	/// Animation is paused.
	Paused,
	/// Animation has completed.
	Completed,
}
