# Feature Issue Checklist (template)

Use this checklist when creating issues or pull requests for a feature. Save a copy as `docs/design/agents/<feature>-issues-checklist.md` and update it as work in progress.

## Issue Summary

- Title: short, descriptive
- Summary: Brief description and user-visible behavior
- Related issues: e.g., fixes #123
- Authors: contributor usernames

## Acceptance Criteria

- Clear, testable criteria that define success for the feature
- Include unit and integration tests where applicable
- Include visual regression baselines for UI changes

## Files / Modules Affected

- List of crates, modules, and files to change (e.g., `engage-ux-components/src/button.rs`)
- Any public API changes and migration notes

## Implementation Details

- Tests: what tests will be added or changed
- Docs: documentation updates required under `docs/` or `docs/design/`
- Backwards compatibility: migration plan if API changes
- Security considerations: input validation, resource usage, SVG handling
- Accessibility: WCAG checks, ARIA roles, screen reader expectations
- Performance: benchmarks or expectations and how to measure them

## PR Checklist

- [ ] Tests added (unit/integration/visual)
- [ ] Documentation updated (`docs/` and `docs/design/`)
- [ ] Changelog entry included
- [ ] CI passes (lint, tests, formatting)
- [ ] Maintainer review requested
- [ ] Accessibility review completed (if applicable)
- [ ] Performance benchmarks attached (if applicable)

## Notes

- Save progress in `docs/design/agents/<feature>-issues-checklist.md` for long-running features.
- Keep each checklist focused and small; prefer multiple smaller PRs over one large change.
