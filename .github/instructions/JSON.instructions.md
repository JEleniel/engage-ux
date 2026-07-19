---
description: 'Agent directives for JSON formatting and linting.'
applyTo: '**/*.json'
---

# JSON Formatting & Linting

If present, the repository's Prettier config (`.prettierrc.json`) is the source of truth for formatting.

## Formatting & Content Rules

- **Precedence**: Schema, specification, and consumer/tool requirements override the generic preferences in this document.
- **Include `$schema` when available**: Include a schema if available. Local schemas should use relative paths.
- **Best-effort schema validation**: When possible, validate the document against its declared schema. Do not block changes solely because validation cannot be performed.
- **Compliance**: Strict RFC 8259 compliance (no comments, no trailing commas).
- **Encoding and Characters**: Encode as UTF-8 without BOM. Use double quotes for keys and strings and Unix line endings (`\n`). Include exactly one trailing newline.
- **Types**: Preserve primitive types; do not turn numbers/booleans into strings. Use ISO 8601 for dates/timestamps with (maximum) millisecond resolution and in UTC when possible, unless the declared schema or consuming system requires another format. Avoid ambiguous numeric formats.
- **Consistent Ordering**: Sort JSON by keys when generating it, maintain the existing order when editing.
- **Prettier**: Instead of wasting time formatting JSON, use `pnpm dlx prettier --write <file>` when available (and it is in the IDE).
