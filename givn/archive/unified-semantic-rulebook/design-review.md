# Design Review: unified-semantic-rulebook

## Grilling (2026-08-31)

Fresh-context grilling subagent; 22 findings across scope, tech choices,
missing scenarios, testability, E2E fidelity, interaction coverage, risk,
ADR qualification, arc42, and ubiquitous language. Code-grounded answers
established: stub embeddings are real bag-of-words vectors (raw cosines can
admit uncontrolled pairs at 0.90 — fixtures need disjoint corpora and
explicit overrides for every admitted pair); the pair/trait count lines and
stage-status lines exist in today's output; the pair-cache accounting line
is new; the archive gate writes no findings file on success.

## Decisions (product owner, one question at a time)

1. **Report scopes keep `delta-to-permanent` / `delta-to-delta`.** No
   rename; candidate_id and carry-forward stay stable; delta scenarios
   corrected (was: `permanent`/`current-change`).
2. **The strict E5 gate is absorbed.** `SIMILARITY_THRESHOLD` (0.95) is
   deleted with `SEMANTIC_SIMILARITY_THRESHOLD`; `@gate-only` /
   `@gate-combination` scenarios, TC-14, and the glossary "Strict E5 gate"
   entry are rewritten; ADR-041 amended to the single 0.90 boundary.
3. **Cache accounting: stdout + documents.** One `pair cache: N reused,
   M computed` stdout line per rulebook run (all three consumers — stdout
   is the only observable surface on the archive success path) plus the
   counters inside both documents.
4. **The archive surface never writes the worklist.** Gate stdout only;
   `semantic-review.md` stays owned by `check review`; mermaid edge
   corrected.
5. **No env channels for the new constants.** `NEIGHBOURS_PER_SCENARIO` and
   `PAIR_CAP` are build constants; cap scenarios drive 25 real admitted
   stub pairs instead of overriding constants.
6. **Cache accounting unit: per unique pair.** Reused only when every stage
   entry hits; computed otherwise. Recorded in design.md.

## Hardening applied

- design.md: scope names, strict-gate absorption, constants table (no env
  channels), cache key stub/model-id pinning, accounting surface + unit,
  archive stdout-only wiring, mermaid edge fix, run-tests.sh `@rulebook`
  group REQUIRED, specs-to-rewrite list corrected (remediation and advisory
  bindings enumerated as binding-only changes; NLI claim dropped),
  E2E step-file ownership noted.
- specs: scenario 4 asserts the existing `the archived change directory
  exists` step; scenario 5 asserts `delta-to-delta`; modified
  `semantic-review` scenarios use `BLOCKED` + a single-candidate fixture
  and corrected scope values; five new scenarios added (`@rulebook @wip`):
  permanent-endpoint cache invalidation, neighbour cut, emergency-budget
  UNSCORED, deterministic cap tie order, zero-candidate accounting.
- arc42: ch. 02 TC-14 rewritten and blocking-report row updated; ch. 04 and
  ch. 06 stale 0.95 rows updated; ch. 10 Q-37 boundary fixed; glossary
  "Observable distinction" and "Strict E5 gate" updated, "Second-stage pair
  budget" de-proposed, "Hybrid selection" recorded.
- ADR-066: structured qualification verdict block added with
  `routing: NEW_ADR + AMEND_ADR-061 + AMEND_ADR-041` (partial supersession
  reworded as amendment); strict-gate fate stated.

## GREEN-first note

"Consumer review keeps a duplicate pair born inside the delta" is
GREEN-first for the scope assertion after correction (a delta↔delta
candidate exists today at 0.95); its RED proof lives in the cache-accounting
and 0.90-boundary scenarios that share the fixture. Recorded so RED is not
fabricated.

## Lint

`givn lint --change unified-semantic-rulebook` exits 0 (11 @wip findings,
expected before implementation).

DESIGN-REVIEW: PASS
