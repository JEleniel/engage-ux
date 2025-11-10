// Keep internal modules private but expose a stable public surface.
mod oal;
mod oal_error;
mod platform;
mod traits;
mod types;

// Re-export the public API explicitly to avoid hidden glob re-export warnings.
pub use oal::Oal;
pub use oal_error::OalError;
// Re-export common traits and types explicitly to avoid hidden glob warnings.
pub use traits::Frame;
pub use traits::Surface;
pub use traits::SurfaceBuilder;
pub use traits::oal::PlatformTrait;
pub use traits::Window;
pub use traits::WindowBuilder;
pub use types::*;
