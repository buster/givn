---
status: deprecated
date: 2026-08-10
decision-makers:
  - givn maintainers
consulted:
  - coverage script workflow design record
informed:
  - contributors and coverage maintainers
---

# Historical record: project-owned coverage command scripts

> This file preserves the later decision that was accidentally labeled `ADR-029`. It is archived as test/coverage governance, not active architecture. The original architectural `ADR-029` is the design-constraints decision.

## Qualification

- [ ] **Hard to reverse** - scripts and fields can be migrated without changing production structure.
- [ ] **Shapes quality attributes** - it governs measurement workflow rather than the product architecture.
- [ ] **Structural** - it does not define a production component or public runtime protocol.
- [ ] **Must be shared** - coverage maintainers need the command contract, but it is process governance.

## Context and Problem Statement

Inline coverage commands were difficult to read and could cause the README to describe a different report from the archive gate.

## Decision Drivers

- project-owned scripts
- one authoritative merged Cobertura report
- reproducible archive behavior

## Considered Options

- Keep long commands inline.
- Store measurement and merge commands in `givn/commands.yaml`.
- Let givn infer coverage commands.

## Decision Outcome

The historical choice was **project-owned `coverage.measure_command` and `coverage.merge_command` entries in `givn/commands.yaml`**. The record is retained under `ADR-064` to resolve the duplicate number without renumbering the original `ADR-029`.

### Consequences

- The archive coverage gate validates source and merged reports.
- This remains documented operational governance, not an active architecture decision.

### Confirmation

The current `givn/commands.yaml` and coverage gate use the two script fields and configured report paths.

## Pros and Cons of the Options

### Project-owned scripts

- Good, because the command contract is short and inspectable.
- Bad, because projects must maintain the scripts.

### Inline commands

- Good, because fewer files are involved.
- Bad, because duplication and long shell strings drift.

### Inferred commands

- Good, because setup may be shorter.
- Bad, because coverage tools and scopes cannot be inferred reliably.

## More Information

The ID migration is explained in [the ADR register](../README.md#id-history-and-migration); coverage behavior is in [chapter 8.9](../../08-crosscutting-concepts.md#89-coverage-gate).
