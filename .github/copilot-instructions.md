# Agent Guidelines

These instructions define the standards for all contributions by agents in this project.

## Engage UX Overview

A fully cross-platform Rust UI toolkit that provides a themable component library without depending on a browser engine. Engage UX uses an OS Abstraction Layer (OAL) for low-level platform interaction, allowing a single set of components to work across Windows, macOS, Linux, Android, and iOS.

A more detailed project overview can be found in the `README.md` file. Documentation is located in the `docs/` directory. If the `docs/` directory does not exist you should create it. Also ensure a `docs/README.md` file with an overview of the documentation structure exists and is kept up to date.

## Coding Standards

### Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

## Common Requirements

The following requirements apply to all generated source code:

- This is a pure Rust project. Do not use any other languages.
- Follow [The Twelve-Factor App](https://12factor.net/).
- UI elements must conform to [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
- Conform to the [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
- Conform to the [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/) if applicable.
- All code must:
    + Compile with zero warnings or errors.
    + Include appropriate unit tests for all generated functions and code.
    + Pass all tests.
    + Be runnable without elevated permissions (e.g., root).
    + Implement appropriate input validation and sanitization.
    + Use secure coding practices to prevent common vulnerabilities.
    + Implement proper error handling and logging.

### Style and Best Practices

- Follow language-specific style guidelines and best practices unless otherwise instructed.
- Prefer tabs over spaces for indentation when appropriate for the language.
- Write clear, concise, and well-documented code.
- Include comments explaining non-obvious logic.
- Avoid hardcoding sensitive information (e.g., API keys, passwords) or configurable values.
- Ensure that libraries used are actively maintained and widely adopted.

### Version Control Guidelines

- Write clear, descriptive commit messages.
- Each commit should represent a single logical change.
- Keep commits small and focused.
- Branch names should be descriptive and follow project conventions.
- Include relevant issue/ticket numbers in commit messages when applicable.

## Documentation Conventions

- Use clear, well-structured GitHub-flavored markdown.
- Match the tone, style and structure of existing documentation.
- Cross-reference related docs where relevant. Include appropriate direct links.
- Cite project details with file and section references.

### Markdown Linting Rules

- Use fenced code blocks with language identifiers; use `text` if none applies.
- Include a blank line before and after headings, code blocks, lists, and tables.
- Use underscores for emphasis, asterisks for strong text.
- Use only ATX-style headings, incrementing by one level at a time.
- Use dashes for horizontal rules.
- Do not break long lines; let them wrap naturally.
- Do not use collapsed/shortcut links or bare URLs; always format as links.
- Use tabs for indentation.
- Do not use emphasis as a heading.
- Consolidate multiple blank lines into one.
- No space between emphasis/strong markers and text.
- No space in links.
- All URLs must be URL escaped.
- Nested unordered lists should use a different marker from their parent.
- Ordered lists: period after the number, start at 1, increment by 1.
- File ends with a single newline.
- Tables: same number of cells per row, with leading/trailing pipes.
- All URLs must be URL escaped.
- Nested unordered lists should use a different marker from their parent list (e.g., `-` for parent, `*` for child).
- Ordered lists should use a period after the number (e.g., `1.`), start at 1, and increment by 1.
- The file should end with a single newline character.
- Tables must have the same number of cells in each row, including the header row. They should use leading and trailing pipes.
