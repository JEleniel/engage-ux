# Common Agent Instructions

## Scope

- You may use any MCP servers you have access to.
- Respect the `.gitignore` file; do not read or modify files listed in it.
- Do not modify files in the `.github` or `.prompts` folders.

## Coding Standards

### Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

### Common Requirements

The following requirements apply to all generated source code:

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

### Style and Best Practices

- Follow language-specific style guidelines and best practices unless otherwise instructed.
- Conform to the appropriate style configuration(s), e.g., `rustfmt.toml`, `.prettierrc.json`, `.markdownlint.json`, etc.
- Prefer tabs over spaces for indentation when appropriate for the language.
- Write clear, concise, and well-documented code.
- Include comments explaining non-obvious logic.
- Avoid hardcoding information (e.g., API keys, passwords) or configurable values.
- Ensure that libraries used are actively maintained and widely adopted.

### Version Control Guidelines

- Write clear, descriptive commit messages.
- Each commit should represent a single logical change.
- Keep commits small and focused.
- Branch names should be descriptive and follow project conventions.
- Include relevant issue/ticket numbers in commit messages when applicable.

## Documentation Conventions

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
