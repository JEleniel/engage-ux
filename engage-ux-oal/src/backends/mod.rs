//! Platform-specific backend implementations
//!
//! This module provides the architecture for platform-specific rendering,
//! window management, and input handling.

pub mod renderer;
pub mod window_backend;
pub mod winit_window;
pub mod softbuffer_renderer;

pub use renderer::{RenderBackend, RenderCommand, RenderContext};
pub use window_backend::{WindowBackend, WindowBackendEvent, WindowBounds, WindowState};
pub use winit_window::WinitWindowBackend;
pub use softbuffer_renderer::SoftbufferRenderer;

/// Platform-specific backend factory
pub trait BackendFactory {
	/// Create a renderer backend for the current platform
	fn create_renderer(&self) -> Box<dyn RenderBackend>;

	/// Create a window backend for the current platform
	fn create_window_backend(&self) -> Box<dyn WindowBackend>;
}

/// Get the backend factory for the current platform
pub fn get_backend_factory() -> Box<dyn BackendFactory> {
	#[cfg(target_os = "windows")]
	return Box::new(crate::backends::platforms::WindowsBackendFactory);

	#[cfg(target_os = "macos")]
	return Box::new(crate::backends::platforms::MacOSBackendFactory);

	#[cfg(target_os = "linux")]
	return Box::new(crate::backends::platforms::LinuxBackendFactory);

	#[cfg(target_os = "android")]
	return Box::new(crate::backends::platforms::AndroidBackendFactory);

	#[cfg(target_os = "ios")]
	return Box::new(crate::backends::platforms::IOSBackendFactory);

	#[cfg(not(any(
		target_os = "windows",
		target_os = "macos",
		target_os = "linux",
		target_os = "android",
		target_os = "ios"
	)))]
	{
		// Fallback to a stub backend for unsupported platforms
		Box::new(crate::backends::platforms::StubBackendFactory)
	}
}

pub mod platforms {
	use super::*;

	// Stub implementation for platforms
	pub struct StubBackendFactory;

	impl BackendFactory for StubBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			Box::new(renderer::StubRenderer)
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			Box::new(window_backend::StubWindowBackend::default())
		}
	}

	// Platform-specific factories (to be implemented)
	#[cfg(target_os = "windows")]
	pub struct WindowsBackendFactory;

	#[cfg(target_os = "windows")]
	impl BackendFactory for WindowsBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Using softbuffer for safe, cross-platform software rendering
			Box::new(softbuffer_renderer::SoftbufferRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Using winit for safe, cross-platform window management
			Box::new(winit_window::WinitWindowBackend::new())
		}
	}

	#[cfg(target_os = "macos")]
	pub struct MacOSBackendFactory;

	#[cfg(target_os = "macos")]
	impl BackendFactory for MacOSBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Using softbuffer for safe, cross-platform software rendering
			Box::new(softbuffer_renderer::SoftbufferRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Using winit for safe, cross-platform window management
			Box::new(winit_window::WinitWindowBackend::new())
		}
	}

	#[cfg(target_os = "linux")]
	pub struct LinuxBackendFactory;

	#[cfg(target_os = "linux")]
	impl BackendFactory for LinuxBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Using softbuffer for safe, cross-platform software rendering
			Box::new(softbuffer_renderer::SoftbufferRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Using winit for safe, cross-platform window management
			Box::new(winit_window::WinitWindowBackend::new())
		}
	}

	#[cfg(target_os = "android")]
	pub struct AndroidBackendFactory;

	#[cfg(target_os = "android")]
	impl BackendFactory for AndroidBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Using softbuffer for safe, cross-platform software rendering
			Box::new(softbuffer_renderer::SoftbufferRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Using winit for safe, cross-platform window management
			Box::new(winit_window::WinitWindowBackend::new())
		}
	}

	#[cfg(target_os = "ios")]
	pub struct IOSBackendFactory;

	#[cfg(target_os = "ios")]
	impl BackendFactory for IOSBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Using softbuffer for safe, cross-platform software rendering
			Box::new(softbuffer_renderer::SoftbufferRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Using winit for safe, cross-platform window management
			Box::new(winit_window::WinitWindowBackend::new())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_backend_factory_creation() {
		let factory = get_backend_factory();
		let _renderer = factory.create_renderer();
		let _window = factory.create_window_backend();
		// Test passes if backends can be created without panic
	}
}
