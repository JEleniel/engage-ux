// This crate provides vendored/generated Wayland XDG protocol client bindings.
// Bindings are generated at compile time using the `wayland-scanner` v0.31
// proc-macro approach. The proc-macro reads the XML in `protocols/` and
// expands the client bindings inline, so no OUT_DIR-generated file is required.

// Use the `wayland-scanner` proc-macro to generate client bindings at
// compile time.
#[cfg(feature = "wayland-scanner")]
mod xdg {
	#![allow(
		dead_code,
		non_camel_case_types,
		non_upper_case_globals,
		non_snake_case,
		unused_imports
	)]
	// Generate client bindings from a compatibility-adjusted XML that
	// removes constructs known to trip the proc-macro parser.
	wayland_scanner::generate_client_code!("protocols/xdg-shell.xml");
}

#[cfg(not(feature = "wayland-scanner"))]
mod xdg {
	#![allow(
		dead_code,
		non_camel_case_types,
		non_upper_case_globals,
		non_snake_case,
		unused_imports
	)]
	// Fallback: include the checked-in, minimal bindings so the crate builds
	// and can be consumed in environments where the proc-macro is disabled
	// or causes expansion errors.
	include!("xdg_client.rs");
}

pub use xdg::*;

// Ergonomic, small wrappers around the raw generated bindings.
// These wrappers are intentionally minimal and forward to the generated
// types; they provide a stable, Rust-idiomatic surface for the consumer to
// call into. As the build.rs generator is replaced with real wayland-scanner
// generation, these wrappers remain the stable API for higher-level code.
pub mod shell;
pub use shell::{XdgManager, XdgSurfaceHandle, XdgToplevelHandle};
pub mod dynamic;
pub use dynamic::parse_protocol_xml;
