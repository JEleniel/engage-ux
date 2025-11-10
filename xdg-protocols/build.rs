fn main() {
	// Minimal build script: proc-macro does codegen during compile. Ensure
	// changes to the protocol XML re-run the build.
	println!("cargo:rerun-if-changed=protocols/xdg-shell.xml");
	// Also watch the compatibility-adjusted XML used for proc-macro generation.
	println!("cargo:rerun-if-changed=protocols/xdg-shell-compat.xml");
}
