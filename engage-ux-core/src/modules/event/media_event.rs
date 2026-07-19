use serde::{Deserialize, Serialize};

/// Represents various media-related events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaEvent {
	/// Event indicating that media playback has started.
	Play {
		/// Title or identifier of the media item played
		title: String,
	},
	/// Event indicating that media playback is paused.
	Pause {
		/// Title or identifier of the media item paused
		title: String,
	},
	/// Event indicating that media playback has stopped.
	Stop {
		/// Title or identifier of the media item stopped
		title: String,
	},
	/// Event indicating that media playback has been seeked.
	Seek {
		/// Title or identifier of the media item
		title: String,
		/// New playback position in seconds
		position: f32,
	},
	/// Event indicating that media playback has ended.
	Ended {
		/// Title or identifier of the media item ended
		title: String,
	},
	/// Event indicating that the media volume has changed.
	VolumeChange {
		/// New volume level (0.0-1.0)
		volume: f32,
	},
}
