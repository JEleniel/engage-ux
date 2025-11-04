//! Integration tests for the animation system

use chrono::Duration as ChronoDuration;
use engage_ux_core::Color;
use engage_ux_core::animation::{Animation, AnimationState, AnimationValue, Easing};

#[test]
fn test_animation_integration() {
	// Test multiple animations working together using per-animation update
	let mut fade = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.with_repeat(Some(1))
	.build();

	let mut scale =
		Animation::builder(engage_ux_core::animation::AnimationType::Scale { from: 1.0, to: 2.0 })
			.with_duration_msec(1000)
			.with_repeat(Some(1))
			.build();

	let mut rotate = Animation::builder(engage_ux_core::animation::AnimationType::Rotate {
		start_angle: 0.0,
		end_angle: 360.0,
	})
	.with_duration_msec(1000)
	.with_repeat(Some(1))
	.build();

	fade.start();
	scale.start();
	rotate.start();

	// Update at 50% progress
	let v_fade = fade.update(ChronoDuration::milliseconds(500));
	let v_scale = scale.update(ChronoDuration::milliseconds(500));
	let v_rotate = rotate.update(ChronoDuration::milliseconds(500));

	assert!(v_fade.is_some());
	assert!(v_scale.is_some());
	assert!(v_rotate.is_some());

	if let Some(AnimationValue::Alpha(alpha)) = v_fade {
		let opacity = alpha as f32 / 255.0;
		assert!((opacity - 0.5).abs() < 0.05);
	} else {
		panic!("Expected Alpha value for fade animation");
	}

	if let Some(AnimationValue::Scale(s)) = v_scale {
		assert!((s - 1.5).abs() < 0.01);
	} else {
		panic!("Expected Scale value for scale animation");
	}

	if let Some(AnimationValue::Angle(angle)) = v_rotate {
		assert!((angle - 180.0).abs() < 1.0);
	} else {
		panic!("Expected Angle value for rotate animation");
	}
}

#[test]
fn test_animation_with_easing() {
	// Test different easing functions produce different results
	let easings = vec![
		Easing::Linear,
		Easing::EaseIn,
		Easing::EaseOut,
		Easing::EaseInOut,
	];

	let mut results = Vec::new();

	for easing in easings {
		let mut anim = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
			start_alpha: 0.0,
			end_alpha: 1.0,
		})
		.with_duration_msec(1000)
		.with_easing(easing)
		.build();
		anim.start();

		if let Some(AnimationValue::Alpha(alpha)) = anim.update(ChronoDuration::milliseconds(500)) {
			let opacity = alpha as f32 / 255.0;
			results.push(opacity);
		}
	}

	// All should be different (except possibly some coincidental equality)
	assert_eq!(results.len(), 4);

	// Linear should be exactly 0.5
	assert!((results[0] - 0.5).abs() < 0.01);

	// EaseIn should be less than linear
	assert!(results[1] < 0.5);

	// EaseOut should be greater than linear
	assert!(results[2] > 0.5);
}

#[test]
fn test_animation_sequence() {
	// Test sequential animations with delays using per-animation updates
	let mut anim1 = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(500)
	.with_repeat(Some(1))
	.build();

	let mut anim2 = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
		start_alpha: 1.0,
		end_alpha: 0.0,
	})
	.with_duration_msec(500)
	.with_delay(ChronoDuration::milliseconds(500))
	.with_repeat(Some(1))
	.build();

	anim1.start();
	anim2.start();

	// At 250ms: only first animation active
	let r1 = anim1.update(ChronoDuration::milliseconds(250));
	let r2 = anim2.update(ChronoDuration::milliseconds(250));
	let active_count = r1.is_some() as usize + r2.is_some() as usize;
	assert_eq!(active_count, 1);

	// At 500ms: first animation completes first iteration, second just finished delay
	let r1 = anim1.update(ChronoDuration::milliseconds(250));
	let r2 = anim2.update(ChronoDuration::milliseconds(250));
	let active_count = r1.is_some() as usize + r2.is_some() as usize;
	assert!(active_count >= 1 && active_count <= 2);

	// At 750ms: first might be on second iteration, second animation active
	let r1 = anim1.update(ChronoDuration::milliseconds(250));
	let r2 = anim2.update(ChronoDuration::milliseconds(250));
	let active_count = r1.is_some() as usize + r2.is_some() as usize;
	assert!(active_count >= 1);

	// At 1000ms: both complete their iterations
	let _r = anim1.update(ChronoDuration::milliseconds(250));
	let _r = anim2.update(ChronoDuration::milliseconds(250));
}

#[test]
fn test_color_animation_interpolation() {
	// Test color interpolation
	let black = Color::rgb(0.0, 0.0, 0.0, 1.0);
	let white = Color::rgb(1.0, 1.0, 1.0, 1.0);

	let mut anim = Animation::builder(engage_ux_core::animation::AnimationType::Color {
		from: black,
		to: white,
	})
	.with_duration_msec(1000)
	.build();
	anim.start();

	// At 25%
	if let Some(AnimationValue::Color(color)) = anim.update(ChronoDuration::milliseconds(250)) {
		let comp = color.components();
		assert!((comp[0] - 0.25).abs() < 0.01);
		assert!((comp[1] - 0.25).abs() < 0.01);
		assert!((comp[2] - 0.25).abs() < 0.01);
	}

	// At 50%
	if let Some(AnimationValue::Color(color)) = anim.update(ChronoDuration::milliseconds(250)) {
		let comp = color.components();
		assert!((comp[0] - 0.5).abs() < 0.01);
		assert!((comp[1] - 0.5).abs() < 0.01);
		assert!((comp[2] - 0.5).abs() < 0.01);
	}
}

#[test]
fn test_animation_state_management() {
	let mut anim = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();

	// Initial state
	assert_eq!(anim.state(), AnimationState::Idle);
	assert!(!anim.is_running());
	assert!(!anim.is_completed());

	// Start
	anim.start();
	assert_eq!(anim.state(), AnimationState::Running);
	assert!(anim.is_running());

	// Pause
	anim.pause();
	assert_eq!(anim.state(), AnimationState::Paused);
	assert!(!anim.is_running());

	// Resume
	anim.resume();
	assert_eq!(anim.state(), AnimationState::Running);

	// Stop
	anim.stop();
	assert_eq!(anim.state(), AnimationState::Idle);
}

#[test]
fn test_animation_repeat_with_alternate() {
	// Test ping-pong animation
	let mut anim = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(100)
	.with_repeat(Some(4))
	.with_alternate(true)
	.build();

	anim.start();

	// First iteration: 0 -> 1
	if let Some(AnimationValue::Alpha(alpha)) = anim.update(ChronoDuration::milliseconds(50)) {
		let opacity = alpha as f32 / 255.0;
		assert!(opacity > 0.0 && opacity < 1.0);
	}

	anim.update(ChronoDuration::milliseconds(50)); // Complete first iteration

	// Second iteration: 1 -> 0 (alternate)
	if let Some(AnimationValue::Alpha(alpha)) = anim.update(ChronoDuration::milliseconds(50)) {
		let opacity = alpha as f32 / 255.0;
		assert!(opacity > 0.0 && opacity < 1.0);
	}

	// Continue until completion
	for _ in 0..10 {
		if anim.is_completed() {
			break;
		}
		anim.update(ChronoDuration::milliseconds(50));
	}

	assert!(anim.is_completed());
}

#[test]
fn test_animation_controller_management() {
	// Manage multiple animations via per-animation API
	let mut items = Vec::new();
	for _ in 0..5 {
		let anim = Animation::builder(engage_ux_core::animation::AnimationType::Fade {
			start_alpha: 0.0,
			end_alpha: 1.0,
		})
		.with_duration_msec(1000)
		.build();
		items.push(anim);
	}

	assert_eq!(items.len(), 5);

	// Start and update all
	for a in items.iter_mut() {
		a.start();
		let _ = a.update(ChronoDuration::seconds(1));
	}

	// Some should be completed
	let completed = items.iter().filter(|a| a.is_completed()).count();
	assert!(completed >= 1);

	// Clearing is just dropping
	items.clear();
	assert_eq!(items.len(), 0);
}
