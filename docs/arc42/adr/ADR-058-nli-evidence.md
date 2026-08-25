---
status: accepted
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - NLI implementation and local model evidence review
informed:
  - agents, CI users, and maintainers consuming semantic output
---

# Add independent NLI admission evidence

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

- [x] **Hard to reverse** - NLI output and its candidate admission semantics affect all semantic review surfaces.
- [x] **Shapes quality attributes** - a second model changes worklist recall, precision, and local CPU cost.
- [x] **Structural** - `nli-evidence` and its local model path form a separate evidence subsystem.
- [x] **Must be shared** - NLI probabilities must not be interpreted as Givn classifications.

## Context and Problem Statement

BGE supplies one relevance signal, while NLI supplies directional relationship evidence. NLI must not replace E5 or create an incomparable independent corpus.

## Decision Drivers

- independent feature selection
- bidirectional auditable probabilities
- shared surface-specific E5 input
- non-authoritative admission

## Considered Options

- Keep NLI diagnostic-only and leave the worklist unchanged.
- Replace E5 or build a separate all-pairs NLI corpus.
- Add NLI over the surface's E5 candidate pool and intersect recommendations when BGE also participates.

## Decision Outcome

Chosen option: **Add local NLI evidence behind `nli-evidence`**. The selected pairs are scored in both directions; label names are resolved from `id2label`; directional probabilities and a symmetric probability vector are reported. The symmetric entailment point `0.5000` emits `NLI_RECOMMENDED` or `NLI_FILTERED` evidence only. It never assigns a Givn classification.

On the Duplicate Audit, NLI receives the full E5 semantic candidate pool independently of BGE's endpoint top-five union. On Explore, advisory review, blocking semantic review, and `spec search --evidence` it receives the E5 candidates selected by that surface, independently of BGE.

### Consequences

- NLI has its own local cache and token preflight.
- Disabled NLI emits no evidence; selected artifact failure emits `NLI_UNAVAILABLE` and retains E5 fallback.
- Combined recommendations intersect BGE and NLI recommendations only when both participate.

### Confirmation

NLI scenarios cover directional and symmetric output, label mapping, inclusive admission, disabled/unavailable fallback, filtered evidence, independent pools, combined intersection, and token caps.

## Pros and Cons of the Options

### Independent NLI evidence

- Good, because it adds a distinct auditable signal without changing E5 identity.
- Bad, because it adds model setup and another CPU scoring path.

### Diagnostic-only NLI

- Good, because it cannot reduce the worklist.
- Bad, because it does not provide the intended admission evidence.

### NLI replacing or independently generating candidates

- Good, because it could expose a different corpus.
- Bad, because recall and identity comparisons against E5 become ambiguous.

## More Information

See [ADR-055](ADR-055-independent-semantic-features.md), [ADR-061](ADR-061-surface-specific-independent-pools.md), and [chapter 8.5a](../08-crosscutting-concepts.md#85a-local-scenario-retrieval).
