---
status: superseded by ADR-069
date: 2026-08-19
decision-makers:
  - givn maintainers
consulted:
  - BGE/NLI evidence implementation and surface contracts
informed:
  - agents, CI users, and maintainers consuming combined evidence
---

# Score BGE and NLI independently over each surface's E5 pool

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

- [x] **Hard to reverse** - candidate identity, layer selection, and combined admission affect review evidence across commands.
- [x] **Shapes quality attributes** - independent selection controls recall, CPU cost, and order dependence.
- [x] **Structural** - each model branch has its own pool selection, tokenizer preflight, evidence, and status.
- [x] **Must be shared** - a layer must not silently receive only another layer's survivors.

## Context and Problem Statement

Passing BGE survivors to NLI makes evaluation order decide what NLI can see. Conversely, forcing every surface into one artificial top-five pool misstates the existing command behavior.

## Decision Drivers

- preserve each surface's E5 candidate contract
- independent BGE and NLI evidence
- no survivor-only second stage
- intersection only after both branches finish
- explicit cap, unscored, filtered, and unavailable states

## Considered Options

- Run NLI only on BGE survivors.
- Let BGE and NLI build unrelated pools and union results.
- Give both branches the same surface-specific E5 candidate set and intersect their recommendations.

## Decision Outcome

Chosen option: **Build the E5 candidates according to the command surface, then give that candidate set independently to BGE and NLI**. On the Duplicate Audit, BGE uses the deterministic endpoint top-five union while NLI uses the full E5 semantic candidate pool. On blocking semantic review, both branches start from the complete semantic report candidates and apply their own ordered budget; this is not the Duplicate Audit's top-five union. Explore and advisory review use the candidate pool those surfaces already selected.

The advisory E5-only path and the second-stage advisory path also retain their
existing identity rules: E5-only ranking has no semantic self-filter, while the
second-stage projected-permanent builder excludes same-file neighbors. Strict
E5 review, blocking semantic review, and Duplicate Audit retain same-file pairs
apart from the specified Modified own-target exclusion.

Combined recommendations are the intersection of participating `BGE_RECOMMENDED` and `NLI_RECOMMENDED` states. Filtered, capped, unscored, and unavailable candidates remain visible and do not become resolved from the other layer alone.

### Consequences

- BGE and NLI output is comparable within a surface but not falsely described as one universal top-five pool.
- A budget, tokenizer cap, or artifact failure can leave unresolved blocking evidence.
- The E5 candidate generator remains the recall boundary for both models.

### Confirmation

Surface-specific scenarios assert Duplicate Audit BGE/NLI pool identity, blocking-review candidate counts, independent branch behavior, intersection semantics, and no survivor-only NLI path.

## Pros and Cons of the Options

### Independent branches over the surface E5 pool

- Good, because each command keeps its existing E5 semantics and model evidence is order-independent.
- Bad, because renderers and tests must track multiple surface contracts.

### NLI on BGE survivors

- Good, because NLI work can be smaller.
- Bad, because BGE can hide candidates before NLI evaluates them.

### Separate pools and union

- Good, because each model can search independently.
- Bad, because candidate identity and recall comparisons become ambiguous.

## More Information

See [ADR-052](ADR-052-bge-advisory-evidence.md), [ADR-058](ADR-058-nli-evidence.md), and [chapter 6.8](../06-runtime-view.md#68-retrieval-primitives).
