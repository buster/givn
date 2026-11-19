# Tasks: ise-component-tier-gate

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` in
`tests/features_runner.rs`; an unbound or skipped step fails the run. This
change adds no runner or framework.

Release note (subject of the behaviour batch, from `proposal.md`):

```text
A whole-system Ise trial now carries a runnable verification contract and cannot be reported green without result evidence for every declared scenario and the rebuilt product it drove, while `givn ise compare` reports frozen-input equality only.
```

**Commit rule.** One atomic commit for the coherent behaviour batch. Every
scenario of the batch records the same revision as
`COMMIT: <hash> -- Batch: component-tier-gate`. A checked line with a missing or
unreachable revision fails review and archive.

## Batch map

| Batch | Scenarios | Commit | Purpose |
|---|---|---:|---|
| setup | — | `bdc01ecb64c74682b62c06e0020b7d2052522a51` | Confirm the existing runner, strict mode, and permanent step tree. |
| component-tier-gate | all 9 delta scenarios | `bdc01ecb64c74682b62c06e0020b7d2052522a51` | Contract generation, read-only comparison label, exact-count trial gate, coverage, and not-run refusal. |
| closeout | — | `bdc01ecb64c74682b62c06e0020b7d2052522a51` | Merge every delta phrase into the permanent step tree and record the acceptance test. |

## Setup

- [x] Runner configured: `./run-tests.sh --non-e2e`, strict mode
  `.fail_on_skipped()`; no new runner or step file is created.
  - [x] COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: setup
  - Evidence: `tests/features_runner.rs` calls `.fail_on_skipped()`; the
    `./run-tests.sh --non-e2e` run in the evidence section exited 0 with 1126
    passed and 115 skipped, and the delta run exited 0 with 9 passed.

## component-tier-gate

- [x] A whole-system rebuild writes the runnable verification contract.
  - Scenario: Preparation writes a runnable verification contract
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 9 passed; the copy holds `givn/ise-verification.yaml` with 10 component and 1 End-to-end unit.

- [x] Frozen-input comparison is labelled input equality only.
  - Scenario: Comparison labels frozen-input equality as unverified
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: the scenario prints `Trial result: unverified`, never a pass line; the copied `scripts/verify.sh` is not executed by `compare`.

- [x] A missing component runner refuses and records zero executed units.
  - Scenario: Verification refuses a missing component runner
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: `component_executed=0/10`, `clean: false`, no `Trial result: passed` line.

- [x] Exact declared counts pass.
  - Scenario: Verification records the exact declared counts
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: `Component scenarios: 10/10 executed, 0 failed, 0 skipped`, `End-to-end tests: 1/1 executed`, `Trial result: passed`, `clean: true`.

- [x] A count mismatch refuses.
  - Scenario: Verification refuses an exact count mismatch
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: 9 executed against 10 declared exits 1 with `component_executed=9/10` and `clean: false`.

- [x] A not-run decision cannot replace a declared corpus.
  - Scenario: A not-run decision cannot replace declared component units
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: `--component-not-run "out of scope"` exits 1 naming that it cannot replace their execution; `clean: false`.

- [x] A count-only component result refuses.
  - Scenario: Verification refuses a count-only component result
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: a 10/10 count with no scenario titles exits 1 naming the missing titles; `clean: false`.

- [x] An End-to-end result bound to another product refuses.
  - Scenario: Verification refuses an End-to-end result bound to another product
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: a result naming `/bin/true` and its digest exits 1 naming `product_sha256`; `clean: false`.

- [x] The served guidance names the gate and the input-only comparison.
  - Scenario: Served ise guidance distinguishes the trial gate from input equality
  - Batch: component-tier-gate
  - COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: component-tier-gate
  - Evidence: the delta guidance scenario passes; `givn instructions ise` names `givn ise verify-trial`, `Trial result: unverified`, exact counts, coverage, and the not-run rule.

- [x] Adversarial result-path checks.
  - Evidence: `tests/ise_component_tier_gate.rs` — 12 passed: missing runner, exact counts, mismatch, not-run, read-only compare, reserved contract path, `../copy/trial.json` escape, a broken symlink result path, a foreign-product End-to-end result, a product path outside the copy, a product parent symlink outside the copy, and a contract change during the scripts.
  - COMMIT: `c58e61d60117a41ed624368c7ee8850f8de048f6` -- Batch: component-tier-gate-hardening

## Closeout

- [x] Every delta phrase is bound in the permanent test tree.
  - [x] COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: closeout
  - Evidence: `tests/steps/trial_input_verification_steps.rs` binds the contract, preparation, trial-gate, count, not-run, and unverified steps; `tests/steps/prose_consolidation_steps.rs` binds the guidance steps; `tests/steps/ise_clean_room_support.rs` carries the fixture. No delta step copy exists.
- [x] The acceptance test is real and runnable.
  - [x] COMMIT: `bdc01ecb64c74682b62c06e0020b7d2052522a51` -- Batch: closeout
  - Evidence: `cargo test --test ise_component_tier_gate -- --test-threads=1` — 12 passed.
- [x] `verify-trial` re-derives the frozen corpus from the copy and refuses a contract that hides it.
  - [x] COMMIT: `7008911cd26f394bd88a47c9febae2aedfc3ff83` -- Batch: component-tier-gate-final
  - Evidence: `cargo test --test ise_component_tier_gate -- --test-threads=1` — 17 passed, including a repaired-checksum contract with `component.execution_units: 0` refusing with "does not describe the frozen corpus", a contract from another revision refusing, product mutation after binding refusing, and duplicate/outline titles surviving the running gate; `cargo test --lib ise::contract::tests` — 8 passed and `cargo test --lib ise::` — 25 passed, including `execution_units == scenarios.len()`.
- [x] A Passuff-shaped trial binds two rebuilt binaries and accepts a missing E2E coverage report.
  - [x] COMMIT: `94b1bb323ed9a599dded45cab218d8204a57dc58` -- Batch: component-tier-gate-hardening
  - Evidence: `cargo test --test ise_passuff_trial -- --test-threads=1` — 2 passed: the copy has no `crates/passuff-e2e`, `PASSUFF_BIN_DIR` holds `passuff` and `passuff-tui`, the E2E script executes both, the coverage script writes only the component report, and an E2E script using checkout binaries records a different digest and refuses.

## Verification evidence

```text
$ cargo test --test ise_component_tier_gate -- --test-threads=1
running 12 tests ... test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --test ise_passuff_trial -- --test-threads=1
running 2 tests ... test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e
Summary [ 1.504s] 9 tests run: 9 passed, 0 skipped

$ cargo test --lib ise::
test result: ok. 25 passed; 0 failed; 0 ignored

$ ./run-tests.sh --non-e2e
Summary [ 214.612s] 1126 tests run: 1126 passed, 115 skipped

$ GIVN_FEATURES=givn/specs/prepare-an-ise-test ./run-tests.sh --e2e --package e2e
2 features, 2 scenarios (2 passed), 16 steps (16 passed)

$ ./run-tests.sh --e2e --package e2e
56 features, 120 scenarios (120 passed), 785 steps (785 passed)

$ givn lint --change ise-component-tier-gate
givn lint: 3 file(s) checked — clean
```
