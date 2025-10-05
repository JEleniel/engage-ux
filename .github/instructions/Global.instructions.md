# General Coding Practices and Style Guide

## Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

## Common Requirements

The following requirements apply to all generated source code:

-   Follow [The Twelve-Factor App](https://12factor.net/).
-   UI elements must conform to [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
-   Conform to the [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
-   Conform to the [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/), if applicable.
-   All code must:
    -   Compile with zero warnings or errors.
    -   Include appropriate passing unit tests for all generated functions and code.
    -   Be runnable without elevated permissions (e.g., root).
    -   Implement appropriate input validation and sanitization.
    -   Use secure coding practices to prevent common vulnerabilities.
    -   Implement proper error handling and logging.

## Coding Style

-   Follow language-specific style guidelines and best practices unless otherwise instructed.
-   Use the language appropriate toole (e.g. `rustfmt`, `prettier`) to automatially format files.
-   Prefer tabs over spaces for indentation when appropriate for the language.
-   Write clear, concise, and well-documented code.
-   Include comments explaining non-obvious logic.
-   Avoid hardcoding information (e.g., API keys, passwords) or configurable values.
-   Unless instructed to use a specific library, ensure that libraries used are actively maintained and widely adopted.
