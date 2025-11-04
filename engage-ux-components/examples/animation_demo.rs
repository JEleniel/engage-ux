//! Animation system demonstration
//!
//! Shows various animation types, easing functions, and animation composition.

use chrono::Duration as ChronoDuration;
use engage_ux_core::Color;
use engage_ux_core::animation::{Animation, AnimationType, Easing};

fn main() {
	println!("Engage UX - Animation System Demo\n");
	println!("==================================\n");

	// Demo 1: Basic fade animation
	println!("1. Fade Animation (0.0 -> 1.0 over 1 second)");
	let mut fade = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();
	fade.start();

	// Simulate frame updates
	for i in 0..=10 {
		if let Some(value) = fade.update(ChronoDuration::milliseconds(100)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 2: Slide animation with easing
	println!("2. Slide Animation with EaseInOut (0,0 -> 100,100)");
	let mut slide = Animation::builder(AnimationType::Move {
		from_x: 0.0,
		from_y: 0.0,
		to_x: 100.0,
		to_y: 100.0,
	})
	.with_duration_msec(1000)
	.with_easing(Easing::EaseInOut)
	.build();
	slide.start();

	for i in 0..=5 {
		if let Some(value) = slide.update(ChronoDuration::milliseconds(200)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 3: Scale animation with delay
	println!("3. Scale Animation with 500ms Delay");
	let mut scale = Animation::builder(AnimationType::Scale { from: 1.0, to: 2.0 })
		.with_duration_msec(1000)
		.with_delay(chrono::Duration::milliseconds(500))
		.build();
	scale.start();

	println!("   During delay:");
	for i in 0..3 {
		let result = scale.update(ChronoDuration::milliseconds(200));
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
		if let Some(value) = scale.update(ChronoDuration::milliseconds(200)) {
			println!("   Frame {}: {:?}", i, value);
		}
	}
	println!();

	// Demo 4: Rotate animation with repeat
	println!("4. Rotate Animation (0° -> 360°, repeat 2 times)");
	let mut rotate = Animation::builder(AnimationType::Rotate {
		start_angle: 0.0,
		end_angle: 360.0,
	})
	.with_duration_msec(1000)
	.with_repeat(Some(2))
	.build();
	rotate.start();

	for i in 0..20 {
		if let Some(value) = rotate.update(ChronoDuration::milliseconds(100)) {
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
	let mut color_anim = Animation::builder(AnimationType::Color {
		from: black,
		to: white,
	})
	.with_duration_msec(1000)
	.build();
	color_anim.start();

	for i in 0..=5 {
		if let Some(value) = color_anim.update(ChronoDuration::milliseconds(200)) {
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
	println!("7. Multiple Animations (per-animation updates)");
	let mut a1 = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(1000)
	.build();
	let mut a2 = Animation::builder(AnimationType::Scale { from: 1.0, to: 2.0 })
		.with_duration_msec(1000)
		.build();
	let mut a3 = Animation::builder(AnimationType::Rotate {
		start_angle: 0.0,
		end_angle: 180.0,
	})
	.with_duration_msec(1000)
	.build();

	a1.start();
	a2.start();
	a3.start();

	println!("   Starting 3 animations...");
	for frame in 0..=5 {
		let r1 = a1.update(ChronoDuration::milliseconds(200));
		let r2 = a2.update(ChronoDuration::milliseconds(200));
		let r3 = a3.update(ChronoDuration::milliseconds(200));
		let mut count = 0;
		if r1.is_some() {
			count += 1;
		}
		if r2.is_some() {
			count += 1;
		}
		if r3.is_some() {
			count += 1;
		}
		println!("   Frame {}: {} active animations", frame, count);
	}
	println!();

	// Demo 8: Alternate mode
	println!("8. Alternate Animation (ping-pong effect)");
	let mut alternate = Animation::builder(AnimationType::Fade {
		start_alpha: 0.0,
		end_alpha: 1.0,
	})
	.with_duration_msec(500)
	.with_repeat(Some(4))
	.with_alternate(true)
	.build();
	alternate.start();

	for i in 0..20 {
		if let Some(value) = alternate.update(ChronoDuration::milliseconds(100)) {
			println!("   Frame {}: {:?}", i, value);
		}
		if alternate.is_completed() {
			break;
		}
	}
	println!();

	println!("Animation demo complete!");
}
