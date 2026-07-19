---
description: 'Agent directives for NDJSON (newline-delimited JSON) formatting and handling.'
applyTo: '**/*.ndjson'
---

# NDJSON Formatting & Handling

NDJSON (newline-delimited JSON) stores one complete JSON value per line (typically one object per line). Treat NDJSON as a streaming format: do not rewrite it into arrays or pretty-printed multi-line JSON.
When a consuming platform, tool, or specification requires a conflicting NDJSON convention, follow the consumer's requirements.

## Format rules

- One JSON value per line. No blank lines.
- Each line MUST be valid JSON on its own (no trailing commas, no comments).
- Do not pretty-print across lines. Keep entries single-line.
- Encoding: UTF-8 without BOM.
- Line endings: LF (`\n`).
- EOF: exactly one trailing newline.
- Do not add unnecessary or trailing whitespace.

## Editing rules

- Prefer append-only updates when the file is a log (add a new line; do not edit existing lines).
- Preserve existing entry order unless the consuming tool explicitly defines a different ordering.
- Avoid changing existing lines unless necessary; minimize diffs.
