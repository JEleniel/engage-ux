---
description: 'Agent directives for YAML formatting and linting.'
applyTo: '**/*.yaml'
---

# Agent Directives — YAML Formatting & Linting

If present, the repository's Prettier config (`.prettierrc.json`) is the source of truth for whitespace formatting. Preserve string quoting per the rules below.

## Principles

- **Safe & Readable**: YAML documents should be human-readable, avoid unnecessary use of advanced YAML features (merge keys, complex tags, excessive anchors), and be robust to common parsers.

## Formatting & Content Rules

- **Indentation**: Use 2 spaces for indentation in YAML. Do not use tabs.
- **Document Start/End**: Use `---` (three dashes) to separate multiple documents within a single file. Do not include `---` in a single-document file unless it is required by the consuming tool. The document-end marker `...` is optional and rarely required; avoid it unless needed.
- **Quoting**: Quote all strings. Prefer single quotes unless the string contains single quotes, in which case use double quotes.
- **Booleans & Nulls**: Use YAML booleans `true`/`false` (lowercase) and `null` for empty values. Avoid using `yes`/`no` unless required by a specific tool that expects them.
- **Anchors & Aliases**: Avoid anchors (`&`) and aliases (`*`). Name anchors descriptively and avoid accidental alias cycles. If the consumer of the YAML cannot handle aliases, reify duplicates instead of using anchors.
- **Avoid Complex Tags**: Do not use explicit YAML tags (`!!python/object:...`) or custom tags unless the receiving application requires them.
- **Comments**: Use `#` comments to explain non-obvious choices, but never store secrets or credentials in commented areas. Comments are permitted and encouraged for clarity.
- **Key Order & Minimal Diffs**: Preserve the existing key ordering when editing a file to reduce noise in diffs. When adding keys to a known manifest (e.g., GitHub Actions), follow commonly accepted semantic ordering for that manifest. Avoid reordering unrelated keys.
- **No Trailing Commas**: YAML does not use trailing commas — ensure lists and mappings are valid for strict parsers.
- **Encoding & EOF**: Save YAML files as UTF-8 without BOM and ensure exactly one trailing newline at EOF. Use Unix (LF) line endings only.

## Validation & Schema

- If a YAML file is schema-validated by the consuming tool, adhere to that schema. Prefer preserving key order and string quoting over stylistic rewrites.
