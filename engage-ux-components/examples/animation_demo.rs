//! Animation system demonstration
//!
//! Shows various animation types, easing functions, and animation composition.

use engage_ux_core::animation::{Animation, AnimationController, Easing};
use engage_ux_core::color::Color;
use std::time::Duration;

fn main() {
	println!("Engage UX - Animation System Demo\n");
	println!("==================================\n");

	// Demo 1: Basic fade animation
	println!("1. Fade Animation (0.0 -> 1.0 over 1 second)");
	let mut fade = Animation::fade(0.0, 1.0, Duration::from_secs(1));
	fade.start();

	// Simulate frame updates
	for i in 0..=10 {
		let progress = i as f32 / 10.0;
		if let Some(value) = fade.update(Duration::from_millis(100)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 2: Slide animation with easing
	println!("2. Slide Animation with EaseInOut (0,0 -> 100,100)");
	let mut slide = Animation::slide(0.0, 0.0, 100.0, 100.0, Duration::from_secs(1))
		.with_easing(Easing::EaseInOut);
	slide.start();

	for i in 0..=5 {
		if let Some(value) = slide.update(Duration::from_millis(200)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 3: Scale animation with delay
	println!("3. Scale Animation with 500ms Delay");
	let mut scale =
		Animation::scale(1.0, 2.0, Duration::from_secs(1)).with_delay(Duration::from_millis(500));
	scale.start();

	println!("   During delay:");
	for i in 0..3 {
		let result = scale.update(Duration::from_millis(200));
		println!(
			"   Frame {}: {:?}",
			i,
			if result.is_none() {
				"None (delayed)"
			} else {
				"Value"
			}
		);
	}

	println!("   After delay:");
	for i in 3..6 {
		if let Some(value) = scale.update(Duration::from_millis(200)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 4: Rotate animation with repeat
	println!("4. Rotate Animation (0° -> 360°, repeat 2 times)");
	let mut rotate = Animation::rotate(0.0, 360.0, Duration::from_secs(1)).with_repeat(Some(2));
	rotate.start();

	for i in 0..20 {
		if let Some(value) = rotate.update(Duration::from_millis(100)) {
			println!("   Frame {}: {:?}", i, value);
		}
		if rotate.is_completed() {
			println!("   Animation completed!");
			break;
		}
	}
	println!();

	// Demo 5: Color transition
	println!("5. Color Transition (Black -> White)");
	let black = Color::rgb(0.0, 0.0, 0.0, 1.0);
	let white = Color::rgb(1.0, 1.0, 1.0, 1.0);
	let mut color_anim = Animation::color(black, white, Duration::from_secs(1));
	color_anim.start();

	for i in 0..=5 {
		if let Some(value) = color_anim.update(Duration::from_millis(200)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 6: Easing function comparison
	println!("6. Easing Functions Comparison (at 50% progress)");
	let easings = vec![
		Easing::Linear,
		Easing::EaseIn,
		Easing::EaseOut,
		Easing::EaseInOut,
		Easing::CubicBezier,
	];

	for easing in easings {
		let value = easing.apply(0.5);
		println!("   {:?}: {:.3}", easing, value);
	}
	println!();

	// Demo 7: Animation Controller
	println!("7. Animation Controller (Multiple Animations)");
	let mut controller = AnimationController::new();

	let fade_idx = controller.add(Animation::fade(0.0, 1.0, Duration::from_secs(1)));
	let scale_idx = controller.add(Animation::scale(1.0, 2.0, Duration::from_secs(1)));
	let rotate_idx = controller.add(Animation::rotate(0.0, 180.0, Duration::from_secs(1)));

	controller.start_all();

	println!("   Starting {} animations...", controller.len());
	for frame in 0..=5 {
		let results = controller.update(Duration::from_millis(200));
		println!("   Frame {}: {} active animations", frame, results.len());
		for (idx, value) in results {
			println!("      Animation {}: {:?}", idx, value);
		}
	}

	controller.cleanup();
	println!(
		"   After cleanup: {} animations remaining",
		controller.len()
	);
	println!();

	// Demo 8: Alternate mode
	println!("8. Alternate Animation (ping-pong effect)");
	let mut alternate = Animation::fade(0.0, 1.0, Duration::from_millis(500))
		.with_repeat(Some(4))
		.with_alternate(true);
	alternate.start();

	for i in 0..20 {
		if let Some(value) = alternate.update(Duration::from_millis(100)) {
			println!("   Frame {}: {:?}", i, value);
		}
		if alternate.is_completed() {
			break;
		}
	}
	println!();

	println!("Animation demo complete!");
}
