---
status: deprecated
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - initial product scope record
informed:
  - contributors
---

# Historical record: initial full-stack MVP scope

> This is retained for historical traceability. MVP scope is a product-planning decision, not an active architecture contract.

## Qualification

- [ ] **Hard to reverse** - release scope can be reprioritized without a system migration.
- [ ] **Shapes quality attributes** - scope affects scheduling, not a lasting quality attribute.
- [ ] **Structural** - it does not define a production boundary.
- [ ] **Must be shared** - contributors need current scope information, but this belongs to roadmap documentation.

## Context and Problem Statement

The first usable release needed more than a core parser: it needed lifecycle, skills, and installation support.

## Decision Drivers

- coherent user workflow
- dogfoodable first release
- avoid a core-only milestone that could not be used end to end

## Considered Options

- Ship core engine only.
- Ship core engine, lifecycle, skills, and installation as one initial stack.
- Defer distribution and agent integration until after core release.

## Decision Outcome

The historical scope selected **the full core/lifecycle/skills/install stack** for the first product scope.

### Consequences

- Implementation and release planning covered more than the merge engine.
- Deferred features remain explicit non-goals rather than hidden architecture.

### Confirmation

The current crate and embedded asset set contain lifecycle, skills, and install-related commands; release completion is tracked separately from this archived scope decision.

## Pros and Cons of the Options

### Full initial stack

- Good, because the workflow is usable as a whole.
- Bad, because more surfaces must be stabilized before release.

### Core only

- Good, because initial implementation is smaller.
- Bad, because users cannot exercise the intended workflow.

### Defer integrations

- Good, because core milestones are isolated.
- Bad, because integration assumptions would be postponed and less tested.

## More Information

Current scope and non-goals are in [chapter 3](../../03-context-and-scope.md).
