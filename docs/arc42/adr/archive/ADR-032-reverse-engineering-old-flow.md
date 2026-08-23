---
status: superseded by ADR-033
date: 2026-06-22
decision-makers:
  - givn maintainers
consulted:
  - reverse-engineering design record
informed:
  - contributors characterizing existing systems
---

# Historical record: reverse engineering as a normal change

> This record is superseded by [ADR-033](../ADR-033-reverse-engineering-peer-flow.md). Its empty-baseline merge observation is retained as historical context.

## Qualification

- [x] **Hard to reverse** - the artifact meanings and characterization boundary affect durable baseline specifications.
- [x] **Shapes quality attributes** - confusing descriptive and prescriptive tests threatens behavior preservation.
- [x] **Structural** - it defined how reverse engineering entered the lifecycle.
- [x] **Must be shared** - characterization agents must agree on the behavior-preservation stance.

## Context and Problem Statement

The first reverse-engineering design treated characterization as a normal change with reverse-mode overrides for proposal, design, tasks, and review.

## Decision Drivers

- create a baseline against an empty permanent spec
- reuse existing merge mechanics
- preserve observed behavior

## Considered Options

- Use the normal change flow with reverse-mode instructions.
- Create a peer characterization flow.
- Add a dedicated bootstrap command.

## Decision Outcome

The historical choice was **a normal change containing Added scenarios against an empty permanent spec**. This was superseded because proposal and design semantics did not fit descriptive characterization.

### Consequences

- Empty-baseline merge mechanics remain in the current implementation.
- The artifact-flow decision is now governed by ADR-033.

### Confirmation

The current merge code creates a feature header from the delta feature name when no permanent text exists; reverse flow tests use the peer-flow rules.

## Pros and Cons of the Options

### Normal change with overrides

- Good, because it reused the existing lifecycle.
- Bad, because it overloaded proposal and design meanings.

### Peer flow

- Good, because characterization artifacts are descriptive.
- Bad, because a second flow needs explicit skip and status support.

### Bootstrap command

- Good, because the intent is explicit.
- Bad, because it duplicates lifecycle and merge behavior.

## More Information

See [ADR-033](../ADR-033-reverse-engineering-peer-flow.md).
