//! Video player component

use engage_ux_core::color::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use engage_ux_core::events::{Event, EventCallback};
use serde::{Deserialize, Serialize};

/// Video state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoState {
	Stopped,
	Playing,
	Paused,
	Buffering,
	Error,
}

/// Video component
#[derive(Clone, Serialize, Deserialize)]
pub struct Video {
	properties: ComponentProperties,
	src: String,
	poster: Option<String>,
	state: VideoState,
	current_time: f64,
	duration: f64,
	volume: f32,
	muted: bool,
	loop_enabled: bool,
	autoplay: bool,
	controls: bool,
	width: Option<f32>,
	height: Option<f32>,
	background_color: Color,
	#[serde(skip)]
	on_play: Option<EventCallback>,
	#[serde(skip)]
	on_pause: Option<EventCallback>,
	#[serde(skip)]
	on_ended: Option<EventCallback>,
	#[serde(skip)]
	on_time_update: Option<EventCallback>,
}

impl Video {
	/// Create a new video player
	pub fn new(id: ComponentId, src: impl Into<String>) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			src: src.into(),
			poster: None,
			state: VideoState::Stopped,
			current_time: 0.0,
			duration: 0.0,
			volume: 1.0,
			muted: false,
			loop_enabled: false,
			autoplay: false,
			controls: true,
			width: None,
			height: None,
			background_color: Color::from_hex("#000000").unwrap(),
			on_play: None,
			on_pause: None,
			on_ended: None,
			on_time_update: None,
		}
	}

	/// Get video source URL
	pub fn src(&self) -> &str {
		&self.src
	}

	/// Set video source URL
	pub fn set_src(&mut self, src: impl Into<String>) {
		self.src = src.into();
		self.state = VideoState::Stopped;
		self.current_time = 0.0;
	}

	/// Get poster image URL
	pub fn poster(&self) -> Option<&str> {
		self.poster.as_deref()
	}

	/// Set poster image URL
	pub fn set_poster(&mut self, poster: Option<String>) {
		self.poster = poster;
	}

	/// Get current state
	pub fn state(&self) -> VideoState {
		self.state
	}

	/// Play video
	pub fn play(&mut self) {
		self.state = VideoState::Playing;
	}

	/// Pause video
	pub fn pause(&mut self) {
		if self.state == VideoState::Playing {
			self.state = VideoState::Paused;
		}
	}

	/// Stop video
	pub fn stop(&mut self) {
		self.state = VideoState::Stopped;
		self.current_time = 0.0;
	}

	/// Get current time in seconds
	pub fn current_time(&self) -> f64 {
		self.current_time
	}

	/// Set current time in seconds (seek)
	pub fn set_current_time(&mut self, time: f64) {
		self.current_time = time.max(0.0).min(self.duration);
	}

	/// Get duration in seconds
	pub fn duration(&self) -> f64 {
		self.duration
	}

	/// Set duration (typically set by player)
	pub fn set_duration(&mut self, duration: f64) {
		self.duration = duration.max(0.0);
	}

	/// Get volume (0.0 to 1.0)
	pub fn volume(&self) -> f32 {
		self.volume
	}

	/// Set volume (0.0 to 1.0)
	pub fn set_volume(&mut self, volume: f32) {
		self.volume = volume.clamp(0.0, 1.0);
	}

	/// Check if muted
	pub fn is_muted(&self) -> bool {
		self.muted
	}

	/// Set muted
	pub fn set_muted(&mut self, muted: bool) {
		self.muted = muted;
	}

	/// Toggle muted
	pub fn toggle_muted(&mut self) {
		self.muted = !self.muted;
	}

	/// Check if loop is enabled
	pub fn is_loop_enabled(&self) -> bool {
		self.loop_enabled
	}

	/// Set loop enabled
	pub fn set_loop_enabled(&mut self, enabled: bool) {
		self.loop_enabled = enabled;
	}

	/// Check if autoplay is enabled
	pub fn is_autoplay(&self) -> bool {
		self.autoplay
	}

	/// Set autoplay
	pub fn set_autoplay(&mut self, autoplay: bool) {
		self.autoplay = autoplay;
	}

	/// Check if controls are shown
	pub fn shows_controls(&self) -> bool {
		self.controls
	}

	/// Set whether to show controls
	pub fn set_controls(&mut self, controls: bool) {
		self.controls = controls;
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

	/// Set background color
	pub fn set_background_color(&mut self, color: Color) {
		self.background_color = color;
	}

	/// Set play callback
	pub fn set_on_play(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_play = Some(std::sync::Arc::new(callback));
	}

	/// Handle play event
	pub fn handle_play(&self, event: &Event) {
		if let Some(ref callback) = self.on_play {
			callback(event);
		}
	}

	/// Set pause callback
	pub fn set_on_pause(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_pause = Some(std::sync::Arc::new(callback));
	}

	/// Handle pause event
	pub fn handle_pause(&self, event: &Event) {
		if let Some(ref callback) = self.on_pause {
			callback(event);
		}
	}

	/// Set ended callback
	pub fn set_on_ended(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_ended = Some(std::sync::Arc::new(callback));
	}

	/// Handle ended event
	pub fn handle_ended(&self, event: &Event) {
		if let Some(ref callback) = self.on_ended {
			callback(event);
		}
	}

	/// Set time update callback
	pub fn set_on_time_update(&mut self, callback: impl Fn(&Event) + Send + Sync + 'static) {
		self.on_time_update = Some(std::sync::Arc::new(callback));
	}

	/// Handle time update event
	pub fn handle_time_update(&self, event: &Event) {
		if let Some(ref callback) = self.on_time_update {
			callback(event);
		}
	}
}

impl Component for Video {
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
	fn test_video_creation() {
		let video = Video::new(1, "video.mp4");
		assert_eq!(video.id(), 1);
		assert_eq!(video.src(), "video.mp4");
		assert_eq!(video.state(), VideoState::Stopped);
	}

	#[test]
	fn test_video_playback() {
		let mut video = Video::new(1, "video.mp4");

		video.play();
		assert_eq!(video.state(), VideoState::Playing);

		video.pause();
		assert_eq!(video.state(), VideoState::Paused);

		video.stop();
		assert_eq!(video.state(), VideoState::Stopped);
		assert_eq!(video.current_time(), 0.0);
	}

	#[test]
	fn test_video_seek() {
		let mut video = Video::new(1, "video.mp4");
		video.set_duration(120.0);

		video.set_current_time(45.0);
		assert_eq!(video.current_time(), 45.0);

		// Should clamp to duration
		video.set_current_time(150.0);
		assert_eq!(video.current_time(), 120.0);
	}

	#[test]
	fn test_video_volume() {
		let mut video = Video::new(1, "video.mp4");
		assert_eq!(video.volume(), 1.0);

		video.set_volume(0.5);
		assert_eq!(video.volume(), 0.5);

		// Should clamp to 0-1
		video.set_volume(1.5);
		assert_eq!(video.volume(), 1.0);
	}

	#[test]
	fn test_video_mute() {
		let mut video = Video::new(1, "video.mp4");
		assert!(!video.is_muted());

		video.set_muted(true);
		assert!(video.is_muted());

		video.toggle_muted();
		assert!(!video.is_muted());
	}

	#[test]
	fn test_video_settings() {
		let mut video = Video::new(1, "video.mp4");
		video.set_loop_enabled(true);
		video.set_autoplay(true);
		video.set_controls(false);

		assert!(video.is_loop_enabled());
		assert!(video.is_autoplay());
		assert!(!video.shows_controls());
	}
}
