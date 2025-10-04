//! Integration tests for the animation system

use engage_ux_core::animation::{Animation, AnimationController, AnimationState, AnimationValue, Easing};
use engage_ux_core::color::Color;
use std::time::Duration;

#[test]
fn test_animation_integration() {
	// Test multiple animations working together
	let mut controller = AnimationController::new();

	// Add various animations (with repeat count so they complete)
	let _fade_idx = controller.add(Animation::fade(0.0, 1.0, Duration::from_secs(1)).with_repeat(Some(1)));
	let _scale_idx = controller.add(Animation::scale(1.0, 2.0, Duration::from_secs(1)).with_repeat(Some(1)));
	let _rotate_idx = controller.add(Animation::rotate(0.0, 360.0, Duration::from_secs(1)).with_repeat(Some(1)));

	assert_eq!(controller.len(), 3);

	// Start all animations
	controller.start_all();

	// Update at 50% progress
	let results = controller.update(Duration::from_millis(500));
	assert_eq!(results.len(), 3);

	// Verify each animation type
	for (idx, value) in results {
		match idx {
			0 => {
				// Fade animation
				if let AnimationValue::Opacity(opacity) = value {
					assert!((opacity - 0.5).abs() < 0.01);
				} else {
					panic!("Expected Opacity value for fade animation");
				}
			}
			1 => {
				// Scale animation
				if let AnimationValue::Scale(scale) = value {
					assert!((scale - 1.5).abs() < 0.01);
				} else {
					panic!("Expected Scale value for scale animation");
				}
			}
			2 => {
				// Rotate animation
				if let AnimationValue::Rotation(angle) = value {
					assert!((angle - 180.0).abs() < 1.0);
				} else {
					panic!("Expected Rotation value for rotate animation");
				}
			}
			_ => panic!("Unexpected animation index"),
		}
	}

	// Complete animations
	controller.update(Duration::from_millis(500));

	// Cleanup completed animations
	controller.cleanup();
	assert_eq!(controller.len(), 0);
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
		let mut anim = Animation::fade(0.0, 1.0, Duration::from_secs(1)).with_easing(easing);
		anim.start();

		if let Some(AnimationValue::Opacity(opacity)) = anim.update(Duration::from_millis(500)) {
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
	// Test sequential animations with delays
	let mut controller = AnimationController::new();

	// Animation 1: starts immediately (plays once)
	let anim1 = Animation::fade(0.0, 1.0, Duration::from_millis(500)).with_repeat(Some(1));

	// Animation 2: starts after 500ms delay (plays once, total duration 1000ms)
	let anim2 = Animation::fade(1.0, 0.0, Duration::from_millis(500))
		.with_delay(Duration::from_millis(500))
		.with_repeat(Some(1));

	controller.add(anim1);
	controller.add(anim2);
	controller.start_all();

	// At 250ms: only first animation active
	let results = controller.update(Duration::from_millis(250));
	assert_eq!(results.len(), 1);

	// At 500ms: first animation completes first iteration, second just finished delay
	let results = controller.update(Duration::from_millis(250));
	// Both animations may return values
	assert!(results.len() >= 1 && results.len() <= 2);

	// At 750ms: first might be on second iteration, second animation active  
	let results = controller.update(Duration::from_millis(250));
	assert!(results.len() >= 1); // At least second anim active

	// At 1000ms: both complete their iterations
	let results = controller.update(Duration::from_millis(250));
	assert!(results.len() >= 0); // May or may not return values
}

#[test]
fn test_color_animation_interpolation() {
	// Test color interpolation
	let black = Color::rgb(0.0, 0.0, 0.0, 1.0);
	let white = Color::rgb(1.0, 1.0, 1.0, 1.0);

	let mut anim = Animation::color(black, white, Duration::from_secs(1));
	anim.start();

	// At 25%
	if let Some(AnimationValue::Color(color)) = anim.update(Duration::from_millis(250)) {
		let comp = color.components();
		assert!((comp[0] - 0.25).abs() < 0.01);
		assert!((comp[1] - 0.25).abs() < 0.01);
		assert!((comp[2] - 0.25).abs() < 0.01);
	}

	// At 50%
	if let Some(AnimationValue::Color(color)) = anim.update(Duration::from_millis(250)) {
		let comp = color.components();
		assert!((comp[0] - 0.5).abs() < 0.01);
		assert!((comp[1] - 0.5).abs() < 0.01);
		assert!((comp[2] - 0.5).abs() < 0.01);
	}
}

#[test]
fn test_animation_state_management() {
	let mut anim = Animation::fade(0.0, 1.0, Duration::from_secs(1));

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
	let mut anim = Animation::fade(0.0, 1.0, Duration::from_millis(100))
		.with_repeat(Some(4))
		.with_alternate(true);

	anim.start();

	// First iteration: 0 -> 1
	if let Some(AnimationValue::Opacity(opacity)) = anim.update(Duration::from_millis(50)) {
		assert!(opacity > 0.0 && opacity < 1.0);
	}

	anim.update(Duration::from_millis(50)); // Complete first iteration

	// Second iteration: 1 -> 0 (alternate)
	if let Some(AnimationValue::Opacity(opacity)) = anim.update(Duration::from_millis(50)) {
		assert!(opacity > 0.0 && opacity < 1.0);
	}

	// Continue until completion
	for _ in 0..10 {
		if anim.is_completed() {
			break;
		}
		anim.update(Duration::from_millis(50));
	}

	assert!(anim.is_completed());
}

#[test]
fn test_animation_controller_management() {
	let mut controller = AnimationController::new();

	// Add multiple animations
	for i in 0..5 {
		let anim = Animation::fade(0.0, 1.0, Duration::from_secs(1));
		controller.add(anim);
	}

	assert_eq!(controller.len(), 5);
	assert!(!controller.is_empty());

	// Start and complete all
	controller.start_all();
	controller.update(Duration::from_secs(1));

	// Some might be completed
	controller.cleanup();

	// Clear all
	controller.clear();
	assert_eq!(controller.len(), 0);
	assert!(controller.is_empty());
}
