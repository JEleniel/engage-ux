use std::sync::Arc;

use crate::errors::Result;
use crate::oal::backend::{
	Backend, PlatformEvent, RenderCallback, SurfaceDescriptor, SurfaceHandle,
};
use engage_ux_core::geometry::Rectangle;

/// Platform is a thin wrapper around a `Backend` implementation providing a
/// small, ergonomic API for consumers. It holds a shared `Arc<dyn Backend>` so
/// backends may be swapped or shared between components.
#[derive(Clone)]
pub struct Platform {
	backend: Arc<dyn Backend>,
}

impl Platform {
	/// Construct a new Platform backed by the provided `Backend`.
	pub fn new<B: Backend + 'static>(backend: B) -> Self {
		Self {
			backend: Arc::new(backend),
		}
	}

	/// Create a new platform surface using the provided descriptor.
	pub fn create_surface(&self, params: SurfaceDescriptor) -> Result<SurfaceHandle> {
		self.backend.create_surface(params)
	}

	/// Poll for platform events and return any pending `PlatformEvent`s.
	pub fn poll_events(&self) -> Vec<PlatformEvent> {
		self.backend.poll_events()
	}

	/// Present the provided dirty rectangles for the given surface.
	pub fn present_frame(&self, surface: SurfaceHandle, dirty: &[Rectangle]) -> Result<()> {
		self.backend.present_frame(surface, dirty)
	}

	/// Invalidate the given logical rectangles on the surface. Backends may
	/// use this as a hint to repaint or flush regions.
	pub fn invalidate_region(&self, surface: SurfaceHandle, rects: &[Rectangle]) -> Result<()> {
		self.backend.invalidate_region(surface, rects)
	}

	/// Destroy the given surface and release associated resources.
	pub fn destroy_surface(&self, surface: SurfaceHandle) -> Result<()> {
		self.backend.destroy_surface(surface)
	}

	/// Reconfigure an existing surface (for example on resize). Backends may
	/// perform efficient in-place reconfiguration or fall back to recreate.
	pub fn reconfigure_surface(
		&self,
		surface: SurfaceHandle,
		params: SurfaceDescriptor,
	) -> Result<()> {
		self.backend.reconfigure_surface(surface, params)
	}

	/// Submit a backend-specific render job. The provided job will be invoked
	/// on the backend's rendering thread/context.
	pub fn submit_render(
		&self,
		surface: SurfaceHandle,
		job: Box<dyn RenderCallback>,
	) -> Result<()> {
		self.backend.submit_render(surface, job)
	}

	/// Update the platform title for a surface. Calls through to the
	/// backend's `set_surface_title` implementation.
	pub fn set_surface_title(&self, surface: SurfaceHandle, title: &str) -> Result<()> {
		self.backend.set_surface_title(surface, title)
	}
}
