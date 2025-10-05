# Console Component

A terminal-like console display with ANSI escape code support.

## Overview

The Console component provides a terminal-like display for showing command output, logs, or other text with ANSI color and formatting support.

## Basic Usage

```rust
use engage_ux_components::Console;

let mut console = Console::new(1);
console.set_bounds(Rect::new(0.0, 0.0, 800.0, 400.0));

// Append text
console.append("System initialized\n");
console.append("\x1b[32mSuccess:\x1b[0m Operation complete\n");
```

## Properties

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| `content` | `Vec<String>` | Console lines | `[]` |
| `max_lines` | `Option<usize>` | Max lines to keep | `None` |
| `auto_scroll` | `bool` | Auto-scroll to bottom | `true` |
| `line_wrap` | `bool` | Wrap long lines | `true` |
| `show_timestamps` | `bool` | Show line timestamps | `false` |

## Methods

```rust
// Append text
console.append("Log message\n");
console.append_line("Auto-adds newline");

// ANSI colors
console.append("\x1b[31mError\x1b[0m\n"); // Red
console.append("\x1b[32mSuccess\x1b[0m\n"); // Green
console.append("\x1b[33mWarning\x1b[0m\n"); // Yellow

// Clear console
console.clear();

// Configuration
console.set_max_lines(Some(1000));
console.set_auto_scroll(true);
console.set_line_wrap(true);
console.set_show_timestamps(true);
```

## Examples

### Build Output Console

```rust
let mut build_console = Console::new(1);
build_console.set_auto_scroll(true);
build_console.set_max_lines(Some(500));

// Append build output
build_console.append("Compiling project...\n");
build_console.append("\x1b[32m✓\x1b[0m Compilation successful\n");
```

### Log Viewer

```rust
let mut log_viewer = Console::new(1);
log_viewer.set_show_timestamps(true);
log_viewer.set_line_wrap(true);

// Append logs
log_viewer.append_line("[INFO] Application started");
log_viewer.append_line("[ERROR] Connection failed");
```

### Command Output

```rust
struct CommandConsole {
    console: Console,
}

impl CommandConsole {
    fn execute(&mut self, command: &str) {
        self.console.append(format!("$ {}\n", command));
        
        // Execute and capture output
        let output = execute_command(command);
        
        if output.success {
            self.console.append(&output.stdout);
        } else {
            self.console.append(format!("\x1b[31m{}\x1b[0m", output.stderr));
        }
    }
}
```

## ANSI Color Codes

```rust
// Foreground colors
console.append("\x1b[30mBlack\x1b[0m\n");
console.append("\x1b[31mRed\x1b[0m\n");
console.append("\x1b[32mGreen\x1b[0m\n");
console.append("\x1b[33mYellow\x1b[0m\n");
console.append("\x1b[34mBlue\x1b[0m\n");
console.append("\x1b[35mMagenta\x1b[0m\n");
console.append("\x1b[36mCyan\x1b[0m\n");
console.append("\x1b[37mWhite\x1b[0m\n");

// Bold
console.append("\x1b[1mBold text\x1b[0m\n");

// Underline
console.append("\x1b[4mUnderlined\x1b[0m\n");
```

## Accessibility

Consoles should be accessible:

```rust
console.set_aria_label("Console output");
console.set_aria_live("polite");
console.set_role("log");
```

## Related Components

- [Text Area](text-area.md) - For editable text
- [Text Editor](text-editor.md) - For rich text

---

[← Back to Components](index.md)
