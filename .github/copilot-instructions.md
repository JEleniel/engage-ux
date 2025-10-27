```markdown
# Machine Coding Agent Prompt

This document defines the mandatory operational, security, and coding standards for autonomous coding agents that contribute to this repository. It also contains conventions that human contributors should follow where applicable. Read the `Scope` and `Agent-only rules` sections carefully if you are implementing or running an automated agent.

## Purpose and Scope

- Purpose: Ensure code contributed by machines and humans is secure, maintainable, and consistent with this project's conventions.
- Scope: These rules apply first to autonomous machine coding agents created to modify this repository. Where noted, rules also apply to human contributors. Agent-only instructions are explicitly marked.

## Coding Practices and Style

### Priorities

When writing or modifying code, strictly adhere to the following priorities, in order:

1. **Security**
2. **Robustness**
3. **Scalability**
4. **Performance**
5. **Maintainability**

### Standards

Code must comply with the following standards, as applicable:

- [The Twelve-Factor App](https://12factor.net/)
- [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/)
- [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/)
- [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/)

### Formatting and Style Rules

- Respect project-specific formatting configurations when present (for example `rustfmt.toml`, `.editorconfig`, or a Prettier config). If a project config exists, use it and do not override that project's preferences.
- If no project formatter/config is present, follow language-idiomatic defaults.
- Preferences for this repository (applies when no project config contradicts it):
	+ Use tabs for indentation.
	+ Use `rustfmt` for Rust sources.
	+ Use Prettier for *Script languages (JavaScript/TypeScript/JSON/Markdown) where applicable.
	+ Use language default formatters for all other languages.
- Always run the appropriate formatter before committing. Automated checks (CI/pre-commit) should verify formatting.
- Write clear, concise, and well-documented code. Comment all non-obvious logic.

### Secrets and Configuration

- Never hardcode secrets, credentials, or environment-specific configuration values. Use environment variables, a secrets manager, or configuration files excluded from version control.
- If you must expose a configuration value in code, define it as a constant (`const`) with a clear name and load the real value from a secure source at runtime.
- If a secret is accidentally committed, follow the repository's incident remediation steps: rotate the secret, remove it from history (if necessary), and create an issue documenting the incident.

### Structured Configuration

- Prefer JSON (Draft 07) for cross-language machine-readable configuration. Provide a matching JSON Schema (Draft 07) with `additionalProperties: false` and `additionalItems: false` when adding new JSON configuration files intended for programmatic consumption.
- Exceptions: Use the ecosystem's conventional format when required (for example `Cargo.toml` for Cargo, `rustfmt.toml` for rustfmt, or other tool-specific files). Document the reason for using a non-JSON format in the same directory (a short README or comment is sufficient).

### Dependencies

- Ensure dependencies are actively maintained and widely adopted. Prefer stable, well-known crates/packages.

## Project Structure

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

## Secrets and Incident Handling

- Use the platform's secret storage (GitHub secrets, environment variables, or an external secrets manager). Do not store secrets in the repository.
- If a secret is discovered in the repo, rotate it immediately and follow the incident remediation checklist (create an issue, remove from history if necessary, run a secret scan).

## How to propose changes

- Open a small, focused pull request with a descriptive title and link to any related issue.
- Include a short PR description of the change, the reason, and any CI steps required to validate it.
- Add reviewers and request maintainers to run CI and approve.

## Contacts and Escalation

- For policy clarifications or urgent issues, open an issue and tag `@JEleniel` (or the relevant team alias) in the PR.

## Examples and templates

- Commit message example: `docs(contrib): clarify agent formatting rules (#42)`
- Branch example: `feature/ENG-123-add-ci-format-checks`

```
