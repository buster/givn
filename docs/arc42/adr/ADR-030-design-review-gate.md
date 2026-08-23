---
status: accepted
date: 2026-06-22
decision-makers:
  - givn maintainers
consulted:
  - workflow design-review implementation
informed:
  - contributors using the lifecycle
---

# Stress-test the design before writing tasks

## Qualification

- [x] **Hard to reverse** - catching a flawed plan before tasks and code avoids an expensive implementation rewrite.
- [x] **Shapes quality attributes** - the gate improves testability, scope control, and architecture conformance.
- [x] **Structural** - `design-review` is a lifecycle node between `design` and `tasks`.
- [x] **Must be shared** - every normal change must pass the same planning-layer checkpoint before implementation.

## Context and Problem Statement

Tasks encode design decisions. If the design has untested scope, missing scenarios, or unresolved technical choices, implementation only makes the error more expensive.

## Decision Drivers

- early detection of design flaws
- explicit one-question-at-a-time review
- no task generation before hardening
- machine-readable sign-off marker

## Considered Options

- Write tasks immediately after design.
- Add a planning review after tasks are written.
- Add `design-review` between design and tasks.

## Decision Outcome

Chosen option: **Add a required `design-review` artifact after `design` and before `tasks`**. A grilling phase identifies scope, technology, missing scenarios, testability, and risk questions; a hardening phase updates design/specs but does not write tasks. Sign-off requires `DESIGN-REVIEW: PASS`.

### Consequences

- The default graph is `proposal -> specs -> design -> design-review -> tasks -> review`.
- `design-review` is a planning gate, not an `apply.requires` archive hook.
- Removing or weakening it through an overlay is discouraged by the generated configuration guidance, but graph fields remain overlay-configurable.

### Confirmation

Manifest, status, and design-review scenarios assert dependency order, sign-off detection, and blocking of incomplete planning.

## Pros and Cons of the Options

### Review before tasks

- Good, because corrections are still plan edits.
- Bad, because the lifecycle adds an explicit checkpoint.

### Review after tasks

- Good, because tasks are available to inspect.
- Bad, because flawed task detail has already spread the flawed design.

### No design review

- Good, because the happy path is shorter.
- Bad, because testability and scope defects surface during implementation.

## More Information

See [chapter 8.2](../08-crosscutting-concepts.md#82-manifest-and-overlaypatch) and [chapter 6](../06-runtime-view.md).
