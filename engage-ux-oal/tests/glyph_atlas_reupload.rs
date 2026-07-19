//! Glyph atlas reupload tests.
//!
//! Validates that the glyph atlas responds to device recreate events.

use engage_ux_oal::oal::device_lifecycle;
use engage_ux_oal::oal::text::GlyphAtlas;

#[test]
fn glyph_atlas_reuploads_on_device_recreated() {
	let atlas = GlyphAtlas::new();

	// Initially no device recorded and no reupload performed.
	assert_eq!(atlas.last_device_id(), None);
	assert!(!atlas.reuploaded());

	// Simulate a device recreation event.
	device_lifecycle::device_recreated(12345, 54321);

	// The atlas should have recorded the new device id and marked a reupload.
	assert_eq!(atlas.last_device_id(), Some(54321));
	assert!(atlas.reuploaded());
}
