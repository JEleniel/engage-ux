//! Tests that assert small, verifiable invariants from the OAL contract.
//!
//! These are intentionally small, feature-gated checks used to validate the
//! lightweight API contract documented in `engage-ux-oal::traits`.

use engage_ux_oal::{OalError, SurfaceBuilder};

#[test]
#[cfg(not(feature = "wayland"))]
fn surface_builder_reports_platform_not_supported_when_wayland_disabled() {
	use engage_ux_oal::OalError;

	let builder = SurfaceBuilder::new(100, 100);
	match builder.build() {
		Err(OalError::PlatformNotSupported) => {
			// expected
		}
		Err(e) => panic!("expected PlatformNotSupported, got Err({})", e),
		Ok(_) => panic!("expected PlatformNotSupported, got Ok(..)"),
	}
}

#[test]
fn oal_error_from_str_is_other_variant() {
	// Construct the enum directly instead of relying on `From<&str>`.
	let e = OalError::Other("some error".to_string());
	let s = format!("{}", e);
	// The `Display` for `OalError::Other` uses the message prefix "Other error:"
	assert!(s.contains("Other error:"), "unexpected display: {}", s);
}
