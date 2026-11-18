# Tasks: ise-component-tier-gate

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` in
`tests/features_runner.rs`; an unbound or skipped step fails the run. This
change adds no runner or framework.

Release note (subject of the behaviour batch, from `proposal.md`):

```text
A whole-system Ise trial now carries a runnable verification contract and cannot be reported green without executing exactly its declared component scenarios, while `givn ise compare` reports frozen-input equality only.
```

**Commit rule.** One atomic commit for the coherent behaviour batch. Every
scenario of the batch records the same revision as
`COMMIT: <hash> -- Batch: component-tier-gate`. A checked line with a missing or
unreachable revision fails review and archive.

## Batch map

| Batch | Scenarios | Commit | Purpose |
|---|---|---:|---|
| setup | — | `ed411f57664b96b5bd33e4355b62c97b001ca5ff` | Confirm the existing runner, strict mode, and permanent step tree. |
| component-tier-gate | all 7 delta scenarios | `ed411f57664b96b5bd33e4355b62c97b001ca5ff` | Contract generation, read-only comparison label, exact-count trial gate, coverage, and not-run refusal. |
| closeout | — | `ed411f57664b96b5bd33e4355b62c97b001ca5ff` | Merge every delta phrase into the permanent step tree and record the acceptance test. |

## Setup

- [x] Runner configured: `./run-tests.sh --non-e2e`, strict mode
  `.fail_on_skipped()`; no new runner or step file is created.
  - [x] COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: setup
  - Evidence: `tests/features_runner.rs` calls `.fail_on_skipped()`; the
    `./run-tests.sh --non-e2e` run in the evidence section exited 0 with 1126
    passed and 115 skipped, and the delta run exited 0 with 7 passed.

## component-tier-gate

- [x] A whole-system rebuild writes the runnable verification contract.
  - Scenario: Preparation writes a runnable verification contract
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 7 passed; the copy holds `givn/ise-verification.yaml` with 10 component and 1 End-to-end unit.

- [x] Frozen-input comparison is labelled input equality only.
  - Scenario: Comparison labels frozen-input equality as unverified
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: the scenario prints `Trial result: unverified`, never a pass line; the copied `scripts/verify.sh` is not executed by `compare`.

- [x] A missing component runner refuses and records zero executed units.
  - Scenario: Verification refuses a missing component runner
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: `component_executed=0/10`, `clean: false`, no `Trial result: passed` line.

- [x] Exact declared counts pass.
  - Scenario: Verification records the exact declared counts
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: `Component scenarios: 10/10 executed, 0 failed, 0 skipped`, `End-to-end tests: 1/1 executed`, `Trial result: passed`, `clean: true`.

- [x] A count mismatch refuses.
  - Scenario: Verification refuses an exact count mismatch
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: 9 executed against 10 declared exits 1 with `component_executed=9/10` and `clean: false`.

- [x] A not-run decision cannot replace a declared corpus.
  - Scenario: A not-run decision cannot replace declared component units
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: `--component-not-run "out of scope"` exits 1 naming that it cannot replace their execution; `clean: false`.

- [x] The served guidance names the gate and the input-only comparison.
  - Scenario: Served ise guidance distinguishes the trial gate from input equality
  - Batch: component-tier-gate
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate
  - Evidence: the delta guidance scenario passes; `givn instructions ise` names `givn ise verify-trial`, `Trial result: unverified`, exact counts, coverage, and the not-run rule.

- [x] Adversarial result-path checks.
  - Evidence: `tests/ise_component_tier_gate.rs` — 8 passed: missing runner, exact counts, mismatch, not-run, read-only compare, reserved contract path, `../copy/trial.json` escape, and a broken symlink result path.
  - COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: component-tier-gate

## Closeout

- [x] Every delta phrase is bound in the permanent test tree.
  - [x] COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: closeout
  - Evidence: `tests/steps/trial_input_verification_steps.rs` binds the contract, preparation, trial-gate, count, not-run, and unverified steps; `tests/steps/prose_consolidation_steps.rs` binds the guidance steps; `tests/steps/ise_clean_room_support.rs` carries the fixture. No delta step copy exists.
- [x] The acceptance test is real and runnable.
  - [x] COMMIT: `ed411f57664b96b5bd33e4355b62c97b001ca5ff` -- Batch: closeout
  - Evidence: `cargo test --test ise_component_tier_gate -- --test-threads=1` — 8 passed.

## Verification evidence

```text
$ cargo test --test ise_component_tier_gate -- --test-threads=1
running 8 tests ... test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e
Summary [ 1.557s] 7 tests run: 7 passed, 0 skipped

$ cargo test --lib ise::
test result: ok. 18 passed; 0 failed; 0 ignored

$ ./run-tests.sh --non-e2e
Summary [ 214.612s] 1126 tests run: 1126 passed, 115 skipped

$ GIVN_FEATURES=givn/specs/prepare-an-ise-test ./run-tests.sh --e2e --package e2e
2 features, 2 scenarios (2 passed), 16 steps (16 passed)

$ ./run-tests.sh --e2e --package e2e
56 features, 120 scenarios (120 passed), 785 steps (785 passed)

$ givn lint --change ise-component-tier-gate
givn lint: 3 file(s) checked — clean
```
