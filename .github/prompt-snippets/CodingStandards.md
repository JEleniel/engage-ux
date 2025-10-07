# Coding Practices and Style

## Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

## Standards

Code must conform to:

- [The Twelve-Factor App](https://12factor.net/).
- [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
- [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
- [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/), if applicable.

## Acceptance Criteria

All code must:

- Compile with zero warnings or errors.
    + Future use code should be appropriately marked to avoid warnings (e.g. prefixed with `_` in Rust).
    + Unused code should be removed.
- Include passing unit tests for all generated functions and code.
    + Include positive and negative cases.
    + Include security tests, e.g. bad input handling.
- Use secure coding practices to prevent common vulnerabilities.
- Never crash. Implement proper error handling and logging.

## Coding Style

- Follow language-specific style guidelines and best practices unless otherwise instructed.
- Use the language appropriate toole (e.g. `rustfmt`, `prettier`, `markdownlint`) to automatially format files.
- Prefer tabs over spaces for indentation when appropriate for the language.
- Write clear, concise, and well-documented code.
- Include comments explaining non-obvious logic.
- Avoid hardcoding information (e.g., API keys, passwords) or configurable values.
- NEVER USE YML or YAML. Use JSON for configuration files. Provide an appropriate [JSON (Draft 07) Schema](https://json-schema.org/draft-07/schema). Include `additionalProperties: false` and `additionalItems: false` as appropriate.

### Version Control Guidelines

- Write clear, descriptive commit messages.
- Each commit should represent a single logical change.
- Keep commits small and focused.
- Branch names should be descriptive and follow project conventions.
- Include relevant issue/ticket numbers in commit messages when applicable.
