
# GitHub Copilot Instructions

## Scope

- Unless specifically instructed, do not modify the following files or files inside the following folders:
    + `.git/`
    + `.github/` (except this file: `.github/copilot-instructions.md`)
    + Any dot (`.`) folder
    + Any dot (`.`) file
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

- Check the current documentation (using MCP) to ensure you are writing current, secure, and idiomatic code.
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
- Prefer one major element per file, and files named with the snake case version of the name of that element (for example, `oal_error.rs`, `surface.rs`, `skia_context.rs`, `window.rs`). Small enums or traits that only support a single type may be grouped together in a single file (for example, `input_event.rs` may contain both the `InputEvent` enum and the `InputEventKind` enum it uses). If in doubt, split it out.
- Organize the crate into developer friendly, logical modules (for example, `accessibility`, `clipboard`, `input`, `rendering`, `windowing`), with each module containing related traits, types, and errors. Expose them so that there is no redundancy in the paths. For example, `engage_ux_oal::windowing::Window` rather than `engage_ux_oal::window::Window`.
- Before creating a new type or trait check whether it can be exposed from a module further up the hierarchy (for example, `engage_ux_core::geometry::Rectangle` rather than defining a new `oal::geometry::Rectangle`).
- Use full words when naming types and traits; avoid abbreviations (for example, `Surface`, `AccessibilityBridge`, `ClipboardManager` rather than `Surf`, `A11yBridge`, `ClipMgr`).

## Documentation standards

- Use clear, well-structured GitHub-Flavored Markdown (GFM).
- Match the tone, style, and structure of existing docs.
- Cross-reference related docs where relevant and include direct links.
- Cite project details with file and section references.
- When generating Markdown, conform to the project's `.markdownlint.json` rules.

## About the project

Engage UX is a cross-platform Rust UI toolkit providing a themable component library without a browser engine or the `winit` library. It uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing components to work across Windows, macOS, Linux, Android, and iOS.

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
- Always provide a final summary in three paragraphs or less. This may be done after the normal summary and before the prompt for next actions, and thus does not conflict with your normal summary behavior.
- The following "Learned Style Preferences" section contains stylistic and idiomatic preferences inferred from work with the repo maintainer; append these as needed to the guidance so future edits follow the same conventions.

## Learned Style Preferences

The following are stylistic and idiomatic preferences inferred from work with the repo maintainer; append these to the guidance so future edits follow the same conventions:

- Module layout: prefer the modern layout (top-level `src/color.rs` + `src/color/*` or `src/modules/colors/*`) rather than legacy `mod.rs` files. Keep modules predictable and avoid duplicate module files.
- Palettes as namespaces: use unit structs as namespaces for palettes (for example `pub struct Ansi; impl Ansi { pub const RED: Color = ... }`). This keeps constants discoverable (e.g. `engage_ux::color::Ansi::RED`).
- Public API stability: expose consumer-friendly re-exports at the crate for developer ease but avoid shims or stubs unless the maintainers explicitly ask for them. This is a prerelease crate so backward compatibility is not a primary concern.
- Documentation: add rustdoc comments to public constants and types. Keep naming neutral and descriptive (avoid brand names or trademarked phrases; explain inspirations without explicit references when necessary).
- Patch discipline: make small, focused patches; run tests immediately after changes; avoid mass reformatting of unrelated files. Use `rustfmt`/project formatter for style consistency, and preserve existing public APIs unless intentionally changing them.
- Communication & verification: commit messages and patch explanations should be concise and include what was changed and how it was verified (tests/build). Prefer a short summary in PRs and a quick test result.
