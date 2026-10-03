# Review: grouped-merge-for-new-capabilities

## Fabrication Audit

**@e2e tag integrity**: the grouped-merge scenario carries `@givn.added
@e2e` since creation; never removed (git history of the delta file).

**Step body scan**: `0` empty step bodies in
`tests/steps/grouped_merge_steps.rs` (two Given fixtures building real
change directories; no assertion-less bodies anywhere in the touched files).

**Commit cross-check**:

| Task | Commit | Production code? |
|---|---|---|
| Planning layer | `7c9696a` | Planning artifacts (pre-implementation by design) |
| flat-layout invariant | `7d88774` | Yes — fixture step file `tests/steps/grouped_merge_steps.rs` + delta |
| grouped merge (`@e2e`) | `c2b0d82` | Yes — `src/cli/archive.rs` `process_delta_file` + fixture |

**Promised components**: the grouped-target branch, the
`routing_group_for_new_capability` helper, both fixture Givens — all in the
tree.

**Strict-mode proof**: `.fail_on_skipped()` (existing); RED runs exited 101
(two independent evidences in tasks.md).

**E2E downgrade check**: the @e2e scenario drives the real `givn archive`
subprocess end-to-end (init → completed change → archive with all gates) and
asserts CLI exit code, merged grouped permanent file, absent flat directory,
and the archived change. Real interface, no repository-only shortcut.

**verify.e2e_command isolation**: `./run-tests.sh --e2e` tag-filtered; change
scope 2 scenarios vs 1 e2e (strict subset; proven in tasks.md Setup of the
predecessor and by the per-scenario runs here).

**Design conformance**: as-built matches design.md exactly (single branch +
helper; one new step file; reused steps otherwise). The routing gate itself
is untouched, as scoped.

**Interaction coverage**: inventory bullet ↔ matrix row ↔ `@e2e` scenario ↔
real archive subprocess — one row, verified.

**Result:** CLEAN — 0 findings.

## Arc42 implementation conformance

| Fact | Durable-doc source | `arc42.md` claim | Implementation evidence | Match? |
|---|---|---|---|---|
| Merge target for new capabilities | `docs/arc42/06-runtime-view.md` stage 7 (clarified: declared group; flat only without a decision) | Row 6 Yes | `process_delta_file` consults the routing table; both scenarios prove both branches | Yes |
| No ADR | chapter 09 unchanged | Row 9 No (CANONICAL_ARTIFACT) | No ADR file created | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

Reuses recorded terms only: Group, Capability, Use case, Delta, Change. No
new terms introduced; none inconsistent.

## Coverage

Measured: `GIVN_FEATURES=givn/changes/grouped-merge-for-new-capabilities/specs
./measure-coverage.sh` + `./merge-coverages.sh`, both exit 0, fresh merged
report. The new production branch (`routing_group_for_new_capability` + the
grouped target arm) is exercised by the @e2e scenario; the flat fallback arm
by the regular scenario. Statement coverage of the changed region: both arms
non-zero (llvm-cov replay, change scope).

**COVERAGE MEASUREMENT: VALID**

## Coverage gap classification

1. Dead code: none. 2. Missing test coverage: none — both branches of the
new decision are covered by the two scenarios at the CLI boundary; the
archive unit tests (241 lib tests) still pass unchanged. 3. Hard to test:
none.

## Deterministic gate dispositions

`givn check review` reported no shape/subset findings for this delta (two
scenarios with distinct Given fixtures and differing Then sets). No long
scenarios (7 steps max). No removed+added pairs. Nothing to disposition.

## Test suite result

```
GIVN_FEATURES=givn/changes/grouped-merge-for-new-capabilities/specs ./run-tests.sh → exit 0 (2 scenarios)
GIVN_FEATURES=…/specs ./run-tests.sh --e2e → 1 scenario (1 passed); exit 0
./run-tests.sh (full permanent corpus) → 360 scenarios (360 passed); exit 0
cargo test --lib → 241 passed
cargo fmt + clippy -D warnings → clean
givn check review --change grouped-merge-for-new-capabilities → exit 0 (all gates)
```

## README impact decision

README-IMPACT: none

The archive command's externally observable behavior is now correct per the
documented group model; no README section describes merge-target placement.

## Sign-off checklist

- [x] Fabrication audit: CLEAN.
- [x] Every checked task has a commit touching production code.
- [x] Strict-mode proof present (RED exits 101).
- [x] Both runners exit 0; e2e strictly smaller than full suite.
- [x] Coverage measured, valid, both decision arms exercised.
- [x] No `@wip` tags; lint clean.
- [x] Canonical E2E policy: one normalized action, one `@e2e` scenario; the
      flat variant is a regular scenario.
- [x] Implementation matches design.md; no silent deviation.
- [x] No invented classification.

REVIEW: PASS