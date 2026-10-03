# Review

## Deterministic overlap gate

### Overlap dispositions

| Scenario A | Scenario B | Decision |
| --- | --- | --- |
| `A BGE-only recommendation flags the review without NLI participation` | `An NLI-only recommendation flags the review without BGE participation` | KEEP-AS-BOUNDARY |

The pair shares its step shape deliberately: both scenarios pin the union
verdict rule for exactly one participating stage (BGE alone, NLI alone).
They assert disjoint fixtures and environments; merging them would lose the
per-stage independence evidence.

Gate re-run after the coverage scenarios: `Semantic Review` Status PASS,
Candidate count 0. The only above-threshold shape match in the delta is the
pair above (embedding 0.9992), already disposed KEEP-AS-BOUNDARY; all other
in-scope matches are `BGE_FILTERED`/`NLI_FILTERED` below the deterministic
admission bar. The three coverage scenarios added during this review
generated no new admitted pair.

### Split-or-keep

| Scenario | Decision |
| --- | --- |
| (none) | Keep all |

No delta scenario is a long scenario: the largest delta scenario is
`Retained text lines keep their established wording` at ~243 tokens, well
under the 512 budget. The token-cap advisories printed by the gate combine
a delta scenario with a pre-existing permanent scenario from
`semantic-review`/`semantic-token-cap-review` (up to ~419 tokens); those
oversized scenarios live outside this change, so none is in scope for a
split here.

## Fabrication audit

Files scanned: `tests/steps/semantic_review_output_steps.rs`,
`tests/steps/semantic_review_output_e2e_steps.rs`, the delta
`.feature`, `run-tests.sh`, `measure-coverage.sh`, `merge-coverages.sh`,
`givn/commands.yaml`, `givn/config.yaml`, `design.md`, `tasks.md`,
`arc42.md`, `src/state.rs`, `src/spec/retrieval.rs`.

| Check | Result |
| --- | --- |
| 0. `@e2e` tag integrity | Clean. All five `@givn.added @e2e` scenarios at feature lines 221/229/237/245/253 retain `@e2e` in tracked state; no tag removal in history. `verify.e2e_command: "./run-tests.sh --e2e"` configured (commands.yaml). |
| 1. Empty/no-op step bodies | None found. No `{}`-only bodies, bare `pass`, or bare `return` in either step file; Then steps carry assertion libraries (assert macros / helper asserts). |
| 2. Task commits touch production source | 34 unique commit hashes recorded in tasks.md. Production `src/` is touched (e.g. `7a63b7e` touches `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs`; plus `c3a84e1`, `6b515fb`, `f3f5161`, `e351cbd`, `8aee752`); no task commit is spec-only. |
| 3. design.md components exist | The design adds no new build block; the reporting policy folds into the existing `spec::retrieval` responsibility row. No phantom component promised. |
| 4. Strict-mode proof | Present in tasks.md: mechanism `GivnWorld::cucumber().fail_on_skipped()` (features_runner.rs:20), proof run exits non-zero (`Step panicked. Captured output: not implemented`, exit 101). |
| 5. `@e2e` assertions | Real-interface: each e2e scenario drives the compiled CLI via `common::run_givn_with_envs` (`env!("CARGO_BIN_EXE_givn")` subprocess) and asserts stdout, stderr, exit code, and filesystem state. No repository-only e2e. |
| 6. Browser-UI driver fidelity | N/A — CLI capability, no browser surface. |
| 7. Single real e2e implementation | `verify.e2e_command` resolves to `./run-tests.sh --e2e`, which filters the `@e2e` groups through the same `features_runner` binaries using the compiled-CLI bindings. No parallel weaker implementation exists (untracked or committed). |
| 8. Specs-instruction normalization / E2E scope | The five inventory entries normalize to four flag variants of "run advisory review" plus the distinct blocking-workflow action; exactly one `@e2e` scenario per distinct action. No excess e2e. |
| 9. Local runnability | design.md `Local runnability` (lines 216-220): deterministic stub backends cover all behaviours; no external services or digital twins exist to start. CLI-only change. |
| 10. verify vs verify.e2e isolation | `verify.command` (`./run-tests.sh`) and `verify.e2e_command` (`./run-tests.sh --e2e`) differ. Isolation proven by scenario counts: permanent scope non-e2e 457 vs e2e 77; change scope non-e2e 25 vs e2e 5 — e2e strictly smaller, and the `@review-output` group itself pins the same tag in both runners. |
| 11. Design conformance | Implementation matches design.md: one step file per surface (`semantic_review_output_steps.rs` / `semantic_review_output_e2e_steps.rs`), dedicated `run_review_output` group, e2e tag-filtered command as named, strict `fail_on_skipped`. No silent deviation found. |
| 12. Findings | None. |
| 14. Coverage measurement validity | See below. |

### Interaction coverage verification (check 13)

| Inventory entry | e2e scenario title | Matrix row | Driving mechanism in steps |
| --- | --- | --- | --- |
| run advisory review without flags, recommended pairs visible, filtered pairs not | Real CLI review surfaces only recommended second-stage pairs | match | `e2e_run_review` → `common::run_givn_with_envs` with `spec review --change`; asserts recommended detail line present, no FILTERED detail line |
| run advisory review with `--verbose` to see every scored pair | Real CLI verbose review lists all second-stage evidence | match | Same binding with `--verbose`; asserts FILTERED rows + raw E5 list present, truncation disclaimer absent |
| run advisory review with structured JSON output | Real CLI JSON review emits one structured document | match | Same binding with `--format json`; stdout parses as schema_version-1 document with counter parity |
| inspect the findings document persisted after a failed advisory review | Real CLI persists findings beside the change after flagged recommendations | match | Same binding on union-flagged corpus; asserts `review-findings.json` exists, parses, matches stdout document |
| run the blocking workflow review over a recommended pair | Real CLI blocking review flags recommendations through the workflow gate | match | Compiled-binary `check review --change`; exits non-zero with flagged message, `semantic-review.md` exists, no findings file |

All five inventory entries map to a matrix row and to an `@e2e` scenario
asserting real CLI subprocess output. No extra matrix row and no unmapped
interaction.

### Coverage measurement validity (check 14)

| Sub-check | Result |
| --- | --- |
| a. Processes mapped | `features_runner` test binaries (per group) plus the compiled `givn` CLI spawned via `CARGO_BIN_EXE_givn` in e2e steps. |
| b. Instrumentation | `cargo llvm-cov` instruments both the test harness and the `CARGO_BIN_EXE_givn` binary; `cargo llvm-cov clean` separates the non-e2e and e2e phases; per-process profraws merged by llvm-cov; two distinct Cobertura reports written (`non-e2e-cobertura.xml`, `e2e-cobertura.xml`) and passed to `merge-coverages.sh` (`coverage.merge_command`), which emits `coverage/cobertura-coverage.xml`. Reports differ — duplicated output not accepted. |
| c. Known exercised production path non-zero | `src/spec/retrieval.rs` covered 5439/6333 (non-e2e) and 4857/6329 (e2e); other production modules also non-zero in both reports. |
| d./e. Merged report fresh & valid | Merged report regenerated after the coverage scenarios; totals moved from 16947/22352 to 17053/22352 covered lines. Not runner-only or test-only. |
| f. Verdict | `COVERAGE MEASUREMENT: VALID`. |

## Coverage gate classification

Merged report: 17,053 / 22,352 lines (76.3%), branches absent in the
Cobertura export. Classification covers the review-output change's new code
window in `src/spec/retrieval.rs`.

### Bucket 1 — dead code

None. All residual uncovered lines are reachable-in-principle guards or
cfg alternates; no unreachable write-only code found.

### Bucket 2 — missing test coverage (resolved)

Three scenarios added during this review (commit `0ac26f3`), RED/GREEN/
REFACTOR-complete, closing the deterministic gaps:

| Gap (line) | Scenario added |
| --- | --- |
| BGE token-cap guidance + admission (6357, 6152-6153) | `Token-capped BGE pairs keep their guidance lines visible` |
| E5 token-cap advisory count/candidate lines (6057, 6062-6063, 5980-5982) | `E5 token-cap evidence prints in the advisory review output` |
| Clean-corpus no-pair-above-threshold fallback line (6028) | `A clean corpus below the threshold reports no problem in fallback mode` |

Post-change hit counts: 6357=2, 6152-6153=2, 6057=2, 6062-6063=4, 5980=99,
5982=2, 6028=1, 6268 (NLI guidance) 34→36.

### Bucket 3 — legitimately hard to test (justified)

| Line(s) | Justification |
| --- | --- |
| 5589, 5606 (`review_problem_verdict` early returns) | Unreachable-by-construction: every caller supplies a `SecondStageReport` for the gate, and invocation is pre-gated on at least one participating stage. Defensive arms. |
| 5691 (`render_review_document` None/None arm) | Callers always pass either a gate or a second-stage report; `second_stage = None, gate = None` is not constructible in the review paths. Defensive. |
| 5597, 5613 (`cfg(not(feature = ...))` alias arms) | Only compiled in partial-feature binaries (where the whole verdict collapses to `false`); the full-feature build — used by the coverage run — compiles these out. `cargo clippy --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate -D warnings` clean. cfg-noise, not a test gap. |
| 6125-6128 (real tokenizer load) | Requires a real tokenizer.json on the automatic backend; the deterministic stub-provider suite never mounts one. Real-model-only path. |
| 6136-6139, 6195-6198 (`BgeRun`/`NliRun` unavailable) | Requires the entire unique-pair pool to be token-capped so zero pairs remain for second-stage scoring — an artificial all-capped fixture that no supported mode produces; yields a warning-and-omit path on both runs. Defensive, non-observable in supported modes. |
| 1147-1148, 1177-1178, 1195 (chatter unknown-stage fallback arms) | Stage labels are set by the fixed enum resolution; the `_ =>` fallback is unreachable in-band. Defensive. |
| 5559, 5909 | Cobertura multi-line attribution artifacts (closing `)?;` lines of calls whose surrounding lines carry 49 hits). Instrumentation artifact, not a code path. |

Lines 7053-7056, 7131, 7939 examined and confirmed to fall in pre-existing
retrieval paths outside this change's new-code window.

## Arc42 implementation conformance

Durable chapters (`docs/arc42/`) vs `arc42.md`, `design.md`, `tasks.md`,
and the implementation. `addons.arc42: true`.

| Chapter | Durable source / claim | Design treatment | Implementation evidence | Match |
| --- | --- | --- | --- | --- |
| 03 context-and-scope | Extended `givn spec ... review` row and `givn/**` artifacts row with review output modes and failed-review findings docs (03:42,46,50) | CLI boundary `--verbose`/`--format json`; findings doc beside change | CLI parses both flags and persists `review-findings.json` on gate failure only (feature + e2e scenarios cover) | MATCH |
| 05 building-block-view | `spec::retrieval` responsibility row extended with decision-first reporting policy (05:60) | Reporting policy implemented inside `spec::retrieval`; no new module | All changes live in `src/spec/retrieval.rs` | MATCH |
| 06 runtime-view | New section 6.8.4.1 "Decision-first review reporting and corroborated gating" (06:243) | Three output modes, chatter gating, persist-on-failure-only, deprecated alias | Implemented and scenario-covered (default/verbose/JSON, findings, deprecated `GIVN_SPEC_NLI_REPORT_LIMIT`) | MATCH |
| 10 quality-requirements | Q-65 review evidence usability, Q-66 failed-review auditability (10:103-104) | Usability/auditability scenarios | Default-leads, verbose completeness, JSON parity, findings persistence all asserted | MATCH |
| 11 risks-and-technical-debt | D-14 deprecated-alias shim, R-39 corroborated-gating recall risk (11:48,63) | Alias parsed-but-ignored; E5 threshold binding fallback | `A deprecated report-limit setting is accepted silently without effect`; fail-safe fallback scenarios | MATCH |
| 12 glossary | "Decision-first reporting", "Findings document" (12:43-44) | Terms used in spec/design language | Terms appear verbatim in the delta spec wording | MATCH |
| 01,02,04,07,08,09 | Unaffected chapters (no change) | Unchanged | No diff in those builds/contracts | MATCH |

`ARC42 CONFORMANCE: CLEAN`.

## Sign-off checklist

- [x] `verify.command` green (`./run-tests.sh`, exit 0; permanent scope 457 scenarios passed; delta scope 30 passed)
- [x] `verify.e2e_command` green (`./run-tests.sh --e2e`, exit 0; 77 scenarios passed; delta e2e 5 passed)
- [x] Coverage measured and merged; `COVERAGE MEASUREMENT: VALID`
- [x] Every uncovered line classified into an exhaustive bucket (1/2/3); no invented bucket
- [x] Bucket-2 gaps resolved with scenarios + RED/GREEN/REFACTOR/COMMIT
- [x] Deterministic overlap findings disposed (KEEP-AS-BOUNDARY; scan clean, candidate count 0)
- [x] No long scenarios requiring split decisions
- [x] Interaction coverage matrix fully cross-referenced with inventory and e2e scenarios
- [x] Fabrication audit clean (all 14 checks)
- [x] Arc42 conformance: `ARC42 CONFORMANCE: CLEAN`
- [x] Re-run both runners after all review work — still GREEN (`givn check review --change semantic-review-output`, exit 0)

REVIEW: PASS
