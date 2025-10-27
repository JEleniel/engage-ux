# Troubleshooting Guide

Solutions to common problems when using Engage UX.

## Understanding Rendering Behavior

⚠️ **IMPORTANT**: Most examples use **stub backends** that do not display visual windows!

**Expected Behavior**:
- Examples like `basic_components`, `theme_demo`, `lcars_theme_demo` print console output only
- They do NOT open windows or show graphics on screen
- This is **intentional** for testing in headless environments

**For Actual Visual Rendering**:
See the working example that creates real windows:
```bash
cargo run --example visual_window_demo -p engage-ux-oal
```

For complete details, see [Actual Rendering Documentation](actual-rendering.md).

## Installation Issues

### Linker Errors During Build

**Problem**: `cargo build` fails with linker errors.

**Solution**: Install platform-specific build tools.

**Windows**:
```bash
# Install Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/
# Or install Visual Studio with C++ development tools
```

**macOS**:
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get update
sudo apt-get install build-essential libx11-dev
```

**Linux (Fedora)**:
```bash
sudo dnf install gcc gcc-c++ libX11-devel
```

**Linux (Arch)**:
```bash
sudo pacman -S base-devel libx11
```

### Rust Version Too Old

**Problem**: Compilation errors about unsupported Rust features.

**Solution**: Update Rust to 1.70 or later.

```bash
# Update rustup and Rust
rustup update

# Check version
rustc --version
```

### Cannot Find Crate `engage-ux-*`

**Problem**: Cargo can't find Engage UX crates.

**Solution**: Ensure correct repository reference in `Cargo.toml`:

```toml
[dependencies]
engage-ux-core = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-components = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-themes = { git = "https://github.com/JEleniel/engage-ux" }
engage-ux-oal = { git = "https://github.com/JEleniel/engage-ux" }
```

Or for local development:

```toml
[dependencies]
engage-ux-core = { path = "../engage-ux/engage-ux-core" }
engage-ux-components = { path = "../engage-ux/engage-ux-components" }
engage-ux-themes = { path = "../engage-ux/engage-ux-themes" }
engage-ux-oal = { path = "../engage-ux/engage-ux-oal" }
```

## Build Issues

### Clean Build Required

**Problem**: Strange compilation errors after updates.

**Solution**: Clean and rebuild:

```bash
cargo clean
cargo build
```

### Out of Memory During Compilation

**Problem**: Compiler runs out of memory.

**Solution**: Reduce parallel jobs:

```bash
# Use fewer parallel jobs
cargo build -j 2

# Or set in .cargo/config.toml
[build]
jobs = 2
```

### Tokio Runtime Errors

**Problem**: Errors about Tokio runtime not found.

**Solution**: Ensure Tokio is in dependencies with correct features:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

For tests:
```rust
#[tokio::test]
async fn my_test() {
    // test code
}
```

For main:
```rust
#[tokio::main]
async fn main() {
    // app code
}
```

## Runtime Issues

### Components Not Rendering

**Problem**: Components are created but nothing appears on screen.

**Explanation**: Phase 2 implements component architecture and logic. Full rendering requires platform-specific backends (Phase 3).

**Workaround**: Use examples to validate component behavior:

```bash
cargo run --example basic_components -p engage-ux-components
```

**Future**: Platform rendering backends coming in Phase 3.

### Events Not Firing

**Problem**: Event handlers not called.

**Solution**: Ensure proper event setup:

```rust
// Correct setup
button.set_on_click(|event| {
    println!("Clicked!");
});

// Trigger event
let event = Event::new(button.id(), EventType::Click);
button.handle_event(&event);
```

For async events:

```rust
#[tokio::main]
async fn main() {
    // Use Tokio runtime
    button.set_on_click(|event| {
        println!("Clicked!");
    });
}
```

### Color Parsing Fails

**Problem**: `Color::from_hex()` returns error.

**Solution**: Check format:

```rust
// Correct formats
Color::from_hex("#FF5733")?;      // 6 digits
Color::from_hex("#FF5733CC")?;    // 8 digits (with alpha)

// Invalid
Color::from_hex("FF5733")?;       // Missing #
Color::from_hex("#F57")?;         // Too short
```

## Theme Issues

### Theme Fails to Load

**Problem**: `Theme::from_json()` returns error.

**Solution**: Validate JSON format:

1. Check JSON syntax:
```bash
# Use a JSON validator
cat theme.json | jq .
```

2. Validate against schema:
```bash
jsonschema -i theme.json schemas/theme.schema.json
```

3. Check required fields:
```json
{
  "name": "My Theme",
  "colors": { ... },
  "typography": { ... },
  "spacing": { ... },
  "borders": { ... },
  "shadows": { ... }
}
```

### Theme Colors Not Applied

**Problem**: Components don't use theme colors.

**Solution**: Explicitly apply theme:

```rust
let theme = Theme::dark();

button.set_background_color(theme.colors.primary);
button.set_text_color(theme.colors.on_primary);
```

### Color Format Not Recognized

**Problem**: Custom color format fails.

**Solution**: Use supported formats:

```json
{
  "colors": {
    "hex": "#FF5733",
    "hex_alpha": "#FF5733CC",
    "rgb": [255, 87, 51],
    "rgba": [255, 87, 51, 0.8],
    "hsl": [9, 100, 60],
    "hsla": [9, 100, 60, 0.8]
  }
}
```

## Test Issues

### Tests Failing

**Problem**: `cargo test` fails.

**Solutions**:

1. Update dependencies:
```bash
cargo update
```

2. Run specific test for details:
```bash
cargo test test_name -- --nocapture
```

3. Check for test isolation:
```rust
// Use separate test data
#[test]
fn test_1() {
    let component = create_test_component(1);
    // ...
}

#[test]
fn test_2() {
    let component = create_test_component(2); // Different ID
    // ...
}
```

### Async Test Timeout

**Problem**: Async tests hang or timeout.

**Solution**: Use proper async test setup:

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = timeout(
        Duration::from_secs(5),
        async_function()
    ).await;
    
    assert!(result.is_ok());
}
```

### Thread Safety Issues

**Problem**: Errors about `Send` or `Sync` bounds.

**Solution**: Use proper synchronization:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

let component = Arc::new(RwLock::new(Button::new(1, "Click")));

// Clone for moving into closure
let component_clone = component.clone();
tokio::spawn(async move {
    let mut c = component_clone.write().await;
    c.set_text("Updated");
});
```

## Performance Issues

### Slow Compilation

**Problem**: `cargo build` takes very long.

**Solutions**:

1. Use incremental compilation (enabled by default):
```toml
[profile.dev]
incremental = true
```

2. Use `cargo check` for faster feedback:
```bash
cargo check
```

3. Build in release mode for production:
```bash
cargo build --release
```

4. Use `sccache` for caching:
```bash
# Install sccache
cargo install sccache

# Configure in ~/.cargo/config.toml
[build]
rustc-wrapper = "sccache"
```

### High Memory Usage

**Problem**: Application uses too much memory.

**Solutions**:

1. Profile memory usage:
```bash
# Install valgrind
cargo install cargo-valgrind

# Run with valgrind
cargo valgrind run
```

2. Check for memory leaks:
```rust
// Ensure proper cleanup
impl Drop for MyComponent {
    fn drop(&mut self) {
        // Clean up resources
    }
}
```

3. Use weak references where appropriate:
```rust
use std::sync::Weak;

// Break reference cycles
let weak_ref = Arc::downgrade(&component);
```

## Platform-Specific Issues

### Windows-Specific

**Problem**: UTF-8 path issues.

**Solution**: Use proper path handling:

```rust
use std::path::PathBuf;

let path = PathBuf::from("C:\\Users\\username\\file.txt");
```

**Problem**: Permission denied errors.

**Solution**: Run with appropriate permissions or change file location.

### macOS-Specific

**Problem**: "Unsigned binary" warnings.

**Solution**: Sign the binary or add exception in System Preferences > Security & Privacy.

**Problem**: Dark mode detection.

**Solution**: Use platform detection:

```rust
use engage_ux_oal::Platform;

let platform = Platform::detect();
if platform.supports_dark_mode() {
    let theme = Theme::dark();
} else {
    let theme = Theme::light();
}
```

### Linux-Specific

**Problem**: Wayland vs X11 issues.

**Solution**: Specify display server:

```bash
# Force X11
export GDK_BACKEND=x11

# Force Wayland
export GDK_BACKEND=wayland
```

**Problem**: Missing system libraries.

**Solution**: Install development packages:

```bash
# Ubuntu/Debian
sudo apt-get install libx11-dev libxext-dev libxrender-dev

# Fedora
sudo dnf install libX11-devel libXext-devel libXrender-devel
```

## IDE Issues

### VS Code IntelliSense Not Working

**Problem**: No code completion or error highlighting.

**Solution**:

1. Install rust-analyzer extension
2. Reload VS Code
3. Run `cargo check` to generate metadata

### Rust Analyzer Crashes

**Problem**: rust-analyzer extension crashes frequently.

**Solution**:

1. Update rust-analyzer
2. Increase memory limit in VS Code settings:
```json
{
  "rust-analyzer.server.extraEnv": {
    "RA_LOG": "info"
  }
}
```

## Getting Help

### Before Asking

1. **Check documentation**: [docs/index.md](index.md)
2. **Search issues**: [GitHub Issues](https://github.com/JEleniel/engage-ux/issues)
3. **Read FAQ**: [faq.md](faq.md)
4. **Try examples**: `cargo run --example basic_components -p engage-ux-components`

### How to Report Issues

Include the following information:

1. **Environment**:
   - OS and version
   - Rust version (`rustc --version`)
   - Cargo version (`cargo --version`)
   
2. **Minimal reproduction**:
   - Smallest code that reproduces the issue
   - Steps to reproduce
   
3. **Expected vs actual**:
   - What you expected to happen
   - What actually happened
   
4. **Error messages**:
   - Full error output
   - Stack traces if available

Example:

```markdown
**Environment:**
- OS: Ubuntu 22.04
- Rust: 1.75.0
- Cargo: 1.75.0

**Steps to reproduce:**
1. Create button with ID 1
2. Set click handler
3. Trigger event

**Expected:** Handler called
**Actual:** Handler not called

**Error:** None, just doesn't work

**Code:**
\```rust
let mut button = Button::new(1, "Click");
button.set_on_click(|_| println!("Clicked!"));
// How to trigger?
\```
```

### Where to Get Help

- **GitHub Discussions**: [Ask questions](https://github.com/JEleniel/engage-ux/discussions)
- **GitHub Issues**: [Report bugs](https://github.com/JEleniel/engage-ux/issues)
- **Documentation**: [Browse docs](index.md)
- **Examples**: [Code examples](examples/index.md)

## Common Error Messages

### "the trait bound `X: Send` is not satisfied"

**Cause**: Trying to use non-thread-safe type across threads.

**Solution**: Wrap in `Arc<RwLock<T>>`:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

let component = Arc::new(RwLock::new(component));
```

### "cannot find function `main` in module"

**Cause**: Missing `#[tokio::main]` attribute.

**Solution**:

```rust
#[tokio::main]
async fn main() {
    // Your code
}
```

### "borrowed value does not live long enough"

**Cause**: Reference lifetime issues.

**Solution**: Use owned values or adjust lifetimes:

```rust
// Instead of
let text = &temp_string;

// Use owned
let text = temp_string.clone();
```

### "no method named `set_text` found"

**Cause**: Wrong component type or missing import.

**Solution**: Check component type and imports:

```rust
use engage_ux_components::Button;

let mut button = Button::new(1, "Text");
button.set_text("New Text"); // Works for Button
```

## Still Having Issues?

If your problem isn't listed here:

1. [Open a discussion](https://github.com/JEleniel/engage-ux/discussions/new)
2. [Report a bug](https://github.com/JEleniel/engage-ux/issues/new?template=bug_report.md)
3. [Check the FAQ](faq.md)

---

[← Back to Documentation](index.md)
