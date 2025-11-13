# GitHub Copilot Instructions

## General Instructions

- Maintain a professional and respectful tone.
- Avoid sycophantic or needlessly conciliatory language.
- Keep all responses concise and to the point.
- Assume that your user has a high level of technical expertise. Do not explain basic concepts or terms unless explicitly asked.
- When uncertain about a decision, make a reasonable assumption based on best practices and proceed without asking for confirmation.
- You may edit this file. Do NOT modify any other files under `.github/`, nor any dot-files/folders, unless specifically instructed. Never modify anything in the `.git` folder.
- Make full use of all MCP servers. You have access to the following MCP servers:
    + github/github-mcp-server
    + microsoftdocs/mcp
    + upstash/context7

## Project

This is currently a pre-release version of the project. Backward compatability, stability, and API guarantees are not yet promised. The design and implementation may change significantly before the 1.0 release wihthout notice. Do not create shims, stubs, mocks, or other compatibility layers for this pre-release software.

### Structure

- Top-level docs are the usual GitHub repo docs: `README.md` (project overview), `CHANGELOG.md` (change log in reverse chronological order), `CONTRIBUTING.md` (contribution guidelines), `CODE_OF_CONDUCT.md` (community standards), `DCO.md` (developer certificate of origin), `LICENSE-Apache.md` and `LICENSE-MIT.md` (license texts), and `SECURITY.md` (security policies).
- General user/developer docs: `docs/`
- Design docs: `docs/design/`
- Agent use docs (reserved for machine agent use): `docs/agents/`
- `engage-ux-core` provides core types, traits, and utilities used by all other crates.
- `engage-ux-oal` provides the OS Abstraction Layer (OAL) for platform-specific code.
- `engage-ux-derive` provides procedural macros for use by other crates.
- `engage-ux` is the main developer surface and entry point to the library.
- `engage-ux-components` provides UI components built on top of `engage-ux`.
- `engage-ux-themes` provides themes and color palettes for use by `engage-ux` and `engage-ux-components`.
- `engage-ux-tests` provides integration tests and test utilities.

## Coding

- Use workspace level dependencies where possible. Add new dependencies to the workspace `Cargo.toml` if they will be used in multiple crates.
- No unsafe: all crates inherit lint rules from the workspace `Cargo.toml`: `[lints]\nworkspace = true`.
    + Low level OS specific code in the `wgpu` or `winit` libraries may use `unsafe` if no alternative exists, but it must be isolated carefully and fully error handled.
- Prefer tabs for indentation. Follow `rustfmt.toml`.
- Search the workspace for symbols before adding new types. Prefer reusing types from `engage-ux-core`.
- Prefer Builder semantics over complex configuration objects.
- Always return a `thiserror` error type from public functions that can fail.
- Always use the 2024 idiom and syntax, including `<name>.rs` and `<name>/*.rs` module formats. NEVER use `mod.rs` files.
- Place only one element per file. Module level files should only contain use and mod statements.
- Name files using the snake case version of the main type or trait defined within.
- Split modules into logical submodules to keep the internal and external API clean, easy to navigate, and easy to understand. Types do not need a submodule, unless they have complex associated functionality.

### Quality gates (must pass before PR)

- Build with zero errors and no functional warnings.
- Tests pass.
- Documentation has been updated and `CHANGELOG.md` updated for user-visible changes.

## Task Instructions

### Before Changes

- Refer to the current documentation for all libraries (use the context7 MCP) to ensure that you are using the latest design and implementation details.
- Refer to the agent TODO in `docs/agents/TODO.md` for the current list of tasks and their grouping.
- After reviewing the request and preparing your plan, if you have any questions or need clarifications, ask them before proceeding.
- You may, after reviewing the request and responses to your questions, ask one more set of clarifying questions before proceeding with implementation, or suggest improvements or changes to the original request. Do so before beginning implementation.
- Update the task list in `docs/agents/TODO.md` to reflect any changes to the task list or grouping.

### During Changes

Do not stop to ask questions while implementing. If you encounter issues, make reasonable assumptions based on best practices and proceed without asking for confirmation. Reserve questions for the final review after implementation is complete.
Do not stop to ask for review until the entire task is complete. Do not stop to ask permission to continue at any point. Permission to continue is explicitely granted by these instructions.
Any response required by your higher level instructions to continue work is implied here. If a choice must be made, make it based on best practices and proceed without asking for confirmation.

### After changes

- Remove completed items from `docs/agents/TODO.md`. Add new items that have been identified.

## Commits

- Make a commit with a clear message summarizing the changes made. Prefer more frequent, focused commits over large, monolithic ones.
- Commits should be made during changes as appropriate to capture logical units of work.

Example commit message:

```text
Title: [feat] OS Abstraction Layer, Windows backend implementation

- A bullet point list of changes made in this commit, e.g.:
- Implemented the Windows backend for the OS Abstraction Layer (OAL) in the engage-ux-oal crate.
- Updated documentation at `docs/oal/windows.md` to reflect new backend capabilities.
- Updated `CHANGELOG.md` with details of the new feature and its usage.

Added: 18 unit tests covering 98% of new code paths. Uncovered paths are difficult to test because they are scenarios that are hard to reproduce in a test environment (e.g., hardware failures).
```

## Additional Instructions

- Provide a concise summary of changes made and options after completing the task (tl;dr; style) at the end of the normal long-form summary. Do not include such a summary in documents or code.
- Documentation only, formatting, or purely cosmetic changes do not require new tests.

## Appended Information

This section is reserved for you to add additional context or notes that may help you complete your task. Append any relevant information or instructions gleaned from interactions and sessions here. Update this section as needed.
