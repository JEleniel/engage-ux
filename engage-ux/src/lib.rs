//! Top-level developer-facing crate for Engage UX.
//!
//! This crate re-exports the primary lower-level crates so consumers can depend
//! on a single, stable entrypoint: `engage-ux`.

// Re-export the lower-level crates under logical module names.
// Consumers can use e.g. `engage_ux::core::Color` or `engage_ux::oal::Monitor`.
pub use engage_ux_core as core;
pub use engage_ux_oal as oal;

// Optional: add commonly used re-exports for convenience.
pub use core::Color;

#[cfg(test)]
mod tests {
	#[test]
	fn sanity() {
		// Basic smoke test to ensure this crate compiles and re-exports work.
		let _ = "engage-ux".to_string();
	}
}
