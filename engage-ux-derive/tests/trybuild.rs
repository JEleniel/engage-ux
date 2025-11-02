#[test]
fn ui_tests() {
	// Placeholder: trybuild UI tests were removed in favor of a simpler integration test.
	// Keep this test to avoid running the old trybuild harness during CI.
	let t = trybuild::TestCases::new();
	// compile_fail for files under tests/ui/*.rs
	t.compile_fail("tests/ui/*.rs");
}
