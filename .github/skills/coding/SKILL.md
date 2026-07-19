---
name: coding
description: Use this skill when writing or modifying source code, or when documenting tests in a `docs/design/Tests.md` file.
---

# Coding Skill

## General guidelines

- If a `docs/design/aurora/` folder exists, read and follow [Aurora Compact Model](../../aurora/Aurora.compact.instructions.md) to understand the design.
    - The Aurora Compact Model exists to save time and tokens by keeping key information in a single compact (thus the name) file.
    - Use the full Aurora instructions only when applying the Architecture skill.
- Security-First Coding Mindset: Evaluate realistic misuse and abuse paths, especially at trust boundaries.
- Failure Handling Focus: Handle failures intentionally; return or log errors cleanly.
- Accessibility for User-Facing Behavior: For user-facing surfaces, require WCAG AA at minimum (AAA preferred where feasible).
- The file-specific rules in `../../instructions/*.instructions.md` take precedence over these instructions.
- Follow best practices for the language being edited. Language-specific configs (for example `rustfmt.toml`, `.markdownlint-cli2.jsonc`, `.prettierrc.json`) are authoritative for formatting and linting details, and file-specific instructions may call out relevant config files or exceptions.
- Use the shortest acceptable path for local files.
- Prefer mature, well-supported dependencies with GPL, MIT, or Apache-2.0 licenses.
    - Well-maintained heuristic (use judgment; not a checklist):
        - Active and responsive maintenance (issues/PRs triaged; CI is healthy).
        - Clear compatibility story (language version, runtime, platform, and feature support) that matches the workspace.
        - Security posture is solid (no known unfixed advisories; timely fixes when issues occur).
        - Adoption is meaningful (downstream usage and/or strong community reputation).
        - Documentation quality is good (README, examples, and changelog/release notes).
        - Exception: clearly stable or feature-complete dependencies MAY be acceptable with explicit justification.
- Dependency management:
    - Prefer the latest stable dependency versions, unless constrained by compatibility, platform support, or security response.
    - Add dependencies at the narrowest practical scope supported by the ecosystem unless multiple packages or components truly share them.
    - Avoid new dependencies when the standard library or existing dependencies already solve the problem.

## Errors and logging

- Prefer typed errors within libraries/modules.
- Add context at application boundaries.
- Language-specific instructions define ecosystem-specific error crates, boundary conventions, and other implementation details.
- Avoid unchecked failures (`unwrap`, `expect`, panics) unless justified by an explicit invariant.
- You MUST NOT disable checks/tests (for example `// @ts-nocheck`, `#[allow(...)]`). Fix the underlying issue instead.
- Include helpful TRACE and DEBUG logging where appropriate for troubleshooting.
- Log at boundaries with appropriate severity.
- Never log secrets at any level.

## Principles of Elegant Code

- Clarity: Code is immediately understandable without external explanation.
- Simplicity: Solve the problem with the smallest complete mechanism.
- Conceptual Compression: Prefer better modeling over repetitive mechanics or terse cleverness.
- Natural Mapping to the Domain: Let the structure mirror the problem space, not the implementation workaround.
- Minimal Incidental Complexity: Every element should carry semantic weight.
- Composability: Parts should combine orthogonally without special-case glue.
- Predictability: Behavior should follow established patterns and avoid surprises.
- Symmetry: Operations, data shapes, and error handling should follow balanced, reusable forms.
- Effortless Extendibility: New features should fit existing structures rather than require new mechanisms.

### Indications of Poor Code or Modeling

- God Objects or God Modules.
- Shotgun Surgery.
- Deeply Nested Logic.
- Primitive Obsession.
- Leaky Abstractions.
- Overgeneralization.
- Hidden Complexity in Convenience APIs.
- Heisencode.
- Silent Data Transformation.

## Things to Watch For

### Failure and Risk Multipliers (a.k.a. "Foot-Guns")

- Temporal Coupling.
- Boolean Parameter Explosion.
- Duplicated Logic.
- Magic Values and Hidden Rules.
- Constant Explosion.
- Implicit Defaults.
- Mutable Shared State.
- Stringly-Typed Interfaces.
- Unchecked Error Paths.
- Configuration as Code Without Validation.
- Global Initialization Side Effects.

## Deliverables

- Source code is modular, clean, readable, idiomatic, and aligned with project conventions.
- Appropriate unit and integration tests are added and passing.
    - Coverage target (aspirational): aim for 90%+ coverage on functional code when practical.
- No warnings (e.g., unused variables) are left behind from your work.
- Notes added for the documentation writer explaining changes, new features, and other relevant information for the project documentation.
- Linting, formatting, and static analysis checks are passing.
- You are responsible for the creation and maintenance of the following document that are directly related to the code you are writing:
    - A `docs/design/analysis/Tests.md` file that lists the modules and their associated tests, including a brief description of what each test covers. This file should also identify and gaps in coverage, whether they are deliberate, and why.
    - If applicable a `docs/design/Testing.md` file that documents any manual, interactive, or other tests that do not fit into the automated unit and integration test suites. This file should include instructions on how to run the tests, what they cover, and any expected behavior or outcomes tailored for humans.

**Tests.md Example**:

```markdown
# Test Inventory

## `tests/main_test.rs` tests `main`

- `restart_device_falls_back_cleanly_on_host`: verifies that the host-side restart fallback mechanism works correctly when triggered from the binary entrypoint.

## `tests/configuration_test.rs` tests `data::configuration`

- `parses_example_configuration`: ensures that the example configuration file is parsed correctly, with all expected fields populated and defaults applied as needed.
- `configuration_serialization_preserves_unmodeled_sections`: checks that when a configuration written, any sections that are not explicitly modeled in the code are preserved without alteration.

### Gaps

- `data::configuration` module currently lacks tests for invalid input handling, such as malformed configuration files or missing required fields. This is not deliberate.
```

## Operating Procedure

1. Confirm the task scope, requirements, constraints, and success criteria from the user request and any available design artifacts. If critical information is missing, stop and get clarification before editing code.
2. Inspect the relevant code, interfaces, tests, and configuration before making changes. Align the implementation with any requirements or design artifacts in `docs/design/` and any Aurora model present.
3. Implement the intended change that satisfies the request, following existing patterns and avoiding new dependencies unless clearly justified.
4. Add or update tests for the happy path, edge cases, and failure paths needed to prove the behavior. Language-specific instructions define test organization and placement rules.
5. Run the relevant formatting, linting, static analysis, and test commands for the affected language or ecosystem, and fix any issues introduced by the change. Repo instructions require verification before completion; file-specific instructions define language- and file-specific checks.
6. Update any documentation you are responsible for.
7. If the request also requires architecture, planning, documentation-only work, or formal review output, switch to the appropriate skill for that phase.

- Note: If present, you can run `./.github/violations.sh` to check for oversize files or functions as well as small functions that may be candidates for cleanup.

## Validation Checklists

### Secure Code Checklist

- Untrusted input handled at every trust boundary with schemas, type checks, length limits, sanitization, and allow-lists.
- Untrusted output contextually encoded before rendering or execution, with integrity verified by signatures, hashes, or checksums where required.
- Authenticated, authorized, and auditable endpoints and APIs, with audit trails redacted of Non-Public Information (NPI).
- Reuse of vetted implementations for security- and protocol-sensitive functionality.
- Parameterized queries and safe APIs for SQL, search, shell, templates, and other external execution.
- End-to-end data protection, including access controls, encryption at rest and in transit, minimized exposure in memory, and OS, HSM, or keychain-managed secrets where practical.
- Secure defaults with no production debugging interfaces or verbose diagnostics unless explicitly enabled.
- Sanitized warning and error messages free of Non-Public Information (NPI).
- Continuously maintained dependencies with pinned inputs and resolved versions, lockfiles, explicit reviewed changes, and mitigated advisories.
- Structured, tamper-resistant logging of security-relevant events.
- Explicit trust boundaries enforced in code.
- Minimal public surface area.

### Error and Exception Handling Checklist

- Rigorous tests that prove behavior; null tests are failures.
- Coverage for critical edge cases and failure paths, not just happy paths.

### General Code Quality Checklist

- Source files no longer than 500 lines.
- Functions no longer than 50 lines.
- Intentional failure handling, with errors logged or returned cleanly.
- WCAG AA minimum for user-facing behavior (AAA preferred where feasible).
- Meaningful tests that validate outcomes and invariants.
- Locality of reasoning and a single source of truth for each capability.
- Single responsibility for each unit.
- Correctness verified against requirements, tests, invariants, and edge cases, with code aligned to `docs/design/` artifacts and any Aurora model present.
- Consistent naming, structure, and patterns across the codebase.
- Explicit assumptions, types, and effects.
- Low coupling through minimal, stable interfaces.
- Deterministic testability without elaborate setup.
- Deterministic behavior unless nondeterminism is explicitly modeled and documented.
- Constraint-driven design that uses limitations to create focus rather than workarounds.

## Cross-skill tasks

- When coding you are also responsible for keeping the documentation and tests up to date. If the request involves documentation updates, implement those as part of the coding task instead of switching to the Documentation skill.
- If the request is planning-only, do not produce code changes; use the Planning skill.
- If the request is documentation-only, do not modify code; use the Documentation skill.
- If the request includes both code and documentation updates, treat it as a coding task and include the documentation updates as part of the deliverables.
- If the request is to review existing changes, use the Reviewing skill to record findings; implement fixes only when explicitly asked.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
