# Copilot Instructions

All paths are relative to the repository root.

[Coding Standards](.github/prompt-snippets/CodingStandards.md)
[Technologies, Libraries, and Frameworks](.github/prompt-snippets/Technologies.md)
[Response Personality](.github/prompt-snippets/Personality.md)
[Project Information](README.md)
[Design Documentation](docs/design/)

## Project Structure

- Place documentation `docs/` folder, with a `docs/README.md` as the entry point.
    + Design documentation must be in the `docs/design/` folder.
    + Files in the `docs/design/agents/` folder are for machine agent use.
- Respect the `.gitignore` file; do not read or modify files listed in it unless otherwise instructed.
- You may read, but not modify files in the `.github` folder.
    + The `.github/templates/` folder contains examples for various files, named with the additional extension `.template` that must be remooved.
    + All other folders in `.github` should be ignored.
- Other dot folders (e.g. `.analyze`) are used by various tooling and should be ignored.

## Secrets and security

- Never store secrets, credentials, or private keys in the repository. Use environment variables or a secrets manager.
- Report security issues privately via the SECURITY.md process.
