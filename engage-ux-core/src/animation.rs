//! Animation system for Engage UX
//!
//! Provides a flexible animation framework supporting various animation types,
//! easing functions, and animation composition.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnimationState {
	/// Animation is idle (not started)
	#[default]
	Idle,
	/// Animation is running
	Running,
	/// Animation is paused
	Paused,
	/// Animation has completed
	Completed,
}

/// Easing function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Easing {
	/// Linear interpolation (constant speed)
	Linear,
	/// Ease in (slow start, accelerate)
	EaseIn,
	/// Ease out (fast start, decelerate)
	EaseOut,
	/// Ease in and out (slow start and end)
	EaseInOut,
	/// Cubic bezier easing (slow start, fast middle, slow end)
	CubicBezier,
	/// Elastic easing (bouncy effect)
	Elastic,
	/// Bounce easing
	Bounce,
}

impl Easing {
	/// Apply the easing function to a progress value (0.0 to 1.0)
	pub fn apply(&self, t: f32) -> f32 {
		let t = t.clamp(0.0, 1.0);
		match self {
			Easing::Linear => t,
			Easing::EaseIn => t * t,
			Easing::EaseOut => t * (2.0 - t),
			Easing::EaseInOut => {
				if t < 0.5 {
					2.0 * t * t
				} else {
					-1.0 + (4.0 - 2.0 * t) * t
				}
			}
			Easing::CubicBezier => t * t * (3.0 - 2.0 * t),
			Easing::Elastic => {
				if t == 0.0 || t == 1.0 {
					t
				} else {
					let p = 0.3;
					let s = p / 4.0;
					-(2.0_f32.powf(10.0 * (t - 1.0))
						* ((t - 1.0 - s) * (2.0 * std::f32::consts::PI) / p).sin())
				}
			}
			Easing::Bounce => {
				if t < 1.0 / 2.75 {
					7.5625 * t * t
				} else if t < 2.0 / 2.75 {
					let t = t - 1.5 / 2.75;
					7.5625 * t * t + 0.75
				} else if t < 2.5 / 2.75 {
					let t = t - 2.25 / 2.75;
					7.5625 * t * t + 0.9375
				} else {
					let t = t - 2.625 / 2.75;
					7.5625 * t * t + 0.984375
				}
			}
		}
	}
}

/// Animation type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationType {
	/// Fade animation (opacity)
	Fade { from: f32, to: f32 },
	/// Slide animation (position)
	Slide {
		from_x: f32,
		from_y: f32,
		to_x: f32,
		to_y: f32,
	},
	/// Scale animation (size)
	Scale { from: f32, to: f32 },
	/// Rotate animation (angle in degrees)
	Rotate { from: f32, to: f32 },
	/// Color transition
	Color {
		from: crate::color::Color,
		to: crate::color::Color,
	},
}

impl AnimationType {
	/// Interpolate the animation at the given progress (0.0 to 1.0)
	pub fn interpolate(&self, progress: f32) -> AnimationValue {
		let progress = progress.clamp(0.0, 1.0);
		match self {
			AnimationType::Fade { from, to } => {
				AnimationValue::Opacity(from + (to - from) * progress)
			}
			AnimationType::Slide {
				from_x,
				from_y,
				to_x,
				to_y,
			} => AnimationValue::Position {
				x: from_x + (to_x - from_x) * progress,
				y: from_y + (to_y - from_y) * progress,
			},
			AnimationType::Scale { from, to } => {
				AnimationValue::Scale(from + (to - from) * progress)
			}
			AnimationType::Rotate { from, to } => {
				AnimationValue::Rotation(from + (to - from) * progress)
			}
			AnimationType::Color { from, to } => {
				// Interpolate in RGB space
				let from_rgb = from.to_rgb();
				let to_rgb = to.to_rgb();
				let from_comp = from_rgb.components();
				let to_comp = to_rgb.components();
				AnimationValue::Color(crate::color::Color::rgb(
					from_comp[0] + (to_comp[0] - from_comp[0]) * progress,
					from_comp[1] + (to_comp[1] - from_comp[1]) * progress,
					from_comp[2] + (to_comp[2] - from_comp[2]) * progress,
					from_comp[3] + (to_comp[3] - from_comp[3]) * progress,
				))
			}
		}
	}
}

/// Animation value result
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationValue {
	/// Opacity value (0.0 to 1.0)
	Opacity(f32),
	/// Position (x, y)
	Position { x: f32, y: f32 },
	/// Scale factor
	Scale(f32),
	/// Rotation angle in degrees
	Rotation(f32),
	/// Color value
	Color(crate::color::Color),
}

/// Animation configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
	/// Animation type
	animation_type: AnimationType,
	/// Duration of the animation
	duration: Duration,
	/// Easing function
	easing: Easing,
	/// Delay before starting
	delay: Duration,
	/// Number of times to repeat (0 = play once, None = infinite)
	repeat: Option<u32>,
	/// Whether to reverse on alternate iterations
	alternate: bool,
	/// Current state
	#[serde(skip)]
	state: AnimationState,
	/// Elapsed time
	#[serde(skip)]
	elapsed: Duration,
	/// Current iteration (for repeating animations)
	#[serde(skip)]
	current_iteration: u32,
}

impl Animation {
	/// Create a new animation
	pub fn new(animation_type: AnimationType, duration: Duration) -> Self {
		Self {
			animation_type,
			duration,
			easing: Easing::Linear,
			delay: Duration::ZERO,
			repeat: None,
			alternate: false,
			state: AnimationState::Idle,
			elapsed: Duration::ZERO,
			current_iteration: 0,
		}
	}

	/// Create a fade animation
	pub fn fade(from: f32, to: f32, duration: Duration) -> Self {
		Self::new(AnimationType::Fade { from, to }, duration)
	}

	/// Create a slide animation
	pub fn slide(from_x: f32, from_y: f32, to_x: f32, to_y: f32, duration: Duration) -> Self {
		Self::new(
			AnimationType::Slide {
				from_x,
				from_y,
				to_x,
				to_y,
			},
			duration,
		)
	}

	/// Create a scale animation
	pub fn scale(from: f32, to: f32, duration: Duration) -> Self {
		Self::new(AnimationType::Scale { from, to }, duration)
	}

	/// Create a rotate animation
	pub fn rotate(from: f32, to: f32, duration: Duration) -> Self {
		Self::new(AnimationType::Rotate { from, to }, duration)
	}

	/// Create a color transition animation
	pub fn color(from: crate::color::Color, to: crate::color::Color, duration: Duration) -> Self {
		Self::new(AnimationType::Color { from, to }, duration)
	}

	/// Set easing function
	pub fn with_easing(mut self, easing: Easing) -> Self {
		self.easing = easing;
		self
	}

	/// Set delay before starting
	pub fn with_delay(mut self, delay: Duration) -> Self {
		self.delay = delay;
		self
	}

	/// Set number of repetitions (None = infinite)
	pub fn with_repeat(mut self, repeat: Option<u32>) -> Self {
		self.repeat = repeat;
		self
	}

	/// Set alternate mode (reverse on alternate iterations)
	pub fn with_alternate(mut self, alternate: bool) -> Self {
		self.alternate = alternate;
		self
	}

	/// Start the animation
	pub fn start(&mut self) {
		self.state = AnimationState::Running;
		self.elapsed = Duration::ZERO;
		self.current_iteration = 0;
	}

	/// Pause the animation
	pub fn pause(&mut self) {
		if self.state == AnimationState::Running {
			self.state = AnimationState::Paused;
		}
	}

	/// Resume the animation
	pub fn resume(&mut self) {
		if self.state == AnimationState::Paused {
			self.state = AnimationState::Running;
		}
	}

	/// Stop the animation
	pub fn stop(&mut self) {
		self.state = AnimationState::Idle;
		self.elapsed = Duration::ZERO;
		self.current_iteration = 0;
	}

	/// Get current state
	pub fn state(&self) -> AnimationState {
		self.state
	}

	/// Check if animation is running
	pub fn is_running(&self) -> bool {
		self.state == AnimationState::Running
	}

	/// Check if animation is completed
	pub fn is_completed(&self) -> bool {
		self.state == AnimationState::Completed
	}

	/// Update animation with elapsed time
	pub fn update(&mut self, delta: Duration) -> Option<AnimationValue> {
		if self.state != AnimationState::Running {
			return None;
		}

		self.elapsed += delta;

		// Handle delay
		if self.elapsed < self.delay {
			return None;
		}

		let animation_elapsed = self.elapsed - self.delay;
		let total_duration = self.duration;

		if animation_elapsed >= total_duration {
			// Animation iteration completed
			self.current_iteration += 1;

			// Check if we should repeat
			if let Some(repeat_count) = self.repeat {
				if self.current_iteration >= repeat_count {
					self.state = AnimationState::Completed;
					return Some(self.get_value(1.0));
				}
			}

			// Reset for next iteration
			self.elapsed = self.delay;
		}

		// Calculate progress (0.0 to 1.0)
		let progress =
			(animation_elapsed.as_secs_f32() / total_duration.as_secs_f32()).clamp(0.0, 1.0);

		// Apply alternate if needed
		let adjusted_progress = if self.alternate && self.current_iteration % 2 == 1 {
			1.0 - progress
		} else {
			progress
		};

		Some(self.get_value(adjusted_progress))
	}

	/// Get animation value at specific progress
	fn get_value(&self, progress: f32) -> AnimationValue {
		let eased_progress = self.easing.apply(progress);
		self.animation_type.interpolate(eased_progress)
	}

	/// Get duration
	pub fn duration(&self) -> Duration {
		self.duration
	}

	/// Get easing
	pub fn easing(&self) -> Easing {
		self.easing
	}
}

/// Animation controller for managing multiple animations
#[derive(Debug)]
pub struct AnimationController {
	animations: Vec<Animation>,
}

impl AnimationController {
	/// Create a new animation controller
	pub fn new() -> Self {
		Self {
			animations: Vec::new(),
		}
	}

	/// Add an animation
	pub fn add(&mut self, animation: Animation) -> usize {
		self.animations.push(animation);
		self.animations.len() - 1
	}

	/// Start an animation by index
	pub fn start(&mut self, index: usize) {
		if let Some(animation) = self.animations.get_mut(index) {
			animation.start();
		}
	}

	/// Start all animations
	pub fn start_all(&mut self) {
		for animation in &mut self.animations {
			animation.start();
		}
	}

	/// Pause an animation by index
	pub fn pause(&mut self, index: usize) {
		if let Some(animation) = self.animations.get_mut(index) {
			animation.pause();
		}
	}

	/// Stop an animation by index
	pub fn stop(&mut self, index: usize) {
		if let Some(animation) = self.animations.get_mut(index) {
			animation.stop();
		}
	}

	/// Update all animations
	pub fn update(&mut self, delta: Duration) -> Vec<(usize, AnimationValue)> {
		let mut results = Vec::new();
		for (index, animation) in self.animations.iter_mut().enumerate() {
			if let Some(value) = animation.update(delta) {
				results.push((index, value));
			}
		}
		results
	}

	/// Remove completed animations
	pub fn cleanup(&mut self) {
		self.animations.retain(|a| !a.is_completed());
	}

	/// Get number of animations
	pub fn len(&self) -> usize {
		self.animations.len()
	}

	/// Check if empty
	pub fn is_empty(&self) -> bool {
		self.animations.is_empty()
	}

	/// Clear all animations
	pub fn clear(&mut self) {
		self.animations.clear();
	}
}

impl Default for AnimationController {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_easing_linear() {
		let easing = Easing::Linear;
		assert_eq!(easing.apply(0.0), 0.0);
		assert_eq!(easing.apply(0.5), 0.5);
		assert_eq!(easing.apply(1.0), 1.0);
	}

	#[test]
	fn test_easing_ease_in() {
		let easing = Easing::EaseIn;
		assert_eq!(easing.apply(0.0), 0.0);
		assert!(easing.apply(0.5) < 0.5);
		assert_eq!(easing.apply(1.0), 1.0);
	}

	#[test]
	fn test_easing_ease_out() {
		let easing = Easing::EaseOut;
		assert_eq!(easing.apply(0.0), 0.0);
		assert!(easing.apply(0.5) > 0.5);
		assert_eq!(easing.apply(1.0), 1.0);
	}

	#[test]
	fn test_animation_fade() {
		let mut anim = Animation::fade(0.0, 1.0, Duration::from_secs(1));
		anim.start();
		assert!(anim.is_running());

		// Update halfway through
		let value = anim.update(Duration::from_millis(500));
		assert!(value.is_some());
		if let Some(AnimationValue::Opacity(opacity)) = value {
			assert!((opacity - 0.5).abs() < 0.01);
		}
	}

	#[test]
	fn test_animation_slide() {
		let mut anim = Animation::slide(0.0, 0.0, 100.0, 100.0, Duration::from_secs(1));
		anim.start();

		let value = anim.update(Duration::from_millis(500));
		assert!(value.is_some());
		if let Some(AnimationValue::Position { x, y }) = value {
			assert!((x - 50.0).abs() < 1.0);
			assert!((y - 50.0).abs() < 1.0);
		}
	}

	#[test]
	fn test_animation_scale() {
		let mut anim = Animation::scale(1.0, 2.0, Duration::from_secs(1));
		anim.start();

		let value = anim.update(Duration::from_millis(500));
		assert!(value.is_some());
		if let Some(AnimationValue::Scale(scale)) = value {
			assert!((scale - 1.5).abs() < 0.01);
		}
	}

	#[test]
	fn test_animation_rotate() {
		let mut anim = Animation::rotate(0.0, 360.0, Duration::from_secs(1));
		anim.start();

		let value = anim.update(Duration::from_millis(500));
		assert!(value.is_some());
		if let Some(AnimationValue::Rotation(angle)) = value {
			assert!((angle - 180.0).abs() < 1.0);
		}
	}

	#[test]
	fn test_animation_delay() {
		let mut anim = Animation::fade(0.0, 1.0, Duration::from_secs(1))
			.with_delay(Duration::from_millis(500));
		anim.start();

		// Should return None during delay
		assert!(anim.update(Duration::from_millis(250)).is_none());
		assert!(anim.update(Duration::from_millis(200)).is_none());

		// Should start animating after delay (elapsed > delay)
		let value = anim.update(Duration::from_millis(100));
		assert!(value.is_some());
	}

	#[test]
	fn test_animation_repeat() {
		let mut anim = Animation::fade(0.0, 1.0, Duration::from_secs(1)).with_repeat(Some(2));
		anim.start();

		// Complete first iteration
		anim.update(Duration::from_secs(1));
		assert!(anim.is_running());

		// Complete second iteration
		anim.update(Duration::from_secs(1));
		assert!(anim.is_completed());
	}

	#[test]
	fn test_animation_pause_resume() {
		let mut anim = Animation::fade(0.0, 1.0, Duration::from_secs(1));
		anim.start();
		assert!(anim.is_running());

		anim.pause();
		assert_eq!(anim.state(), AnimationState::Paused);

		anim.resume();
		assert!(anim.is_running());
	}

	#[test]
	fn test_animation_controller() {
		let mut controller = AnimationController::new();
		assert!(controller.is_empty());

		let index = controller.add(Animation::fade(0.0, 1.0, Duration::from_secs(1)));
		assert_eq!(controller.len(), 1);

		controller.start(index);
		let results = controller.update(Duration::from_millis(500));
		assert_eq!(results.len(), 1);

		controller.clear();
		assert!(controller.is_empty());
	}

	#[test]
	fn test_animation_controller_multiple() {
		let mut controller = AnimationController::new();

		controller.add(Animation::fade(0.0, 1.0, Duration::from_secs(1)));
		controller.add(Animation::scale(1.0, 2.0, Duration::from_secs(1)));
		assert_eq!(controller.len(), 2);

		controller.start_all();
		let results = controller.update(Duration::from_millis(500));
		assert_eq!(results.len(), 2);
	}

	#[test]
	fn test_animation_color() {
		use crate::color::Color;

		let from = Color::rgb(0.0, 0.0, 0.0, 1.0);
		let to = Color::rgb(1.0, 1.0, 1.0, 1.0);
		let mut anim = Animation::color(from, to, Duration::from_secs(1));
		anim.start();

		let value = anim.update(Duration::from_millis(500));
		assert!(value.is_some());
		if let Some(AnimationValue::Color(color)) = value {
			// Color should be somewhere between black and white
			let comp = color.components();
			// At 50% progress, should be around 0.5 for RGB components
			assert!((comp[0] - 0.5).abs() < 0.1);
			assert!((comp[1] - 0.5).abs() < 0.1);
			assert!((comp[2] - 0.5).abs() < 0.1);
		}
	}
}
