---
name: documentation
description: Use this skill when writing or updating documentation, including in-code documentation comments.
---

# Documentation Skill

## General guidelines

- This skill must **never** generate, modify, or suggest changes to source code beyond documentation comments.
- If a `docs/design/aurora/` folder exists, you may read and follow the [Aurora Compact Model](../../aurora/Aurora.compact.instructions.md) to understand the design.
    - The Aurora Compact Model exists to save time and tokens by keeping key information in a single compact (thus the name) file.
    - Use the full Aurora instructions only when applying the Architecture skill.
- Accuracy-First Mindset: Treat every claim as potentially wrong until verified against source-of-truth artifacts.
- Task-Completion Focus: Prioritize whether a reader can complete the task safely and correctly.
- Security and Privacy Awareness: Ensure guidance does not expose secrets, Non-Public Information (NPI), or unsafe operational patterns.
- Accessibility for User-Facing Documentation: Require WCAG AA at minimum (AAA preferred where feasible).

## Outputs

- Repository-level Markdown documentation files (e.g., `README.md`, `CONTRIBUTING.md`).
- User documentation in Markdown under `docs/`, starting with `docs/README.md`. This explicitly excludes `docs/design/`, which is design documentation.
- Structured reference material (tables, lists, sections).

## Principles of Elegant Documentation

- Clarity: Readers can understand intent and action without external explanation.
- Simplicity: The document solves the reader's need with the shortest complete path.
- Traceability: Every key claim can be traced to a verifiable source of truth.
- Composability: Sections can be reused and linked without duplicating or fragmenting meaning.
- Predictability: Similar tasks are documented in similar patterns.
- Minimal Incidental Complexity: Content includes only what is necessary to execute safely and correctly.

### Indications of Poor Documentation

- Contradictory Guidance.
- Orphaned Documents.
- Unbounded Scope.
- Terminology Drift.
- Non-Deterministic Instructions.
- Leaky Internal Detail.

## Deliverables

- Clear, concise, easily navigable documentation artifacts that reflect the current state of the system.
- Explanation of inputs, outputs, and constraints.
- Noted assumptions and limitations.
- Cross-references to relevant artifacts.

## Operating Procedure

1. Confirm the audience, task scope, success criteria, and constraints from the user request and any relevant repository artifacts. If critical information is missing, stop and get clarification before writing.
2. Inspect the code, configuration, design artifacts, and existing documentation needed to verify every technical claim before updating documentation.
3. Update existing documentation in place unless a genuinely new artifact is needed for clarity, discoverability, or scope separation.
4. Write or revise content so that prerequisites, inputs, outputs, examples, failure paths, and constraints are explicit where they matter to safe and correct use.
5. Keep terminology, links, and cross-references consistent across related documents so readers can follow the documentation set without guessing.
6. Validate the result against the deliverables and validation checklists in this skill, ensuring that links resolve, documentation claims are verified, and any applicable file-specific instructions (for example Markdown formatting rules) pass.
7. If the request also requires code changes, planning, architecture work, or formal review output, switch to the appropriate skill for that phase.

## Validation Checklists

### Documentation Accuracy Checklist

- Verifiable claims backed by code, schemas, configuration, protocol contracts, or approved design artifacts.
- Accurate API and interface details, including names, parameters, flags, defaults, return values, and error behavior.
- Explicit version, platform, and environment scope where behavior differs.
- No internal contradictions within the document set.
- Examples, snippets, and sample outputs that reflect current behavior and constraints.

### Documentation Operability Checklist

- Declared prerequisites, including tools, permissions, credentials, environment variables, and setup steps.
- End-to-end procedure coverage for setup, execution, validation, and cleanup where applicable.
- Documented failure paths and recovery steps, not just the happy path.
- Clear inputs, outputs, and side effects.
- Included upgrade, migration, and compatibility notes where relevant.
- Copy-paste-safe commands and procedures.
- Clear warnings for destructive or irreversible actions.
- Explicit placeholders that cannot be mistaken for production values.
- Representative sample output where needed for validation.
- No secret or Non-Public Information leakage in examples, logs, or screenshots.
- Resolvable internal and external links.
- Specific reference targets when practical.
- No empty or misleading links.
- Stable cross-references.
- Evidence-backed references to auditable source artifacts.

### Documentation Clarity and Accessibility Checklist

- Clear heading structure that supports scanning.
- Concise wording with minimal redundancy or ambiguity.
- Consistent terminology across the document set.
- Actionable, sequenced, testable steps.
- Enough context before action for readers to understand risk and intent.
- Descriptive link text.
- Inclusive language.
- Readable formatting for assistive technology compatibility.
- Non-visual context alongside visual cues.
- Keyboard and screen reader usability.

## Things to Watch For

### Failure and Confusion Multipliers (a.k.a. "Foot-Guns")

- Hidden prerequisites.
- Version ambiguity.
- Unsafe defaults in examples.
- Happy-path-only procedures.
- Silent destructive steps.
- Documentation drift.
- Copy-paste hazards.
- Reference rot.

## Cross-skill tasks

- If the request requires source code changes, do not proceed under this skill; switch to the Coding skill or split the work into phases.
- If the request is plan-only (for example updating `docs/design/ProjectPlan.md`), use the Planning skill.
- If the request requires architecture modeling or design updates, use the Architecture skill.
- If the request is a formal review of documentation, use the Reviewing skill and record findings in the specified review files.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
