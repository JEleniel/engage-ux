use std::path::PathBuf;
use xdg_protocols::dynamic::{Interface, Protocol, parse_protocol_xml};

#[test]
fn parse_xdg_shell_protocol() {
	let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let xml = manifest.join("protocols/xdg-shell.xml");
	let proto = parse_protocol_xml(&xml).expect("parse protocol");

	// Basic assertions about the protocol contents we expect from the
	// bundled xdg-shell.xml.
	assert_eq!(proto.name, "xdg-shell");
	let names: Vec<_> = proto.interfaces.iter().map(|i| i.name.as_str()).collect();
	assert!(names.contains(&"xdg_wm_base"));
	assert!(names.contains(&"xdg_surface"));
	assert!(names.contains(&"xdg_toplevel"));

	// Find xdg_surface and assert it has configure event and ack_configure request
	let surf = proto
		.interfaces
		.iter()
		.find(|i| i.name == "xdg_surface")
		.unwrap();
	assert!(surf.requests.iter().any(|r| r.name == "ack_configure"));
	assert!(surf.events.iter().any(|e| e.name == "configure"));
}
