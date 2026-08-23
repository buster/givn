---
status: accepted
date: 2026-06-22
decision-makers:
  - givn maintainers
consulted:
  - workflow and agent-skill design record
informed:
  - contributors and users
---

# Keep Explore as a pre-lifecycle skill

## Qualification

- [ ] **Hard to reverse** - the skill has no persisted artifact state.
- [x] **Shapes quality attributes** - keeping exploration unstructured improves early problem framing and avoids premature process state.
- [x] **Structural** - Explore is deliberately outside the manifest artifact graph.
- [x] **Must be shared** - agents must know when to explore and when to enter the formal proposal flow.

## Context and Problem Statement

Some problems are still questions rather than changes. Forcing a user to create a change and proposal before comparing approaches adds ceremony and pollutes lifecycle state.

## Decision Drivers

- divergent early thinking
- no false lifecycle state
- transition to a formal proposal when scope is clear

## Considered Options

- Make Explore a manifest artifact with a template and sign-off.
- Keep Explore as a skill and command outside lifecycle state.
- Omit guided exploration entirely.

## Decision Outcome

Chosen option: **Keep `givn-explore` as a pre-lifecycle skill and command**. It creates no artifact, has no sign-off marker or gate, and transitions to `/givn-propose` when the problem is sufficiently defined.

### Consequences

- Explore does not appear in the manifest or `givn status` artifact list.
- Exploration notes are not managed by givn.
- Complete-scenario retrieval is a separate `givn spec explore` command and does not change this workflow stance.

### Confirmation

Skill synchronization tests assert the Explore skill/command is emitted while manifest and state tests assert it is not a lifecycle artifact.

## Pros and Cons of the Options

### Pre-lifecycle skill

- Good, because it supports divergence without creating workflow state.
- Bad, because its output is not tracked by givn.

### Lifecycle artifact

- Good, because exploration would be visible in status.
- Bad, because a thinking stance becomes a gate and must be completed before formal planning.

### No guided exploration

- Good, because the product surface is smaller.
- Bad, because users lose a supported path for ambiguous problems.

## More Information

See [chapter 8.13](../08-crosscutting-concepts.md#813-reverse-engineering-flow) for the related reverse flow and [chapter 5](../05-building-block-view.md).
