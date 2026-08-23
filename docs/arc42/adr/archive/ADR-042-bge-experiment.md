---
status: superseded by ADR-052
date: 2026-08-16
decision-makers:
  - givn maintainers
consulted:
  - cross-encoder experiment and benchmark maintainers
informed:
  - authors and agents consuming semantic evidence
---

# Historical record: retrieve with E5 and decide with a reranker

> This experiment is superseded by [ADR-052](../ADR-052-bge-advisory-evidence.md). BGE is now advisory evidence, not a replacement decision gate.

## Qualification

- [x] **Hard to reverse** - introducing a second local model changes dependencies, runtime, and candidate evidence.
- [x] **Shapes quality attributes** - recall, false positives, and CPU cost are materially affected.
- [x] **Structural** - it defined a two-stage retrieval boundary.
- [x] **Must be shared** - model logits must not be confused with the E5 threshold.

## Context and Problem Statement

Independent embeddings can over-rank valid boundaries that share setup language. A pair model can inspect both complete texts jointly.

## Decision Drivers

- bounded local model work
- multilingual pair comparison
- measured recall and CPU cost
- no hosted inference

## Considered Options

- Keep embedding-only comparison.
- Use string or frequency heuristics.
- Retrieve embedding neighbors and rerank pairs locally.

## Decision Outcome

The historical experiment chose **local BGE reranking over retrieved neighbors**, with bidirectional raw logits. Its product decision was superseded by ADR-052, which limits BGE to non-classifying admission evidence and preserves E5 provenance.

### Consequences

- The current BGE model path remains feature-gated and local.
- The old experiment's gate interpretation and same-file limitation are not current behavior.

### Confirmation

Current BGE scenarios and the surface-specific evidence records implement ADR-052 instead of this experiment's former gate.

## Pros and Cons of the Options

### Local reranking

- Good, because both texts are visible to the pair model.
- Bad, because local model setup and calibration add cost.

### Embedding-only

- Good, because it is simpler and faster.
- Bad, because related setup text creates noisy candidates.

### String heuristics

- Good, because they are cheap and local.
- Bad, because they are brittle across language and interface boundaries.

## More Information

See [ADR-052](../ADR-052-bge-advisory-evidence.md), [ADR-057](../ADR-057-bge-local-artifacts.md), and [chapter 8.5a](../../08-crosscutting-concepts.md#85a-local-scenario-retrieval).
