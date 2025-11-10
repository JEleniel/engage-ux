use xdg_protocols::{XdgManager, XdgSurfaceHandle, XdgToplevelHandle, XdgWmBase};

#[test]
fn xdg_wrappers_compile_and_forward() {
	// Create the stubbed base (provided by build.rs generated stub)
	let base = XdgWmBase;

	// Wrap with ergonomic manager and exercise forwarding methods.
	let mgr = XdgManager::new(base);
	let surf = mgr.get_xdg_surface(&());
	let surf_handle = XdgSurfaceHandle::new(surf);
	let toplevel = surf_handle.get_toplevel();
	let top_handle = XdgToplevelHandle::new(toplevel);

	// Call a variety of methods to ensure the thin wrappers forward
	// to the generated/stubbed bindings and compile.
	top_handle.set_title("test-title");
	top_handle.set_minimized();
	top_handle.set_maximized();
	top_handle.unset_maximized();
	top_handle.request_close();

	mgr.pong(42);
	surf_handle.ack_configure(7);
}
