// Vendored/generated xdg client bindings (checked-in fallback)

pub struct XdgWmBase;
pub struct XdgSurface;
pub struct XdgToplevel;

impl XdgWmBase {
	pub fn get_xdg_surface(&self, _surface: &()) -> XdgSurface {
		XdgSurface
	}
	pub fn pong(&self, _serial: u32) {}
}

impl XdgSurface {
	pub fn get_toplevel(&self) -> XdgToplevel {
		XdgToplevel
	}
	pub fn ack_configure(&self, _serial: u32) {}
}

impl XdgToplevel {
	pub fn set_title(&self, _title: &str) {}
	pub fn set_minimized(&self) {}
	pub fn set_maximized(&self) {}
	pub fn unset_maximized(&self) {}
	pub fn request_close(&self) {}
}
