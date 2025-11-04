use super::*;
use crate::Color;
use chrono::Duration as ChronoDuration;

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
	let mut anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();
	anim.start();
	assert!(anim.is_running());

	// Update halfway through
	let value = anim.update(ChronoDuration::milliseconds(500));
	assert!(value.is_some());
	if let Some(AnimationValue::Alpha(alpha)) = value {
		let opacity = alpha as f32 / 255.0;
		assert!((opacity - 0.5).abs() < 0.05);
	}
}

#[test]
fn test_animation_slide() {
	let mut anim = Animation::builder(AnimationType::Move {
		from_x: 0.0,
		from_y: 0.0,
		to_x: 100.0,
		to_y: 100.0,
	})
	.with_duration_msec(1000)
	.build();
	anim.start();

	let value = anim.update(ChronoDuration::milliseconds(500));
	assert!(value.is_some());
	if let Some(AnimationValue::Point(p)) = value {
		// midway between 0 and 100 should round to 50
		assert_eq!(p.x, 50);
		assert_eq!(p.y, 50);
	}
}

#[test]
fn test_animation_scale() {
	let mut anim = Animation::builder(AnimationType::Scale { from: 1.0, to: 2.0 })
		.with_duration_msec(1000)
		.build();
	anim.start();

	let value = anim.update(ChronoDuration::milliseconds(500));
	assert!(value.is_some());
	if let Some(AnimationValue::Scale(scale)) = value {
		assert!((scale - 1.5).abs() < 0.01);
	}
}

#[test]
fn test_animation_rotate() {
	let mut anim = Animation::builder(AnimationType::Rotate {
		start_angle: 0.0,
		end_angle: 360.0,
	})
	.with_duration_msec(1000)
	.build();
	anim.start();

	let value = anim.update(ChronoDuration::milliseconds(500));
	assert!(value.is_some());
	if let Some(AnimationValue::Angle(angle)) = value {
		assert!((angle - 180.0).abs() < 1.0);
	}
}

#[test]
fn test_animation_delay() {
	let mut anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.with_delay_msec(500)
	.build();
	anim.start();

	// Should return None during delay
	assert!(anim.update(ChronoDuration::milliseconds(250)).is_none());
	assert!(anim.update(ChronoDuration::milliseconds(200)).is_none());

	// Should start animating after delay (elapsed > delay)
	let value = anim.update(ChronoDuration::milliseconds(100));
	assert!(value.is_some());
}

#[test]
fn test_animation_repeat() {
	let mut anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.with_repeat(Some(2))
	.build();
	anim.start();

	// Complete first iteration
	anim.update(ChronoDuration::seconds(1));
	assert!(anim.is_running());

	// Complete second iteration
	anim.update(ChronoDuration::seconds(1));
	assert!(anim.is_completed());
}

#[test]
fn test_animation_pause_resume() {
	let mut anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();
	anim.start();
	assert!(anim.is_running());

	anim.pause();
	assert_eq!(anim.state(), AnimationState::Paused);

	anim.resume();
	assert!(anim.is_running());
}

#[tokio::test]
async fn test_active_animation_basic() {
	use tokio::time::{Duration, timeout};

	let anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();

	let active = anim.into_active(1);
	let mut rx = active.event_receiver();
	active.start().await;

	// Wait for a Value event within 300ms
	let got_value = timeout(Duration::from_millis(300), async {
		loop {
			match rx.recv().await {
				Ok(AnimationEvent::Value(_)) => break true,
				Ok(AnimationEvent::Completed) => break false,
				Ok(_) => continue,
				Err(_) => break false,
			}
		}
	})
	.await
	.unwrap_or(false);

	assert!(got_value, "Expected a Value event from ActiveAnimation");
	active.stop().await;
}

#[tokio::test]
async fn test_active_animation_multiple() {
	use tokio::time::{Duration, timeout};

	let anim1 = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();

	let anim2 = Animation::builder(AnimationType::Scale { from: 1.0, to: 2.0 })
		.with_duration_msec(1000)
		.build();

	let a1 = anim1.into_active(11);
	let a2 = anim2.into_active(12);
	let mut r1 = a1.event_receiver();
	let mut r2 = a2.event_receiver();

	a1.start().await;
	a2.start().await;

	// Wait for both to emit a Value event
	let got_both = timeout(Duration::from_millis(400), async {
		let mut got1 = false;
		let mut got2 = false;
		while !(got1 && got2) {
			tokio::select! {
				res = r1.recv() => {
					match res {
						Ok(AnimationEvent::Value(_)) => got1 = true,
						_ => {}
					}
				}
				res = r2.recv() => {
					match res {
						Ok(AnimationEvent::Value(_)) => got2 = true,
						_ => {}
					}
				}
			}
		}
		got1 && got2
	})
	.await
	.unwrap_or(false);

	assert!(
		got_both,
		"Expected both active animations to produce a Value event"
	);
	a1.stop().await;
	a2.stop().await;
}

#[test]
fn test_animation_color() {
	let from = Color::from_rgb(0, 0, 0);
	let to = Color::from_rgb(255, 255, 255);
	let mut anim = Animation::builder(AnimationType::Color { from, to })
		.with_duration_msec(1000)
		.build();
	anim.start();

	let value = anim.update(ChronoDuration::milliseconds(500));
	assert!(value.is_some());
	if let Some(AnimationValue::Color(color)) = value {
		let comp = [color.red, color.green, color.blue, color.alpha];
		assert!((comp[0] as f32 - 127.5).abs() < 20.0);
		assert!((comp[1] as f32 - 127.5).abs() < 20.0);
		assert!((comp[2] as f32 - 127.5).abs() < 20.0);
	}
}

#[test]
fn test_animation_zero_duration_update_returns_some() {
	// Zero-duration animations are treated as no-ops and immediately completed.
	let mut anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(0)
	.build();
	anim.start();
	assert!(anim.is_completed());
	// update should return None since animation is already completed
	let value = anim.update(ChronoDuration::milliseconds(10));
	assert!(value.is_none());
}

#[test]
fn test_update_before_start_returns_none() {
	let mut anim = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();
	// Not started yet - update should return None
	assert!(anim.update(ChronoDuration::milliseconds(100)).is_none());
}
