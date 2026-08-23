---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - Gherkin and workflow design record
informed:
  - contributors and users
---

# Keep specification and implementation concerns separate

## Qualification

- [x] **Hard to reverse** - implementation-specific specifications create durable coupling that is expensive to remove later.
- [x] **Shapes quality attributes** - the boundary improves portability, reviewability, and maintainability.
- [x] **Structural** - proposal/spec artifacts own observable WHAT/WHY while design/tasks own HOW.
- [x] **Must be shared** - authors and agents must not encode private module or runner details as behavior contracts.

## Context and Problem Statement

An executable feature can accumulate class names, routes, frameworks, and step mechanics. That makes the durable specification invalid when the implementation changes and prevents domain-level review.

## Decision Drivers

- implementation-independent behavior
- explicit ownership of technology choices
- durable Gherkin contracts
- reviewable design constraints

## Considered Options

- Keep implementation details in all artifacts.
- Put WHAT/WHY in proposals and feature files and HOW in design/tasks.
- Keep only tests as authoritative and discard durable prose.

## Decision Outcome

Chosen option: **Specifications remain implementation-independent; design and tasks own implementation detail**. An implementation aspect may appear in a feature only when it is itself observable behavior, such as a JSON response or persistence across restart.

### Consequences

- Default instructions explicitly enforce the boundary.
- Technology choices, runner setup, and step-definition locations belong in design/tasks.
- Reviews must flag implementation leakage rather than silently accepting it.

### Confirmation

Instruction and prose-consolidation scenarios check the policy and its generated guidance; design/task artifacts carry the implementation fields.

## Pros and Cons of the Options

### Separate WHAT/WHY from HOW

- Good, because the feature contract survives a reasonable reimplementation.
- Bad, because authors must decide whether a detail is externally observable.

### Implementation details everywhere

- Good, because one document may be sufficient for a contributor.
- Bad, because the specification becomes coupled and hard to reuse.

### Tests only

- Good, because executable behavior remains concrete.
- Bad, because intent and architecture become implicit and harder to review.

## More Information

See [chapter 8.3](../08-crosscutting-concepts.md#83-the-gherkin-delta-model) and [chapter 8.3a](../08-crosscutting-concepts.md#83a-normative-authoring-guidance).
