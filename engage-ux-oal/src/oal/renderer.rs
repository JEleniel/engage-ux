use crate::errors::Result;
use crate::oal::{Canvas, DeviceMetrics, Unit};

/// Renderer trait - minimal contract for OAL renderers.
pub trait Renderer: Send {
	/// Render the given canvas and view into the backing surface.
	fn render(&mut self, canvas: &Canvas, metrics: &DeviceMetrics, unit: Unit) -> Result<()>;

	/// Optional GPU render submission hook. Default implementation returns
	/// None indicating no GPU job is provided.
	fn try_gpu_render(
		&mut self,
		_canvas: &Canvas,
		_metrics: &DeviceMetrics,
		_unit: Unit,
	) -> Option<Box<dyn crate::oal::backend::RenderCallback>> {
		let _ = (_canvas, _metrics, _unit);
		None
	}
}

/// A noop software renderer for tests and headless operation.
pub struct NoopRenderer;

impl NoopRenderer {
	/// Create a new `NoopRenderer` instance. Provided for tests and
	/// headless operation where rendering is intentionally a no-op.
	pub fn new() -> Self {
		Self
	}
}

impl Default for NoopRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl Renderer for NoopRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		Ok(())
	}
}

/// Choose the best available renderer: keep simple for now.
pub fn choose_renderer() -> Box<dyn Renderer> {
	Box::new(NoopRenderer::new())
}

/// Invalidate any GPU caches associated with a previous device id.
///
/// Real GPU renderers should drop device-local caches keyed by the
/// provided device identifier. For the minimal/noop renderer this is a no-op.
pub fn invalidate_gpu_caches_for_device(_old_device_id: usize) {
	// No-op for the simple renderer used in tests/headless builds.
}

#[cfg(feature = "native-winit")]
pub struct WgpuRenderer {
	pub color: wgpu::Color,
}

#[cfg(feature = "native-winit")]
impl WgpuRenderer {
	pub fn new(color: wgpu::Color) -> Self {
		Self { color }
	}
}

#[cfg(feature = "native-winit")]
impl Renderer for WgpuRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		Ok(())
	}

	fn try_gpu_render(
		&mut self,
		_canvas: &Canvas,
		_metrics: &DeviceMetrics,
		_unit: Unit,
	) -> Option<Box<dyn crate::oal::backend::RenderCallback>> {
		Some(Box::new(
			move |_backend_ctx: &mut dyn std::any::Any| -> Result<()> {
				let _ = _backend_ctx;
				Ok(())
			},
		))
	}
}
