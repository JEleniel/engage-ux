use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, DeriveInput, Meta, parse_macro_input};

use darling::FromMeta;

#[derive(FromMeta, Debug)]
struct EventAttr {
	#[darling(rename = "crate")]
	#[darling(default)]
	crate_path: Option<String>,

	#[darling(default)]
	name: Option<String>,
}

#[proc_macro_derive(Event, attributes(event))]
pub fn derive_event(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident.clone();

	let default_crate = "engage_ux_core::events".to_string();

	// parse type-level attribute
	let (crate_path, base_name) = match parse_event_attr(&input.attrs) {
		Ok((c, n)) => (c.unwrap_or(default_crate.clone()), n),
		Err(e) => return e.to_compile_error().into(),
	};

	// build crate token stream
	let crate_tokens: proc_macro2::TokenStream = match crate_path.parse() {
		Ok(t) => t,
		Err(_) => {
			return syn::Error::new_spanned(
				&name,
				"invalid crate path in #[event(crate = \"...\")] ",
			)
			.to_compile_error()
			.into();
		}
	};

	let error_ident = format_ident!("{}EventError", name);

	let generated = match &input.data {
		syn::Data::Struct(_) => {
			let event_name = base_name.unwrap_or_else(|| name.to_string());
			let event_name_lit = proc_macro2::Literal::string(&event_name);

			quote! {
				#[derive(thiserror::Error, Debug)]
				pub enum #error_ident {
					#[error("failed to serialize event payload: {0}")]
					Serialize(#[from] ::serde_json::Error),
					#[error("invalid event attribute or crate path")]
					InvalidAttribute,
				}

				impl ::std::convert::TryFrom<#name> for #crate_tokens::EventType {
					type Error = #error_ident;

					fn try_from(src: #name) -> Result<Self, Self::Error> {
						let data = ::serde_json::to_string(&src)?;
						Ok(#crate_tokens::EventType::Custom { name: #event_name_lit.to_string(), data })
					}
				}

				impl #name {
					pub fn try_to_event(self, target: ::engage_ux_core::types::ComponentId) -> Result<#crate_tokens::Event, #error_ident> {
						let evt_type: #crate_tokens::EventType = ::std::convert::TryFrom::try_from(self)?;
						Ok(#crate_tokens::Event::new(target, evt_type))
					}
				}
			}
		}

		syn::Data::Enum(enm) => {
			let mut arms = Vec::new();
			for variant in &enm.variants {
				let v_ident = &variant.ident;
				let (_v_crate, v_name) = match parse_event_attr(&variant.attrs) {
					Ok((c, n)) => (c, n),
					Err(e) => return e.to_compile_error().into(),
				};

				let name_lit = match v_name {
					Some(n) => proc_macro2::Literal::string(&n),
					None => proc_macro2::Literal::string(&format!("{}::{}", name, v_ident)),
				};

				let pat = match &variant.fields {
					syn::Fields::Named(_) => quote! { #name::#v_ident { .. } },
					syn::Fields::Unnamed(_) => quote! { #name::#v_ident ( .. ) },
					syn::Fields::Unit => quote! { #name::#v_ident },
				};

				// Produce an Option<EventType> from the match so the match does not
				// always diverge (which caused an `unreachable_code` warning when
				// subsequent code followed the match). Use `?` for serde errors.
				arms.push(quote! {
					#pat => {
						let data = ::serde_json::to_string(&src)?;
						Some(#crate_tokens::EventType::Custom { name: #name_lit.to_string(), data })
					}
				});
			}

			quote! {
				#[derive(thiserror::Error, Debug)]
				pub enum #error_ident {
					#[error("failed to serialize event payload: {0}")]
					Serialize(#[from] ::serde_json::Error),
					#[error("invalid event attribute or crate path")]
					InvalidAttribute,
				}

				impl ::std::convert::TryFrom<#name> for #crate_tokens::EventType {
					type Error = #error_ident;

					fn try_from(src: #name) -> Result<Self, Self::Error> {
						let maybe = match &src {
							#(#arms)*
							_ => None,
						};

						if let Some(evt) = maybe {
							return Ok(evt);
						}

						let data = ::serde_json::to_string(&src)?;
						Ok(#crate_tokens::EventType::Custom { name: format!("{}", stringify!(#name)).to_string(), data })
					}
				}

				impl #name {
					pub fn try_to_event(self, target: ::engage_ux_core::types::ComponentId) -> Result<#crate_tokens::Event, #error_ident> {
						let evt_type: #crate_tokens::EventType = ::std::convert::TryFrom::try_from(self)?;
						Ok(#crate_tokens::Event::new(target, evt_type))
					}
				}
			}
		}

		_ => {
			return syn::Error::new_spanned(
				name,
				"Event derive is only supported for structs and enums",
			)
			.to_compile_error()
			.into();
		}
	};

	// Re-emit the original input but strip `#[event(...)]` attributes and the
	// `Event` entry inside any `#[derive(...)]` attribute. We perform this by
	// converting the original input tokens to a string, manipulating it, and
	// parsing back into a token stream. This is slightly hacky but simple and
	// robust for our uses.
	let mut original_tokens = input.clone().into_token_stream().to_string();

	// Remove all `#[event(...)]` attributes.
	while let Some(start) = original_tokens.find("#[event") {
		if let Some(end) = original_tokens[start..].find(']') {
			original_tokens.replace_range(start..start + end + 1, "");
		} else {
			break;
		}
	}

	// Aggressively remove occurrences of the token `Event` (which is only used
	// to refer to our derive) to avoid re-invoking the macro when re-emitting
	// the original input. This also removes it from any derive lists.
	let mut out = original_tokens.clone();
	out = out.replace("Event", "");
	// Clean up leftover commas/spaces inside derive lists: '(A, , B)' -> '(A, B)'
	while out.contains(", ,") {
		out = out.replace(", ,", ",");
	}
	out = out.replace("( ,", "(");
	out = out.replace(", )", ")");

	// Parse the cleaned original tokens back into a TokenStream
	let original_ts: proc_macro2::TokenStream = out
		.parse()
		.unwrap_or_else(|_| input.clone().into_token_stream());

	let expanded = quote! {
		#original_ts
		#generated
	};

	expanded.into()
}

/// Parse `#[event(...)]` attributes. Accepts either a keyed form or the
/// shorthand string literal form `#[event("name")]`.
fn parse_event_attr(attrs: &[Attribute]) -> Result<(Option<String>, Option<String>), syn::Error> {
	for attr in attrs.iter() {
		if let Meta::List(list) = &attr.meta {
			if !list.path.is_ident("event") {
				continue;
			}

			// Try darling for structured forms like #[event(name = "..", crate = "..")]
			if let Ok(ev) = EventAttr::from_meta(&attr.meta) {
				return Ok((ev.crate_path, ev.name));
			}

			// Fallback: naive token parsing for shorthand #[event("name")] or name = "..." / crate = "..."
			let s = list.tokens.to_string();
			let trimmed = s.trim();
			if !trimmed.contains('=') {
				// shorthand form, look for a single string literal
				if let Some(first_q) = trimmed.find('"') {
					if let Some(end_q) = trimmed[first_q + 1..].find('"') {
						let name = trimmed[first_q + 1..first_q + 1 + end_q].to_string();
						return Ok((None, Some(name)));
					}
				}
			}

			// look for name = "..." or crate = "..."
			if let Some(idx) = s.find("crate") {
				if let Some(eq_idx) = s[idx..].find('=') {
					let rest = &s[idx + eq_idx + 1..];
					if let Some(start_q) = rest.find('"') {
						if let Some(end_q) = rest[start_q + 1..].find('"') {
							let crate_path = rest[start_q + 1..start_q + 1 + end_q].to_string();
							return Ok((Some(crate_path), None));
						}
					}
				}
			}

			if let Some(idx) = s.find("name") {
				if let Some(eq_idx) = s[idx..].find('=') {
					let rest = &s[idx + eq_idx + 1..];
					if let Some(start_q) = rest.find('"') {
						if let Some(end_q) = rest[start_q + 1..].find('"') {
							let name = rest[start_q + 1..start_q + 1 + end_q].to_string();
							return Ok((None, Some(name)));
						}
					}
				}
			}
		}
	}

	Ok((None, None))
}

#[cfg(test)]
mod tests {
	use super::*;
	use syn::parse_quote;

	#[test]
	fn parse_shorthand_attr() {
		let a: Attribute = parse_quote!(#[event("my_shorthand")]);
		let (c, n) = parse_event_attr(&[a]).unwrap();
		assert_eq!(c, None);
		assert_eq!(n, Some("my_shorthand".to_string()));
	}

	#[test]
	fn parse_keyed_attr() {
		let a: Attribute = parse_quote!(#[event(name = "the_name", crate = "my::crate")]);
		let (c, n) = parse_event_attr(&[a]).unwrap();
		assert_eq!(c, Some("my::crate".to_string()));
		assert_eq!(n, Some("the_name".to_string()));
	}

	#[test]
	fn no_event_attr_returns_none() {
		// An unrelated attribute should produce (None, None)
		let a: Attribute = parse_quote!(#[allow(dead_code)]);
		let (c, n) = parse_event_attr(&[a]).unwrap();
		assert_eq!(c, None);
		assert_eq!(n, None);
	}
}
