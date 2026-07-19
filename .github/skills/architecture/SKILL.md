---
name: architecture
description: Use this skill when creating or updating the architecture of a system, project, or organization, including Aurora models and supporting design artifacts.
---

# Architecture Skill

## General guidelines

- Security-First Architecture Mindset: Evaluate architecture with an adversarial mindset and validate trust boundaries, privilege boundaries, and abuse resistance.
- Model-to-Reality Focus: Verify that architecture artifacts represent the implemented system and intended evolution.
- Decision Traceability Focus: Require explicit rationale, constraints, and trade-offs for architecture decisions.
- Operability and Evolvability Focus: Prioritize designs that are observable, resilient, and changeable without cascading rework.

## Principles of Elegant Architecture

- Clarity: Architectural structure and intent are understandable without external explanation.
- Simplicity: The architecture solves requirements with the smallest complete set of mechanisms.
- Composability: Components compose orthogonally without special-case glue.
- Predictability: Similar concerns are handled with consistent patterns and constraints.
- Traceability: Decisions, constraints, and trade-offs are discoverable and auditable.
- Minimal Incidental Complexity: Mechanisms exist for domain value, not accidental implementation convenience.

### Indications of Poor Architecture or Modeling

- God Components: A single component owns too many unrelated responsibilities.
- Leaky Layers: Callers must understand internals across boundaries to use interfaces correctly.
- Inconsistent Vocabulary: Different names are used for the same architecture concepts.
- Model-Implementation Mismatch: Aurora artifacts and architecture docs disagree with observed behavior.
- Unbounded Interfaces: Public interfaces are broader than required capability.
- Decision Orphans: Important architecture decisions lack rationale or traceability.

## Aurora

The Aurora modeling system captures architectural elements—such as missions, drivers, requirements, and their relationships—in a formal model that is easy to reason about.

Details on the Aurora modeling system are in the [Aurora instructions](../../aurora/Aurora.instructions.md).

- Do not use the Aurora compact model when using this skill.

### Aurora Model Fidelity Checklist

- Aurora Coverage Is Complete
- No Model Drift
- Constraint Traceability
- Decision-to-Model Traceability
- Terminology Consistency

## Deliverables

- Root the architecture in `docs/design/Requirements.md`, provided by the user, which captures the requirements and constraints the architecture must satisfy. Maintain this document as requirements evolve.
- Produce one or more Aurora models and any supporting documentation needed to capture the architecture in enough detail to guide implementation. Use multiple models only when they reduce complexity and can be kept clearly related.
- Capture architectural decisions, trade-offs, and rationale in supporting documentation. This may include ADRs, hardware specifications, and other reference material needed to understand or justify the model.
- Store Aurora models under `docs/design/aurora/`, following the standard Aurora structure and naming conventions.
- Store supporting documentation under `docs/design/`, organized into appropriate subdirectories (for example, `docs/design/adrs/` for Architectural Decision Records).

## Operating Procedure

1. Confirm the task scope, stakeholders, constraints, and success criteria from `docs/design/Requirements.md` and the user request. If critical information is missing, stop and get clarification before modeling.
2. Inspect existing architecture artifacts under `docs/design/` and `docs/design/aurora/`. Update them in place unless a genuinely new architectural boundary warrants a separate model.
3. Capture or update supporting design documentation for assumptions, constraints, external dependencies, trust boundaries, and key decisions before or alongside the model changes.
4. Create or update Aurora models under `docs/design/aurora/` using the full Aurora instructions. Prefer canonical card types and relationships; introduce non-canonical elements only when no canonical option is semantically correct and the user agrees.
5. Keep terminology, identifiers, and references consistent across requirements, supporting documentation, and Aurora artifacts so that each important decision and constraint traces back to a single source of truth.
6. Include external references—such as standards, RFCs, or third-party documentation—when they materially support the architecture, and link them to the relevant model elements for context and traceability.
7. Review the result against the deliverables and validation checklists in this skill, confirming that the model and supporting documents are complete enough to guide implementation without guesswork.
8. If the request also includes implementation work, complete the architecture phase first, then switch to the Coding skill for implementation.

## Validation Checklists

### General Architecture Checklist

- ADRs, trade-offs, and rejected alternatives captured.
- Authoritative and non-authoritative data sources identified.
- Components decomposed with clear responsibilities, separation of concerns, and isolated state.
- Clearly stated assumptions and invariants.
- Data provenance and lineage requirements captured.
- Defined versioning and compatibility strategy.
- Intentional dependencies.
- Planned deployment and rollback strategy.
- Documented inputs and outputs.
- Failure modes and remediation analysis completed.
- Identified stakeholders and external actors.
- Loosely coupled interdependencies with clear boundaries and contracts.
- Modeled data flows and transformation points.
- Concrete NFR coverage.
- Defined performance, availability, and degradation expectations.
- Planned horizontal and vertical scaling characteristics.
- Planned observability (what must be measurable).
- Purpose-driven APIs.
- Risks, threat models, security controls, and mitigations captured.
- System evolution strategy.
- Testability requirements.
- Traceable ownership and responsibility.
- Well-defined system, domain, and trust boundaries.

### Architectural Domain Coverage Checklist

- Domains and Capabilities
- Use-Case and Workflow
- Components, Interfaces, and Services
- APIs and Contracts
- Data Architecture
- State Machines and Events
- Security and Trust
- Reliability and Resilience
- Performance and Scalability
- Deployment and Runtime
- Observability and Operations
- Dependencies and Integration
- Migration and Evolution
- Testing and Verification

### External References and Link Integrity Checklist

- Resolvable Aurora external references.
- Resolvable accompanying document links.
- Consistent model-to-document links.
- Explicit reference context.
- Clear authoritative references.
- Specific, useful references.
- Stable reference links outside the workspace.

## Cross-skill tasks

- If the request includes implementation work (code changes), treat architecture modeling as its own phase first (produce/confirm the model and supporting documentation), then switch to the Coding skill for implementation.
- If the request is to review an existing architecture or model without changing it, use the Reviewing skill to record findings and recommendations.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
