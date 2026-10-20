# Design: change-verification-receipt

## Problem

`givn check tasks` runs the change suite but writes no durable result.
`givn check review` does not run tests and cannot prove the delta is green.
`givn archive` merges before the first integration run and re-runs the change
suite on every retry. The runner-result schema was discovered field by field in
a real trial.

## Decisions

- **Decides D1 (proposal): `check tasks` requires a clean committed tree except
  evidence/verification paths.** It resolves the change, rejects dirty paths
  outside `proof::is_evidence_path`, runs the configured verify command with
  `GIVN_FEATURES` and `GIVN_RESULT_FILE`, validates the counts, and writes
  `givn/changes/<id>/verification/tasks-verification.json`. Realized in
  `src/cli/check.rs` (`run_change_verification`).
- **Decides D2 (proposal): `scope` is optional.** `RunnerResult` stores
  `scope: Option<String>`; archive validates it only when present and labels
  the invocation itself. New runners write four counts. Realized in
  `src/proof.rs` (`RunnerResult::read`, `read_counts`).
- **Decides D3 (proposal): the merged suite still runs once.** Archive
  validates the change receipt before the move, merges, publishes the
  candidate, and runs only the regular/e2e scopes against `givn/specs`.
  Realized in `src/cli/archive.rs` (`ensure_change_receipt`, existing scopes).
- **D4: the receipt binds the tested revision, delta features, and verify
  command.** `ChangeReceipt` stores `head`, `features_digest`,
  `command_digest`, and the green counts. A receipt is current only when all
  three match and the counts are green. Feature digest is SHA-256 over sorted
  relative feature paths and content. Realized in `src/proof.rs`
  (`ChangeReceipt`, `digest_features`, `digest_text`).
- **D5: the result helper is the discoverable contract.** `givn test-result`
  requires `GIVN_RESULT_FILE`, validates reconciliation, and atomically writes
  `{total, passed, failed, skipped}` with no scope. `givn init` writes a
  fail-closed `run-tests.sh` when absent; the commands template points
  `verify.command` at it. Realized in `src/cli/test_result.rs`,
  `src/main.rs`, `src/cli/init.rs`, and `assets/commands.yaml.tmpl`.
- **D6: review refuses a missing or stale receipt.** After integrity, run
  declaration, overlap, retirement, and routing checks, review reads the
  receipt and compares it with the current revision, features, and command.
  It runs no tests. Realized in `src/cli/check.rs` (`check_change_receipt`).

D1's rejected alternative is a dirty-tree digest: it adds a second provenance
model and forces archive to reason about uncommitted code. D3's rejected
alternative is trusting the change run as integration proof: a shared-step
conflict is only visible in the merged suite.

## Architecture impact

| File | Change |
|---|---|
| `src/cli/test_result.rs` | New helper; atomic count writer. |
| `src/main.rs`, `src/cli/mod.rs` | `test-result` subcommand and dispatch. |
| `src/proof.rs` | Optional scope; `ChangeReceipt`; digests; atomic receipt writer. |
| `src/cli/check.rs` | `check tasks` receipt run; `check review` receipt validation. |
| `src/cli/archive.rs` | Receipt validation before a fresh archive. |
| `src/cli/init.rs` | Fail-closed `run-tests.sh` scaffold. |
| `tests/steps/workflow_enforcement_steps.rs` | New review/check-tasks/result-helper steps. |
| `tests/steps/archive_proof_steps.rs` | Stale-receipt and merged-suite-only steps. |
| `tests/steps/archive_proof_support.rs` | Fixture receipt writer; configured command parser. |

No new dependency. The receipt lives under the change's `verification/`
directory, which is already an evidence path.

## Receipt schema

```json
{
  "schema": 1,
  "change": "verify-demo",
  "status": "passed",
  "head": "<tested revision>",
  "features_path": "givn/changes/verify-demo/specs",
  "features_digest": "<sha256>",
  "command_digest": "<sha256>",
  "command": "./run-tests.sh",
  "runner_exit": 0,
  "result": {"total": 2, "passed": 2, "failed": 0, "skipped": 0},
  "finished_at_unix": 0
}
```

`result.scope` is absent. Staleness is a mismatch in `head`,
`features_digest`, or `command_digest`, or a non-green result.

## Test design

Delta specs:

```text
givn/changes/change-verification-receipt/specs/verify-change/givn-workflow-enforcement.feature
givn/changes/change-verification-receipt/specs/archive-change/archive-proof.feature
```

Step definitions:

| File | Binds |
|---|---|
| `tests/steps/workflow_enforcement_steps.rs` | Clean change fixture, receipt presence/staleness, result helper, review assertions. |
| `tests/steps/workflow_enforcement_e2e_steps.rs` | Existing CLI invocation path; reused for the check-tasks E2E scenario. |
| `tests/steps/archive_proof_steps.rs` | Stale-receipt archive refusal and merged-suite-only invocation. |
| `tests/steps/archive_proof_support.rs` | Current receipt fixture, configured command parser, invocation log. |

Runner: `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e`. Single scenario:
`GIVN_FEATURES=givn/changes/change-verification-receipt/specs/verify-change cargo nextest run --test features_runner -E 'test(<title>)'`.
Strict mode: `.fail_on_skipped()`. No new `@e2e` action beyond check tasks.

## Interaction coverage matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run the change-scoped checks | Consumer runs check tasks and receives a change verification receipt | CLI | compiled `givn` subprocess |
| write a runner result | (non-E2E; the helper is exercised inside the check-tasks receipt scenario and its unit tests) | CLI | compiled `givn` subprocess |
| archive a completed change | Consumer archives a completed change and receives a proven archive | CLI | existing archive E2E driver |
| repair a blocked archive | Consumer repairs a blocked archive and the rerun proves it | CLI | existing archive E2E driver |
| inspect an unproven archive candidate | Consumer inspects an unproven archive candidate | CLI | existing archive E2E driver |

The full inventory remains 11 entries after adding the two workflow rows; the
archive rows above are the entries this change touches.

## E2E infrastructure

`./run-tests.sh --e2e` runs `@e2e`; the step file is
`tests/steps/workflow_enforcement_e2e_steps.rs`; the real interface is the
compiled CLI in a temporary project; no external service. Strict mode is
`.fail_on_skipped()`.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable

The change alters CLI commands and receipt files; no rendered screen and no
delta `@e2e` transcript.

## Coverage process boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Non-e2e feature runner | `measure-coverage.sh` (`not @e2e`) | `features_runner` and spawned `givn` | per-test profraw | report to `coverage/non-e2e-cobertura.xml` | check tasks/review scenarios |
| E2E feature runner | `measure-coverage.sh` (`@e2e`) | the same binaries | separate profraw | report to `coverage/e2e-cobertura.xml` | check-tasks E2E |
| Ready fixture | `measure-coverage.sh` (`cov_ready`) | `ready_fixture` | discarded | not merged | none |
