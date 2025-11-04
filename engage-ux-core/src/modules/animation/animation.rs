use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, watch};
use tokio::task::JoinHandle;
use tokio::time::Duration as TokioDuration;

use super::animation_state::AnimationState;
use super::animation_types::{AnimationType, AnimationValue};
use super::easing::Easing;

/// Configuration for a single animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
	/// Animation type (e.g., fade, slide, scale, rotate, color).
	animation_type: AnimationType,
	/// Duration of the animation.
	duration: ChronoDuration,
	/// Easing function to use for the animation.
	easing: Easing,
	/// (Previously had a separate timing selector; consolidated into `easing`.)
	/// Delay before starting the animation.
	delay: ChronoDuration,
	/// Number of times to repeat the animation (`0` = play once, `None` = infinite).
	repeat: Option<u32>,
	/// Whether to reverse on alternate iterations.
	alternate: bool,
	/// Current state of the animation.
	#[serde(skip)]
	state: AnimationState,
	/// Instant (UTC) when the animation was paused. Used to compute paused duration on resume.
	#[serde(skip)]
	paused_instant: Option<DateTime<Utc>>,

	/// Instant (UTC) when the animation was started or resumed. Used to compute elapsed internally.
	#[serde(skip)]
	start_instant: Option<DateTime<Utc>>,
	/// Current iteration (for repeating animations).
	#[serde(skip)]
	current_iteration: u32,
}

/// Events emitted by a self-running animation.
#[derive(Debug, Clone)]
pub enum AnimationEvent {
	/// Emitted when the animation has started.
	Started,
	/// Emitted when the animation has been paused.
	Paused,
	/// Emitted when the animation has been resumed after a pause.
	Resumed,
	/// Emitted when the animation has completed all iterations.
	Completed,
	/// Emitted periodically with the current animation value.
	Value(AnimationValue),
}

/// Control commands sent to the animation task.
#[derive(Debug)]
enum AnimationCommand {
	Start,
	Pause,
	Resume,
	Stop,
}

/// A handle to an animation that has been spawned as a background tokio task.
pub struct ActiveAnimation {
	_id: u64,
	event_tx: broadcast::Sender<AnimationEvent>,
	cmd_tx: mpsc::Sender<AnimationCommand>,
	watch_tx: watch::Sender<Option<AnimationValue>>,
	_handle: JoinHandle<()>,
}

impl ActiveAnimation {
	/// Get a fresh receiver for animation events. Each call returns a new
	/// `broadcast::Receiver<AnimationEvent>` that the caller can use to
	/// independently receive lifecycle and value events.
	pub fn event_receiver(&self) -> broadcast::Receiver<AnimationEvent> {
		self.event_tx.subscribe()
	}

	/// Subscribe to the latest animation value as a watch signal.
	/// The receiver initially contains `None` and will be updated with
	/// `Some(AnimationValue)` each frame. Useful for clients that want
	/// the latest value without buffering all intermediate events.
	pub fn watch_values(&self) -> watch::Receiver<Option<AnimationValue>> {
		self.watch_tx.subscribe()
	}

	/// Send start command.
	pub async fn start(&self) {
		let _ = self.cmd_tx.send(AnimationCommand::Start).await;
	}

	/// Send pause command.
	pub async fn pause(&self) {
		let _ = self.cmd_tx.send(AnimationCommand::Pause).await;
	}

	/// Send resume command.
	pub async fn resume(&self) {
		let _ = self.cmd_tx.send(AnimationCommand::Resume).await;
	}

	/// Send stop command.
	pub async fn stop(&self) {
		let _ = self.cmd_tx.send(AnimationCommand::Stop).await;
	}

	/// Subscribe with a synchronous callback. The callback is invoked from a
	/// background tokio task whenever a new `AnimationEvent::Value` is emitted.
	/// The callback receives an owned immutable clone of the `AnimationValue`.
	/// If you need UI-thread affinity, marshal inside the callback to the UI
	/// dispatcher provided by your backend.
	pub fn subscribe<F>(&self, mut callback: F)
	where
		F: FnMut(AnimationValue) + Send + 'static,
	{
		// Each subscriber gets its own receiver so they don't interfere.
		let mut rx = self.event_tx.subscribe();
		tokio::spawn(async move {
			while let Ok(evt) = rx.recv().await {
				if let AnimationEvent::Value(val) = evt {
					// pass an owned clone (AnimationValue is Clone)
					callback(val);
				}
			}
		});
	}

	/// Subscribe with an async callback. The provided function is called for
	/// each value and awaited. Value is cloned and passed as an owned value to
	/// the async callback.
	pub fn subscribe_async<F, Fut>(&self, f: F)
	where
		F: Fn(AnimationValue) -> Fut + Send + Sync + 'static,
		Fut: std::future::Future<Output = ()> + Send + 'static,
	{
		let mut rx = self.event_tx.subscribe();
		let f = std::sync::Arc::new(f);
		tokio::spawn(async move {
			while let Ok(evt) = rx.recv().await {
				if let AnimationEvent::Value(val) = evt {
					let fut = (f)(val);
					fut.await;
				}
			}
		});
	}
}

impl Animation {
	// NOTE: `Animation::new` was removed — use `Animation::builder` instead.

	/// Start the animation
	pub fn start(&mut self) {
		// If duration is zero, treat the animation as a no-op and mark as completed.
		if self.duration == ChronoDuration::zero() {
			// Force animation type to None for zero-duration (no-op) animations.
			self.animation_type = AnimationType::None {};
			self.state = AnimationState::Completed;
			self.current_iteration = 0;
			self.start_instant = None;
			self.paused_instant = None;
			return;
		}

		self.state = AnimationState::Running;
		self.current_iteration = 0;
		self.paused_instant = None;
		self.start_instant = Some(Utc::now());
	}

	/// Pause the animation
	pub fn pause(&mut self) {
		if self.state == AnimationState::Running {
			self.state = AnimationState::Paused;
			// record pause instant so resume can compute elapsed when needed
			self.paused_instant = Some(Utc::now());
			self.start_instant = None;
		}
	}

	/// Resume the animation
	pub fn resume(&mut self) {
		if self.state == AnimationState::Paused {
			self.state = AnimationState::Running;
			// resume by setting start_instant such that now - start_instant == paused duration
			if let Some(paused_at) = self.paused_instant {
				let now = Utc::now();
				let paused_duration = now.signed_duration_since(paused_at);
				self.start_instant = Some(now - paused_duration);
			} else {
				self.start_instant = Some(Utc::now());
			}
			self.paused_instant = None;
		}
	}

	/// Stop the animation
	pub fn stop(&mut self) {
		self.state = AnimationState::Idle;
		self.paused_instant = None;
		self.current_iteration = 0;
		self.start_instant = None;
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

	/// Update animation with elapsed time calculated from the start instant.
	/// The external `delta` parameter is ignored; elapsed is computed internally
	/// from when the animation was started or resumed.
	pub fn update(&mut self, _delta: ChronoDuration) -> Option<AnimationValue> {
		if self.state != AnimationState::Running {
			return None;
		}

		// Must have a start instant to compute elapsed; if missing, nothing to do.
		if self.start_instant.is_none() {
			return None;
		}

		// If caller provided a non-zero delta, move the start instant backwards by
		// that delta so that elapsed computed from the wall clock reflects the
		// accumulated deltas across successive update() calls. If delta is zero,
		// compute elapsed from the wall clock directly.
		if _delta > ChronoDuration::zero() {
			if let Some(s) = self.start_instant {
				self.start_instant = Some(s - _delta);
			}
		}

		let elapsed = Utc::now().signed_duration_since(self.start_instant.unwrap());

		// Handle delay
		if elapsed < self.delay {
			return None;
		}

		let animation_elapsed = elapsed - self.delay;
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

			// Reset for next iteration: adjust the start instant so future elapsed is computed from now - delay.
			self.start_instant = Some(Utc::now() - self.delay);
			self.paused_instant = None;
		}

		// Calculate progress (0.0 to 1.0) using milliseconds for float division
		let anim_ms = animation_elapsed.num_milliseconds() as f32;
		let total_ms = total_duration.num_milliseconds() as f32;
		let progress = if total_ms > 0.0 {
			(anim_ms / total_ms).clamp(0.0, 1.0)
		} else {
			1.0
		};

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
		// Apply easing transformation to the normalized progress and interpolate.
		let eased = self.easing.apply(progress);
		self.animation_type.interpolate(eased)
	}

	/// Get duration
	pub fn duration(&self) -> ChronoDuration {
		self.duration
	}

	/// Get easing
	pub fn easing(&self) -> Easing {
		self.easing
	}
}

/// Builder for `Animation` to allow ergonomic construction via chaining.
pub struct AnimationBuilder {
	animation: Animation,
}

impl AnimationBuilder {
	/// Create a new `AnimationBuilder` from an `AnimationType`.
	///
	/// Defaults:
	/// - duration: 300 milliseconds
	/// - timing: Linear
	pub fn new(animation_type: AnimationType) -> Self {
		let default_duration = ChronoDuration::milliseconds(300);
		Self {
			animation: Animation {
				animation_type,
				duration: default_duration,
				easing: Easing::Linear,
				delay: ChronoDuration::zero(),
				repeat: None,
				alternate: false,
				state: AnimationState::Idle,
				paused_instant: None,
				start_instant: None,
				current_iteration: 0,
			},
		}
	}

	/// Set easing on the animation being built.
	pub fn with_easing(mut self, easing: Easing) -> Self {
		self.animation.easing = easing;
		self
	}

	// `with_timing` removed: timing functions consolidated into `Easing`.

	/// Convenience to set duration in milliseconds.
	pub fn with_duration_msec(mut self, msec: u16) -> Self {
		self.animation.duration = ChronoDuration::milliseconds(msec as i64);
		self
	}

	/// Convenience to set delay in milliseconds.
	pub fn with_delay_msec(mut self, msec: u16) -> Self {
		self.animation.delay = ChronoDuration::milliseconds(msec as i64);
		self
	}

	/// Set delay for the animation being built.
	pub fn with_delay(mut self, delay: ChronoDuration) -> Self {
		self.animation.delay = delay;
		self
	}

	/// Set repeat count (None = infinite) for the animation being built.
	pub fn with_repeat(mut self, repeat: Option<u32>) -> Self {
		self.animation.repeat = repeat;
		self
	}

	/// Set alternate (reverse on alternate iterations) for the animation being built.
	pub fn with_alternate(mut self, alternate: bool) -> Self {
		self.animation.alternate = alternate;
		self
	}

	/// Finalize and return the `Animation`.
	pub fn build(self) -> Animation {
		self.animation
	}
}

impl Animation {
	/// Start a builder from an `AnimationType`.
	///
	/// The builder will default to 300ms duration and Linear timing unless overridden.
	pub fn builder(animation_type: AnimationType) -> AnimationBuilder {
		AnimationBuilder::new(animation_type)
	}

	/// Convert this animation into an actively running animation spawned on the tokio runtime.
	/// Returns an `ActiveAnimation` handle which can be used to control the animation and
	/// subscribe to events. The spawned task does not start running until `start()` is
	/// sent through the handle.
	pub fn into_active(self, id: u64) -> ActiveAnimation {
		// broadcast channel for events (small buffer)
		let (event_tx, _event_rx) = broadcast::channel(16);
		let (cmd_tx, mut cmd_rx) = mpsc::channel::<AnimationCommand>(8);
		// watch channel for latest value (None = no value yet)
		let (watch_tx, _watch_rx) = watch::channel::<Option<AnimationValue>>(None);

		// Spawn a task that owns the animation and drives it.
		let mut animation = self;
		let event_tx_clone = event_tx.clone();

		let watch_tx_clone_for_task = watch_tx.clone();
		let handle = tokio::spawn(async move {
			// animation starts in its current state; we wait for Start command to begin
			let mut _running = animation.is_running();

			if _running {
				let _ = event_tx_clone.send(AnimationEvent::Started);
			}

			// Tick interval ~60fps
			let tick = TokioDuration::from_millis(16);

			loop {
				tokio::select! {
					Some(cmd) = cmd_rx.recv() => {
						match cmd {
							AnimationCommand::Start => {
								animation.start();
								_running = true;
								let _ = event_tx_clone.send(AnimationEvent::Started);
							}
							AnimationCommand::Pause => {
								animation.pause();
								_running = false;
								let _ = event_tx_clone.send(AnimationEvent::Paused);
							}
							AnimationCommand::Resume => {
								animation.resume();
								_running = true;
								let _ = event_tx_clone.send(AnimationEvent::Resumed);
							}
							AnimationCommand::Stop => {
								animation.stop();
								_running = false;
								let _ = event_tx_clone.send(AnimationEvent::Completed);
								break;
							}
						}
					}
					_ = tokio::time::sleep(tick), if _running => {
						// provide a small delta to the update call; animation.update will
						// compute elapsed internally but accepts delta adjustments.
						if let Some(value) = animation.update(ChronoDuration::milliseconds(16)) {
							// send value event (ignore if no subscribers)
							let _ = event_tx_clone.send(AnimationEvent::Value(value.clone()));
							// update watch with the latest value (ignore if send fails)
							let _ = watch_tx_clone_for_task.send(Some(value));
						}

						if animation.is_completed() {
							let _ = event_tx_clone.send(AnimationEvent::Completed);
							let _ = watch_tx_clone_for_task.send(None);
							break;
						}
					}
					else => {
						// If not running and no commands: yield briefly to avoid busy loop
						tokio::time::sleep(TokioDuration::from_millis(50)).await;
					}
				}
			}
		});

		ActiveAnimation {
			_id: id,
			event_tx: event_tx.clone(),
			cmd_tx,
			watch_tx,
			_handle: handle,
		}
	}
}
