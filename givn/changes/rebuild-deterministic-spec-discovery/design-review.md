# Design Review: rebuild-deterministic-spec-discovery

## Coverage

| Question | Answer | Where |
|---|---|---|
| New components? | None — spec + step rewrite onto shipped commands | design.md §Overview |
| Step definitions? | One new file, four capabilities covered | design.md §Step Definitions |
| Removal mechanics? | 73 `@givn.removed` placeholders match existing step functions until archive | design.md §Technology decisions |
| Deterministic-only? | No scenario requires embeddings or a model download | proposal.md |
| Feature-free proof? | All new scenarios pass in a build without `spec-retrieval` (delta verify run) | tasks.md |
| Inventory reconciliation? | spec-discovery group.md Interactions rewritten to the 6 new e2e scenarios | proposal.md |

## Stress-test

- **Do removal placeholders keep the delta executable?** Yes — each
  placeholder matches one retained old step; the delta-scoped verify run
  is green (97/97).
- **Does the new duplicates command handle a missing corpus?** Fixed in
  this change: missing `givn/specs/` is an empty corpus, exit 0 — covered
  by a scenario.
- **Subset direction bug?** Found and fixed during RED: the subset
  relation is now checked in both directions of each unordered pair.

DESIGN-REVIEW: PASS
