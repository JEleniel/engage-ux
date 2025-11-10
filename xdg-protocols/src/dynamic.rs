use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Protocol {
	pub name: String,
	pub interfaces: Vec<Interface>,
}

#[derive(Debug, PartialEq)]
pub struct Interface {
	pub name: String,
	pub version: Option<u32>,
	pub requests: Vec<Message>,
	pub events: Vec<Message>,
}

#[derive(Debug, PartialEq)]
pub struct Message {
	pub name: String,
	// For the prototype we keep arguments as (name, type) pairs.
	pub args: Vec<(String, String)>,
}

/// Parse the protocol XML at `path` and return a lightweight description.
pub fn parse_protocol_xml(path: &Path) -> anyhow::Result<Protocol> {
	let txt = fs::read_to_string(path)?;
	let doc = roxmltree::Document::parse(&txt)?;

	let root = doc
		.descendants()
		.find(|n| n.has_tag_name("protocol"))
		.ok_or_else(|| anyhow::anyhow!("missing protocol root"))?;

	let proto_name = root.attribute("name").unwrap_or("unknown").to_string();

	let mut interfaces = Vec::new();

	for intf in root
		.children()
		.filter(|n| n.is_element() && n.has_tag_name("interface"))
	{
		let name = intf.attribute("name").unwrap_or("").to_string();
		let version = intf.attribute("version").and_then(|s| s.parse().ok());

		let mut requests = Vec::new();
		let mut events = Vec::new();

		for child in intf.children().filter(|n| n.is_element()) {
			if child.has_tag_name("request") || child.has_tag_name("event") {
				let msg_name = child.attribute("name").unwrap_or("").to_string();
				let mut args = Vec::new();
				for arg in child
					.children()
					.filter(|a| a.is_element() && a.has_tag_name("arg"))
				{
					let an = arg.attribute("name").unwrap_or("").to_string();
					let at = arg.attribute("type").unwrap_or("").to_string();
					args.push((an, at));
				}
				let msg = Message {
					name: msg_name,
					args,
				};
				if child.has_tag_name("request") {
					requests.push(msg);
				} else {
					events.push(msg);
				}
			}
		}

		interfaces.push(Interface {
			name,
			version,
			requests,
			events,
		});
	}

	Ok(Protocol {
		name: proto_name,
		interfaces,
	})
}
