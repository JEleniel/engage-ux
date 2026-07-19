---
description: 'Agent directives for Markdown formatting and linting.'
applyTo: '**/*.md'
---

# Markdown Style Guide

If present, the repository's Markdown lint config (`.markdownlint-cli2.jsonc`) is the source of truth for formatting.
When a consuming platform or tool requires a conflicting Markdown convention, follow the consumer's requirements.

## Guidelines

- Use only ATX headings (`#`, `##`, …) in title case; do not indent headings.
- Use a single H1.
- Increase heading levels one at a time; do not skip levels.
    - Sibling headings must be unique.
- Do not use emphasis/strong as the entire heading text.
- Do not hard-wrap paragraph text.
- Do not use extra spaces in prose. Tables may use spacing within cells.
- Do not manually align Markdown with spaces outside tables.
- End files with exactly one trailing newline.
- Avoid multiple blank lines together.
- Unordered lists MUST follow `.markdownlint-cli2.jsonc` (`ul-style: dash`, `ul-indent.indent: 4`): use `-` for list items.
- Indent each nested list level using 4 spaces.

```markdown
- Level 1
    - Level 2
        - Level 3
```

- Ordered lists MUST use sequential numbers.
- You MUST use fenced code blocks with backticks.
- You MUST include a language for all code blocks. Use `text` when no specific language applies.
- When documenting a command, include sample output.
- Do not use bare URLs; use Markdown links. Avoid reversed links. Do not create empty links.
- You MAY use inline HTML but SHOULD avoid it unless it is necessary.
- You MUST use leading and trailing pipe characters for all table rows and ensure consistent column counts in every row.
- Keep blank lines around tables, code blocks, callouts, and headings.
- Prefer direct links `[]()` over reference style links `[][]`.

## Frontmatter

- Markdown frontmatter MUST use `---` delimiters.
- Only include frontmatter when required by the consuming tool.

## Tips

- Don't waste a lot of time manually formatting Markdown, if `markdownlint-cli2` is available use it instead.
