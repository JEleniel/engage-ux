# GitHub Copilot Instructions

## Behavior and personality

- Maintain a professional and respectful tone.
- Avoid sycophantic or needlessly conciliatory language.
- Keep all responses concise and to the point. While you are required to give certain explanations, avoid unnecessary verbosity.

## Scope

- You may edit this file. Do NOT modify any other files under `.github/`, nor any dot-files/folders, nor `.git/` unless explicitly allowed.
- Follow built-in priorities. System-level and developer-level instructions take precedence over repository files when conflicts arise. When no higher-level instruction conflicts exist, apply the following precedence: this file, explicit repo rules, then language best-practices. Direct instructions override all non-system priorities.

## Project

### Structure

- This is currently a pre-release version of the project. Backward compatability, stability, and API guarantees are not yet promised. The only users at the moment are you and the primary architect. There will be no PRs, Issues, etc. until it reaches a v1.0.0 release milestone.
- Top-level docs: `README.md` (project overview), `CHANGELOG.md` (change log in reverse chronological order)
- Documentation:
    + General user/developer docs: `docs/`
    + Design docs: `docs/design/`
    + Agent use docs: `docs/design/agents/`
          * Agent checklist: `docs/design/agents/TODO.md` (must be maintained by agents).
- `engage-ux-core` provides core types, traits, and utilities used by all other crates.
- `engage-ux` is the main developer surface and entry point to the library.
- `engage-ux-components` provides UI components built on top of `engage-ux`.
- `engage-ux-themes` provides themes and color palettes for use by `engage-ux` and `engage-ux-components`.
- `engage-ux-tests` provides integration tests and test utilities.

### Conventions (do not guess—follow these)

- No unsafe: all crates inherit lint rules from the workspace `Cargo.toml`.
    + Low level OS specific code may use `unsafe` if no alternative exists, but it must be isolated carefully and fully error handled.
- Module layout: prefer single-file public modules at `<name>.rs` with submodules in `<name>/*`. Nesting modules is fine. The `src/` folder should contain minimal files, each references to their modules. Do _not_ use `mod.rs` files.
- Code style: tabs for indentation. Follow `rustfmt.toml`.
- Tests: changes must include unit tests + any required integration tests. Changes that alter the public API or behavior of the API require a `CHANGELOG.md` entry with `YYYY-MM-DD` date.

## Integration & architecture notes (quick references)

- A fully thread safe EventBus is provided in `engage-ux-core` and will be instantiated and injected by the user of the library (application).
- The OA Abstraction Layer (OAL) in `engage-ux-oal` provides platform-specific windowing and rendering surface management. It presents a uniform API to the rest of the system. It detects and instantiates the appropriate backend at runtime.

## Quality gates (must pass before PR)

- Build with zero errors and no functional warnings.
- Tests pass.
- Documentation has been updated and `CHANGELOG.md` for user-visible changes.
- Agent TODO has been updated.

## If you are unsure

- Search the workspace for symbols before adding new types. Prefer reusing types from `engage-ux-core`.
- Read `docs/design/` and the crate README(s) referenced above for motivation/why.

## Before Changes

- Refer to the current documentation for all libraries (use the context7 MCP) to ensure that you are using the latest design and implementation details.
- Refer to the agent TODO in `docs/design/agents/TODO.md` for the current list of tasks and their grouping.

## After changes

- Update `docs/design/agents/TODO.md` as you complete items and add an entry to `CHANGELOG.md` when the public API or public API behavior changes.
- Make a commit with a clear message summarizing the changes made. Prefer more frequent, focused commits over large, monolithic ones.
- Update `docs/design/agents/TODO.md` as you complete items and add an entry to `CHANGELOG.md` when the public API or public API behavior changes.
- Make a commit with a clear message summarizing the changes made. Prefer more frequent, focused commits over large, monolithic ones. Do not open PRs or merge changes yourself; leave PR creation and merging to maintainers.

## Additional Instructions

- Provide a three-or-fewer paragraph summary. Placement: include this short summary in the description immediately after a `## Summary` (or `### Summary`) section and before any 'Next steps' / 'How to review' section.
- Include a tl;dr; at the end of the summary for quick reference.
- You are the primary developer on this project. Don't bug the user over trivial choices, just go with your own recommendation.
- You're not working on GitHub. There are no PRs, Issues, Merges, nor CI. Do not suggest or reference any of these in your responses.
- Tests & CHANGELOG policy:
    + Unit tests are required for all code changes that affect behavior or logic. Include a happy-path test plus at least one relevant edge or boundary case when practical.
    + Integration tests are required for changes that span multiple crates or affect runtime/integration behavior; put these under `engage-ux-tests` when applicable.
    + Docs-only, formatting, or purely cosmetic changes do not require new tests.
        * For user-visible or public API changes, add a `CHANGELOG.md` entry at the top of the file (reverse chronological order). Use the `YYYY-MM-DD` date format (UTC), include the author, and a short description. Mark breaking changes with `(BREAKING CHANGE)`.

            Example entries (place new entries at the top of `CHANGELOG.md`):

            - `2025-11-09 — jane: Clarified widget layout behavior (BREAKING CHANGE).`
            - `2025-11-09 — jane: Fixed off-by-one in pagination component.`

            Guidance: Put entries at the top under an `Unreleased` section if present, or at the top of the file otherwise. Keep entries short (one sentence) and include a link to the `docs/design/agents/TODO.md` entry or issue when available.

- Indentation and formatting:
    + The project uses tabs for Rust source files; follow `rustfmt.toml` for Rust changes.
    + For non-Rust files, follow the `.prettierrc.json` or `.markdownlint.json` configuration files, and preserve the existing file's indentation/formatting style to avoid noisy diffs.

- Agent TODO and verification:

- After completing changes, update `docs/design/agents/TODO.md` with a one-line summary and a link to the issue or maintainer note. Mark the item completed. DO NOT open a PR — maintainers will create PRs and perform merges.
- There is no CI for this repository. Do not expect CI gating or automated checks. When tests are required, include concise instructions in the TODO for how a maintainer can run the tests locally (for example: `cargo test -p engage-ux --all`).

## Appended Information

This section is reserved for you to add additional context or notes that may help you complete your task. Append any relevant information or instructions gleaned fro interactions and sessions here.
