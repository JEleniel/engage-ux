use std::sync::Arc;

// Re-export the generated (or stub) types so we can forward calls.
use crate::{XdgSurface, XdgToplevel, XdgWmBase};

/// Lightweight manager wrapper around the underlying `XdgWmBase` global.
/// In practice this will be created from the generated global when available.
#[derive(Clone)]
pub struct XdgManager {
	inner: Arc<XdgWmBase>,
}

impl XdgManager {
	/// Wrap an existing generated `XdgWmBase` proxy.
	pub fn new(base: XdgWmBase) -> Self {
		Self {
			inner: Arc::new(base),
		}
	}

	/// Convenience: obtain an xdg_surface for a wl_surface. Forwarded to the
	/// generated binding. The real generated binding takes a `WlSurface`; the
	/// current stub uses `()` so callers should match the real signature once
	/// bindings are generated.
	pub fn get_xdg_surface(&self, surface: &()) -> XdgSurface {
		// Forward to generated binding
		self.inner.get_xdg_surface(surface)
	}

	/// Pong a serial (forwarded)
	pub fn pong(&self, serial: u32) {
		self.inner.pong(serial)
	}
}

/// Thin safe wrapper around the generated `XdgSurface` proxy.
#[derive(Clone)]
pub struct XdgSurfaceHandle {
	inner: Arc<XdgSurface>,
}

impl XdgSurfaceHandle {
	pub fn new(s: XdgSurface) -> Self {
		Self { inner: Arc::new(s) }
	}

	/// Create a toplevel on this surface.
	pub fn get_toplevel(&self) -> XdgToplevel {
		self.inner.get_toplevel()
	}

	/// Acknowledge configure sequence.
	pub fn ack_configure(&self, serial: u32) {
		self.inner.ack_configure(serial)
	}
}

/// Thin safe wrapper around the generated `XdgToplevel` proxy.
#[derive(Clone)]
pub struct XdgToplevelHandle {
	inner: Arc<XdgToplevel>,
}

impl XdgToplevelHandle {
	pub fn new(t: XdgToplevel) -> Self {
		Self { inner: Arc::new(t) }
	}

	pub fn set_title(&self, title: &str) {
		self.inner.set_title(title)
	}

	pub fn set_minimized(&self) {
		self.inner.set_minimized()
	}

	pub fn set_maximized(&self) {
		self.inner.set_maximized()
	}

	pub fn unset_maximized(&self) {
		self.inner.unset_maximized()
	}

	pub fn request_close(&self) {
		self.inner.request_close()
	}
}
