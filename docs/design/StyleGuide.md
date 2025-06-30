# Engage UX Developer's Style Guide

## Normative Reference

The [Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/) is the baseline for all code in this project. Variations from the baseline are clearly documented here, and configuration files are included to lint and format according to this document.

> "Formatting code is a mostly mechanical task which takes both time and mental effort. By using an automatic formatting tool, a programmer is relieved of this task and can concentrate on more important things." - Rust Style Guide

## Additional Style Rules

### Use Tabs, not Spaces

Use tab characters for indentation, spaces for alignment.

```rust
fn lorem() -> usize {
	42 // tabs before 42
}
```

### Unix Newlines

Always use the Line Feed (`\n`)character to end lines, in the Unix style.

### Always Use the Vertical Layout

Always separate elements into the vertical layout. Avoid using any of the "small" layout options.

```rust
use foo::{
    xxx,
    yyy,
    zzz,
};

trait Lorem {
    fn lorem(
        ipsum: Ipsum,
        dolor: Dolor,
        sit: Sit,
        amet: Amet,
    );

    fn lorem(
        ipsum: Ipsum,
        dolor: Dolor,
        sit: Sit,
        amet: Amet,
    ) {
        // body
    }

    fn lorem(
        ipsum: Ipsum,
        dolor: Dolor,
        sit: Sit,
        amet: Amet,
        consectetur: Consectetur,
        adipiscing: Adipiscing,
        elit: Elit,
    );

    fn lorem(
        ipsum: Ipsum,
        dolor: Dolor,
        sit: Sit,
        amet: Amet,
        consectetur: Consectetur,
        adipiscing: Adipiscing,
        elit: Elit,
    ) {
        // body
    }
}
```

### Wildcard Suffixes

Use the expansion sigil `..` instead of multiple wildcard suffixes `_, _`.

```rust
fn main() {
    let (lorem, ipsum, ..) = (1, 2, 3, 4);
}
```

### Floating Number Trailing Zeros

Use a trailing zero for all floating point numbers that do not have a suffix.

```rust
fn main() {
    let values = [1.0, 2.0, 3e10, 4f32];
}
```

### Multiline Blocks

Multiline closure and match arm bodies should be wrapped in a block.

```rust
fn main() {
    result.and_then(|maybe_value| {
        match maybe_value {
            None => foo(),
            Some(value) => bar(),
        }
    });

    match lorem {
        None => {
            |ipsum| {
                println!("Hello World");
            }
        }
        Some(dolor) => foo(),
    }
}
```

### Format All Code

Format code snippet included in doc comments following these same rules.

Format the metavariable matching patterns in macros.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```rust
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

macro_rules! foo {
    ($a:ident : $b:ty) => {
        $a(42): $b;
    };
    ($a:ident $b:ident $c:ident) => {
        $a = $b + $c;
    };
}
```

### Uppercase Hex Literals

Format hexidecimal literals in all uppercase.

```rust
let hex = 0x10ABCDEF;
```

### Comments

For one or two line comments, use the single line comment sigil, `//`. For comments or three or more lines, use the multiline comment sigils, `/*` and `*/`.

Do not use inline comments.

The comment should precede the item it explains.

Use `//!` and `///` for doc comments.

```rust
// Single line comment
fn foo() -> void {}

// Two line
// commane
fn foo() -> void {}

/*
	Three
	line
	comment
*/
fn foo() -> void {}

//! Example documentation

/// Example item documentation
pub enum Foo {}
```

### "Small" Items

Do not use small item style formatting. This reduces the complexity of merging and increases readability for our visually challenged developers.

Do not use single line block elements. The only exception is that the empty block should be written as `{}`.

### Sorting

Group `use` statements by crate. Source header elements should be ordered as follows:

1. `extern` declarations
2. `mod` declarations
3. `use` statements
	1. `std`, `core`, and `alloc`
	2. External crates
	3. `self`, `super`, and `crate` imports

```rust
extern "C" {
	public static lorem: c_init;
}

mod components;

use alloc::alloc::Layout;
use core::f32;
use std::sync::Arc;

use broker::database::PooledConnection;
use chrono::Utc;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

use super::schema::{Context, Payload};
use super::update::convert_publish_payload;
use crate::models::Event;
```

### Generics

Use `where` clauses for all composed generic types.

```rust
fn lorem<Ipsum, Dolor, Sit, Amet>() -> T
where
    Ipsum: Eq,
    Dolor: Eq,
    Sit: Eq,
    Amet: Eq,
{
    // body
}
```

### Use the Try Shorthand

Use the try shorthand `?` instead of `try!`.

```rust
fn main() {
    let lorem = ipsum.map(|dolor| dolor.sit())?;
}
```
