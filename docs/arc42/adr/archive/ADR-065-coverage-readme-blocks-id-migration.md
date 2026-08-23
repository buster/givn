---
status: deprecated
date: 2026-08-10
decision-makers:
  - givn maintainers
consulted:
  - coverage README output design record
informed:
  - contributors and coverage maintainers
---

# Historical record: managed README coverage blocks

> This file preserves the later decision that was accidentally labeled `ADR-030`. It is archived as test/coverage governance, not active architecture. The original architectural `ADR-030` is the design-review gate.

## Qualification

- [ ] **Hard to reverse** - README markers can be changed without restructuring the product.
- [ ] **Shapes quality attributes** - the blocks report measurement but do not define runtime quality.
- [ ] **Structural** - they are repository output conventions, not production boundaries.
- [ ] **Must be shared** - coverage maintainers need the marker names, but this is process governance.

## Context and Problem Statement

Coverage badges need to remain visible near the README title while the detailed report and reproduction information need a separate managed area.

## Decision Drivers

- visible badges
- limited ownership of README content
- atomic replacement and rollback
- truthful `n/a` branch output

## Considered Options

- Maintain one generic coverage block.
- Maintain separate `coverage-badge` and `coverage` blocks.
- Let the coverage tool rewrite the whole README.

## Decision Outcome

The historical choice was **two named managed README blocks**. The record is retained under `ADR-065` to resolve the duplicate number without renumbering the original `ADR-030`.

### Consequences

- The archive coverage gate updates only its named blocks and restores them on failure.
- This remains operational/test governance, not an active architecture contract.

### Confirmation

Coverage badge and README block tests assert marker placement, merged-report values, rollback, and `n/a` for a zero branch denominator.

## Pros and Cons of the Options

### Two managed blocks

- Good, because givn owns only explicit regions.
- Bad, because projects must preserve the markers.

### One generic block

- Good, because there is one marker.
- Bad, because badges are less visible and surrounding README ownership is unclear.

### Rewrite the whole README

- Good, because output generation is straightforward.
- Bad, because user documentation would be overwritten.

## More Information

The ID migration is explained in [the ADR register](../README.md#id-history-and-migration); coverage behavior is in [chapter 8.9](../../08-crosscutting-concepts.md#89-coverage-gate).
