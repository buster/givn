# Design Review: coverage-merge-union

## Phase 1: Grilling

Fresh-context grilling subagent (task ses_f8d1b99e2ffeuFag0ekscipqSw) read the
proposal, delta spec, and design, and verified every claim against the code,
the real coverage reports, and the archived precedents.

### Findings and resolutions

1. **CRITICAL — scenario 1's fixture trips `assert_distinct`** — resolved:
   report B now hits all four lines (4/4) while A hits 1,2 (2/4); both
   reports still measure the same lines, distinctness passes (2 ≠ 4), the
   union stays 4/4, and the defective sum becomes 6/8 (design corrected).
2. **HIGH — no defined behavior for aggregate-only source reports** —
   resolved: the merge unions only when both reports carry line data;
   otherwise it falls back to the current aggregate summation, documented in
   the function contract. This preserves archive behaviour for the existing
   legacy fixtures (empty `<packages />`) and avoids a 0/0 merged report
   that `validate` would reject.
3. **HIGH — script recount must pin the walk and the fallback** — resolved:
   recount walks `package/class/lines/line` only (never all descendants —
   the real report holds 42817 line nodes, 16598 class-level), keys by
   distinct (filename, number), and keeps summed totals when either input
   lacks `<packages>`.
4. **MEDIUM — run commands omitted `GIVN_FEATURES` scoping** — resolved:
   design names the scoped invocations
   (`GIVN_FEATURES=givn/changes/coverage-merge-union/specs ...`); the
   single-scenario command is scoped the same way.
5. **MEDIUM — stale verification number** — resolved: the design cites
   6946/16598 ≈ 41.8% per the archived review evidence.
6. **LOW — fixture arithmetic pinned** — resolved: root attrs are
   consistent for `validate` (0.5, 1.0, 0.67, 0.5); the then-step reads the
   fixed attribute order of `write_legacy_merged_report`.
7. **LOW — `@number` deserialization vs empty-`<packages />` fixtures** —
   verified non-breaking (no line elements → `CoberturaLine` never
   deserialized; pinned by `parse_tolerates_packages_without_classes`).
8. **LOW — tags/lint** — verified correct; no `@e2e` added (the
   interaction gate compares permanent-tree inventories only; an added
   @e2e would create over-coverage).
9. **LOW — steps/world** — verified: all reused steps exist; inline shell
   commands replace separate script files (simpler, `sh -c` supports it).
10. **arc42** — row 08 re-derived as Yes (the coverage-gate section of
    docs/arc42/08-crosscutting-concepts.md documents the merge contract and
    must gain the union rule + branch limitation); rows 05/09/12 No
    (precedent: coverage-script-workflow marked 08 Yes for the same
    section). arc42.md updated accordingly.
11. **INFO — hit-collision semantics** — pinned: hits for the same
    (file, line) sum across reports and across classes sharing a filename.

No human decisions were required; every finding had a derivable answer.

## Phase 2: Hardening

- design.md updated with all resolutions (fixture data, both-or-sum rule,
  script walk/fallback, scoped run commands, corrected numbers).
- arc42.md row 08 marked Yes with the durable-chapter update named.
- Delta spec unchanged (scenario titles and asserted numbers already match
  the corrected fixtures); `givn lint --change coverage-merge-union`
  result: exits 0 with the @wip-tagged scenarios pending implementation.
- No `@e2e` tags exist in the delta; nothing to preserve.

DESIGN-REVIEW: PASS
