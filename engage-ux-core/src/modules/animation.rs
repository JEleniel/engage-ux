/// High-level animation APIs and the builder
#[allow(clippy::module_inception)]
pub mod animation;
/// Animation runtime state definitions
pub mod animation_state;
/// Animation types and produced values
pub mod animation_types;
/// Easing functions used when interpolating animations
pub mod easing;

pub use animation::Animation;
pub use animation::{ActiveAnimation, AnimationEvent};
pub use animation_state::AnimationState;
pub use animation_types::{AnimationType, AnimationValue};
pub use easing::Easing;
