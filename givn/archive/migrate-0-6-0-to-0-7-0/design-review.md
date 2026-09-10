# Design Review: migrate-0-6-0-to-0-7-0

## Method

Fresh-context grilling subagent reviewed the proposal, design, arc42
assessment, bundle assets, catalog, version surface, migration loader, upgrade
runtime, and the archive-proof contract, then exercised the route live in a
scratch project. The orchestrator resolved every finding and applied the
hardening below.

## Findings and resolutions

| # | Severity | Finding | Resolution |
|---|---|---|---|
| 1 | High | The proposal promised migration documentation but only `CHANGELOG.md` was touched; `docs/migrations/README.md` still claimed the 0.3.0-to-0.4.0 bundle was the current release. | Updated `docs/migrations/README.md` to name the 0.6.0-to-0.7.0 bundle and the archive-proof migration. |
| 2 | Medium | Aggregate placeholder substitution is aggregate-scoped, so on multi-hop routes the bundle intro would read "0.5.0 → 0.7.0 upgrade" inside the 0.6.0→0.7.0 phase. | The bundle intro now names its own fixed phase versions; only `{{UPGRADE_COMMIT}}` remains aggregate-scoped. |
| 3 | Medium | Generated aggregate proposal/design titles leaked the staging directory name (`.migrate-...tmp-<pid>`). | Surgical fix: `write_aggregate_migration_files` receives the final change id for titles; regression test added and live scratch run re-verified. |
| 4 | Medium | The bundle under-documented forward repair on a failed first archive. | Bundle blocking checks now name the pending candidate, `givn status` routing, and resume-by-rerun without recreating the merge. |
| 5 | Low | The wrapper snippet used `$scope`/`$runner_status` without assigning them. | Snippet now assigns `scope=regular|e2e` and captures `runner_status=$?`. |
| 6 | Low | Evidence-path exemptions were stated unconditionally. | Bundle now qualifies `README.md` by the readme addon and coverage paths by the coverage addon. |
| 7 | Low | arc42 cited Q-49..Q-57 for release-bundle inspection; the relevant scenario is Q-77. | Citation corrected. |
| 8 | Low | Glossary cited `proof::RunnerResult::validate`; the symbol is `read`. | Code symbol corrected. |
| 9 | Low | behave/cucumber-js advice said to map "per-scenario status" from step-level JSON. | Advice now derives scenario status from step results. |

## Confirmed clean

- Catalog entry validates: unique id, 0.6.0→0.7.0 advances, order 50, both
  assets exist and are embedded.
- Live route resolves end-to-end (scratch 0.6.0 project → aggregate change with
  bundle content) and is idempotent on rerun.
- Version surfaces are consistent at 0.7.0; no test or spec assumes a fixed
  package version.
- The seven fail-closed runner-result rules in the bundle mirror
  `proof::RunnerResult::read` exactly.
- All archive-proof consumer actions are covered (Git root, committed state,
  evidence exemptions, review-no-suite, legacy archives, e2e
  `not_applicable`, pre-candidate e2e-config block).
- `## Step Definitions` N/A is accepted by the integrity hook.
- The archive finalization fixes (`e7ea3f17`, `046d4611`) are ancestors of HEAD
  and present in source.

All questions are resolved and the hardening is applied.

DESIGN-REVIEW: PASS
