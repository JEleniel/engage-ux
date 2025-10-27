# Machine Coding Agent Prompt

This document defines the mandatory operational and coding standards for all autonomous coding agents contributing to this repository.

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

### Coding Style

- Follow idiomatic language style guides and best practices.
- Use the appropriate formatter (`rustfmt`, `prettier`, `markdownlint`, etc.) and respect the project’s configuration.
- Prefer **tabs** for indentation when idiomatic; otherwise follow language conventions.
- Write **clear, concise, and well-documented** code.
- Comment all non-obvious logic.
- Never hardcode secrets, credentials, or configuration values — use environment variables or a secrets manager.
- Use **JSON (Draft 07)** for structured configuration. Provide a matching JSON Schema (Draft 07) with `additionalProperties: false` and `additionalItems: false`.
- Avoid YAML and TOML unless required by a third-party tool.
- Ensure all dependencies are **actively maintained** and **widely adopted**.

## Project Structure

- All documentation must reside in the `docs/` directory, written in **GitHub Flavored Markdown** with **Mermaid** for diagrams.
    + The entry point is `docs/README.md`.
    + Design documentation is stored in `docs/design/`.
    + Machine agent documentation is stored in `docs/design/agents/`.
    + Documentation must be **ready for GitHub Pages and Jekyll**.
- Respect `.gitignore`; do **not** read or modify ignored files unless specifically instructed to do so.
- You may read but not modify files in `.github/` unless instructed.
    + Templates are stored in `.github/templates/` with the `.template` extension.
    + All other subfolders in `.github/` must be ignored.
- Ignore all other dot folders (e.g., `.analyze`).

## Version Control Guidelines

- Use **clear, descriptive commit messages**.
- Keep commits **small and focused**.
- Use **descriptive branch names** following project conventions.
- Reference **issue or ticket numbers** in commit messages when applicable.

## Behavior Guidelines

1. Maintain a professional, respectful tone at all times.
2. Keep responses concise and precise.
3. Ensure outputs are **accurate, complete, and logically consistent**.
4. Summarize final results in **one paragraph or less**.

## Inviolable Constraints

Do NOT violate the following constraints under any circumstances unless _directly_ instructed to do so:

- Do NOT create a Pull Request.
- Do NOT stash files.
- Do NOT make _any_ changes on GitHub.
- Do NOT lose _**any**_ of the user's work or changes.
- Do NOT commit any changes that _you_ did not make.
