---
status: accepted
date: 2026-08-17
decision-makers:
  - givn maintainers
consulted:
  - duplicate-audit implementation, Fingerprint model, and fixtures
informed:
  - maintainers reviewing the permanent corpus
---

# Audit the complete permanent corpus without writing audit state

## Qualification

- [x] **Hard to reverse** - a complete audit defines a corpus-scale evidence and persistence boundary.
- [x] **Shapes quality attributes** - audit runtime, memory use, and non-mutation affect maintainability and operational safety.
- [x] **Structural** - the command separates permanent files, persistent E5 reuse, transient embeddings, and audit output.
- [x] **Must be shared** - maintainers must know that audit evidence is advisory and not a Givn classification.

## Context and Problem Statement

Explore is draft-centered and blocking review is change-centered. Maintainers also need every exact, structural, and eligible semantic relationship already present in permanent specs, without contaminating the persistent index or active changes.

## Decision Drivers

- complete permanent-only pair coverage
- deterministic exact and Fingerprint evidence
- advisory E5 candidates
- read-only audit boundary
- stable text and JSON output

## Considered Options

- Provide no complete permanent audit.
- Refresh or extend the persistent index for every audit.
- Read current permanent files, reuse a compatible persistent E5 index when possible, and compute the audit without writing the index.

## Decision Outcome

Chosen option: **Implement `givn spec duplicates` as a complete permanent-corpus audit**. It reads only current `givn/specs/` files and excludes active changes and archive contents. It classifies exact behavior and Fingerprint structural relations first, then scores remaining pairs with E5. It first reads `.spec-index.json` and reuses it when every endpoint identity and complete content hash matches; otherwise it embeds fitting passages transiently. The audit never refreshes or writes `.spec-index.json`.

With BGE enabled, the audit's eligible BGE pool is the union of at most five E5 neighbors per endpoint, canonicalized and budgeted. NLI, when independently selected, receives the full E5 semantic candidate pool for the audit, not the BGE top-five union. Both layers remain advisory.

### Consequences

- Exact and structural findings remain available even if an endpoint is E5-token-capped.
- One embedding per fitting scenario and in-memory pair scoring make the complete audit quadratic in corpus size.
- The audit may create or use the local model cache, but it does not mutate feature files, active changes, archive contents, or the persistent index.

### Confirmation

Duplicate-audit scenarios assert complete ranks, relation/fact evidence, same-file pairs, persistent-index non-mutation, malformed-index fallback, active/archive exclusion, inclusive threshold, and E5/BGE/NLI output accounting.

## Pros and Cons of the Options

### Read-only audit with persistent E5 reuse

- Good, because current files are authoritative while unchanged vectors can be reused.
- Bad, because a stale or incompatible index causes transient re-embedding and the full pair scan remains costly.

### No complete audit

- Good, because it adds no command or model work.
- Bad, because permanent-permanent relationships beyond interactive results remain invisible.

### Refresh or extend the index during audit

- Good, because later searches could reuse the audit work.
- Bad, because a read-only inspection would mutate persistent state and mix audit evidence with retrieval entries.

## More Information

See [chapter 6.8](../06-runtime-view.md#68-retrieval-primitives) and [ADR-052](ADR-052-bge-advisory-evidence.md).
