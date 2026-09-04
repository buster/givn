# Review: rebuild-deterministic-spec-discovery

## Fabrication Audit

**Step definition scan** — the new step definitions perform real fixture
setup and real CLI subprocess invocations with concrete assertions
(exit codes, stdout content, JSON shape, tree snapshots). No stub bodies.

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Deterministic rewrite + steps + group.md inventory | (implementation commit) | yes (spec/tree.rs fixes, steps) |
| Post-archive verification | (archive commit) | permanent spec merge |

**Promised-architecture cross-check** — the commands under test
(`spec tree/find/duplicates/route`) exist from Phases 4.1–4.5; no new
components promised or built.

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| spec tree / find / duplicates / route | yes | `src/spec/` |

**Strict-mode proof** — delta-scoped verify:
`GIVN_FEATURES=givn/changes/rebuild-deterministic-spec-discovery/specs ./run-tests.sh`
→ 97 scenarios (97 passed). The delta executes in a build without
`spec-retrieval` loaded at scenario level (no scenario touches the
model paths).

**Downgraded e2e scenario scan** — the six `@e2e` scenarios drive the real
compiled CLI subprocess and assert observable stdout/stderr/exit results.

**Local environment check** — N/A (no run surface).

**E2e isolation check** — N/A for this change.

**Design.md conformance check** — matches design.md; the RED-phase
findings (missing-corpus handling, subset direction) were fixed in the
production code and are covered by scenarios.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Wholesale rewrite, old steps retained for placeholders | yes | yes | yes | n/a |

## Retirements

N/A

## Overlap dispositions

The removal+addition pairs are supersessions: each removed scenario's
behaviour is either re-expressed deterministically or intentionally
dropped with the semantic gate. No shape-match findings involve this
delta beyond the fixture-driven subset warnings reported tree-wide.

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN

README-IMPACT: none
