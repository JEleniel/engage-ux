//! Crate `engage-ux-derive` — procedural derives for Engage UX
//!
//! This crate provides a small set of procedural macros used by Engage UX
//! to make it ergonomic to convert strongly-typed payloads into the
//! framework's `Event` representation.  Currently the crate exposes a
//! single derive macro:
//!
//! - `#[derive(Event)]` — implement conversion of a `struct` or `enum`
//!   into an `engage_ux_core::events::Event` (via `TryFrom` + helper
//!   method `try_to_event`).
//!
//! Features and behaviour
//! - Works on `struct` and `enum` types that implement `serde::Serialize`.
//! - Supports a type-level or variant-level attribute `#[event(...)]` to
//!   override the event name or the path to the events crate.
//! - By default the generated code references `engage_ux_core::events` as
//!   the events crate path; you can override this with
//!   `#[event(crate = "my::events")]`.
//!
//! Example
//! ```rust,ignore
//! use engage_ux_derive::Event;
//! use serde::Serialize;
//! use engage_ux_core::types::ComponentId;
//!
//! #[derive(Serialize, Event)]
//! #[event("my_custom_event")]
//! struct MyPayload { value: i32 }
//!
//! fn make_event(target: ComponentId) -> engage_ux_core::events::Event {
//!     // `try_to_event` returns Result<Event, _>; unwrap here for brevity
//!     MyPayload { value: 42 }.try_to_event(target).unwrap()
//! }
//! ```
//!
//! Notes
//! - The derive macro serializes payloads with `serde_json` internally and
//!   maps serialization errors into a generated `*EventError` type.
//! - The macro intentionally strips the `Event` token from the re-emitted
//!   original tokens to avoid recursive expansion when the original type is
//!   re-emitted into the final expanded output.
//!
//! See the repository docs for a user guide: `docs/api/core/derive.md`.

use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, DeriveInput, Meta, parse_macro_input};

use darling::FromMeta;

// Type alias to reduce verbose return types in `parse_event_attr` and
// silence clippy's `type_complexity` suggestion.
type EventAttrTuple = (Option<String>, Option<String>, Option<String>);
#[derive(FromMeta, Debug)]
struct EventAttr {
	#[darling(rename = "crate")]
	#[darling(default)]
	crate_path: Option<String>,

	#[darling(default)]
	name: Option<String>,

	#[darling(default)]
	category: Option<String>,
}

/// Derive macro `Event`.
///
/// When applied to a `struct` or `enum` that implements
/// `serde::Serialize`, this macro generates:
/// - a `TryFrom<YourType>` impl to convert into
///   `engage_ux_core::events::EventType` (serializing the payload to JSON),
/// - a convenience method `try_to_event(self, target: ComponentId) -> Result<Event, _>`.
///
/// Supported attributes:
/// - `#[event("name")]` — shorthand to set the event name
/// - `#[event(name = "...")]` — explicit name
/// - `#[event(crate = "path::to::events")]` — override the events crate path
///
/// The macro emits a generated `*EventError` enum for mapping serde errors
/// and attribute parsing errors.
#[proc_macro_derive(Event, attributes(event))]
pub fn derive_event(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident.clone();

	let default_crate = "engage_ux_core::events".to_string();

	// parse type-level attribute (crate path, optional name, optional category)
	let (crate_path, base_name, base_category) = match parse_event_attr(&input.attrs) {
		Ok((c, n, cat)) => (c.unwrap_or(default_crate.clone()), n, cat),
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
			// For structs we require a category to be present on the type-level
			let event_name = base_name.unwrap_or_else(|| name.to_string());
			let category = match &base_category {
				Some(c) => c.clone(),
				None => {
					return syn::Error::new_spanned(
						&name,
						"missing required `category = \"...\"` in #[event(...)] for struct",
					)
					.to_compile_error()
					.into();
				}
			};

			let event_name_lit = proc_macro2::Literal::string(&event_name);
			let category_lit = proc_macro2::Literal::string(&category);

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
						let data = ::serde_json::to_value(&src)?;
						Ok(#crate_tokens::EventType::Custom { name: #event_name_lit.to_string(), category: #category_lit.to_string(), data })
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
				let (_v_crate, v_name, v_category) = match parse_event_attr(&variant.attrs) {
					Ok((c, n, cat)) => (c, n, cat),
					Err(e) => return e.to_compile_error().into(),
				};

				let name_lit = match v_name {
					Some(n) => proc_macro2::Literal::string(&n),
					None => proc_macro2::Literal::string(&format!("{}::{}", name, v_ident)),
				};

				// determine category for this variant: variant-level overrides type-level
				let chosen_category = match &v_category {
					Some(c) => c.clone(),
					None => match &base_category {
						Some(bc) => bc.clone(),
						None => {
							return syn::Error::new_spanned(
								&variant.ident,
								"missing required `category = \"...\"` in #[event(...)] for enum variant",
							)
							.to_compile_error()
							.into();
						}
					},
				};
				let category_lit = proc_macro2::Literal::string(&chosen_category);

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
						let data = ::serde_json::to_value(&src)?;
						Some(#crate_tokens::EventType::Custom { name: #name_lit.to_string(), category: #category_lit.to_string(), data })
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

						let data = ::serde_json::to_value(&src)?;
						let category = match &#base_category {
							Some(c) => c.to_string(),
							None => return Err(#error_ident::InvalidAttribute),
						};
						Ok(#crate_tokens::EventType::Custom { name: format!("{}", stringify!(#name)).to_string(), category, data })
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
fn parse_event_attr(
	attrs: &[Attribute],
) -> Result<EventAttrTuple, syn::Error> {
	for attr in attrs.iter() {
		if let Meta::List(list) = &attr.meta {
			if !list.path.is_ident("event") {
				continue;
			}

			// Try darling for structured forms like #[event(name = "..", crate = "..")]
			if let Ok(ev) = EventAttr::from_meta(&attr.meta) {
				return Ok((ev.crate_path, ev.name, ev.category));
			}

			// Fallback: naive token parsing for shorthand #[event("name")] or name = "..." / crate = "..."
			let s = list.tokens.to_string();
			let trimmed = s.trim();
			if !trimmed.contains('=') {
				// shorthand form, look for a single string literal
				if let Some(first_q) = trimmed.find('"')
					&& let Some(end_q) = trimmed[first_q + 1..].find('"') {
						let name = trimmed[first_q + 1..first_q + 1 + end_q].to_string();
						return Ok((None, Some(name), None));
					}
			}

			// look for name = "..." or crate = "..."
			if let Some(idx) = s.find("crate")
				&& let Some(eq_idx) = s[idx..].find('=') {
					let rest = &s[idx + eq_idx + 1..];
					if let Some(start_q) = rest.find('"')
						&& let Some(end_q) = rest[start_q + 1..].find('"') {
							let crate_path = rest[start_q + 1..start_q + 1 + end_q].to_string();
							return Ok((Some(crate_path), None, None));
						}
				}

			if let Some(idx) = s.find("name")
				&& let Some(eq_idx) = s[idx..].find('=') {
					let rest = &s[idx + eq_idx + 1..];
					if let Some(start_q) = rest.find('"')
						&& let Some(end_q) = rest[start_q + 1..].find('"') {
							let name = rest[start_q + 1..start_q + 1 + end_q].to_string();
							return Ok((None, Some(name), None));
						}
				}

			if let Some(idx) = s.find("category")
				&& let Some(eq_idx) = s[idx..].find('=') {
					let rest = &s[idx + eq_idx + 1..];
					if let Some(start_q) = rest.find('"')
						&& let Some(end_q) = rest[start_q + 1..].find('"') {
							let category = rest[start_q + 1..start_q + 1 + end_q].to_string();
							return Ok((None, None, Some(category)));
						}
				}
		}
	}

	Ok((None, None, None))
}

#[cfg(test)]
mod tests {
	use super::*;
	use syn::parse_quote;

	#[test]
	fn parse_shorthand_attr() {
		let a: Attribute = parse_quote!(#[event("my_shorthand")]);
		let (c, n, cat) = parse_event_attr(&[a]).unwrap();
		assert_eq!(c, None);
		assert_eq!(n, Some("my_shorthand".to_string()));
		assert_eq!(cat, None);
	}

	#[test]
	fn parse_keyed_attr() {
		let a: Attribute = parse_quote!(#[event(name = "the_name", crate = "my::crate")]);
		let (c, n, cat) = parse_event_attr(&[a]).unwrap();
		assert_eq!(c, Some("my::crate".to_string()));
		assert_eq!(n, Some("the_name".to_string()));
		assert_eq!(cat, None);
	}

	#[test]
	fn no_event_attr_returns_none() {
		// An unrelated attribute should produce (None, None)
		let a: Attribute = parse_quote!(#[deprecated]);
		let (c, n, cat) = parse_event_attr(&[a]).unwrap();
		assert_eq!(c, None);
		assert_eq!(n, None);
		assert_eq!(cat, None);
	}
}
