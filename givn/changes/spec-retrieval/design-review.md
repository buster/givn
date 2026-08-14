# Design Review: spec-retrieval

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the change remain advisory and local? | Keep indexing/searching and the explicit review advisory in the CLI, exclude own deltas, and never make retrieval a gate. | Resolved: proposal and design specify `givn spec review --change`; retrieval never changes a gate result. |
| 2 | Tech choices | Can `fastembed` be adopted without weakening the single-binary constraints? | Make `fastembed` optional behind `spec-retrieval`; keep the default and musl distribution builds free of native ONNX Runtime. | Resolved in design.md, ADR-039, and Arc42 chapters 2 and 7. |
| 3 | Dependency policy | Does the chosen version satisfy TC-13? | Use the latest compatible lockfile resolution under the explicitly authorized lockfile-derived `GIVN_DEP_AGE_ALLOWLIST` exception, and record the override in tasks.md. | Resolved by explicit user authorization; the normal 30-day policy remains the default outside this change. |
| 4 | Missing scenarios | Are incremental reuse, unavailable-model behavior, and review candidates observable? | Add scenarios for unchanged-vector reuse, explicit model setup failure, and advisory review output. | Resolved: all three scenarios are implemented and GREEN. |
| 5 | Testability | Can the strict subprocess scenarios reach GREEN without a multi-gigabyte model or network? | Use `GIVN_SPEC_EMBEDDER=stub` in the runner, with token-derived deterministic vectors; retain production fastembed when unset. | Resolved in design.md; the e2e gate stays offline and strict. |
| 6 | E2E fidelity | Does every happy-path CLI action use the real interface and a real driver? | Keep four distinct CLI actions on the compiled `givn` subprocess and assert CLI output/index files. | Resolved: the interaction matrix is a one-to-one CLI mapping. |
| 7 | Interaction coverage | Does the feature inventory map to valid driving mechanisms? | Map index, search, and review to real compiled subprocesses; treat own-delta exclusion as the search variant. | Resolved: all three inventory rows have real subprocess mechanisms; no browser/API mechanism is implied. |
| 8 | Risk | What is the likely production failure? | Treat native runtime/linking, model/cache footprint, dependency age, and first-use setup as explicit risks; never turn them into a correctness gate. | Resolved in ADR-039 and risks R-18/R-19/R-20. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Runner strictness | `tests/features_runner.rs` uses `.fail_on_skipped()`, and `run-tests.sh` filters the e2e subset with `--tags @e2e`. Undefined/pending steps cannot silently pass. |
| Interface fidelity | `tests/common/mod.rs` invokes the compiled `givn` binary in a temporary project root; the retrieval steps will use that same black-box seam. |
| Existing architecture | Permanent specs are rooted at `givn/specs`; active deltas are under `givn/changes`, so indexing only the permanent tree excludes the running change by construction. |
| Model evidence | `tools/embed-analysis/MODEL-REPORT.md` supports E5-large as a retrieval candidate but explicitly rejects score gates; the product design preserves that advisory boundary. |
| Review integration | `givn spec review --change` performs the advisory pass, and `givn check review --change` invokes it after the deterministic gate without changing the exit code. |

## Open questions

No unresolved design questions remain for this change. Production target support for a retrieval-enabled musl artifact is explicitly not claimed; the default build remains the supported distribution artifact.

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Yes | Yes | Yes |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | Yes |
| 3 | 03 context-and-scope | Yes | Yes | Yes | Yes |
| 4 | 04 solution-strategy | Yes | Yes | Yes | Yes |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes |
| 7 | 07 deployment-view | Yes | Yes | Yes | Yes |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes |
| 12 | 12 glossary | Yes | Yes | Yes | Yes |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's own claims.
- [x] No row is marked Yes by the reviewer while arc42.md says No or omits it.
- [x] The optional native dependency, feature gate, cache, and deterministic test backend are recorded in chapter 09.
- [x] ADR-039's native-runtime and dependency-age consequences have counterparts in chapter 11.
- [x] No claimed chapter is placeholder-only.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `proposal.md` | Defined advisory review-time candidate reporting and offline deterministic e2e behavior. |
| `design.md` | Documented review integration, E5 prefixes, opt-in feature gating, native-runtime tradeoffs, the stub embedder, and the authorized dependency-age exception. |
| `specs/spec-retrieval/spec-retrieval.feature` | Added unchanged-vector reuse, unavailable-model, and advisory review scenarios. |
| `arc42.md` | Marked deployment as affected by the opt-in runtime/cache footprint. |
| `docs/arc42/02-architecture-constraints.md` | Added the opt-in retrieval build constraint. |
| `docs/arc42/05-building-block-view.md` | Recorded fastembed and its feature-gated transitive runtime. |
| `docs/arc42/07-deployment-view.md` | Documented default versus retrieval-enabled artifacts and local cache footprint. |
| `docs/arc42/09-architecture-decisions.md` | Normalized ADR-039 metadata and recorded native-runtime/age-policy consequences. |
| `docs/arc42/11-risks-and-technical-debt.md` | Added the native-runtime, footprint, and target-linking risk. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the decisions reached during grilling.
- [x] `specs/**/*.feature` was updated for missing observable behavior.
- [x] `givn lint --change spec-retrieval` exits 0 with no parse errors or remaining `@wip` findings.
- [x] Architecture documentation (arc42) was independently checked above.

**DESIGN-REVIEW: PASS**
