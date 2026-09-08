---
status: superseded by ADR-069
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - BGE evidence implementation and benchmark records
informed:
  - agents and maintainers consuming semantic output
---

# Use BGE as advisory admission evidence, not as a Givn classifier

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

- [x] **Hard to reverse** - BGE statuses and output semantics affect review behavior across several commands.
- [x] **Shapes quality attributes** - the layer trades false-positive worklist size against recall and CPU cost.
- [x] **Structural** - it defines a second-stage model boundary separate from E5 classification and feature editing.
- [x] **Must be shared** - users must not mistake `BGE_FILTERED` or `BGE_RECOMMENDED` for a domain decision.

## Context and Problem Statement

E5 candidates can contain valid boundaries or unrelated scenarios that share setup vocabulary. BGE can prioritize evidence, but its relevance logit cannot decide whether executable behavior is a duplicate, variant, or valid boundary.

## Decision Drivers

- reduce unnecessary semantic worklist pressure
- preserve every E5 candidate and score provenance
- keep classification and remediation agent-owned
- support local, feature-gated inference

## Considered Options

- Keep every E5 candidate blocking and show BGE only as explanation.
- Hide low BGE scores and infer a clean review.
- Record raw E5 candidates, explicit BGE recommended/filtered states, and block only the recommended evidence where the surface requires it.

## Decision Outcome

Chosen option: **Use the symmetric BGE raw-logit point `3.0` only for `BGE_RECOMMENDED` versus `BGE_FILTERED` evidence**. Filtered evidence is visible and non-blocking; recommended candidates retain the normal agent classification/remediation requirement. BGE never assigns a Givn classification and never edits specifications.

The BGE input pool is surface-specific: the Duplicate Audit uses its endpoint top-five union; advisory review uses the candidate pool produced by its retrieval path; blocking semantic review uses the complete E5 semantic report candidates before its own budget selection; Explore applies BGE to its at-most-five ranked results; `spec search --evidence` applies BGE to the five displayed search results.

### Consequences

- A false BGE negative can remove a candidate from a recommended blocking worklist, so benchmark recall is a release concern.
- An unapproved or explicitly disabled product BGE path leaves the E5-only contract; a runtime artifact failure is represented as `BGE_UNAVAILABLE` when BGE was selected.
- BGE evidence remains independent of `spec-similarity-gate`.

### Confirmation

BGE scenarios assert bidirectional score output, inclusive threshold admission, raw-E5 preservation, filtered non-blocking behavior, unavailable fallback, and no automatic classification.

## Pros and Cons of the Options

### Advisory admission evidence

- Good, because it reduces noisy work while retaining provenance.
- Bad, because false negatives require benchmark controls and explicit fallback semantics.

### E5-only blocking

- Good, because recall is not reduced by a second model.
- Bad, because related-text false positives remain blocking.

### Silent suppression

- Good, because output is smaller.
- Bad, because lost evidence can create a false clean result.

## More Information

The superseded experiment is [ADR-042](archive/ADR-042-bge-experiment.md); budget selection is [ADR-063](ADR-063-reranker-pair-budget.md).
