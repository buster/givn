# Review: harness-binary-stability

## Fabrication audit

No delta specs exist (no-delta change), so there are no step definitions to
grep, no `@e2e` tags to audit, and no interaction inventory to cross-reference.

| Check | Result |
|---|---|
| Task 1 commit touches production harness source | `e7f07e8` — run-tests.sh gains the prebuild after the zero-scenario guard |
| Task 2 commit touches production harness source | `993daa7` — measure-coverage.sh gains the instrumented prebuild before the profraw clean |
| Task 3 verification evidence | tasks.md records the cold-tree removal, two full suite runs, and the warm re-run, all exit 0 |
| design.md components exist in the tree | run-tests.sh prebuild present; measure-coverage.sh prebuild present; no other components promised |
| Strict-mode proof | Pre-existing and unchanged: `cucumber-rs` `.fail_on_skipped()` (tests/features_runner.rs); no setup task in this change, no new steps to skip |
| verify.command vs verify.e2e_command | Distinct (`./run-tests.sh` vs `./run-tests.sh --e2e`); e2e suites reported strictly fewer scenarios (69 + 5 vs 363) |
| Local runnability | `./run-tests.sh` runs the full stack (Rust workspace + repo scripts); no containers or third-party services |
| Implementation vs design.md | Conforms exactly: both prebuild commands, insertion points, and `--locked` flags match the reviewed design; no deviations |

**FABRICATION AUDIT: CLEAN**

## Arc42 implementation conformance

| Chapter | arc42.md claim | Implementation evidence | Match |
|---|---|---|---|
| 01–08, 10–11 | Not affected | No `src/` changes in `e7f07e8`/`993daa7`; repo-owned scripts only; no product runtime, deployment, quality, or risk change | Yes |
| 09 architecture-decisions | No ADR (canonical destination: design.md Alternatives section) | Rationale recorded in design.md "Alternatives considered and rejected"; no ADR file created | Yes |
| 12 glossary | Not extended | No product domain term introduced; "ready-fixture process" is harness-internal vocabulary, defined in design.md | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

No new product terms. Specs, design, and code reuse existing glossary
vocabulary (readiness, suite, replay); harness terms are defined locally in
design.md and deliberately kept out of the product glossary.

**LANGUAGE CONFORMANCE: CLEAN**

## Deterministic gate dispositions

The change carries no delta scenarios, so there are no shape matches, no
long-scenario split-or-keep decisions, and no removed+added pairs. The
overlap and semantic gates have nothing to classify.

## README impact decision

README-IMPACT: none

The change alters how the repository's own suites execute internally; no
command, flag, config, or output a consumer observes changed. The README's
documented workflow (`./run-tests.sh`, `./measure-coverage.sh`) is unchanged.

## Coverage measurement validity

- `./measure-coverage.sh` (with the new prebuild) ran to completion: all
  replay groups passed, exit 0; the fixture prebuild landed in the replay
  target dir and no mid-replay build occurred.
- Both source reports are freshly produced, structurally distinct
  (assert_distinct passes: different counters and per-file distributions),
  and show non-zero production coverage (non-e2e 6556/16598 lines; e2e
  5924/16598).
- The merge command ran and produced a fresh merged report. Known defect
  carried by the CURRENT merge arithmetic: the merged root totals sum the
  two reports' counters (12480/33196 = 37.6%) instead of computing the line
  union (class-level union in the same file: 6946/16598 = 41.8%). This is a
  reporting-arithmetic defect in merge-coverages.sh — not an
  instrumentation-validity failure — and it is the subject of the follow-up
  change; it is not classified as a coverage gap here.

**COVERAGE MEASUREMENT: VALID** (measurement instrumentation; merge
arithmetic defect tracked for the follow-up change)

## Coverage gate (completeness)

The change adds no production Rust code, so it introduces no uncovered
regions in the Cobertura report. The two changed files are POSIX shell
scripts, which llvm-cov does not instrument; their behaviour is verified by
the cold-tree reproduction in tasks.md (suite exit 0 on a cold tree, no
mid-suite build) — the strongest executable check available for a harness
script, as the suite cannot execute its own harness. No dead code, no missing
tests, no hard-to-test gaps introduced by this change.

## Sign-off checklist

- [x] Fabrication audit clean
- [x] Arc42 conformance clean
- [x] Ubiquitous language clean
- [x] Deterministic gate dispositions complete (none required)
- [x] README impact recorded (`README-IMPACT: none`)
- [x] Coverage measurement valid; gaps classified (none introduced)
- [x] `./run-tests.sh` GREEN (363/363) and `./run-tests.sh --e2e` GREEN
      (69 + 5) on a cold tree and on a warm re-run (tasks.md evidence)

REVIEW: PASS
