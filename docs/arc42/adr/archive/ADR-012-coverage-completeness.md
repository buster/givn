---
status: deprecated
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - coverage workflow design record
informed:
  - contributors and reviewers
---

# Historical record: coverage is classified by completeness

> This is retained for historical traceability. It is test and coverage governance, not an active architecture contract.

## Qualification

- [ ] **Hard to reverse** - coverage policy can change without restructuring the system.
- [ ] **Shapes quality attributes** - it guides verification work but does not define a lasting system quality boundary.
- [ ] **Structural** - it does not define a production component, dependency, or public protocol.
- [ ] **Must be shared** - reviewers should follow it, but it is a process convention rather than architecture.

The record is archived under the register rule that pure test/coverage governance is not an active architecture decision.

## Context and Problem Statement

A percentage alone does not explain uncovered behavior. Review needed a way to account for every coverage gap.

## Decision Drivers

- classify every gap
- avoid arbitrary percentage targets
- connect missing behavior to a scenario or justification

## Considered Options

- Use a fixed percentage threshold.
- Classify each uncovered line as dead code, missing coverage, or legitimately hard to test.
- Omit coverage review.

## Decision Outcome

The historical policy chose **completeness of gap classification instead of a percentage gate**.

### Consequences

- Review records a classification for every reported gap.
- Archive-level coverage commands and Cobertura validation remain separate project-owned governance.

### Confirmation

The coverage review instructions and archive coverage gate implement the classification and report-validation workflow.

## Pros and Cons of the Options

### Completeness classification

- Good, because a number is not mistaken for an explanation.
- Bad, because review requires deliberate gap accounting.

### Percentage threshold

- Good, because automation is simple.
- Bad, because high coverage can coexist with untested behavior.

### No coverage review

- Good, because it removes process overhead.
- Bad, because uncovered behavior has no explicit owner.

## More Information

Coverage behavior remains documented as a quality and operational concern in [chapter 8.9](../../08-crosscutting-concepts.md#89-coverage-gate) and [chapter 10](../../10-quality-requirements.md).
