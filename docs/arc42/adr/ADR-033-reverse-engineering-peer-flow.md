---
status: accepted
date: 2026-06-22
decision-makers:
  - givn maintainers
consulted:
  - reverse-engineering and characterization design record
informed:
  - contributors characterizing existing systems
---

# Model reverse engineering as a peer flow

## Qualification

- [x] **Hard to reverse** - mixing descriptive characterization with prescriptive planning would corrupt the meaning of durable artifacts.
- [x] **Shapes quality attributes** - the boundary protects behavior preservation and test validity.
- [x] **Structural** - reverse engineering has a separate artifact flow and skip semantics.
- [x] **Must be shared** - characterization agents must not fix or redesign behavior while recording the baseline.

## Context and Problem Statement

The initial reverse-engineering design reused proposal and design artifacts with instruction overrides. Those names describe desired future behavior, not a characterization of existing behavior.

## Decision Drivers

- descriptive AS-IS specifications
- behavior-preserving characterization
- proportionate artifacts
- shared step-writing mechanics

## Considered Options

- Treat characterization as a normal change with reverse-mode overrides.
- Create a separate peer flow producing feature specs, tasks, and review.
- Add a dedicated binary bootstrap mode.

## Decision Outcome

Chosen option: **Use a peer flow with `specs/<cap>/<cap>.feature`, `tasks.md`, and `review.md` only**. `givn new --skip proposal,design,design-review` and `.givn-skip` make the omission explicit; characterization uses RED/GREEN/REFACTOR mechanics but does not change production behavior except for testability seams.

### Consequences

- Reverse specs describe observed behavior and suspected bugs are flagged for follow-up rather than fixed.
- Empty-baseline archive merge remains the path for creating the first permanent feature.
- The shared `givn-steps` skill avoids duplicate step-writing rules.

### Confirmation

Reverse command, skip-state, characterization, and empty-baseline merge scenarios assert the peer artifact set and behavior-preserving boundary.

## Pros and Cons of the Options

### Separate peer flow

- Good, because artifact names match descriptive intent.
- Bad, because commands and state must support a second flow.

### Normal flow with overrides

- Good, because it reuses more lifecycle machinery.
- Bad, because proposal/design semantics need repeated exceptions.

### Binary bootstrap mode

- Good, because the mode could be explicit at runtime.
- Bad, because it adds product logic where existing merge/state behavior is sufficient.

## More Information

This supersedes [ADR-032](archive/ADR-032-reverse-engineering-old-flow.md). Mechanics are in [chapter 8.13](../08-crosscutting-concepts.md#813-reverse-engineering-flow).
