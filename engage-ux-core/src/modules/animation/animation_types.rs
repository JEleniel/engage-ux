use crate::{Color, geometry::Point};
use serde::{Deserialize, Serialize};

/// Types of supported animations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationType {
	/// No-op animation type used for zero-duration or disabled animations.
	None {},
	/// Fade animation (opacity).
	Fade {
		/// Starting opacity value (0.0-1.0).
		start_alpha: f32,
		/// Ending opacity value (0.0-1.0).
		end_alpha: f32,
	},
	/// Slide animation removed — use `Move` which covers translation between two points.
	/// Scale animation (size).
	Scale {
		/// Starting scale factor.
		from: f32,
		/// Ending scale factor.
		to: f32,
	},
	/// Rotate animation (angle in degrees).
	Rotate {
		/// Starting rotation angle in degrees.
		start_angle: f32,
		/// Ending rotation angle in degrees.
		end_angle: f32,
	},
	/// Color transition animation.
	Color {
		/// Starting color value.
		from: Color,
		/// Ending color value.
		to: Color,
	},
	/// Brightness animation (0.0 to 1.0).
	Brightness {
		/// Starting brightness value.
		starting_brightness: f32,
		/// Ending brightness value.
		ending_brightness: f32,
	},
	/// Saturation animation (0.0 to 1.0).
	Saturation {
		/// Starting saturation value.
		starting_saturation: f32,
		/// Ending saturation value.
		ending_saturation: f32,
	},
	/// Hue animation (0.0 to 360.0).
	Hue {
		/// Starting hue value.
		starting_hue: f32,
		/// Ending hue value.
		ending_hue: f32,
	},
	/// Move animation (translation)
	/// Unlike the slide animation, move animations transition
	/// between two arbitrary points.
	Move {
		/// Starting X coordinate
		from_x: f32,
		/// Starting Y coordinate
		from_y: f32,
		/// Ending X coordinate
		to_x: f32,
		/// Ending Y coordinate
		to_y: f32,
	},
}

/// A generic value produced by an animation at a given progress.
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationValue {
	/// Alpha/opacity value (0..255)
	Alpha(u8),
	/// A 2D point produced by move animations
	Point(Point),
	/// Scale factor
	Scale(f32),
	/// Rotation angle in degrees
	Angle(f32),
	/// Color value
	Color(Color),
	/// Brightness value (0.0..1.0)
	Brightness(f32),
	/// Saturation value (0.0..1.0)
	Saturation(f32),
	/// Hue value (0.0..360.0)
	Hue(f32),
}

impl AnimationType {
	/// Compute the animated value for an already-eased progress (0.0..1.0).
	/// This is the method other modules expect (`interpolate` in animation.rs).
	pub fn interpolate(&self, eased_progress: f32) -> AnimationValue {
		let p = eased_progress.clamp(0.0, 1.0);

		// helper for lerping floats (used for alpha expressed as 0.0..1.0)
		let lerp_f32 = |a: f32, b: f32, p: f32| -> f32 {
			let p = p.clamp(0.0, 1.0);
			a + (b - a) * p
		};

		// helper for lerping float coordinates into an integer Point (crate Point uses i32)
			let lerp_to_point = |ax: f32, ay: f32, bx: f32, by: f32, p: f32| -> Point {
			let p = p.clamp(0.0, 1.0);
			let lerp_f = |a: f32, b: f32| -> f32 { a + (b - a) * p };
			Point {
				x: lerp_f(ax, bx).round(),
				y: lerp_f(ay, by).round(),
				style: None,
			}
		};

		match self {
			AnimationType::None { .. } => AnimationValue::Alpha(0),
			AnimationType::Fade {
				start_alpha,
				end_alpha,
				..
			} => {
				let a = lerp_f32(*start_alpha, *end_alpha, p).clamp(0.0, 1.0);
				AnimationValue::Alpha((a * 255.0).round() as u8)
			}
			AnimationType::Scale { from, to, .. } => AnimationValue::Scale(from + (to - from) * p),
			AnimationType::Rotate {
				start_angle,
				end_angle,
				..
			} => AnimationValue::Angle(start_angle + (end_angle - start_angle) * p),
			AnimationType::Color { from, to, .. } => AnimationValue::Color(from.lerp(to, p)),
			AnimationType::Brightness {
				starting_brightness,
				ending_brightness,
				..
			} => AnimationValue::Brightness(
				starting_brightness + (ending_brightness - starting_brightness) * p,
			),
			AnimationType::Saturation {
				starting_saturation,
				ending_saturation,
				..
			} => AnimationValue::Saturation(
				starting_saturation + (ending_saturation - starting_saturation) * p,
			),
			AnimationType::Hue {
				starting_hue,
				ending_hue,
				..
			} => AnimationValue::Hue(starting_hue + (ending_hue - starting_hue) * p),
			AnimationType::Move {
				from_x,
				from_y,
				to_x,
				to_y,
				..
			} => AnimationValue::Point(lerp_to_point(*from_x, *from_y, *to_x, *to_y, p)),
		}
	}
}
