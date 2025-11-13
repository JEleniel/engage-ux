use crate::errors::Result;
use crate::oal::backend::SurfaceHandle;
use crate::oal::{Canvas, DeviceMetrics, Unit, WindowDesc};
use engage_ux_core::event::{Event, EventBus};
use std::sync::Arc;
use uuid::Uuid;

/// Simple Window handle returned by OAL.
///
/// The `Window` type is intentionally lightweight: it represents the logical
/// window state (id, view, canvas) and provides helpers to present and
/// request frames. Platform backends may associate additional native handles
/// with the logical `id`.
#[derive(Clone)]
pub struct Window {
	/// Stable identifier for the window (UUID-based u128).
	pub id: u128,
	/// Human-readable title for the window. May be unused in headless builds.
	#[allow(dead_code)]
	pub(crate) title: String,
	/// Logical canvas backing this window. The canvas size equals the
	/// underlying native window size expressed in OAL Units. There is no
	/// separate movable `View` – the Canvas represents the entire window.
	pub canvas: Canvas,
	event_bus: Arc<EventBus>,
	metrics: DeviceMetrics,
	/// Optional platform surface handle associated with this logical window.
	pub(crate) platform_surface: Option<SurfaceHandle>,
}

impl Window {
	/// Create a new logical `Window` from a `WindowDesc`.
	pub(crate) fn new(desc: WindowDesc, bus: Arc<EventBus>, metrics: DeviceMetrics) -> Self {
		let id = Uuid::new_v4().as_u128();
		let canvas = Canvas::new(desc.size_in_units.0, desc.size_in_units.1);

		Self {
			id,
			title: desc.title,
			canvas,
			event_bus: bus,
			metrics,
			platform_surface: None,
		}
	}

	/// Return a cloned snapshot of the window state useful for rendering/presenting
	/// without holding internal locks across platform calls.
	pub(crate) fn snapshot(&self) -> (Canvas, DeviceMetrics, Option<SurfaceHandle>) {
		(
			self.canvas.clone(),
			self.metrics.clone(),
			self.platform_surface,
		)
	}

	/// Return a reference to the window's `Canvas`.
	pub fn canvas(&self) -> &Canvas {
		&self.canvas
	}

	/// Present the window using the provided `renderer`.
	pub fn present<R: crate::oal::renderer::Renderer>(
		&mut self,
		renderer: &mut R,
		unit: Unit,
	) -> Result<()> {
		renderer.render(&self.canvas, &self.metrics, unit)
	}

	/// Request a frame to be produced for this window (emits `FrameRequested`).
	pub fn request_frame(&self) {
		// Notify via EventBus that a frame was requested
		let _ = self.event_bus.emit(Event::Custom {
			source_component_id: self.id,
			timestamp: chrono::Utc::now(),
			payload: "FrameRequested".to_string(),
		});
	}
}
