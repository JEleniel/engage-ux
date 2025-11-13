use crate::errors::Result;
use crate::oal::backend::{Backend, HeadlessBackend, PlatformEvent, SurfaceHandle, SurfaceParams};
use engage_ux_core::geometry::Rectangle;

/// Lightweight X11 backend shim. Delegates to `HeadlessBackend` as a
/// placeholder until a full X11 implementation using `winit` is provided.
#[derive(Clone, Debug)]
pub struct X11Backend {
	inner: HeadlessBackend,
}

impl X11Backend {
	pub fn new() -> Self {
		Self {
			inner: HeadlessBackend::new(),
		}
	}
}

impl Backend for X11Backend {
	fn create_surface(&self, params: SurfaceParams) -> Result<SurfaceHandle> {
		self.inner.create_surface(params)
	}

	fn poll_events(&self) -> Vec<PlatformEvent> {
		self.inner.poll_events()
	}

	fn present_frame(&self, surface: SurfaceHandle, dirty: &[Rectangle]) -> Result<()> {
		self.inner.present_frame(surface, dirty)
	}

	fn invalidate_region(&self, surface: SurfaceHandle, rects: &[Rectangle]) -> Result<()> {
		self.inner.invalidate_region(surface, rects)
	}

	fn destroy_surface(&self, surface: SurfaceHandle) -> Result<()> {
		self.inner.destroy_surface(surface)
	}

	fn reconfigure_surface(&self, surface: SurfaceHandle, params: SurfaceParams) -> Result<()> {
		self.inner.reconfigure_surface(surface, params)
	}

	fn submit_render(
		&self,
		surface: SurfaceHandle,
		job: Box<dyn crate::oal::backend::RenderCallback>,
	) -> Result<()> {
		self.inner.submit_render(surface, job)
	}
}
