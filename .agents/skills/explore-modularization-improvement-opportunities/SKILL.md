---
name: explore-modularization-improvement-opportunities
description: Explore a codebase to find opportunities for architectural improvement, focusing on making the codebase more testable by deepening shallow modules. Use when user wants to improve architecture, find refactoring opportunities, consolidate tightly-coupled modules, or make a codebase more AI-navigable.
---

# Explore Modularization Improvement Opportunities

Explore a codebase like an AI would, surface architectural friction, discover opportunities for improving testability, and propose module-deepening refactors as GitHub issue RFCs.

A **deep module** (John Ousterhout, "A Philosophy of Software Design") has a small interface hiding a large implementation. Deep modules are more testable, more AI-navigable, and let you test at the boundary instead of inside. Assess depth primarily at the caller-facing boundary, not by counting internal files or private helpers.

## Process

### 1. Explore the codebase

Use the Agent tool with subagent_type=Explore or the @explore agent to navigate the codebase naturally. Do NOT follow rigid heuristics. Explore organically, note where you experience friction, and treat that friction as a clue to investigate rather than proof that a module is shallow:

- Where does understanding one concept require bouncing between many small files?
- Where do public interfaces expose nearly as much complexity as they hide?
- Where have pure functions been extracted just for testability, but the real bugs hide in how they're called?
- Where do public boundaries between modules create integration risk for callers?
- Which parts of the codebase are untested, or hard to test?

Friction during exploration can reveal candidate areas, but do not present a module as shallow until you have verified that the caller-facing interface is wide, leaky, or forces callers to coordinate too much detail themselves.

### Assess module depth

Before calling a module shallow or proposing a deepening refactor, inspect the boundary that external callers must understand.

Count as evidence of shallowness:

- Public types, functions, methods, commands, or schemas that external callers must learn
- Integration points and call patterns that leak internal coordination to callers
- Trivial wrappers that expose many entry points without hiding meaningful complexity

Do not count as evidence of shallowness by itself:

- Internal file count or private submodule structure
- Private helpers, internal data structures, or implementation-only indirection
- Language-specific internal visibility constructs that are not part of the real external contract
- Parameter count alone (many independent configuration values being passed through does not constitute coordination burden)

Heuristics:

- A deep module usually has a narrow boundary that hides substantial behavior or decision-making
- A shallow module often makes callers assemble workflows, pass many coordinated values, or understand multiple low-level concepts
- A small public API can still be shallow if callers must manage the real complexity themselves
- A fragmented internal implementation can be a maintainability smell, but it is not automatically a module-depth problem

### 2. Present candidates

Present a numbered list of deepening opportunities. **Limit to 3-5 highest-impact candidates** — if you find more, filter by: integration risk × test coverage gap × frequency of changes to the affected modules.

For each candidate, show:

- **Cluster**: Which modules/concepts are involved
- **Why they're coupled**: Shared types, call patterns, co-ownership of a concept
- **Boundary evidence**: Which caller-facing API surface or usage patterns make this candidate shallow, leaky, or coordination-heavy
- **Dependency category**: See [DEPENDENCIES.md](references/DEPENDENCIES.md) for the four categories
- **Test impact**: What existing tests would be replaced by boundary tests

Before presenting a candidate, verify:

- The concern is visible at a caller-facing boundary, not just inside the implementation
- The candidate exposes meaningful complexity to callers rather than hiding it well already
- The implementation contains non-trivial behavior worth hiding behind a smaller or clearer interface
- You are not flagging the module solely because it spans many files or feels annoying to read

Do NOT propose interfaces yet. Ask the user: "Which of these would you like to explore?"

### 3. User picks a candidate

### 4. Frame the problem space

Write a user-facing explanation of the problem space for the chosen candidate:

- The constraints any new interface would need to satisfy
- The dependencies it would need to rely on
- A rough illustrative code sketch to make the constraints concrete — this is not a proposal, just a way to ground the constraints

Ask for the user for feedback or confirmation.

**WAIT for explicit user confirmation** before proceeding to the next step.

### 5. Design multiple interfaces

Spawn 3+ sub-agents in parallel using the Agent tool. Each must produce a **radically different** interface for the deepened module.

Prompt each sub-agent with a separate technical brief (file paths, coupling details, dependency category, what's being hidden). This brief is independent of the user-facing explanation in Step 4. Give each agent a different design constraint:

- Agent 1: "Minimize the interface — aim for 1-3 entry points max"
- Agent 2: "Maximize flexibility — support many use cases and extension"
- Agent 3: "Optimize for the most common caller — make the default case trivial"
- Agent 4 (if applicable): "Design around the ports & adapters pattern for cross-boundary dependencies"

Each sub-agent outputs:

1. Interface signature (types, methods, params)
2. Usage example showing how callers use it
3. What complexity it hides internally
4. Dependency strategy (how deps are handled — see [DEPENDENCIES](references/DEPENDENCIES.md))
5. Trade-offs

Present designs sequentially, then compare them in prose.

**Decision criteria** — weigh designs by (in order):
1. **Testability improvement**: Does it reduce the number of test scenarios needed while maintaining coverage?
2. **Caller simplicity**: Does it reduce call-sites and cognitive load at the boundary?
3. **Migration feasibility**: Can existing callers migrate incrementally without breaking changes?
4. **Flexibility**: Is the interface extensible without modification (Open/Closed)?

After comparing, give your own recommendation: which design you think is strongest and why. If elements from different designs would combine well, propose a hybrid. Be opinionated — the user wants a strong read, not just a menu.

**Output format**: Each design must explicitly map to [TASK_TEMPLATE.md](references/TASK_TEMPLATE.md) sections:
- Interface signature → "Proposed Interface"
- Dependency strategy → "Dependency Strategy"
- Test approach → "Testing Strategy"
- Trade-offs → "Implementation Recommendations" (what to watch for)

### 6. User picks an interface (or accepts recommendation)

### 7. Draft and Create Task or Issue

Draft task or issue (depending on your context) using the template in [TASK_TEMPLATE](references/TASK_TEMPLATE.md). **Show the complete draft to the user** with the question: "Create this task? (yes/no/edit)"

Only create the task/issue after explicit user approval. Share the URL or task id once created.

## Prerequisites

- **Codebase size**: Minimum ~10 modules/files. Don't run on trivial codebases (< 500 LOC total) — the overhead isn't worth it.
- **Test infrastructure**: Some existing test coverage is required. If the codebase has 0% coverage, boundary tests won't help — establish basic testing practices first.
- **Language support**: Works best with languages that support interface definitions (TypeScript, Go, Rust, Scala, Java, C#). Scripting languages (Python, Ruby) work but require more discipline in interface design.

## When to Skip This Skill

**Don't use this skill if:**

- **Greenfield/prototype**: The codebase is new and rapidly changing — wait until patterns stabilize.
- **Config-heavy**: Most "logic" is configuration wiring (YAML, JSON schemas) — there's nothing to deepen.
- **Framework-locked**: The architecture is dictated by a framework (Rails, Django, Next.js conventions) — work within the framework's patterns instead.
- **No failing tests**: If the codebase has no bugs and developers aren't complaining about navigation, don't invent problems.
- **True external only**: If all dependencies are third-party services (Stripe, AWS) with no internal logic to consolidate, deepening won't yield benefits.

## Success Criteria

Stop and report completion when:

1. **No candidates found**: After exploration, no shallow modules or tight coupling identified — report: "Codebase appears well-modularized."
2. **All candidates are True External**: Dependencies are all third-party with no internal logic to consolidate — recommend dependency injection patterns instead.
3. **Task created**: User-approved task/issue is created with the chosen interface design.
4. **User rejects all candidates**: User decides none are worth pursuing — report why and close.
