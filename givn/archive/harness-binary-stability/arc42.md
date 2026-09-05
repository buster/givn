# Arc42 Impact Assessment: harness-binary-stability

This change is a pure acceptance-harness infrastructure change: the
repository's own suite script and coverage-replay script each prepare the
ready-fixture process before the first scenario runs, so a cold build tree
produces the same suite results as a warm one and no mid-suite build replaces
the compiled suite binary. No observable givn command behaviour, spec
coverage, gate decision, deployment shape, or product domain vocabulary
changes; the product building blocks (`src/`) are untouched.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; the change only stabilizes how the repository's own suites execute. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint change; no new dependencies. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface change; the harness is internal tooling. | |
| 4 | 04 solution-strategy | No | The repository's strategy is unchanged; this is test-infrastructure tuning with no architectural direction impact. | |
| 5 | 05 building-block-view | No | No new component or module; `src/` building blocks are untouched. The changed files are repo-owned scripts (`run-tests.sh`, `measure-coverage.sh`). | |
| 6 | 06 runtime-view | No | No new product runtime flow or sequence; the prepared fixture process only affects how the suite itself executes. | |
| 7 | 07 deployment-view | No | Binary shape, feature builds, releases, and hosts are unchanged. | |
| 8 | 08 crosscutting-concepts | No | No cross-cutting runtime concept (error handling, security, config) changes. | |
| 9 | 09 architecture-decisions | No | ADR disposition `NONE`. The prebuild-versus-alternatives decision is reversible, fixes no component/ownership boundary or durable contract, and its complete rationale is already owned by the change's `design.md` (Alternatives considered and rejected); the qualification gate's lower-level-artifact dimension fails by design. | |
| 10 | 10 quality-requirements | No | No new quality scenario; suite reproducibility on a cold tree is an internal tooling property, not a product quality attribute. | |
| 11 | 11 risks-and-technical-debt | No | No new persistent risk or technical debt; the residual stale-variant scan behaviour predates this change and is documented in `design.md`. | |
| 12 | 12 glossary | No | The term "ready-fixture process" is harness-internal vocabulary, not product domain language; the glossary holds the product's ubiquitous language and is not extended. | |

## Notes

- The change carries no delta spec by design (no-delta precedent:
  migrate-0-4-0-to-0-5-0, group-specs-by-use-case); the regression guards are
  the existing permanent `release-strict-model-overrides` and
  `review-disposition-gates` scenarios, and the full `./run-tests.sh` suite
  exit 0. No capability is created, extended, or retired.

STATUS: DONE
