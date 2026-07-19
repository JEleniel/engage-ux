---
name: planning
description: Use this skill when creating or maintaining a project plan, including task decomposition, sequencing, dependencies, and progress tracking.
---

# Planning Skill

## General guidelines

- This skill must **never** generate, modify, or suggest changes to source code or documentation other than the project plan.
- If a `docs/design/aurora/` folder exists, read and follow [Aurora Compact Model](../../aurora/Aurora.compact.instructions.md) to understand the design.
    - The Aurora Compact Model exists to save time and tokens by keeping key information in a single compact (thus the name) file.
    - Use the full Aurora instructions only when applying the Architecture skill.
- Outcome-First Planning Focus: Express intended outcomes, checkpoints, and deliverables rather than implementation details.
- Sequencing Focus: Order tasks so prerequisites, dependencies, and review gates are visible before dependent work.
- Verifiable Progress Focus: Make status and deliverables objective enough that completion can be checked without interpretation.
- Technology-Agnostic Planning Focus: Avoid prescribing technologies or implementation details unless the user explicitly requires them.

## Principles of Elegant Planning

- Clarity: Tasks, priorities, and dependencies are understandable without outside interpretation.
- Specificity: Each task has a clear purpose, scope, and completion target.
- Measurability: Progress and completion can be verified from status and deliverables.
- Relevance: Every task contributes directly to project goals or required control gates.
- Sequencing: The plan makes ordering, prerequisites, and review points explicit.
- Minimal Incidental Complexity: The plan coordinates work without prescribing unnecessary implementation detail.

### Indications of Poor Planning

- Vague Tasks.
- Hidden Dependencies.
- Milestone Bundling.
- Implementation Leakage.
- Missing Review Gates.
- Stale Status.

## Deliverables

- A Project Plan containing structured tasks, dependencies, priorities, and progress status at `docs/design/ProjectPlan.md`.
    - The plan must include tasks for implementation, documentation, reviews, and analysis as well as tasks for remediation of any identified gaps.
    - Tasks must be present for every step and element required to reach the project goals and align with the designs.
    - Tasks must be broken down into inseperable units of work that can be independently tracked and verified.
    - Reviews must be in the order analysis, architecture, code, and documentation.
- The Project Plan should use the following task format:

```markdown
1. [x] <priority P0-P3>: Description of task
    - Description: A clear, specific description of the work to be done, including the subsystem or component involved and the intended outcome.
    - Deliverables:
        - Clear, specific, concise deliverables that can be verified upon completion.
    - References: (Optional) Links to relevant Aurora cards, documentation, designs, or project artifacts that provide context for the task.
    - Notes: (Optional) Any assumptions, constraints, or additional information relevant to the task.
    - Dependends on: (Optional) A single task that must be completed before this task can be started, if applicable. This should be used to make dependencies explicit when they are not obvious from the task order or when they are critical to the success of the task.
```

## Operating Procedure

1. Confirm the planning scope, goals, constraints, and target plan path from the user request and any relevant project artifacts. If critical information is missing, stop and get clarification before updating the plan.
2. Inspect the existing plan and related artifacts in `docs/design/` before changing task structure, priorities, or status.
3. Decompose the requested work into tasks with clear priorities, statuses, descriptions, and deliverables, using optional notes, dependencies, and Aurora card references only when they add clarity.
4. Sequence tasks so prerequisites, dependencies, and required review gates appear before the work that depends on them.
5. Keep the plan outcome-focused and technology-agnostic unless the user explicitly requires technology choices or implementation detail in the plan.
6. Update task status to reflect reality, including blocked work and incomplete dependencies, rather than aspirational progress.
7. Validate the result against the deliverables and validation checklists in this skill before considering the plan update complete.

## Validation Checklists

### Plan Structure Checklist

- Correct plan target path.
- Plan format matches the documented structure.
- Consistent numbering and hierarchy.
- Priorities present and within the defined range.
- The plan includes tasks for implementation, documentation, reviews, and analysis as well as tasks for remediation of any identified gaps.
- Tasks are present for every step and element required to reach the project goals and align with the designs.
- Reviews are in the order analysis, architecture, code, and documentation.

### Task Quality Checklist

- Each task touches only one subsystem or one inseperable unit of work.
- Each task includes references to related documentation or Aurora cards providing traceability and context.
- The order of tasks minimizes the total work, for example avoiding touching a subsystem then later refactoring the same subsystem.
- Each task depends on a task of equal or higher priority.
- Every task includes verification criteria and steps.
- Tasks are specific, include the work to be done clearly stated, avoid vague or "business" language, and are directly actionable.
- Deliverables are specific, concise, and objectively verifiable.

### Dependency and Sequencing Checklist

- Task dependencies form a clean tree structure with no circular dependencies.
- Tasks only depend on equal or higher priority parents.
- Research tasks are included for any work that has an open question.
- Explicit dependencies where they matter.
- Prerequisites listed before dependents.
- Status that reflects real progress and blockers.

## Things to Watch For

### Planning Foot-Guns

- Vague tasks with no verifiable deliverable.
- Hidden dependencies.
- Overstuffed tasks that bundle unrelated work.
- Review gates missing from delivery plans.
- Implementation details masquerading as project goals.
- Status drift between the plan and reality.
- Tasks thare not broken into the smallest possible unit of work that can be independently tracked and verified.
- Tasks that depend on lower priority tasks, which can lead to blockers and delays.
- Tasks that touch more than three files or components, which can indicate that the task is not sufficiently decomposed.

## Cross-skill tasks

- If the request includes code or documentation edits beyond the project plan, split the work into phases and use the appropriate skill for each phase.
- If the request is implementation-focused, do not implement changes under this skill; use the Coding skill.
- If the request requires documentation work beyond the project plan, use the Documentation skill for that phase.
- If the request requires architecture modeling or design updates, use the Architecture skill.
- If the request is to review an existing plan, use the Reviewing skill to record findings.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
