//! OS Abstraction Layer (OAL) for Engage UX
//!
//! Provides platform-specific implementations for:
//! - Window management
//! - Graphics rendering
//! - Input handling
//! - File system access

pub mod backends;
pub mod platform;
pub mod window;

pub use backends::{BackendFactory, RenderBackend, WindowBackend, get_backend_factory};
pub use platform::Platform;
pub use window::Window;

#[cfg(test)]
mod tests {
	#[test]
	fn oal_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
