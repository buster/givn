---
status: superseded by ADR-069
date: 2026-08-15
decision-makers:
  - givn maintainers
consulted:
  - semantic gate implementation and dogfood evidence
informed:
  - authors and reviewers of permanent specifications
---

# Enforce a binary-fixed E5 uniqueness boundary

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

- [x] **Hard to reverse** - changing the threshold changes which specifications can be archived and requires a new evidence baseline.
- [x] **Shapes quality attributes** - the gate protects specification uniqueness and review integrity.
- [x] **Structural** - it defines the E5 comparison boundary for projected permanent and change-local pairs.
- [x] **Must be shared** - no project or agent may weaken the boundary through local configuration.

## Context and Problem Statement

Local embeddings expose scenarios that are semantically close even when deterministic fingerprints differ. A disposition note alone does not prevent duplicates or unclear boundaries from entering the permanent specification.

## Decision Drivers

- conservative semantic uniqueness
- fixed and reproducible threshold
- agent-owned edits
- compatibility with feature-gated retrieval

## Considered Options

- Keep semantic retrieval advisory only.
- Make the threshold project-configurable.
- Enforce a private binary-fixed E5 threshold and require remediation below it.

## Decision Outcome

Chosen option: **Use the private cosine threshold `0.95` when `spec-similarity-gate` is compiled**. The gate compares active Added/Modified deltas with permanent entries and the projected post-change permanent tree; scores at or above the boundary fail. No project config, CLI option, environment variable, or review note overrides it.

### Consequences

- True duplicates must be removed; valid boundaries must be changed until the score is below `0.9500`.
- The feature-free and retrieval-only builds do not provide the strict E5 gate; retrieval-only archive/review remains advisory unless the gate feature is enabled.
- The separate semantic report gate may add agent classifications but cannot override a strict-gate failure.

### Confirmation

Similarity scenarios cover same-file and cross-file projected pairs, delta-to-permanent comparisons, inclusive threshold handling, configuration resistance, and archive-time re-evaluation.

## Pros and Cons of the Options

### Binary-fixed E5 gate

- Good, because every project shares one auditable safety boundary.
- Bad, because false positives require specification edits and local model setup.

### Advisory-only retrieval

- Good, because authors are not blocked by model findings.
- Bad, because close duplicates can accumulate permanently.

### Configurable threshold

- Good, because projects can tune strictness.
- Bad, because the author could weaken the gate to pass the author's own change.

## More Information

The retrieval decision is [ADR-039](ADR-039-local-retrieval.md); change-local report behavior is [ADR-046](ADR-046-semantic-review-report.md).
