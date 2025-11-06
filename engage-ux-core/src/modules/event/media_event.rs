use serde::{Deserialize, Serialize};

/// Represents various media-related events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaEvent {
	/// Event indicating that media playback has started.
	Play { title: String },
	/// Event indicating that media playback is paused.
	Pause { title: String },
	/// Event indicating that media playback has stopped.
	Stop { title: String },
	/// Event indicating that media playback has been seeked.
	Seek { title: String, position: f32 },
	/// Event indicating that media playback has ended.
	Ended { title: String },
	/// Event indicating that the media volume has changed.
	VolumeChange { volume: f32 },
}
