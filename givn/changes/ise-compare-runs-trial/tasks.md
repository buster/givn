# Tasks: ise-compare-runs-trial

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` plus a non-zero exit
on failed or skipped stats in `tests/features_runner.rs` and
`e2e/tests/e2e_runner.rs`. This change adds no runner or framework.

Release note (subject of the behaviour batch, from `proposal.md`):

```text
`givn ise compare` now runs the trial when the frozen inputs match, refuses a missing component runner, and keeps a read-only `--inputs-only` inspection path.
```

**Commit rule.** One atomic commit for the coherent behaviour batch. Every
scenario of the batch records the same revision as
`COMMIT: <hash> -- Batch: compare-runs-trial`. A checked line with a missing or
unreachable revision fails review and archive.

## Batch map

| Batch | Scenarios and checks | Commit | Purpose |
|---|---|---:|---|
| setup | — | `eaefc0bf20f7be001d206a200ebd154973b40ef3` | Confirm the existing runner and strict mode. |
| compare-runs-trial | all delta scenarios and safety checks | `eaefc0bf20f7be001d206a200ebd154973b40ef3` | Compare executes the contract, inputs-only stays read-only, result paths are safe, and the guidance names the component workflow. |

## Setup

- [x] Runner configured: `./run-tests.sh --non-e2e` and
  `./run-tests.sh --e2e --package e2e`; strict mode `.fail_on_skipped()`; no
  new runner or step file is created.
  - [x] COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: setup
  - Evidence: an undefined-step probe exited 1 with `Step doesn't match any
    function`:
    ```text
    Feature: strict probe
      Scenario: undefined step
       ✘  Given a step that was never bound
          Step doesn't match any function
    1 feature
    1 scenario (1 failed)
    exit status: 1
    ```

## compare-runs-trial

- [x] `trial-input-verification`: Verifier compares the spec copy with the
  manifest and runs the trial.
  - Scenario: Verifier compares the spec copy with the manifest and runs the trial
  - Batch: compare-runs-trial
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `GIVN_FEATURES=givn/changes/ise-compare-runs-trial/specs
    ./run-tests.sh --e2e --package e2e` — 1 passed, 6 steps; the transcript is
    `evidence/visual/verifier-compares-the-spec-copy-with-the-manifest-and-runs-the-trial/transcript.txt`.
    The fixture's component runner parses the frozen feature and executes the
    `Given nothing` step for the non-`@e2e` scenario.

- [x] `trial-input-verification`: Comparison labels frozen-input equality as
  unverified.
  - Scenario: Comparison labels frozen-input equality as unverified
  - Batch: compare-runs-trial
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `--inputs-only` prints `Trial result: unverified (inputs only; no
    trial command was executed)`, writes no result, and the prior result test
    confirms it touches no file.

- [x] `trial-input-verification`: Comparison refuses a missing component runner.
  - Scenario: Comparison refuses a missing component runner
  - Batch: compare-runs-trial
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: the default compare exits 1 with `component_executed=0/10`,
    records `clean: false`, and prints no pass line; the Passuff-shaped
    `0/2` variant passes the same checks.

- [x] `trial-input-verification`: Comparison refuses a trial result inside the
  spec copy.
  - Scenario: Comparison refuses a trial result inside the spec copy
  - Batch: compare-runs-trial
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `--result <copy>/trial-result.json` exits 1 naming `inside the
    spec copy`; no pass line.

- [x] `serve-extended-rules`: Served ise guidance distinguishes the trial gate
  from input equality.
  - Scenario: Served ise guidance distinguishes the trial gate from input equality
  - Batch: compare-runs-trial
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: the guidance names compare as the normal trial command,
    verify-trial as the explicit executor, and `--inputs-only` as unverified.

- [x] `serve-extended-rules`: Served ise guidance requires the ordinary
  component workflow.
  - Scenario: Served ise guidance requires the ordinary component workflow
  - Batch: compare-runs-trial
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: the guidance requires the component runner, steps, and fakes,
    executes the frozen component feature files, records per-tier coverage and
    an uninstrumented End-to-end report, and refuses coverage as a substitute
    for execution.

## Result safety

- [x] Refuse a result path equal to the manifest and validate the result path
  before the comparison, including on an input difference.
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `a_result_path_equal_to_the_manifest_refuses_without_deleting_it`
    and `verify_trial_refuses_a_result_equal_to_the_manifest_without_deleting_it`
    pass; the manifest bytes survive.

- [x] Remove only a regular Givn-owned trial result and regular Givn runner
  siblings; refuse a directory, symlink, malformed file, or unrelated file.
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `an_existing_non_givn_result_is_never_deleted`,
    `a_non_givn_raw_sibling_is_never_deleted`, and
    `a_symlink_result_path_refuses` pass; each unrelated file survives.

- [x] Refuse a result inside the checkout and a default result beside a manifest
  inside the checkout; keep the previous Givn result revocable on an input
  difference.
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `a_result_path_inside_the_checkout_refuses`,
    `a_default_result_inside_the_checkout_refuses`, and
    `a_previous_givn_result_is_revoked_on_an_input_difference` pass.

- [x] Cover the safety cases in `tests/ise_component_tier_gate.rs`.
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `cargo test --locked --test ise_component_tier_gate --
    --test-threads=1` — 27 passed.

## Passuff-shaped integration

- [x] Passuff-shaped copy runs the trial through compare by default; assert the
  two-binary product binding, `2/2` component and `1/1` End-to-end counts, and
  `clean: true`.
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: `cargo test --locked --test ise_passuff_trial --
    --test-threads=1` — 4 passed; the clean case records the two-binary product
    digest and accepts the declared subprocess End-to-end report.

- [x] Passuff-shaped copy without a component runner refuses through compare;
  assert `component_executed=0/2` and `clean: false`.
  - COMMIT: `eaefc0bf20f7be001d206a200ebd154973b40ef3` -- Batch: compare-runs-trial
  - Evidence: the `0/2` variant exits 1, records `clean: false`, and prints no
    pass line. This fixture proves the Givn gate; the current Passuff consumer
    still needs its own runner, result writing, and `product_path`.

## Verification

- `./run-tests.sh --non-e2e`: 1135 passed, 115 skipped.
- `GIVN_FEATURES=givn/changes/ise-compare-runs-trial/specs
  ./run-tests.sh --non-e2e`: 5 passed, 1 skipped.
- `GIVN_FEATURES=givn/changes/ise-compare-runs-trial/specs
  ./run-tests.sh --e2e --package e2e`: 1 passed.
- `cargo fmt --check` both workspaces and
  `cargo clippy --workspace --all-targets -- -D warnings`: clean.
