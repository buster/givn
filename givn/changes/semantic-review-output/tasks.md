# Tasks: semantic-review-output

Coverage pre-check (mandatory): the four User Interaction Inventory
entries map 1:1 to the four `@givn.added @e2e` scenarios and to the four
Interaction Coverage Matrix rows in `design.md` (titles corrected during
design-review F6). Driving mechanism for every row: compiled-CLI
subprocess. No gaps.

Runner: `./run-tests.sh` · E2E: `./run-tests.sh --e2e`
Feature scope for this change:
`--features spec-retrieval,cross-encoder-reranker,nli-evidence`

## Setup

- [ ] Create step skeletons `tests/steps/semantic_review_output_steps.rs`
  and `tests/steps/semantic_review_output_e2e_steps.rs` (all bodies
  `unimplemented!()`), register both in `tests/steps/mod.rs`.
- [ ] Re-prove strict mode with one stubbed step and paste command +
  non-zero output below.
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`
    (`tests/features_runner.rs:20`)
  - Proof command:
    `GIVN_FEATURES=givn/changes/semantic-review-output/specs cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate --test features_runner -- --name "Default output reports only recommended second-stage pairs"`
  - Proof output: <paste>
- [ ] Audit sweep (design.md characterization list): grep all
  `tests/steps/*.rs` for assertions on presence of `*_FILTERED`,
  `*_UNSCORED`, or token-cap detail in advisory-review stdout; record
  every hit that the default flip will invert. Findings: <paste>
- [ ] Add the `@review-output` runner group to `run-tests.sh`
  (`run_review_output`, features incl. `spec-similarity-gate`;
  existing nli groups exclude the tag) and prove both commands:
  full-suite count vs e2e count recorded below.
  Counts: <paste>
- [ ] Named permanent-pin migration task (executed within the flipping
  scenario's commit): rewrite permanent scenario language and step
  bindings at `tests/steps/nli_evidence_steps.rs:160-168` ("combined"
  intersection wording + BGE_FILTERED detail pin),
  `tests/steps/bge_reranking_evidence_steps.rs:999`,
  `tests/steps/nli_evidence_e2e_steps.rs:253`. Evidence: <paste>

## Scenario: Default output reports only recommended second-stage pairs

- [ ] RED: remove this scenario's `@wip`; run targeted with stubs. Output: <paste>
- [ ] GREEN: visibility filter on NLI detail loop
  (`print_nli_results`) and BGE per-pair loop — default prints only
  `*_RECOMMENDED` rows; token-cap guidance lines stay visible;
  status/accounting/distribution lines unchanged. Production files:
  <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: fmt/clippy clean; rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: The E5 candidate list remains the default gate evidence

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: assert existing candidate-line format and gate-failure
  message survive the reporting refactor. Production files: <list>.
  Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Recommended pairs are ordered by model confidence before embedding score

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: sort recommended rows per stage by model confidence (NLI
  symmetric entailment desc; BGE symmetric raw desc) before printing.
  Production files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Retained text lines keep their established wording

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: byte-exact assertions on stage status line, accounting
  line, distribution line, pair detail format. Production files:
  <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Verbose output lists every scored second-stage pair

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: plumb `--verbose` through `SpecCommands::Review` →
  `cli::spec::run_review` → `retrieval::review`; verbose prints
  filtered/unscored rows after recommended rows in embedding-score
  order, no listing limit. Production files: <list>. Test files:
  <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Default output collapses artifact resolution chatter to one line per stage

- [ ] RED: remove `@wip`; targeted run (resolution-notice stub seam
  active). Output: <paste>
- [ ] GREEN: hoist repeated resolution notices behind `verbose`; default
  emits one stderr summary line per stage; add test-only seam that makes
  stub backends emit repeated notices deterministically. Production
  files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Verbose output restores full artifact resolution detail

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: `--verbose` restores the full repeated resolution lines on
  stderr. Production files: <list>. Test files: <list>. Targeted
  output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: The truncation disclaimer appears only for a truncated listing

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: disclaimer prints only when a listing was actually
  truncated; default and verbose runs print none. Production files:
  <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: A deprecated report-limit setting is accepted silently without effect

- [ ] RED: remove `@wip`; targeted run with variable set. Output: <paste>
- [ ] GREEN: `GIVN_SPEC_NLI_REPORT_LIMIT` parsed but ignored; comparison
  bindings normalize runtime values before comparing runs. Production
  files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Structured JSON output is one complete review document

- [ ] RED: remove `--format json` scenario's `@wip`; targeted run. Output: <paste>
- [ ] GREEN: JSON serializer extending the duplicate-audit contract
  (`schema_version`, `source`, `change`, `index`, `verdict`, candidates
  with admission/evidence); banner content embedded as `index` section,
  stdout pure. Production files: <list>. Test files: <list>. Targeted
  output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: JSON counters match the text counters of the same run

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: parity binding compares text counters against JSON fields
  across two runs (runtime-normalized), same verdict and admissions.
  Production files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Structured JSON preserves exit codes and failure messages

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: identical exit code and stderr failure message in text and
  JSON modes on a failing corpus. Production files: <list>. Test files:
  <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: A failed similarity gate persists its findings beside the change

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: on gate failure write the document to
  `givn/changes/<id>/review-findings.json` (overwrite prior), failure
  message names the path, persisted bytes match stdout document.
  Production files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Successful reviews write no findings document

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: passing gate writes nothing under the change directory.
  Production files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: An unwritable findings location warns without masking the gate failure

- [ ] RED: remove `@wip`; targeted run. Output: <paste>
- [ ] GREEN: unwritable findings path degrades to a stderr warning
  naming the path; gate failure message and exit code unchanged.
  Production files: <list>. Test files: <list>. Targeted output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## E2E setup

- [ ] Confirm local runnability (design.md): deterministic stub backends
  only; compiled-CLI subprocess harness reused from
  `tests/steps/semantic_review_e2e_steps.rs`. Startup proof: <paste>
- [ ] Prove the tag filter is real: run `./run-tests.sh` and
  `./run-tests.sh --e2e`, record both scenario counts; e2e count must be
  strictly smaller. Counts: <paste>

## Scenario: Real CLI review surfaces only recommended second-stage pairs

- [ ] RED: remove `@wip`; e2e targeted run. Output: <paste>
- [ ] GREEN: compiled-binary subprocess runs
  `givn spec review --change running-change`; assert recommended detail
  present, FILTERED detail absent on stdout. Files touched: <list>.
  Output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Real CLI verbose review lists all second-stage evidence

- [ ] RED: remove `@wip`; e2e targeted run. Output: <paste>
- [ ] GREEN: same harness with `--verbose`; FILTERED rows present, no
  truncation disclaimer. Files touched: <list>. Output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Real CLI JSON review emits one structured document

- [ ] RED: remove `@wip`; e2e targeted run. Output: <paste>
- [ ] GREEN: same harness with `--format json`; stdout parses as
  schema-version-1 document; counters equal an equivalent text run.
  Files touched: <list>. Output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Real CLI persists findings beside the change after a failed gate

- [ ] RED: remove `@wip`; e2e targeted run. Output: <paste>
- [ ] GREEN: failing corpus; `review-findings.json` exists beside the
  change and matches stdout document. Files touched: <list>. Output:
  <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>

## Scenario: Real CLI blocking review flags recommendations through the workflow gate

- [ ] RED: remove `@wip`; e2e targeted run. Output: <paste>
- [ ] GREEN: compiled-binary subprocess runs `givn check review --change
  running-change` on a union-flagged corpus; non-zero exit with flagged
  message; semantic-review.md exists; no findings file written by this
  surface. Files touched: <list>. Output: <paste>
- [ ] REFACTOR: rerun targeted. Output: <paste>
- [ ] COMMIT: hash: <hash>
