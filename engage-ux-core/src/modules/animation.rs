//! Animation module (modern layout)
//!
//! This file re-exports submodules split into `src/modules/animation/*`.

pub mod animation;
pub mod animation_state;
pub mod animation_types;
pub mod easing;

pub use animation::Animation;
pub use animation::{ActiveAnimation, AnimationEvent};
pub use animation_state::AnimationState;
pub use animation_types::{AnimationType, AnimationValue};
pub use easing::Easing;
// TimingFunction removed: use `Easing` instead.

#[cfg(test)]
mod animation_tests;
