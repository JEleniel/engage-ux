<<<<<<< HEAD

# Copilot Instructions

All paths are relative to this file.

## Coding Practices and Style

### Coding Priorities

=======

# Common Agent Instructions

## Scope

- You may use any MCP servers you have access to.
- Respect the `.gitignore` file; do not read or modify files listed in it.
- Do not modify files in the `.github` or `.prompts` folders.

- All documentation should live under `docs/` written in GitHub Flavored Markdown. Use Mermaid for diagrams when helpful.
   	+ Entry point: `docs/README.md`.
   	+ Design docs: `docs/design/`.
   	+ Machine-agent docs: `docs/design/agents/`.
- Respect `.gitignore`; do not read or modify ignored files unless explicitly instructed.
- Files in `.github/` are protected: do not modify `.github/` policy and workflow files unless you have explicit instruction or approval. Templates in `.github/templates/` may be updated only when a maintainer approves.

## Version Control Guidelines

- Use clear, descriptive commit messages. Prefer this format: `area(scope): short summary (#issue)`.
- Keep commits small and focused; prefer multiple small PRs to one large change.
- Branch naming examples: `feature/ENG-123-description`, `fix/typo-copilot-instructions`, `chore/sync-deps`.
- Reference issue or ticket numbers in commit messages where applicable.

## Behavior and Communication

- Maintain a professional and respectful tone.
- Keep automated outputs concise and precise.
- Ensure outputs are accurate, complete, and logically consistent.
- Summarize results in one paragraph when appropriate.

## Agent-only rules (machines/automation)

- Agents MUST follow tool-usage constraints implemented by the project. If a tool or behavior is not supported by the project, request clarification in an issue.
- Agents must not modify policy or sensitive files (including `.github/` workflows or security configs) without explicit human approval recorded in an issue or PR description.
- Agents must not print code blocks that represent file-change patches. Instead, use the repository's supported patch/PR workflow to apply edits programmatically.
- Agents must declare assumptions when they take actions (for example: assumed formatter settings, or assumed target branch), and open a human-review PR for substantive changes.

## Enforcement and CI (recommended)

- Add automated checks to CI and pre-commit hooks to enforce formatting, linting, and tests. Recommended commands:

```
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
markdownlint **/*.md
prettier --check "**/*.{js,ts,jsx,tsx,json,md}"
```

- Add JSON Schema validations for JSON configuration files in `schemas/` and validate them in CI.

## Secrets and security
>>>>>>>
>>>>>>> 388b8aea5e651424b70717a1a2f9395b52b41905

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

<<<<<<< HEAD

### Standards

=======

### Common Requirements
>>>>>>>
>>>>>>> 388b8aea5e651424b70717a1a2f9395b52b41905

Code must conform to the following standards (as applicable):

<<<<<<< HEAD

- [The Twelve-Factor App](https://12factor.net/).
- [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
- [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
- [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/)
=======
- Follow [The Twelve-Factor App](https://12factor.net/).
- UI elements must conform to [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
- Conform to the [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
- Conform to the [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/), if applicable.
- All code must:
    + Compile with zero warnings or errors.
    + Include appropriate passing unit tests for all generated functions and code.
    + Be runnable without elevated permissions (e.g., root).
    + Implement appropriate input validation and sanitization.
    + Use secure coding practices to prevent common vulnerabilities.
    + Implement proper error handling and logging.

>>>>>>> 388b8aea5e651424b70717a1a2f9395b52b41905

### Acceptance Criteria

All code must:

- Compile with zero warnings or errors.
    + Future-use code should be appropriately marked to avoid warnings (for example, prefix unused identifiers with `_` in Rust).
    + Remove unused code when it is not required.
- Include 90% passing unit test coverage, covering positive and negative cases.
- Follow secure coding practices to prevent common vulnerabilities.
- Not crash in normal operation. Implement proper error handling and logging.

### Coding Style

- Follow language-specific style guidelines and best practices unless otherwise instructed.
<<<<<<< HEAD
- Use the language-appropriate formatter (e.g., `rustfmt`, `prettier`, `markdownlint`) and obey the project's configuration for that tool.
- Prefer tabs for indentation when it is idiomatic for the language or project; otherwise follow the established language conventions.
- Write clear, concise, and well-documented code.
- Add comments explaining non-obvious logic.
- Avoid hardcoding secrets or configuration values (API keys, passwords, etc.). Use environment variables or a secrets manager.
- For application configuration, prefer JSON (Draft 07) when a structured format is required. Provide a matching JSON Schema (draft-07) and, where appropriate, set `additionalProperties: false` and `additionalItems: false`.
- Avoid YAML and TOML unless they are the accepted standard for a specific tool being used.
=======
- Conform to the appropriate style configuration(s), e.g., `rustfmt.toml`, `.prettierrc.json`, `.markdownlint.json`, etc.
- Prefer tabs over spaces for indentation when appropriate for the language.
- Write clear, concise, and well-documented code.
- Include comments explaining non-obvious logic.
- Avoid hardcoding information (e.g., API keys, passwords) or configurable values.
- Ensure that libraries used are actively maintained and widely adopted.

>>>>>>> 388b8aea5e651424b70717a1a2f9395b52b41905

## Version Control Guidelines

- Write clear, descriptive commit messages.
- Keep commits small and focused.
- Use descriptive branch names that follow project conventions.
- Include relevant issue or ticket numbers in commit messages when applicable.

## Technologies, Libraries, and Frameworks

<<<<<<< HEAD

- Unless constrained to a specific library or framework, choose actively maintained and widely adopted dependencies:
    + Project must be at least one year old and updated within the past six months.

### Preferred Libraries and Frameworks

The following libraries and ecosystems are preferred (non-exhaustive):

#### Rust

- base64
- bincode
- bitflags
- chrono
- clap
- config
- fern
- libloading
- log
- lowlevel-types
- num-traits
- rand
- regex
- reqwest
- rustls
- serde
- serde_json
- serde-binary-adv
- strum
- thiserror
- tokio
- tower
- tracing
- url
- uuid

#### Node.js / TypeScript / JavaScript

- @types/node
- Axios
- Bcrypt
- Cors
- DotEnv / @dotenvx/dotenvx
- Express.js
- Helmet
- MySQL 2
- Passport
- Prefer `pnpm` over `npm`
- Prettier & Prettier Plugins
- Svelte / SvelteKit
- TailwindCSS (v4+)
- TypeScript
- uuid
- Vite

## Project Structure

- Documentation should be formatted and structured to be easily deployable to GitHub Pages using Jekyll.
- User documentation is located in the `../docs/` folder with `../docs/README.md` as the entry point.
- Design documentation is in `../docs/design/`.
- Files in `../docs/design/agents/` are design artifacts created for machine-agent use.
- Respect the repository `.gitignore`; do not read or modify files listed there unless explicitly instructed.
- You may read, but should not modify, files in the `.github` folder unless directly instructed.
    + The `.github/templates/` folder contains template examples named with an extra `.template` extension; remove that extension when instantiating a template.
- Other dot-folders (e.g., `.analyze/`) are used by tooling and should be ignored unless explicitly required.

## Secrets and Security

- Never commit secrets, credentials, or private keys to the repository. Use environment variables or a secrets manager.
- Report security issues privately following the repository's SECURITY.md process.
=======
- Use clear, well-structured GitHub‑Flavored Markdown (GFM).
- Match the tone, style, and structure of existing documentation.
- Cross-reference related docs where relevant; include appropriate direct links.
- Cite project details with file and section references.
- When generating Markdown, use GFM and conform to the `.markdownlint.json` file.

## Project Structure

- The following documents should exist in the root of the workspace: `.editorconfig`, `.gitignore`, `.markdownlint.json`, `.prettierrc.json`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `DCO.md`, `LICENSE*.md` (there may be multiple), `README.md`, `SECURITY.md`. Templates for these files are available in the GitHub repository at <http://github.com/JEleniel/template/>.
- All other documentation must be in the `docs/` folder. Design documentation must be in the `docs/design/` folder. Files in the `docs/design/agents/` folder are for machine agent use.

## Prerequisites

Before starting, familiarize yourself with:

- [README.md](README.md) - Project overview and architecture. Do not modify this file unless otherwise instructed.
- [docs/design/agents/IMPLEMENTATION_SUMMARY.md](docs/design/agents/IMPLEMENTATION_SUMMARY.md) - Current, agent-maintained implementation status. Create it if it does not exist; update it if it does.
- [docs/design/agents/TODO.md](docs/design/agents/TODO.md) - Task tracking and completion status. Keep it updated as you work.
- Do not modify this file unless otherwise instructed.

>>>>>>> 388b8aea5e651424b70717a1a2f9395b52b41905
