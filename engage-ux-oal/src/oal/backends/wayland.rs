use std::sync::Arc;

use crate::errors::Result;
use crate::oal::backend::{Backend, HeadlessBackend, PlatformEvent, SurfaceHandle, SurfaceParams};
use engage_ux_core::geometry::Rectangle;

/// Wayland backend wrapper.
///
/// If the crate is built with `native-winit`, this will delegate to the
/// `WinitBackend` (which supports Wayland via winit). Otherwise it falls
/// back to `HeadlessBackend` so the crate remains usable in non-native
/// environments (tests, CI, etc.).
#[derive(Clone)]
pub struct WaylandBackend {
	inner: Arc<dyn Backend>,
}

impl WaylandBackend {
	pub fn new() -> Self {
		// Choose the most capable backend available at compile time.
		#[cfg(feature = "native-winit")]
		{
			// Use the winit adapter when available (it supports Wayland/X11).
			let b: Arc<dyn Backend> = Arc::new(crate::oal::WinitBackend::new());
			Self { inner: b }
		}

		#[cfg(not(feature = "native-winit"))]
		{
			let b: Arc<dyn Backend> = Arc::new(HeadlessBackend::new());
			Self { inner: b }
		}
	}
}

impl Backend for WaylandBackend {
	fn create_surface(&self, params: SurfaceParams) -> Result<SurfaceHandle> {
		self.inner.as_ref().create_surface(params)
	}

	fn poll_events(&self) -> Vec<PlatformEvent> {
		self.inner.as_ref().poll_events()
	}

	fn present_frame(&self, surface: SurfaceHandle, dirty: &[Rectangle]) -> Result<()> {
		self.inner.as_ref().present_frame(surface, dirty)
	}

	fn invalidate_region(&self, surface: SurfaceHandle, rects: &[Rectangle]) -> Result<()> {
		self.inner.as_ref().invalidate_region(surface, rects)
	}

	fn destroy_surface(&self, surface: SurfaceHandle) -> Result<()> {
		self.inner.as_ref().destroy_surface(surface)
	}

	fn reconfigure_surface(&self, surface: SurfaceHandle, params: SurfaceParams) -> Result<()> {
		self.inner.as_ref().reconfigure_surface(surface, params)
	}

	fn set_surface_title(&self, surface: SurfaceHandle, title: &str) -> Result<()> {
		self.inner.as_ref().set_surface_title(surface, title)
	}

	fn submit_render(
		&self,
		surface: SurfaceHandle,
		job: Box<dyn crate::oal::backend::RenderCallback>,
	) -> Result<()> {
		self.inner.as_ref().submit_render(surface, job)
	}
}
