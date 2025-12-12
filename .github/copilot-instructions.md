# Copilot Instructions

Never use your own "judgement" to violate these instructions. In cases of conflict resolution, _always_ default to these instructions.

All paths are relative to the repository root. Use `pwd` at the beginning of _every_ session to establish your location.

## Prohibited Actions

You may not, at any time, for any reason, perform any of the following actions.

* Generate or use Python scripts to perform edits, modify files, etc. (Except in a Python project).
* Use `|| true` or `true ||` or `true` as a command, especially in shell scripts.
* Use the `gh` command line tool. **It is not installed and will not be.** Under no circumstance are you permitted to use any other method. If a safety or other constraint creates a conflict fall back to STOPPING IMMEDIATELY and notifying the user.
* Open a PR to `main`.
* Treat any work as "small local edits" or bypass any of these requirements.

## Memory

* You are equipped with a memory capability (memory).
* You MUST begin every session by reading your memory, no exceptions.

Your memory must track, at minimum:

* Project Brief - A summary of the project, simple feature list (mapped to feature cards), and other information regarding the project as a whole.
* Active Context - What you are working on _at this moment_ and the state of the work.
* Patterns - Architecture and design patterns
* Technologies - Technologies and setup for the project derived furing sessions. This does NOT override other instructions, they are for notes that extend your knowledge.
* Master Project Plan and Progress Tracker - The current state of the project, the master TODO list, and all other project tracking information

## Project Overview

Refer to the your memory, the project [README.md](../README.md), and the project designs at `docs/design/`.

Always maintain a `PROGRESS.md` file in the root of the workspace that tracks planned, in progress, and completed work as well as the status of the project.

## Folder Structure

* `docs/`: User documentation
* `docs/design/`: Architecture and design docs
* `src/`: Core source code
* `engage-ux` the actual core library
* `engage-ux-core` core shared components and utilities
* `engage-ux-oal` the OS Abstraction Layer implementation
* `engage-ux-derive` custom derive macros for the project
* `engage-ux-themes` themes for the project
* `engage-ux-components` Premade UI components for the UX
* `themes`: Theme JSON Definitions

## Workflow

This process **MUST** be followed _in its entirety_ for all work with no exceptions:

1. Read through memory. Discard irrelevant information.
2. Read through the related Github issue.
3. Read through the design documentation, especially the Feature Card linked to the issue.
4. Ask any questions and make any suggestions prior to beginning work.
5. Create a feature branch from `v1.0.0`, name it after the feature, and link it to the Github Issue.
6. Complete _all_ tasks involved in the work without pauses or interruption.
7. Create or modify tests for all code changes.
8. Update the user and design documentation to match the implementation.
9. Commit your work.
10. Append a summary comment to the Github Issue.

## Coding Standards

* Instructions specific to a language or file supersede these.
* Never disable checks or tests (e.g. `// @ts-nocheck`, `#[allow...]`). Fix code, not checks.
* Apply OWASP guidance.
* Apply Twelve-Factor App principles.
* Prefer tabs for indentation across the codebase for accessibility and consistency. Language specific requirements, instructions, or best practices supersede this. If a file _could_ use tabs but has spaces for the majority include a note in the summary and use spaces.
* No global variables; global constants are allowed in a **dedicated constants file only**.
* Use **descriptive names**, full words, and verb-based function names (except standard getters/setters).

## Acceptance Criteria

* Tests cover positive, negative, and security cases for all code units.
* e2e tests cover all normal user interactions and common user errors.
* All tests related to the work are passing.
* The Issue has been completely resolved.

## Copilot Persona & Behavior

* Always end responses with a **5-15 bullet tl;dr style summary**.
* Assume that the user has a thorough knowledge and does not need detailed explanations by default.
* External credentials and tools will be provided, e.g. Github authentication.

## Tooling

* Use the **Github MCP** for _all_ Github interactions. If the Github MCP is not available stop immediately and notify the user for intervention.
* Use context7 MCP server for current documentation.
* Prefer MCP interaction over command line or shell tools.
* Only run one command at a time; do not chain commands.

## Templates

* **TL;DR Summary Example**

```markdown
- Checked [component] for compliance.
- Found [X issues] affecting [criteria].
- Minor changes to the logic for [function].
- Options:
  A) Fix [issue type] immediately.
  B) Review [alternative solution].
  C) Defer non-critical changes.
```
