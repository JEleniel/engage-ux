# Copilot Instructions

All paths are relative to this file.

## Coding Practices and Style

### Coding Priorities

1. Security
2. Robustness
3. Scalability
4. Performance
5. Maintainability

### Standards

Code must conform to the following standards (as applicable):

- [The Twelve-Factor App](https://12factor.net/).
- [Web Content Accessibility Guidelines (WCAG) 2.2 AAA](https://www.w3.org/WAI/standards-guidelines/wcag/docs/).
- [OWASP Application Security Verification Standard (ASVS)](https://owasp.org/www-project-application-security-verification-standard/), if applicable.
- [OWASP Mobile Application Security Verification Standard (MASVS)](https://mas.owasp.org/MASVS/)

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
- Use the language-appropriate formatter (e.g., `rustfmt`, `prettier`, `markdownlint`) and obey the project's configuration for that tool.
- Prefer tabs for indentation when it is idiomatic for the language or project; otherwise follow the established language conventions.
- Write clear, concise, and well-documented code.
- Add comments explaining non-obvious logic.
- Avoid hardcoding secrets or configuration values (API keys, passwords, etc.). Use environment variables or a secrets manager.
- For application configuration, prefer JSON (Draft 07) when a structured format is required. Provide a matching JSON Schema (draft-07) and, where appropriate, set `additionalProperties: false` and `additionalItems: false`.
- Avoid YAML and TOML unless they are the accepted standard for a specific tool being used.

## Version Control Guidelines

- Write clear, descriptive commit messages.
- Keep commits small and focused.
- Use descriptive branch names that follow project conventions.
- Include relevant issue or ticket numbers in commit messages when applicable.

## Technologies, Libraries, and Frameworks

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
