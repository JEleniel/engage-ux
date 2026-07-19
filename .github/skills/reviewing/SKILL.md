---
name: reviewing
description: Use this skill when conducting an informal or formal review (not analysis) of code, documentation, architecture, or pre-release artifacts.
---

# Reviewing Skill

## General guidelines

- If a `docs/design/aurora/` folder exists, you may read and follow [Aurora Compact Model](../../aurora/Aurora.compact.instructions.md) to understand the design.
    - The Aurora Compact Model exists to save time and tokens by keeping key information in a single compact (thus the name) file.
    - Use the full Aurora instructions only when applying the Architecture skill.
- Evidence-First Review Focus: Base findings on observable evidence in the repository, relevant artifacts, or verified behavior.
- Scope Discipline Focus: Review only the requested scope and avoid drifting into unrelated design or implementation work.
- Risk and Severity Focus: Evaluate findings in terms of impact, likelihood, and user or system consequences.
- Actionability Focus: Recommendations should be concrete, smallest-safe, and verifiable.
- In order to ensure enough context for the review, break it down into manageable chunks and/or delegate it to subagents as needed. You may also ask for additional information or clarification from the user if necessary.
- If the user asks for a "formal review", treat it as a formal review and write the results to the appropriate files. Otherwise, provide feedback and recommendations in the conversation without writing to the review files.
- If unsure whether it is formal or not, ask.
- You MUST NOT alter code, documentation, or other files except the review output files.
- Reviews recommend improvements and fixes. Reviews do not make decisions and do not record decisions.

## Principles of Effective Review

- Clarity: Findings, risks, and recommendations are understandable without outside interpretation.
- Evidence: Every finding is grounded in observable evidence.
- Relevance: Review effort stays within scope and focuses on issues that materially affect quality, safety, or readiness.
- Traceability: Findings can be traced to specific artifacts, behaviors, or checklist criteria.
- Actionability: Recommendations describe the smallest safe fix or next step.
- Minimal Incidental Complexity: The review adds signal without burying the user in noise.

### Indications of Poor Review

- Unsupported Findings.
- Scope Creep.
- Severity Inflation.
- Missing Verification Guidance.
- Advisory as Decision.
- Checklist Cargo Culting.

## Files and folders to ignore

- Read only the `Compact.json` files under `docs/design/aurora/` for context on the Aurora model. Do not read any other files in that directory for reviews.
- Do not read any files or folders under `docs/design/` named `MIS-*`. Those are just the human-readable version of the model and contain no relevant information not already in the Compact Model.
- Do not read files under `.github/` unless specifically instructed to review them.
- The following files are irrelevant for reviews:
    - `.gitignore`
    - `.gitattributes`
    - `*.lock`
    - `.editorconfig`
    - `rustfmt.toml`
    - `rust-toolchain.toml`
    - `assets/`

## Deliverables

- Record findings in the appropriate review file under `docs/design/analysis/`:
    - `docs/design/analysis/Review-Architecture.md`
    - `docs/design/analysis/Review-Code.md`
    - `docs/design/analysis/Review-Documentation.md`
    - `docs/design/analysis/Review-Prerelease.md`
    - `docs/design/analysis/Review-Security.md`
- Apply the canonical checklist for the review type and record findings according to checklist requirements.

## Operating Procedure

1. Confirm the review scope, review type, and whether the request is formal or informal. If that is unclear, stop and ask.
2. Gather enough context from the relevant artifacts to evaluate the requested scope without drifting into unrelated areas.
3. Apply the appropriate review criteria for the artifact type and assess issues in terms of evidence, risk, severity, and user or system impact.
4. Record findings with enough detail to be actionable, including the specific issue, why it matters, and how the user can verify a fix.
5. For formal reviews, write findings to the appropriate review file under `docs/design/analysis/`. For informal reviews, provide the findings in the conversation without writing review files.
6. Keep review output limited to findings, risks, verification guidance, and smallest-safe recommendations. Do not implement fixes under this skill unless explicitly switching to another skill.
7. Validate the final review output against the deliverables and validation checklists in this skill before considering the review complete.

## Validation Checklists

### Review Process Checklist

- Correct review scope and review type.
- Enough context gathered to support the findings.
- Formal versus informal output handled correctly.
- Findings recorded in the correct location when the review is formal.

### Finding Quality Checklist

- Evidence-backed findings.
- Severity proportionate to risk and impact.
- Clear explanation of why each finding matters.
- Smallest-safe recommendations.
- Explicit verification guidance.

### Output Quality Checklist

- Findings grouped and presented clearly.
- No implementation changes outside allowed review output files.
- No decisions recorded under the guise of review findings.
- No unrelated scope drift.

## Things to Watch For

### Foot-Guns

- Findings without evidence.
- Severity inflation.
- Unclear scope boundaries.
- Recommendations too vague to act on.
- Reviews that silently become implementation work.
- Missing verification guidance.
- Multiple paths for the same functionality.
- Oversized files or functions.
- Small functions that are null functions, thin wrappers over a constant, error type mappers, or otherwise add no value.

## Cross-skill tasks

- If asked to implement fixes, do not proceed under this skill. Either switch to the appropriate skill or provide recommendations only and leave implementation to a separate task.
- If asked to update a project plan, use the Planning skill.
- If asked to produce or update an Aurora architecture model, use the Architecture skill.
- If asked to produce documentation or code changes rather than review findings, switch to the appropriate skill for that work.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
