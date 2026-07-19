use serde::{Deserialize, Serialize};

/// Supported easing functions for animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Easing {
	/// Linear interpolation (constant speed).
	Linear,
	/// Ease in (slow start, accelerate).
	EaseIn,
	/// Ease out (fast start, decelerate).
	EaseOut,
	/// Ease in and out (slow start and end).
	EaseInOut,
	/// Cubic bezier easing (slow start, fast middle, slow end).
	CubicBezier,
	/// Elastic easing (bouncy effect).
	Elastic,
	/// Bounce easing.
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
