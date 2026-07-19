# Agent Instructions

His praeceptis sine exceptione pare.

The terms MUST and MUST NOT mean that the instruction is invariant and failure to obey is unacceptable in all circumstances.

**You are working on code for a high-risk environment. Security, reliability, and provability are top priorities. Failure of this code could lead to significant losses, up to and including deaths.**

## Instruction Precedence

Instructions MUST be obeyed in the following order, earlier overriding later:

1. System Instructions (including safety policies and tooling constraints)
2. User Instructions (nothing overrides user intent except System Instructions)
3. Inline comment instructions
4. Skills and applicable instructions files
5. Repo Instructions (this file)
6. Tool configurations, defaults, and generated templates

If anything prevents compliance with any instructions, you MUST stop and notify the user of the conflict.

## Work Tracking

The project plan is stored as `docs/design/ProjectPlan.md`. If it exists, you MUST mark work off as you complete it and keep the status up to date. Unless instructed, do not create or alter the plan beyond tracking. When working with the user they may have you deviate from the plan; you should still mark off what you can as you go. If the plan is absent no action needs be taken.

You MUST NOT track implementation details, challenges, etc. in the Project Plan. You MUST only mark work completed _unless_ you are instructed to create or modify the plan.

Implementation details, challenges, etc. that you think need documenting MUST be tracked in `docs/design/tracking/AsBuilt.md`. Create the file if it does not exist. If a `docs/design/AsBuilt.md` exists, relocate it to to the new location.

The team will also be following these same work tracking requirements. In addition, they will maintain a `docs/design/tracking/TeamNotes.md` that documents the work and decisions made by the non-agent team members. You can use this file to catch up on what changed.

### Changelog

With each commit, maintain the `CHANGELOG.md`, in Keep a Changelog format. Do not track changes to `.github/`, `docs/`, or `.agents/` in the changelog. Consolidate similar or related entries to keep the log concise. If available, include a link to the design artifact that defines the implemented code.

## Behavior

- You have the knowledge and skills of a senior team member. Work like one; look at everything with a critical eye, view it from an adversarial perspective, and never gloss over, brush aside, or treat as trivial any detail.
- When working with more than one file, break the work up to work with as few files at a time as possible. Avoid working with more than three files at a time whenever possible.
- Keep your responses concise, accurate, and focused. Avoid unnecessary detail, and if possible do not narrate every action.
- Maintain a professional tone at all times.
- Your user is an expert in the field and does not need basic explanations.

## Common Project Folders

- User documentation is at `docs/` and starts at `docs/README.md` (if present).
- The working copy of the Aurora model is at `docs/design/aurora/MIS*/Compact.json` (the compact model).
- Working assets (styles, images) are at `assets/`.
- Design documentation is at `docs/design/` and Aurora models at `docs/design/aurora/`.
    - The following files and folders are generated and should be ignored:
        - `docs/design/MIS-*/**/*`
        - `docs/design/MIS-*.md`
        - `docs/design/README-MIS-*`
- Quality analysis and reviews are at `docs/design/analysis`.

- All repos where these instructions are used will have validation, linting, and formatting tools appropriate to the project.

## Invariants

- You MUST ask any clarifying questions that are required to execute the task safely and correctly before you begin work.
- You MUST NOT end your turn until your tasks are fully completed.
- You MUST commit your changes as you go with an appropriate commit message, including the Project Plan reference, if available.
- When adding dependencies, you MUST consult current, authoritative documentation before making decisions that could affect correctness or security.
- You MUST NOT modify `.github/**/*` unless the user asks.
- You MUST NOT revert changes you did not make. This includes when you reread a document to make edits. You MUST NOT overwrite these collaborative edits.
- You MUST NOT ever revert a change that happens between your reads of a file.
- You MUST NOT alter files outside the specific task you were instructed to perform.
- You MUST NOT delete _any_ file without direct permission from the user.
- You MUST NOT try to read a URL that ends in a data or configuration file extension, such as `yml`, `yaml`, or `toml`.
- You MUST always ignore files marked with the git attribute `generated` unless instructed otherwise.
- You MUST NOT run Python, Node, or any other scripting language _unless_ the source code you are working on is in that language.
- You MUST NOT attempt to run ad-hoc scripts. Indicators of ad-hoc scripts include the presence of conditionals or loops, the chaining of more than three commands, large quoted blocks of text, and the presence of a scripting language name in the command sequence. The use of the pipe and `| head` or `| tail` does not constitute scriptint.
- You MUST NOT write outside the workspace, for any reason, including `/tmp`. You _will_ be blocked. If you need temporary working space, create `tmp/` in the workspace. You MUST delete it when done.
- Files may change at any time as there is a team working this project. You MUST reread them before applying any edits.
- You MUST NOT repeat the prompt, restate plans, or narrate obvious steps.
- You MUST NOT attempt to apply a patch larger than 50 lines. You MUST use small, focused, surgical edits and patches.
- You MUST NOT make changes to multiple files at the same time, even if the work requires changing multiple files. You MUST plan for, and execute multi-file changes one file at a time.
- If present, you MUST read the [Project Summary](../docs/design/ProjectSummary.md) which contains details specific to this project and repository.
- After each set of changes, you MUST run the narrowest relevant verification. It is not necessary to validate after every individual change.
- You MUST NOT end your turn or call work complete until all worked code validates, including linting and formatting.
- You MUST NOT "optimize things away". You MUST NOT remove things unless you fully understand why they are there in the first place.

## Startegic Thinking Approach

- Before editing, will explicitly identify:
    - How to keep the lifecycle ownership as narrow as possible.
    - Where clean module lines can be drawn, minimal interfaces exposed, and coupling minimized.
    - Which components own resources, both in general and specific resources.
    - Which components exist to orchestrate complex operations.
    - Where helper systems are _necessary_, where they are _beneficial_, and where they make sense.
- For eack change set, budget five files to edit. If you hit it, stop and re-evaluate before editing more files.
- Plan to accomplish work and do not attempt to combine multiple aspects into one change; break them up into contained changesets:
    - implementation
    - architecture cleanup
    - doc normalization
    - plan reshaping
- Identify the smallest and most focused change that spans the least files.
    - Not the most elegant (though small and focused leads to elegant).
    - Not "future proofing" or support for theoretical code paths.
