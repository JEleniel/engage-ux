//! Platform-specific backend implementations
//!
//! This module provides the architecture for platform-specific rendering,
//! window management, input handling, and accessibility.

pub mod renderer;
pub mod screen_reader;
pub mod softbuffer_renderer;
pub mod window_backend;
pub mod winit_window;

#[cfg(target_os = "windows")]
pub mod screen_reader_windows;

#[cfg(target_os = "macos")]
pub mod screen_reader_macos;

#[cfg(target_os = "linux")]
pub mod screen_reader_linux;

#[cfg(target_os = "android")]
pub mod screen_reader_android;

#[cfg(target_os = "ios")]
pub mod screen_reader_ios;

pub use renderer::{RenderBackend, RenderCommand, RenderContext};
pub use screen_reader::{ScreenReaderBackend, StubScreenReader};
pub use softbuffer_renderer::SoftbufferRenderer;
pub use window_backend::{WindowBackend, WindowBackendEvent, WindowBounds, WindowState};
pub use winit_window::WinitWindowBackend;

#[cfg(target_os = "linux")]
pub mod tiny_skia_renderer;

#[cfg(target_os = "linux")]
pub mod linux_accessibility;

#[cfg(target_os = "linux")]
pub use tiny_skia_renderer::TinySkiaRenderer;

#[cfg(target_os = "linux")]
pub use linux_accessibility::{
	AccessibilityError, AtSpiAccessibilityBridge, AtSpiState, aria_role_to_atspi_role,
};

/// Platform-specific backend factory
pub trait BackendFactory {
	/// Create a renderer backend for the current platform
	fn create_renderer(&self) -> Box<dyn RenderBackend>;

	/// Create a window backend for the current platform
	fn create_window_backend(&self) -> Box<dyn WindowBackend>;

	/// Create a screen reader backend for the current platform
	fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend>;
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

		fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend> {
			Box::new(screen_reader::StubScreenReader::new())
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

		fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend> {
			Box::new(screen_reader_windows::WindowsScreenReader::new())
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

		fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend> {
			Box::new(screen_reader_macos::MacOSScreenReader::new())
		}
	}

	#[cfg(target_os = "linux")]
	pub struct LinuxBackendFactory;

	#[cfg(target_os = "linux")]
	impl BackendFactory for LinuxBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Using tiny-skia for high-quality 2D graphics rendering on Linux
			// tiny-skia provides Cairo/Skia-like capabilities with safe Rust
			Box::new(tiny_skia_renderer::TinySkiaRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Using winit for safe, cross-platform window management
			// Winit supports both X11 and Wayland on Linux
			Box::new(winit_window::WinitWindowBackend::new())
		}

		fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend> {
			Box::new(screen_reader_linux::LinuxScreenReader::new())
		}
	}

	#[cfg(target_os = "android")]
	pub struct AndroidBackendFactory;

	#[cfg(target_os = "android")]
	impl BackendFactory for AndroidBackendFactory {
		fn create_renderer(&self) -> Box<dyn RenderBackend> {
			// Android implementation using softbuffer for software rendering.
			// Softbuffer provides a safe abstraction over Android's native Canvas API
			// through winit's Android support layer, ensuring compatibility with
			// the Android graphics stack while maintaining 100% safe Rust code.
			Box::new(softbuffer_renderer::SoftbufferRenderer::new())
		}

		fn create_window_backend(&self) -> Box<dyn WindowBackend> {
			// Android window management using winit.
			// Winit integrates with Android's Activity lifecycle and provides
			// native touch event handling, window state management, and
			// accessibility support (TalkBack ready through Android's native APIs).
			Box::new(winit_window::WinitWindowBackend::new())
		}

		fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend> {
			Box::new(screen_reader_android::AndroidScreenReader::new())
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

		fn create_screen_reader(&self) -> Box<dyn ScreenReaderBackend> {
			Box::new(screen_reader_ios::IOSScreenReader::new())
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
		let _screen_reader = factory.create_screen_reader();
		// Test passes if backends can be created without panic
	}
}
