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

	pub fn create_surface(&self, params: SurfaceDescriptor) -> Result<SurfaceHandle> {
		self.backend.create_surface(params)
	}

	pub fn poll_events(&self) -> Vec<PlatformEvent> {
		self.backend.poll_events()
	}

	pub fn present_frame(&self, surface: SurfaceHandle, dirty: &[Rectangle]) -> Result<()> {
		self.backend.present_frame(surface, dirty)
	}

	pub fn invalidate_region(&self, surface: SurfaceHandle, rects: &[Rectangle]) -> Result<()> {
		self.backend.invalidate_region(surface, rects)
	}

	pub fn destroy_surface(&self, surface: SurfaceHandle) -> Result<()> {
		self.backend.destroy_surface(surface)
	}

	pub fn reconfigure_surface(
		&self,
		surface: SurfaceHandle,
		params: SurfaceDescriptor,
	) -> Result<()> {
		self.backend.reconfigure_surface(surface, params)
	}

	pub fn submit_render(
		&self,
		surface: SurfaceHandle,
		job: Box<dyn RenderCallback>,
	) -> Result<()> {
		self.backend.submit_render(surface, job)
	}
}
