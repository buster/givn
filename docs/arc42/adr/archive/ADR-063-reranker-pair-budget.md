---
status: superseded by ADR-069
date: 2026-08-21
decision-makers:
  - givn maintainers
consulted:
  - semantic-reranker-budget proposal, design, and current runtime report
informed:
  - maintainers operating BGE/NLI audits and benchmark tooling
---

# Bound second-stage inference with an E5-ranked pair budget

## Qualification

- [x] **Hard to reverse** - the budget changes runtime work, unscored evidence, and BGE approval compatibility.
- [x] **Shapes quality attributes** - it sets a local CPU and memory ceiling while changing second-stage recall coverage.
- [x] **Structural** - the setting controls selection before BGE/NLI tokenization and inference on every participating surface.
- [x] **Must be shared** - agents and maintainers must distinguish omitted capacity from model-filtered evidence.

## Context and Problem Statement

The previous fixed pair refusal discarded all second-stage evidence when a pool was too large. A bounded selection is still needed for local CPU work, but the highest E5 candidates should be processed first and the omitted tail must remain visible.

## Decision Drivers

- bounded and predictable local work
- highest E5 candidates first
- explicit unscored tails
- independent BGE/NLI application
- unchanged 512-token safety boundary
- budget-aware approval provenance

## Considered Options

- Keep a fixed pair-count refusal.
- Remove the pair-count guard and score every eligible pair.
- Use a positive configurable budget and score the E5-ranked prefix.

## Decision Outcome

Chosen option: **Use `GIVN_SPEC_MAX_RERANK_PAIRS` as a positive runtime budget**, defaulting to `2048` when unset. Explicit empty, zero, negative, malformed, overflowing, or non-UTF-8 values fail before second-stage inference. Each surface preserves its own E5 eligibility, orders eligible candidates by descending E5 score with deterministic endpoint tie-breakers, and gives at most the prefix to BGE and independently to NLI. Omitted candidates are `BGE_UNSCORED` or `NLI_UNSCORED`, not filtered.

This record remains **proposed**: the current `givn status --change semantic-reranker-budget` reports open implementation tasks and final verification. The checkout contains the ordered-budget implementation, but the change is not a completed or accepted architectural contract yet.

### Consequences

- Duplicate Audit BGE retains its endpoint top-five union; NLI retains its independent full E5 semantic pool. Blocking review is not described as a universal top-five pool.
- Selected candidates still use the actual BGE/NLI tokenizer guard; 512 fits and 513 produces layer-specific token-cap evidence.
- Increasing the budget increases local runtime and requires matching BGE approval metadata.
- The current PR #27 real-model report records `394` permanent scenarios, `1274` E5 semantic candidates, and `795` BGE top-five-union pairs, but the report depends on local ephemeral raw JSON reports and is evidence for this checkout, not a timeless baseline.

### Confirmation

The open change's executable scenarios cover the default, E5-descending selection, unscored tails, independent BGE/NLI selection, invalid values, and CLI output. Final `givn status`, full tests, E2E tests, and blocking review remain required before changing this ADR to `accepted`.

## Pros and Cons of the Options

### Configurable ordered prefix

- Good, because highest-confidence evidence is retained under a measurable work bound.
- Bad, because lower-ranked candidates receive no second-stage evidence until the budget changes.

### Fixed refusal

- Good, because the maximum is simple.
- Bad, because one oversized pool discards even its highest-ranked candidates.

### Uncapped inference

- Good, because no candidate is omitted for capacity reasons.
- Bad, because local cost grows with the candidate pool and can become unusable.

## More Information

This supersedes the refusal behavior in [ADR-053](ADR-053-bge-pair-cap.md), depends on [ADR-061](ADR-061-surface-specific-independent-pools.md), and refines [ADR-060](ADR-060-actual-tokenizer-caps.md).
