# Review: coverage-merge-union

## Fabrication audit

| Check | Result |
|---|---|
| Delta scenarios + step definitions | 2 scenarios in `givn/changes/coverage-merge-union/specs/archival/extract-coverage-gate.feature`; new steps in `tests/steps/extract_coverage_gate_steps.rs` fully implemented (fixture report builder, legacy commands.yaml writer, two givens, regex then-step asserting the merged root counters) — no empty bodies |
| Checked tasks have production commits | `23bf23e` (src/cobertura.rs + step file, both scenarios), `f197526` (merge-coverages.sh + unit tests), `4f38936` (docs/arc42/08) |
| design.md components exist | `line_hits` field, `@number` deserialization, `line_hits()` aggregation, union merge with aggregate fallback, legacy fixture givens, merged-counter then-step, script recount — all present |
| Strict-mode proof | Pre-existing: `cucumber-rs` `.fail_on_skipped()` (tests/features_runner.rs); RED evidence recorded per scenario in tasks.md (exit 1 against the sum denominator, then exit 0) |
| @e2e integrity | No `@e2e` tags in the delta; none removed (git history clean) |
| Implementation vs design.md | Conforms: both-or-sum rule, union semantics, script walk (`package/class/lines/line`, hits summed, aggregate fallback), scoped run commands |
| Runner isolation | `verify.command` = `./run-tests.sh`, `verify.e2e_command` = `./run-tests.sh --e2e` — distinct; e2e suites report strictly fewer scenarios (69 + 5 vs 365) |

**FABRICATION AUDIT: CLEAN**

## Arc42 implementation conformance

| Chapter | arc42.md claim | Implementation evidence | Match |
|---|---|---|---|
| 08 crosscutting-concepts | Yes — merge contract updated | docs/arc42/08-crosscutting-concepts.md states the union rule, the aggregate-only fallback, and the branch-summation limitation (`4f38936`) | Yes |
| 01–07, 09–12 | No | No goal/scope/building-block/runtime/deployment/ADR/quality/glossary change; `crate::cobertura` gains a field and corrected logic within its existing responsibility | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

No new product terms. "Merged report", "source report", "union" are used
consistently with the existing coverage vocabulary; no glossary changes
required.

**LANGUAGE CONFORMANCE: CLEAN**

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A line measured by both reports is counted once in the merged report` | `A line only one report measures counts once and uncovered lines are not dropped` | boundary |

Rationale: the two scenarios pin different union aspects that must hold
independently. The first deduplicates a line measured by both reports (the
denominator stays 4 where the sum produced 8); the second pins that a line
only one report measures enters the union and uncovered lines are not
dropped (2/4 vs the sum's 2/5). Merging them would lose one of the two
contracts; each fixture is minimal for its own assertion.

No long scenarios in the delta; no removed+added pairs.

## README impact decision

README-IMPACT: none

The README's managed coverage badge block is regenerated mechanically by the
archive gate's coverage pipeline (it will republish the corrected number);
no authored README section changes. No command, flag, config, or output a
consumer observes changed.

## Coverage measurement validity

- `./measure-coverage.sh` re-run after the code change: all replay groups
  passed (368/368), exit 0; the fixture prebuild held (no mid-replay build).
- Both source reports freshly produced and distinct (`assert_distinct`
  passes); the merged report is freshly produced by `./merge-coverages.sh`.
- Known production path non-zero: `src/cobertura.rs` merge code is exercised
  by the new scenarios; overall production coverage 6951/16692 lines.
- The merged root rate (0.4164) equals the class-level union computed from
  the same file — the sum defect (37.6% vs 41.8%) is closed.

**COVERAGE MEASUREMENT: VALID**

## Coverage gate (completeness)

New production code in this change: the `line_hits` aggregation, the union
branch, the aggregate fallback, and the script recount — all exercised by
the two new scenarios and two new unit tests. Residual uncovered lines in
`src/cobertura.rs`: the aggregate-fallback branch is covered by the new unit
test (`aggregate_only_reports_fall_back_to_summation`) at the unit level and
by the permanent extract-coverage-gate archive scenarios (aggregate-only
fixtures) at the acceptance level; the malformed-report parse-failure path
(missing `number` attribute) is the pre-existing generic parse-error path,
bucket 3 (legitimately hard to test through the CLI: Cobertura DTD mandates
the attribute, so no conforming producer emits it). No dead code introduced.

## Sign-off checklist

- [x] Fabrication audit clean
- [x] Arc42 conformance clean
- [x] Ubiquitous language clean
- [x] Deterministic gate dispositions complete (one shape match disposed as boundary)
- [x] README impact recorded (`README-IMPACT: none`)
- [x] Coverage measurement valid; gaps classified
- [x] `./run-tests.sh` GREEN (365/365 including the two delta scenarios) and
      `./run-tests.sh --e2e` GREEN; replay measurement 368/368

REVIEW: PASS
