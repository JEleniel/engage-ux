
# GitHub Copilot Instructions

## Scope

- Unless specifically instructed, do not modify the following files or files inside the following folders:
    + `.git/`
    + `.github/`
    + Any dot (`.`) folder
    + Any dot (`.`) file
    + Any configuration file, with the exception of managing dependencies.
- You may use any MCP servers you have access to.

## Behavior and personality

- Maintain a professional and respectful tone.
- Avoid sycophantic or needlessly conciliatory language.

### Instruction priority

When two instructions conflict, follow them in this order (top = highest):

1. Explicit, written instructions.
2. Repository configuration files and `.github/` policies (when edits are explicitly permitted).
3. This file's guidance.
4. Language or ecosystem best practices.

## Coding standards

- Follow language-specific style guidelines and best practices unless otherwise instructed.
- Conform to applicable standards:
    + [The Twelve-Factor App](https://12factor.net/)
    + [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/)
- Follow the repository's established formatter and configuration (for example, `rustfmt.toml` for Rust). If the repository has no formatter/settings, use the language's well-accepted default. Prefer automatic formatting tools over manual whitespace edits.
- Formatting and linting are provided by the IDE or project tooling (e.g., Prettier, rustfmt). Read and follow the project's configuration files and CI checks.
- Follow secure coding practices to prevent common vulnerabilities.
- Implement proper error handling and logging.
- Avoid hardcoding secrets or configurable values (API keys, passwords).
- Ensure libraries used are actively maintained (updated within the past six months) and are widely adopted.

## Documentation standards

- Use clear, well-structured GitHub-Flavored Markdown (GFM).
- Match the tone, style, and structure of existing docs.
- Cross-reference related docs where relevant and include direct links.
- Cite project details with file and section references.
- When generating Markdown, conform to the project's `.markdownlint.json` rules.

## About the project

Engage UX is a cross-platform Rust UI toolkit providing a themable component library without a browser engine. It uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing components to work across Windows, macOS, Linux, Android, and iOS.

### Project structure

- Documentation should live in `docs/` unless it is a standard top-level GitHub documentation file (for example `README.md`, `CONTRIBUTING.md`, `LICENSE*`, `CODE_OF_CONDUCT.md`).
    + Design docs go in `docs/design/`.
    + The `docs/design/agents/` folder is reserved for machine-agent use.
- The `engage-ux-oal` crate contains OS-specific code.
- The `engage-ux-core` crate contains foundational types and logic and depends on the OAL.
- The `engage-ux-themes` crate contains optional themes.
- The `engage-ux-components` crate contains optional UI components built on Engage UX.

## Expected outcome for changes

- Code must compile with zero errors. Warnings unrelated to functional correctness (for example unused imports) are acceptable; avoid warnings that indicate functional risk.
- Include unit tests that cover the new behavior (posiive and negative cases).
- Code should run without elevated privileges on typical developer machines.
- Update relevant documentation to reflect design or usage changes.
- Update `CHANGELOG.md` for user-facing or public API changes. If it doesn't exist, create it with a short entry and date (YYYY-MM-DD) using the project's format when present.

## Important Addiditional Instructions

- Keep responses complete and concise. Avoid including any information that is not required by your higherlevel instructions and is not relevant to the current context.
- When summarizing information, focus on the most critical points and avoid unnecessary details.
- Ensure outputs are accurate and logically consistent.
- Always provide a final summary in five paragraphs or less.

## Learned Style Preferences

The following are stylistic and idiomatic preferences inferred from work with the repo maintainer; append these to the guidance so future edits follow the same conventions:

- Module layout: prefer the modern layout (top-level `src/color.rs` + `src/color/*` or `src/modules/colors/*`) rather than legacy `mod.rs` files. Keep modules predictable and avoid duplicate module files.
- Palettes as namespaces: use unit structs as namespaces for palettes (for example `pub struct Ansi; impl Ansi { pub const RED: Color = ... }`). This keeps constants discoverable (e.g. `engage_ux::color::Ansi::RED`).
- Public API stability: expose consumer-friendly re-exports at the crate for developer ease but avoid shims or stubs unless the maintainers explicitly ask for them. This is a prerelease crate so backward compatibility is not a primary concern.
- Documentation: add rustdoc comments to public constants and types. Keep naming neutral and descriptive (avoid brand names or trademarked phrases; explain inspirations without explicit references when necessary).
- Patch discipline: make small, focused patches; run tests immediately after changes; avoid mass reformatting of unrelated files. Use `rustfmt`/project formatter for style consistency, and preserve existing public APIs unless intentionally changing them.
- Communication & verification: commit messages and patch explanations should be concise and include what was changed and how it was verified (tests/build). Prefer a short summary in PRs and a quick test result.
