---
status: accepted
date: 2026-06-22
decision-makers:
  - givn maintainers
consulted:
  - first dogfood implementation review
informed:
  - contributors using implementation skills
---

# Carry design constraints into every scenario task

## Qualification

- [x] **Hard to reverse** - task prompts shape repeated implementation work and can systematically encode the wrong architecture.
- [x] **Shapes quality attributes** - carrying constraints protects conformance and maintainability.
- [x] **Structural** - each task identifies the design-owned module, boundary, or protocol for its scenario.
- [x] **Must be shared** - every implementation agent must follow the same design-before-shortest-path rule.

## Context and Problem Statement

A task containing only RED/GREEN/REFACTOR instructions lets an implementation agent satisfy assertions while ignoring `design.md`. The resulting tests may pass over an implementation that violates the planned architecture.

## Decision Drivers

- design conformance during implementation
- minimum code without minimum architecture
- agent-readable task context
- explicit orchestration guardrail

## Considered Options

- Leave task bodies generic and rely on agents to read design opportunistically.
- Copy scenario-specific design constraints into each task and require design review by the orchestrator.
- Move all architecture detail into step definitions.

## Decision Outcome

Chosen option: **Every scenario task contains a Design constraints block and implementation prompts require reading `design.md` before editing**. GREEN means the scenario passes and the implementation follows the design; it does not mean merely that assertions pass.

### Consequences

- Task generation must map each scenario to its design-owned constraints.
- The implementation prompt becomes longer but less vulnerable to shortest-path drift.
- This decision concerns prompt-layer conformance, not a runtime architecture validator.

### Confirmation

Task, implement, and design-review asset tests check the constraint block, full-design precondition, and conformance guardrail.

## Pros and Cons of the Options

### Constraints in tasks plus design read

- Good, because the agent's immediate context contains the relevant architecture.
- Bad, because task text must be kept synchronized with design changes.

### Generic tasks

- Good, because task templates stay short.
- Bad, because passing tests can conceal architecture violations.

### Architecture only in step definitions

- Good, because tests can enforce some details.
- Bad, because step definitions cannot express every structural design constraint.

## More Information

This is the original `ADR-029`; the later duplicate `ADR-029` label was migrated to [ADR-064](archive/ADR-064-coverage-commands-id-migration.md).
