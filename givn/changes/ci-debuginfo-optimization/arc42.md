# Arc42 Impact Assessment: ci-debuginfo-optimization

This change is a pure build- and CI-infrastructure change: the development
profile stops emitting full debug information (`debug =
"line-tables-only"`), the CI job disables incremental compilation
(`CARGO_INCREMENTAL=0`), and the CI cache key is bumped once so the
post-change push does not restore stale full-debug artifacts. No observable
givn command behaviour, spec coverage, gate decision, deployment shape, or
domain vocabulary changes; the release profile and the
development/installed gating are untouched.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; the change only lowers CI disk usage of the existing build. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint change; no new dependencies. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface change. | |
| 4 | 04 solution-strategy | No | The repository's strategy is unchanged; this is a build/CI tuning change with no architectural direction impact. | |
| 5 | 05 building-block-view | No | No new component or module; `src/spec/retrieval.rs` and all building blocks are untouched. | |
| 6 | 06 runtime-view | No | No new runtime flow or sequence. | |
| 7 | 07 deployment-view | No | Binary shape, feature builds, releases, and hosts are unchanged; only the CI build profile and cache key move. | |
| 8 | 08 crosscutting-concepts | No | No cross-cutting runtime concept (error handling, security, config) changes; the debug profile is a build setting, not runtime configuration. | |
| 9 | 09 architecture-decisions | No | ADR disposition `NONE`. The debug-profile, incremental-off, and cache-key decisions are lower-level build/CI settings that are reversible, have no structural boundary or external contract, and are recorded in `design.md` (ADR-004, the single static binary, is untouched). | |
| 10 | 10 quality-requirements | No | No new quality scenario. | |
| 11 | 11 risks-and-technical-debt | No | No new persistent risk or technical debt; the transitional cache-key consequence is resolved within this change, not deferred. | |
| 12 | 12 glossary | No | No new domain terms; the change reuses the existing `development build` / `installed build` vocabulary. | |

## Notes

- The change carries no scenario tests by user decision; the regression
  guards are the existing permanent
  `release-strict-model-overrides` and `coverage-script-workflow` scenarios,
  the full `./run-tests.sh` suite exit 0, and the archive-time
  `measure-coverage.sh` run.
- ADR disposition: `NONE` (three-reversible build/CI settings; canonical
  destinations `Cargo.toml [profile.dev]`, `.github/workflows/ci.yml`, and
  the change's `design.md`).

STATUS: DONE
