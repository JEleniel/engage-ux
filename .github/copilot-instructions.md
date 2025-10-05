# Common Agent Instructions

## Scope

-   You may use any MCP servers you have access to.
-   Respect the `.gitignore` file; do not read or modify files listed in it.
-   Do not modify files in the `.github` or `.prompts` folders.

### Version Control Guidelines

-   Write clear, descriptive commit messages.
-   Each commit should represent a single logical change.
-   Keep commits small and focused.
-   Branch names should be descriptive and follow project conventions.
-   Include relevant issue/ticket numbers in commit messages when applicable.

## Project Structure

-   The following documents should exist in the root of the workspace: `.gitignore`, `.markdownlint.json`, `.prettierrc.json`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `DCO.md`, `LICENSE*.md` (there may be multiple), `README.md`, `SECURITY.md`. Templates for these files are available in the GitHub repository at <http://github.com/JEleniel/template/>.
-   All other documentation must be in the `docs/` folder. Design documentation must be in the `docs/design/` folder. Files in the `docs/design/agents/` folder are for machine agent use.

## Prerequisites

Before starting, familiarize yourself with:

-   [README.md](README.md) - Project overview and architecture. Do not modify this file unless otherwise instructed.
-   All files in the `docs/` folder, especially the `docs/design/` subfolder - Design documentation. Do not modify these files unless otherwise instructed.

## Development and tools

-   Use the project's linters, formatters, and type checkers (prettier, markdownlint, eslint/tsc as applicable). Fix violations before committing.
-   Run unit/integration tests locally and update or add tests for new behavior.
-   Do not commit generated artifacts or build outputs; rely on .gitignore for exclusions.

## Secrets and security

-   Never store secrets, credentials, or private keys in the repository. Use environment variables or a secrets manager.
-   Report security issues privately via the SECURITY.md process.

## Pull requests and commits

-   Keep commits focused and atomic; each PR should implement a single logical change.
-   Write descriptive commit messages and PR descriptions; reference issue/ticket numbers where relevant.
-   Include testing, changelog, and documentation updates in the same PR when applicable.

## Documentation and design

-   Place user-facing docs in docs/ and design docs in docs/design/.
-   Update README.md only when changes affect project-level details; otherwise update relevant docs in docs/.

## Communication

-   Open an issue for ambiguous or large-scope changes before implementation.
-   If in doubt about architecture or conventions, ask maintainers or follow existing patterns in the codebase.
