
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
- Keep responses complete and concise.
- Ensure outputs are accurate and logically consistent.
- Keep summaries to one paragraph when possible.
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
