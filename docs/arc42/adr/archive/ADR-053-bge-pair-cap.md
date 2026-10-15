---
status: superseded by ADR-063
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - BGE runtime measurements and budget design
informed:
  - maintainers operating semantic audits
---

# Historical record: fixed BGE pair-cap refusal

> This record is superseded by [ADR-063](ADR-063-reranker-pair-budget.md). The endpoint top-five eligibility remains, but overflow is now intended to be handled by an E5-ranked budget.

## Qualification

- [x] **Hard to reverse** - pair selection and unavailable semantics affect evidence and runtime.
- [x] **Shapes quality attributes** - the cap bounded CPU and memory work.
- [x] **Structural** - it defined the BGE candidate selection boundary.
- [x] **Must be shared** - callers must interpret refusal versus scored evidence consistently.

## Context and Problem Statement

An all-pairs BGE pass grows too costly. A top-five endpoint union bounds candidates, but a fixed unique-pair refusal discarded all BGE evidence above the old cap.

## Decision Drivers

- predictable CPU work
- deterministic top-five union
- explicit E5 fallback

## Considered Options

- Score every pair.
- Use the endpoint union without a cap.
- Refuse the whole BGE run above a fixed cap.

## Decision Outcome

The historical contract chose **refuse the BGE invocation when the bounded union exceeded the fixed cap**. ADR-063 supersedes that refusal with a configurable positive budget and an explicit unscored tail; it does not remove the Duplicate Audit's top-five eligibility rule.

### Consequences

- Historical oversized runs reported unavailable rather than partial evidence.
- Current code and documentation must not present pair-cap refusal as the active contract.

### Confirmation

The old fixed-cap scenarios were removed or migrated; current budget scenarios assert enabled partial selection instead.

## Pros and Cons of the Options

### Fixed refusal

- Good, because the maximum work was simple.
- Bad, because it discarded the highest-ranked evidence with the rest.

### Uncapped union

- Good, because no candidate is omitted for capacity.
- Bad, because large corpora can overwhelm local inference.

### Ordered budget

- Good, because it preserves highest-ranked evidence and makes omission explicit.
- Bad, because the tail still lacks second-stage evidence.

## More Information

See [ADR-063](ADR-063-reranker-pair-budget.md) and [chapter 11](../../11-risks-and-technical-debt.md).
