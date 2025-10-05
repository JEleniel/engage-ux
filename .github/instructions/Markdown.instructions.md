---
applyTo: '*.md*'
---

# Markdown Style Guide

This document defines formatting and style conventions for all Rust codez These rules are enforced by the project's `rustfmt.toml` configuration.

---

## Documentation Conventions

-   Use clear, well-structured GitHub‑Flavored Markdown (GFM).
-   Match the tone, style, and structure of existing documentation.
-   Cross-reference related docs where relevant; include appropriate direct links.
-   Cite project details with file and section references.

## Formatting Rules

-   Include blanks around fences = true,
-   Include blanks around lists = true,
-   Include blanks around tables = true,
-   Use grave (backtick) code fences and always include a language. Use `text` if no other language applies.
-   Example commands should show output = true,
-   Use underscore for emphasis and asterisks for strong.
-   Increment heading increment by one level at a time. Do not skip levels.
-   Do not use heading 1 except for the first line of the file.
-   Use atx style headings (with #) starting at the first character of the line and a single space after the hash marks.
-   Use dashes for horizontal rules.
-   Let lines wrap naturally. Do not hard wrap lines.
-   Do not use link fragments = true,
-   Include link image reference definitions = true,
-   Do not use collapsed of shortcut link image styles.
-   Indent sublists by one tab.
-   Use hard tabs for indentation.
-   Include alternative text for images.
-   Do not use bare URLs. Use link references instead.
-   Do not put blank lines inside blockquotes.
-   Sibling headings must be unique.
-   Do not use emphasis or strong as a heading.
-   No empty links = true,
-   Consolidate blank lines
-   Do not use two spaces together.
-   Do not use reversed links
-   Format code according to the language and other guidelines specified in the language section of the style guide.
-   Do not include additional spaces inside emphasis or strong, links, or code spans.
-   no trailing punctuation = true,
-   no trailing spaces
-   Use sequential numbers for ordered lists.
-   End the file with a single trailing newline = true,
-   Ensure tables have both leading and trailing pipe characters and that all rows have the correct column count. Align column according to the type of data.
-   Each nested unordered list should have a different bullet style.
-   Inline HTML is allowed but should be avoided unless necessary.

## Enforcement and usage

These rules are enforced by the repository's Markdownlint configuration (`.markdownlint.json`). Validate and autofix files with the project's Markdownlint setup.
