---
name: analysis
description: 'Use this skill when asked to analyze (not review) code structure, maintainability, coupling, complexity, call chains, module boundaries, or to produce a formal call map. Use it for analysis, not line-level review.'
---

# Code Analysis Skill

This skill produces a formal, clean-room analysis of source code structure from a higher level than a code review. Use it to evaluate module boundaries, dependency shape, call chains, state flow, abstraction quality, and maintainability risks that are easy to miss when looking only at individual lines or functions. Each run is expected to replace prior analysis artifacts for the requested scope so the result reflects only the current pass.

## General Guidelines

- Evidence-First Analysis Focus: Base findings on observable code structure, dependency relationships, and verified behavior in the repository.
- Scope Discipline Focus: Establish the analysis scope early and keep the depth of analysis proportional to that scope.
- Systemic Thinking Focus: Evaluate how modules, layers, functions, and data flow interact rather than treating issues as isolated local defects.
- Measurable Quality Focus: Prefer objective indicators such as dependency fan-in or fan-out, public API breadth, function length outliers, nesting depth, parameter count outliers, and pass-through wrapper frequency.
- Improvement-Oriented Evaluation: Identify concrete structural or behavioral changes that would improve correctness, clarity, maintainability, or long-term evolvability.
- Analysis-Not-Review Boundary: This skill performs formal structural analysis, not line-level review, defect triage, or implementation work.
- Call-Map Capture Boundary: Generating the call map is a mechanical capture task; record direct calls, uses, and dependencies without turning the call map itself into analysis.

## Principles of High-Quality Code Structure

- Clarity of Intent: Code communicates purpose and behavior directly. The code is the algorithm and comments explain use.
- Controlled Complexity: Structural and logical complexity remains bounded and predictable.
- Locality: Behavior can be understood without navigating large portions of the codebase.
- Balanced Decomposition: Modules and functions are decomposed to an appropriate granularity.
- Predictable Structure: Similar responsibilities follow similar structural patterns.

## Structural Analysis Dimensions

### Module Structure

Evaluate the organization of modules and directories.

Indicators to analyze:

- Module tree **breadth vs depth**
- Excessive nesting that obscures system structure
- Flat structures that obscure domain boundaries
- Cyclic dependencies between modules
- Modules with unclear or mixed responsibilities
- Modules acting as dependency hubs

Healthy characteristics:

- Logical domain boundaries
- Limited cross-module coupling
- Predictable module hierarchy
- Clear ownership of responsibilities
- State remains contained
- State, capabilities, and behaviors have one source of truth
- No local copies of external state or logic

### Dependency Structure

Analyze dependency relationships.

Indicators to examine:

- Dependency fan-in and fan-out
- Tight coupling across layers
- Hidden transitive dependencies
- Cross-domain imports
- Overuse of global/shared utilities

Risks include:

- Fragile dependency graphs
- Unintended architecture violations
- Difficult refactoring boundaries

### Function Structure

Evaluate functions as units of behavior.

Indicators:

- Function length
- Cognitive complexity
- Number of parameters
- Mixed responsibilities
- Nested control structures
- Hidden state mutation

Healthy functions:

- Perform one conceptual task
- Maintain predictable control flow
- Minimize hidden side effects

### Call Chain Structure

Evaluate execution depth and propagation patterns.

Indicators:

- Excessive call chain length
- Overuse of pass-through functions
- Deeply nested delegation layers
- Context propagation through multiple layers
- Error propagation complexity

Risks:

- Hard-to-trace behavior
- Hidden coupling across distant modules
- Increased debugging difficulty

Note:

- Exclude deliberate recursion from the call chain analyses unless it creates unexpected complexity or coupling.
- Be on the lookout for uintentional recursion that may be hidden in call chains (e.g., A -> B -> C -> D -> E -> B) and analyze it as a structural issue rather than a normal call chain.

### State and Data Flow

Analyze how state moves through the system.

Indicators:

- Mutable shared state
- State mutation across layers
- Inconsistent data ownership
- Excessive parameter passing
- Hidden state transitions

Healthy characteristics:

- Clear ownership of state
- Minimal mutation boundaries
- Predictable data transformations

### Abstraction Quality

Evaluate whether abstractions simplify or complicate reasoning.

Indicators of weak abstraction:

- Abstractions that expose internal details
- Wrapper layers that add little value
- Interfaces broader than required
- Generic utilities that obscure domain meaning

Healthy abstractions:

- Reduce cognitive load
- Hide implementation details
- Provide domain-relevant operations

## Indications of Poor Code Structure

- God modules accumulating unrelated responsibilities
- Overly large files or functions
- Utility modules becoming implicit dependency hubs
- Excessive wrapper layers
- Indirection without conceptual benefit
- Deep module hierarchies with thin value
- Long call chains with minimal logic per layer
- Overly broad public APIs

## Opportunities for Improvement

When analysis reveals structural weaknesses, identify actionable improvements such as:

- Flattening or restructuring module hierarchies
- Extracting cohesive modules from large components
- Reducing call chain depth
- Consolidating redundant abstractions
- Encapsulating mutable state
- Narrowing public interfaces
- Simplifying control flow
- Removing pass-through layers

Improvements should prioritize:

- Reduced cognitive complexity
- Shorter reasoning paths
- Clearer responsibility boundaries
- Safer state management
- Better long-term evolvability

## Files and Folders to Ignore

- Ignore generated artifacts, build output, vendored code, and derived documentation unless the user explicitly includes them in scope.
- Ignore repository folders such as `target/`, `debug/`, `release/`, and other cache or tool output directories when they are not the subject of the analysis.
- Treat analysis artifacts under `docs/design/analysis/` as outputs to replace, not evidence to inherit.
- Read files under `.github/` only when the user explicitly asks to analyze repository customization or workflow assets.

## Call Map and Call Graph

When generating the call map, capture direct calls, uses (such as accessing constants, types, or other non-function items), and dependencies between components (e.g. structs). The call map should be a mechanical capture of these relationships without interpretation or analysis.

- List each call pair only once, even if it occurs multiple times in the codebase. The call map should capture the existence of the relationship, not its frequency.
- Low level and internal details, such as formulas, local variables, etc. do not need to be captured.
- Any given call pair only need be listed once, no matter how many time A calls B.
- Do not capture dependencies between _files_ or modules, only between functions, types, constants, and other code items. The call map should be focused on the relationships between code items, not the file or module structure.

The Call Graph is a Mermaid graph (flowchart) built directly and mechanically from the Call Map. Do not try to optimize the diagram, just perform a module by module, line by line mapping. To convert the Call Map into the Call Graph, apply the following rules:

1. Orient the graph left to right.
2. Modules are subgraphs.
3. Functions, types, constants, and other code items are nodes within their respective module subgraph.
4. Each element is in its respective subgraph, and all callers point to the one and only one node for the callee, even if the callee is called from multiple places.
5. External dependencies can be ignored. Calls into them are not our concern, and calls from them into our code are not our concern.
6. Calls use solid edges (-->), uses are dotted (-.->), and depends on are thick (==>)
7. All edges are directed.
8. All display text is quoted to ensure that special characters don't break the diagram.

**Example Call Map Entry:**

```markdown
# Call Map

## hardware::sensors::dht22

- `validate_dht22_startup_samples` calls:
    - `hardware::sensors::validate_float_reading`
    - `hardware::sensors::validate_temperature_spread`
- and uses:
    - `DHT22_SELF_TEST_SAMPLE_COUNT`
    - `hardware::sensors::STARTUP_TEMPERATURE_STABILITY_TOLERANCE_F`
    - `HUMIDITY_FIELD`
    - `TEMPERATURE_FIELD`
- and depends on:
    - `Dht22StartupSample`
    - `hardware::sensors::SensorError`
```

## Deliverables

- Replace any existing `docs/design/analysis/Code.md` and `docs/design/analysis/CallMap.md` with a clean-room analysis for the current scope. Do not merge with or preserve prior analysis text unless the user explicitly asks for that.
- Produce a structured analysis at `docs/design/analysis/Code.md` including:
    - Identified structural weaknesses
    - Measured or observed complexity indicators
    - Risk areas affecting maintainability or correctness
    - Opportunities for structural simplification
    - Concrete improvement recommendations
    - Lessons learned applicable to future development
- Create a Call Map at `docs/design/analysis/CallMap.md` that identifies components, their functions, and what their functions call, use, and depend on.
- Generate the Call Graph at `docs/design/analysis/CallGraph.md` from the Call Map.

## Operating Procedure

1. Confirm the requested analysis scope, constraints, and success criteria from the user request and relevant repository artifacts. If critical scope information is missing, stop and get clarification before analyzing.
2. Inspect the module hierarchy, file layout, and dependency structure needed to understand the system within the requested scope.
3. Build the call map as a direct capture of functions, their calls, their uses, and their dependencies.
4. Analyze module structure, dependency shape, function complexity, call chains, state flow, and abstraction quality using observable evidence.
5. Identify structural weaknesses, complexity outliers, architecture drift, and fragility patterns that materially affect correctness, clarity, maintainability, or future change cost.
6. Propose the smallest safe structural improvements that would reduce complexity, tighten boundaries, or improve evolvability.
7. Replace `docs/design/analysis/Code.md` and `docs/design/analysis/CallMap.md` with the current analysis results for the requested scope.
8. Validate the result against the deliverables and validation checklists in this skill before considering the analysis complete.

Note:

- If present, the `./.github/violatins.sh` can be run to identify oversized files and functions as well as small functions that may be opportunities.

## Validation Checklists

### Structural Complexity Checklist

- Module hierarchy balanced between breadth and depth
- Limited cyclic dependencies
- No modules acting as uncontrolled dependency hubs
- Clear responsibility boundaries between modules
- Predictable directory and module organization

### Function Complexity Checklist

- Functions limited in scope and responsibility
- Control flow understandable without deep nesting
- Parameter lists manageable and meaningful
- Minimal reliance on hidden or global state

### Execution Structure Checklist

- Call chains remain reasonably short
- No unnecessary pass-through layers
- Error handling flows clearly through call paths
- Context does not propagate through excessive layers

### State and Data Flow Checklist

- State ownership clearly defined
- Mutable state confined to appropriate boundaries
- Data transformations explicit and traceable
- No hidden state dependencies

### Abstraction Quality Checklist

- Abstractions reduce complexity rather than increase it
- Interfaces expose minimal required capabilities
- Domain concepts represented clearly
- No unnecessary layers of indirection

### Analysis Output Checklist

- `docs/design/analysis/Code.md` replaced for the current scope rather than incrementally merged.
- `docs/design/analysis/CallMap.md` replaced for the current scope rather than incrementally merged.
- Findings are evidence-backed, scoped, and structurally focused.
- The call map stays descriptive and does not quietly become a second analysis document.

## Cross-skill Tasks

- If the request is to review an existing analysis or code artifact rather than produce a fresh formal analysis, use the Reviewing skill.
- If structural problems reveal architectural issues, switch to the Architecture skill to model and correct system design.
- If the request includes implementing structural improvements, perform analysis first and then switch to the Coding skill for refactoring.

## Glossary

See the shared [Skills glossary](../GLOSSARY.md).
