# Tasks — change-verification-receipt

Release note: Make a green change-scoped test run a durable, revision-bound fact
that review and archive can reuse without rerunning the change suite.

## Task 0 — Setup and runner contract

- [x] Verify the Gherkin runner, `.fail_on_skipped()`, and the single-scenario
  command; scaffold the receipt/result unit tests.
  - RED/GREEN/REFACTOR: `cargo check --tests`, `cargo test --lib` → passing.
  - [x] COMMIT: 3c1bd143dde0cb4c15a276ec8cf14f0323fc5b1c -- feat(givn-workflow-enforcement): durable change-scoped receipt

## Task 1 — Check tasks writes a receipt

- [x] **Scenario: Consumer runs check tasks and receives a change verification receipt**
  - Clean tree, `GIVN_FEATURES`, `GIVN_RESULT_FILE`, four green counts, receipt
    with head/features/command digests.
  - GREEN: `GIVN_FEATURES=givn/changes/change-verification-receipt/specs/verify-change cargo nextest run --test features_runner` → 5 passed.
  - [x] COMMIT: 58581be9a835a23fbe0ec1422ac10e1eb2583d86 -- test(givn-workflow-enforcement): bind the receipt

## Task 2 — Result helper

- [x] **Scenario: The result helper writes four counts without a scope**
  - GREEN: `givn test-result --total 3 --passed 3 --failed 0 --skipped 0` → counts without scope; unit tests pass.
  - [x] COMMIT: 60e2194d339ee0fa7fccaf40739e62aa22246373 -- test(givn-workflow-enforcement): result helper counts

## Task 3 — Review refuses a missing receipt

- [x] **Scenario: Review refuses a change without a verification receipt**
  - GREEN: scenario exits non-zero and routes to `givn check tasks`.
  - [x] COMMIT: d333e93bd0a9f8f892409d42cca75c2971147721 -- test(givn-workflow-enforcement): missing receipt

## Task 4 — Review refuses a stale receipt

- [x] **Scenario: Review refuses a receipt after the delta specs changed**
  - GREEN: scenario exits non-zero with the stale message; command-digest unit test passes.
  - [x] COMMIT: 9528ab12cd621904c8f2c60645055bcf5fa6636a -- test(givn-workflow-enforcement): stale receipt

## Task 5 — Review accepts a current receipt without running tests

- [x] **Scenario: Review accepts a current verification receipt**
  - GREEN: scenario exits 0 and prints the receipt pass line; no verify invocation.
  - [x] COMMIT: 77439b039d1e89e4e7b28351bbb9f1c4fa317a84 -- test(givn-workflow-enforcement): current receipt

## Task 6 — Archive refuses a stale receipt before merge

- [x] **Scenario: Archive refuses a stale change verification receipt before merging**
  - GREEN: scenario exits non-zero, names the stale receipt, leaves the active change and no candidate.
  - [x] COMMIT: b25c78e24b1fdb7bcb31fc586f03f169734974af -- docs(archive-proof): pre-merge receipt binding

## Task 7 — Archive runs only the merged suite

- [x] **Scenario: Archive reuses the change receipt and runs only the merged suite**
  - GREEN: archive exits 0; invocation log has `regular givn/specs` and no `givn/changes/`.
  - [x] COMMIT: b01751d359334c69dd28f1d4e555c99bd1ddaa10 -- docs(archive-proof): merged-suite integration run

## Task 8 — Full-suite and archive readiness

- [x] Run both scopes, lint, and prepare the change receipt.
  - `./run-tests.sh` → 695 passed; `./run-tests.sh --e2e` → 104 passed; `givn lint` clean.
  - [x] COMMIT: bdde0b331a3c567d5cf709a07b06de362e9ce1d3 -- docs(givn-workflow-enforcement): clean-tree ordering
