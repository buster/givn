---
status: superseded by ADR-069
date: 2026-08-17
decision-makers:
  - givn maintainers
consulted:
  - semantic-remediation implementation and scenarios
informed:
  - agents and reviewers of semantic classifications
---

# Verify semantic remediation against the projected tree

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - accepting classifications without checking the projected specification would allow durable false resolutions.
- [x] **Shapes quality attributes** - the verifier protects semantic review integrity without adding a classifier.
- [x] **Structural** - it defines a second verification path over the in-memory projected permanent tree.
- [x] **Must be shared** - classification names must have concrete, repeatable remediation contracts.

## Context and Problem Statement

A candidate can disappear or fall below the E5 boundary because of a deletion, rename, or weak wording change. A lower score alone does not prove that a variant was consolidated or that a valid boundary became explicit.

## Decision Drivers

- preserve agent ownership of edits
- verify disappeared and below-boundary pairs
- require class-specific observable evidence
- reuse the existing E5 and Fingerprint paths

## Considered Options

- Treat disappearance or a lower score as automatic resolution.
- Let Givn apply classification edits.
- Project the delta read-only and verify each prior classification contract.

## Decision Outcome

Chosen option: **Verify prior classifications against a read-only projected tree and recompute prior pairs independently of current report admission**. `DUPLIKAT` requires the redundant endpoint to disappear; `VARIANTE` requires the active endpoint to become a `Scenario Outline` with examples and a score below `0.9500`; `VALID-BOUNDARY` requires both endpoints, changed Fingerprint assertions/facts, and a score below `0.9500`.

The current implementation's verifier accepts only prior entries with `left_scope: current-change` and `right_scope: permanent`. A prior `delta-to-delta` classification is therefore not a supported successful remediation path yet.

### Consequences

- Score-only edits do not resolve a variant or boundary.
- Givn emits a deterministic verification receipt but does not edit feature files.
- Remediation may fail after report generation; the blocked report is still written.

### Confirmation

Remediation scenarios cover all class contracts, stale identity, disappearance, below-boundary recomputation, score-only negative cases, delta-to-delta limitation, and non-mutation.

## Pros and Cons of the Options

### Projected-tree verification

- Good, because classification is tied to observable projected content.
- Bad, because it adds a second retrieval/evidence path and requires explicit agent edits.

### Automatic resolution

- Good, because the workflow is shorter.
- Bad, because disappearance or score change can hide an unremediated specification.

### Automatic feature edits

- Good, because common transformations could be automated.
- Bad, because Givn would decide domain intent and own destructive edits.

## More Information

See [ADR-046](ADR-046-semantic-review-report.md) and [chapter 8.5e](../08-crosscutting-concepts.md#85e-projected-remediation-verification).
