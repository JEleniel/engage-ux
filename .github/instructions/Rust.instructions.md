---
applyTo: '*.rs'
---

# Rust Style Guide

This document defines formatting and style conventions for all Rust codez These rules are enforced by the project's `rustfmt.toml` configuration.

---

## Formatting Rules

-   **Indentation:** Use hard tabs for indentation. Do not use spaces.
-   **Line Endings:** Use Unix-style newlines (`\n`).
-   **Comment Width:** Limit comments to 100 characters per line.
-   **Comment Formatting:** Normalize comments and doc attributes. Wrap comments for readability.
-   **Doc Comments:** Format code in documentation comments. Use `//!` for module/crate docs and `///` for item docs.
-   **Imports:** Group imports by standard, external, and crate. Use crate-level granularity and reorder implementation items.
-   **Hex Literals:** Use uppercase for hex literals.
-   **Wildcards:** Condense wildcard suffixes in patterns.
-   **Macros:** Format macro matchers for clarity.
-   **Strings:** Format string literals for consistency.
-   **Field Initialization:** Use field init shorthand where possible.
-   **Try Shorthand:** Prefer the `?` operator for error propagation.
-   **General:** Normalize all code and documentation attributes.
-   Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/) for idiomatic code for all rules not covered here.

---

## Best Practices

-   Write clear, concise, and well-documented code.
-   Include comments for non-obvious logic.
-   Do not use unsafe code; all code must be 100% safe Rust.
-   Ensure code compiles and passes all tests and lints.

---

## Enforcement

Run the following command to check formatting:

```bash
cargo fmt --all -- --check
```
