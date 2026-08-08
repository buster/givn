# Design Review: fix-archive-merge-docstrings

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the spec cover exactly the reported bug plus same-defect variants? | Yes — modified+docstring, added+docstring, modified+table, one e2e. `@givn.removed` correctly untouched (removed scenarios never render). Proposal's open question (tables?) resolved in-spec as "yes". | Resolved |
| 2 | Tech choices | Is AST re-rendering of docstrings with 6-space `"""` correct? | Yes. The gherkin 0.16 parser dedents content; re-indenting to 6 spaces reproduces content; `"""` is the repo convention. Correction: the crate's other delimiter is ```` ``` ````, not `"""json` — the design says "content-type annotation not preserved", which is accurate. | Resolved (content-type note corrected) |
| 3 | Missing scenarios | Boundary: empty docstring. | The crate returns `Some("")` for `"""\n"""`. A renderer that skips empty content would delete an intentional empty docstring. Emit the two delimiter lines whenever `docstring().is_some()`. | Resolved — pinned in design + unit tests |
| 4 | Testability | Can scenarios 1–3 genuinely RED with no production change? | Yes. Pre-fix the merge still writes the file (docstring dropped), archive completes (verify `true`, merged specs have no `@e2e`, coverage passes), and the content assertion fails. Post-fix it flips green. | Resolved |
| 5 | Risk | E2E scenario's own archive gate. | archive.rs bails only when `e2e_command` is empty AND the merged permanent spec gains `@e2e`. The fixture's delta is `@givn.modified`-only (no `@e2e`), so the gate takes its silent-empty pass. Fixture deltas must remain `@e2e`-free. | Resolved — pinned in design |
| 6 | Risk | Coverage gate sentinel. | `givn init` writes coverage commands as `givn missing-coverage`, and `coverage_gate.rs` aborts on the sentinel. Fixture must set both coverage commands to `true` and write two distinct dummy Cobertura XMLs (gate requires the reports to differ). | Resolved — full fixture commands.yaml pinned in design |
| 7 | Testability | Regex ambiguity with existing step library. | cucumber-rs 0.23 errors on ambiguous multi-match; `#[given(regex)]` is not auto-anchored. New regexes fully anchored `^...$`; literal text verified disjoint from all existing steps. | Resolved |
| 8 | Tech choices | Do `--name` / `--tags` and `GIVN_FEATURES` point at the change's specs work? | Verified in cucumber-0.23.0 cli.rs (`--name <regex>`, `--tags <tagexpr>`) and tests/features_runner.rs (`GIVN_FEATURES`). Yes. | Resolved |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Root cause | `render_scenario_stripped` (merge.rs:209) emits `{keyword} {value}` per step and drops `Step.docstring`/`Step.table`. Text-level splitting (`parse_scenarios_from_text`/`extract_feature_header`) preserves docstrings for non-replaced scenarios — only the AST-reconstruction path drops them. |
| Archive gates | archive.rs post-merge verify-e2e (empty `e2e_command` + `@e2e` in merged specs ⇒ bail) and coverage-gate sentinel behaviour confirmed; both avoided by the pinned fixture config. |
| E2E interface | CLI capability; `common::run_givn` subprocess seam is the established real-interface driver; assertion reads the on-disk permanent spec — a real-interface output, no DB side-check. |
| Existing suite | All four fixture Given lines and the new Then are textually disjoint from every existing step in tests/steps. |

## Open questions

None.

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | No | No | Yes | — |
| 6 | 06 runtime-view | No | No | Yes | — |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | No | No | Yes | — |
| 9 | 09 architecture-decisions | No | No | Yes | — |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — D-9 records the bounded docstring-annotation fidelity loss |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's own claims.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every non-trivial architecture/technology decision in design.md has a corresponding MADR entry in chapter 09. (None exist in design.md — a correctness bug fix, no new ADR warranted.)
- [x] No chapter claimed as "updated" is still scaffolded placeholder text. (All 12 chapter files verified: exist, substantive content, no ASCII-art diagrams.)

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| design.md | Pinned combined attachment order (docstring then table); empty-docstring rule; full fixture `commands.yaml` (coverage commands `true` + distinct dummy Cobertura XMLs); "fixture deltas must not carry `@e2e`" guarantee; `^...$` regex anchoring; unit-test section. |
| docs/arc42/11-risks-and-technical-debt.md | Added deliberate-debt D-9: AST reconstruction canonicalises docstring delimiters on replaced scenarios. |
| givn/changes/fix-archive-merge-docstrings/arc42.md | Created marker with full 12-row assessment, STATUS: DONE. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated if any missing scenarios were identified. (No missing scenarios added; boundaries covered by unit tests in the design.)
- [x] `givn lint --change fix-archive-merge-docstrings` exits 0 or 2 (no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
