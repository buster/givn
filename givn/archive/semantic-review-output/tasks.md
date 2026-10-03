# Tasks: semantic-review-output

Coverage pre-check (mandatory): the five User Interaction Inventory
entries map 1:1 to the five `@givn.added @e2e` scenarios and to the five
Interaction Coverage Matrix rows in `design.md`. Driving mechanism for
every row: compiled-CLI subprocess. No gaps.

Runner: `./run-tests.sh` · E2E: `./run-tests.sh --e2e`
Group for this capability: `run_review_output` (`@review-output`),
features `spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate`.
Targeted command template:
`GIVN_FEATURES=givn/changes/semantic-review-output/specs cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --name "<title>"`

## Setup

- [x] Create step skeletons `tests/steps/semantic_review_output_steps.rs`
  and `tests/steps/semantic_review_output_e2e_steps.rs` (all bodies
  `unimplemented!()`), register both in `tests/steps/mod.rs`.
- [x] Re-prove strict mode with one stubbed step.
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`
    (`tests/features_runner.rs:20`)
  - Proof output: `1 scenario (1 failed)`, `3 steps (2 passed, 1 failed)`,
    `Step panicked. Captured output: not implemented`, exit 101.
- [x] Audit sweep of all `tests/steps/*.rs` for pins on filtered,
  unscored, or token-cap detail in advisory-review stdout. Findings:
  `bge_reranking_evidence_steps.rs:982,999,1827`,
  `nli_evidence_steps.rs:167,605`, `nli_evidence_e2e_steps.rs:253`,
  `gate_combination_reports_both` (`similarity gate` stdout pin).
  All migrated deliberately inside the flipping commits; the
  gate-combination stdout line is preserved verbatim in fallback mode.
- [x] Add the `@review-output` runner group to `run-tests.sh`
  (`run_review_output`; existing nli groups exclude the tag).
  Counts: full suite after implementation `387+42+9+12+7+23 = 480
  scenarios, 480 passed`; e2e subset `6 scenarios` — strictly smaller.
  Final verification: `./run-tests.sh` exit 0 (480 passed) and
  `./run-tests.sh --e2e` exit 0 after the download-start dedupe
  resolution (`f2b5ccc` and the chatter-refinement commit).
- [x] Named permanent-pin migration task (executed within the flipping
  commits): permanent scenarios and bindings at
  `givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature`
  ("Advisory change review presents..." wording),
  `tests/steps/bge_reranking_evidence_steps.rs:999,1827`,
  `tests/steps/nli_evidence_steps.rs` combined-recommendation step.
  Evidence: all permanent suite groups green after migration.

## Scenario: Default output reports only recommended second-stage pairs

- [x] RED: removed this scenario's `@wip`; ran targeted with unimplemented
  Then stubs. Output: `1 scenario (1 failed)`,
  `3 steps (2 passed, 1 failed)`, exit 101.
- [x] GREEN: visibility filter on NLI detail loop and BGE per-pair loop;
  token-cap guidance stays visible; status/accounting/distribution lines
  unchanged. Carried in this commit: CLI flags, corroborated union
  gating, findings persistence option, chatter scoping, JSON document
  builder, runner group, permanent-pin migrations.
  Production files: `src/main.rs`, `src/cli/spec.rs`,
  `src/spec/retrieval.rs`, `run-tests.sh`, `tests/steps/mod.rs`,
  `tests/steps/*_steps.rs` (see commit). Targeted output: `7 steps (7 passed)`.
- [x] REFACTOR: fmt + clippy clean; reran targeted: `7 steps (7 passed)`.
- [x] COMMIT: hash: `7a63b7e`

## Scenario: The raw E5 list is verbose-only detail

- [x] RED: unimplemented verbose/hide bindings failed. Exit 101.
- [x] GREEN: corroborated default hides raw E5 detail; evaluated-pairs
  count kept; verbose restores established candidate lines.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/semantic_review_output_steps.rs`.
  Targeted output: `7 steps (7 passed)`.
- [x] REFACTOR: clean; reran targeted: `7 steps (7 passed)`.
- [x] COMMIT: hash: `dea4b7e`

## Scenario: E5-only hits without a recommendation are not problems

- [x] RED: unimplemented Given/Then failed. Exit 101.
- [x] GREEN: all-filtered fixture via directional overrides for both
  stages; corroborated verdict passes with zero recommendations.
  Production files: none (behaviour already implemented by S1 commit);
  Test files: `semantic_review_output_steps.rs`.
  Targeted output: `4 steps (4 passed)`.
- [x] REFACTOR: clean; reran targeted: `4 steps (4 passed)`.
- [x] COMMIT: hash: `c8fe1eb`

## Scenario: A recommended pair from either stage is the problem signal

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: union message `semantic review flagged 3 recommended
  candidate(s)` on stderr naming findings path; file exists.
  Production files: none beyond S1. Test files: regular steps file.
  Targeted output: `6 steps (6 passed)`.
- [x] REFACTOR: clean; reran targeted: `6 steps (6 passed)`.
- [x] COMMIT: hash: `4e7c520`

## Scenario: Without participating second-stage layers the E5 threshold stays binding

- [x] RED: stub bindings failed. Exit 101.
- [x] GREEN: disabled backends keep today's byte-identical output and
  threshold failure message; fixed gate-only verdict to use gate
  candidates. Production files: `src/spec/retrieval.rs`. Test files:
  regular steps file. Targeted output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `6b515fb`

## Scenario: Recommended pairs are ordered by model confidence before embedding score

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: inverted-confidence fixture asserts NLI entailment-desc and
  BGE symmetric-raw-desc ordering, not embedding order.
  Production files: `src/spec/retrieval.rs` (recommended-row sorting).
  Test files: regular steps file. Targeted output: `6 steps (6 passed)`.
- [x] REFACTOR: clippy fix commit `f9b356a`; reran targeted: `6 steps (6 passed)`.
- [x] COMMIT: hash: `52f50e9`

## Scenario: Retained text lines keep their established wording

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: byte-exact prefixes for NLI/BGE status, accounting,
  distribution, and pair-detail lines incl. advisory disclaimers.
  Production files: none. Test files: regular steps file.
  Targeted output: `7 steps (7 passed)` (fragment fix in `ea28a97`).
- [x] REFACTOR: clean; reran targeted: `7 steps (7 passed)`.
- [x] COMMIT: hash: `b715b4c`

## Scenario: Verbose output lists every scored second-stage pair

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: verbose lists recommended first (confidence order), then
  remaining rows embedding-order; caps visible; details+unscored=raw.
  Production files: `src/spec/retrieval.rs`. Test files: regular steps.
  Targeted output: `6 steps (6 passed)` (assertion tighten-up `4659245`).
- [x] REFACTOR: clean; reran targeted: `6 steps (6 passed)`.
- [x] COMMIT: hash: `c3a84e1`

## Scenario: Default output collapses artifact resolution chatter to one line per stage

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: resolution-notice seam plus per-stage dedupe summary on
  stderr; resolving-lines suppressed by default. Production files:
  `src/spec/retrieval.rs` (seam + gating helpers). Test files: regular
  steps. Targeted output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `784fa84`

## Scenario: Verbose output restores full artifact resolution detail

- [x] RED: stub Then failed. Exit 101.
- [x] GREEN: verbose prints both repeated resolution lines per stage via
  the same gated helpers. Production files: none. Test files: regular
  steps. Targeted output: `4 steps (4 passed)`.
- [x] REFACTOR: clean; reran targeted: `4 steps (4 passed)`.
- [x] COMMIT: hash: `4f35197`

## Scenario: The truncation disclaimer appears only for a truncated listing

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: disclaimer absent in default and verbose runs.
  Production files: none (disclaimer made conditional in S1 commit).
  Test files: regular steps. Targeted output: `6 steps (6 passed)`.
- [x] REFACTOR: clean; reran targeted: `6 steps (6 passed)`.
- [x] COMMIT: hash: `047ac1b`

## Scenario: A deprecated report-limit setting is accepted silently without effect

- [x] RED: stub steps failed. Exit 101.
- [x] GREEN: variable parsed but ignored; runtime/cache-normalized
  outputs identical; nothing names the variable. Production files: none.
  Test files: regular steps. Targeted output: `7 steps (7 passed)`
  (normalization fix `8bf2f43`).
- [x] REFACTOR: clean; reran targeted: `7 steps (7 passed)`.
- [x] COMMIT: hash: `1ec2577`

## Scenario: Structured JSON output is one complete review document

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: single pretty JSON document with schema/source/change/
  index/verdict/stage runs/candidates/combined; banners embedded;
  stdout pure. Production files: `src/spec/retrieval.rs` (document now
  rendered from the second-stage pool for identity consistency).
  Test files: regular steps. Targeted output: `11 steps (11 passed)`.
- [x] REFACTOR: clean; reran targeted: `11 steps (11 passed)`.
- [x] COMMIT: hash: `f3f5161`

## Scenario: JSON counters match the text counters of the same run

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: counter parity for both stages plus e5_candidates;
  identities/admissions agree between formats.
  Production files: none. Test files: regular steps.
  Targeted output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `75a7d09`

## Scenario: Structured JSON preserves exit codes and failure messages

- [x] RED: stub steps failed. Exit 101.
- [x] GREEN: identical non-zero exits and error-line equality across
  formats. Production files: none. Test files: regular steps.
  Targeted output: `5 steps (5 passed)` (error-line fix `9764daa`).
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `940b7f7`

## Scenario: Flagged recommendations persist their findings beside the change

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: findings written, schema-version-1, matches emitted
  document sections, stale sentinel replaced. Production files: none.
  Test files: regular steps. Targeted output: `7 steps (7 passed)`.
- [x] REFACTOR: clean; reran targeted: `7 steps (7 passed)`.
- [x] COMMIT: hash: `3a06372`

## Scenario: Successful reviews write no findings document

- [x] RED: stub Given/Then failed. Exit 101.
- [x] GREEN: clean review leaves no findings artifact.
  Production files: none. Test files: regular steps (alias Given added
  in `3d7920a`). Targeted output: `4 steps (4 passed)`.
- [x] REFACTOR: clean; reran targeted: `4 steps (4 passed)`.
- [x] COMMIT: hash: `651d138`

## Scenario: An unwritable findings location warns without masking the flagged failure

- [x] RED: stub steps failed. Exit 101.
- [x] GREEN: blocking directory at findings path yields warning naming
  path; flagged failure unchanged. Production files: none. Test files:
  regular steps. Targeted output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `0e750ff`

## E2E setup

- [x] Local runnability confirmed: deterministic stub backends only;
  compiled-binary subprocess harness (`common::run_givn_with_envs`);
  project verify hooks neutralised via `givn/commands.yaml` override;
  overlap dispositions pre-written.
- [x] Tag filter proven: full-suite counts above vs `6 scenarios`
  for `--tags '@review-output and @e2e'` — strictly smaller.

## Scenario: Real CLI review surfaces only recommended second-stage pairs

- [x] RED: stub Then failed. Exit 101.
- [x] GREEN: compiled binary run; recommended detail present, FILTERED
  absent. Files touched: e2e steps file. Output: `5 steps (5 passed)`
  (stub-silencing fixes `4f66f79`,`538b6d1`).
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `17fa9ca`

## Scenario: Real CLI verbose review lists all second-stage evidence

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: FILTERED rows present under verbose; no disclaimer.
  Files touched: e2e steps file. Output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `27a5183`

## Scenario: Real CLI JSON review emits one structured document

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: pure JSON stdout parses at schema version 1; counters equal
  an equivalent text run. Files touched: e2e steps file.
  Output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `e38a2ee`

## Scenario: Real CLI persists findings beside the change after flagged recommendations

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: findings file exists beside the change and matches the
  emitted document sections. Files touched: e2e steps file (borrow fix
  `7a96974`). Output: `5 steps (5 passed)`.
- [x] REFACTOR: clean; reran targeted: `5 steps (5 passed)`.
- [x] COMMIT: hash: `5a6bf60`

## Scenario: Real CLI blocking review flags recommendations through the workflow gate

- [x] RED: stub Thens failed. Exit 101.
- [x] GREEN: compiled-binary `check review` on a union-flagged corpus
  exits non-zero; semantic-review.md exists; no findings file written by
  this surface. Required projected+source endpoint overrides for both
  surfaces and a distinct delta body so report candidates exist.
  Files touched: e2e steps file. Output: `6 steps (6 passed)`
  (debug-instrumentation removal `8aee752`).
- [x] REFACTOR: clean; reran targeted: `6 steps (6 passed)`.
- [x] COMMIT: hash: `e351cbd`
