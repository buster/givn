# Tasks: ise-ship-component-tier

Runner: `./run-tests.sh` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` plus a non-zero exit
on failed or skipped stats in `tests/features_runner.rs`. This change adds no
runner and no framework.

Release note (subject of each behaviour batch, from `proposal.md`):

```text
An Ise change that ships rebuilt code now executes the retained component corpus before review and archive, and the served guidance names the whole-system working-tree ship and both verification tiers.
```

**Commit rule.** One atomic commit per behaviour batch. Every scenario in the
batch records the same revision as `COMMIT: <hash> -- Batch: <name>`. A checked
scenario with a missing or unreachable revision fails review and archive.

## Batch map

| Batch | Scenarios and checks | Commit | Purpose |
|---|---|---:|---|
| ship-gate | 5 `givn-workflow-enforcement` scenarios, fixture, predicate unit tests | `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` | The ship gate runs, refuses, and binds the component evidence. |
| ship-guidance | 5 `serve-extended-rules` scenarios and the five assets | `e784c7fc43f0371def88eead4d94563b5f3bca64` | The served guidance names the whole-system ship and both tiers. |
| docs | reference, glossary, arc42 chapters, ADR-070 amendment, skills | pending | The durable documentation matches the gate. |

## Setup

- [x] Runner and strict mode confirmed: `./run-tests.sh` executes the delta via
  `GIVN_FEATURES`; `tests/features_runner.rs` calls `.fail_on_skipped()` and
  exits non-zero on failed or skipped stats. No runner change is needed.
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: the first RED run failed on undefined steps
    (`Step doesn't match any function`) before the implementation landed.

## ship-gate

- [x] `givn-workflow-enforcement`: A whole-system Ise ship executes the
  retained component corpus.
  - Scenario: A whole-system Ise ship executes the retained component corpus
  - Batch: ship-gate
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: `givn check tasks --change ise-boundary` exits 0, prints
    `component_executed=2/2`, and the receipt records `givn/specs` with two
    scenarios.

- [x] `givn-workflow-enforcement`: An Ise ship with a delta also executes the
  retained component corpus.
  - Scenario: An Ise ship with a delta also executes the retained component corpus
  - Batch: ship-gate
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: `givn check tasks --change ise-boundary` exits 0; the receipt
    records one delta scenario and a `component` record for `givn/specs` with
    two scenarios.

- [x] `givn-workflow-enforcement`: A whole-system Ise ship without a runner
  result is refused.
  - Scenario: A whole-system Ise ship without a runner result is refused
  - Batch: ship-gate
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: the exit-0 runner that writes no result is refused with
    `component_executed=0/2`.

- [x] `givn-workflow-enforcement`: A whole-system Ise ship that executes fewer
  component scenarios is refused.
  - Scenario: A whole-system Ise ship that executes fewer component scenarios is refused
  - Batch: ship-gate
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: a result reporting one scenario is refused with
    `component_executed=1/2`.

- [x] `givn-workflow-enforcement`: Review refuses an Ise ship without a
  component receipt.
  - Scenario: Review refuses an Ise ship without a component receipt
  - Batch: ship-gate
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: `givn check review --change ise-boundary` reports
    `change verification receipt missing` instead of the not-applicable branch.

- [x] `proof` predicate unit tests: a ship requires evidence, a non-ship does
  not, and a zero-unit corpus does not.
  - Batch: ship-gate
  - [x] COMMIT: `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` -- Batch: ship-gate
  - Evidence: `cargo test --lib proof::tests` passes 24 tests, including the
    three new ship-predicate cases; `cargo test --lib ise::ship` passes 6
    reader cases; `./run-tests.sh --non-e2e` passes 1138 tests.

- [x] Hardening: the `Rebuilt code` reader requires a plain declaration line,
  so a design that quotes the rule does not declare a ship.
  - Batch: ship-gate
  - [x] COMMIT: `e427e87f9f5bfeed72c8d6d26ea2655235a4e90c` -- Batch: ship-gate
  - Evidence: the first `givn check tasks` on this `No rebuild` change printed
    `component_executed=1317/1317`; after the fix it reports 10 delta
    scenarios and no component tier.

## ship-guidance

- [x] `serve-extended-rules`: The ise instruction documents the whole-system
  working-tree ship.
  - Scenario: The ise instruction documents the whole-system working-tree ship
  - Batch: ship-guidance
  - [x] COMMIT: `e784c7fc43f0371def88eead4d94563b5f3bca64` -- Batch: ship-guidance
  - Evidence: the served `ise` instruction contains the whole-system heading and
    the `component_executed` refusal.

- [x] `serve-extended-rules`: The design guidance requires the component runner
  for a whole-system rebuild.
  - Scenario: The design guidance requires the component runner for a whole-system rebuild
  - Batch: ship-guidance
  - [x] COMMIT: `e784c7fc43f0371def88eead4d94563b5f3bca64` -- Batch: ship-guidance
  - Evidence: the served design instruction requires one step-definition file
    per capability and calls a missing runner incomplete.

- [x] `serve-extended-rules`: The design-review guidance grills the two-tier
  runner.
  - Scenario: The design-review guidance grills the two-tier runner
  - Batch: ship-guidance
  - [x] COMMIT: `e784c7fc43f0371def88eead4d94563b5f3bca64` -- Batch: ship-guidance
  - Evidence: the served design-review instruction names
    `Two-tier runner coverage` as a branch.

- [x] `serve-extended-rules`: The tasks guidance keeps the retained corpus when
  the delta is skipped.
  - Scenario: The tasks guidance keeps the retained corpus when the delta is skipped
  - Batch: ship-guidance
  - [x] COMMIT: `e784c7fc43f0371def88eead4d94563b5f3bca64` -- Batch: ship-guidance
  - Evidence: the served tasks instruction states that the skip removes the
    delta scenario batch.

- [x] `serve-extended-rules`: The review guidance treats a missing declared tier
  as critical.
  - Scenario: The review guidance treats a missing declared tier as critical
  - Batch: ship-guidance
  - [x] COMMIT: `e784c7fc43f0371def88eead4d94563b5f3bca64` -- Batch: ship-guidance
  - Evidence: the served review instruction classifies the missing tier as a
    critical finding and refuses `REVIEW: PASS`.

## docs

- [x] `docs/reference.md` records the receipt `component` field, the
  `component_executed` refusal, and the `verify` hook result contract.
  - [x] COMMIT: `deb9905338f4851ffe6f0a081ca065ff72e77dae` -- Batch: docs
- [x] `docs/glossary.md` defines **Component evidence**.
  - [x] COMMIT: `deb9905338f4851ffe6f0a081ca065ff72e77dae` -- Batch: docs
- [x] `docs/arc42/02`, `04`, `05`, `06`, `08`, `09`, `11` carry the ship gate,
  the amended ADR-070, and R-174.
  - [x] COMMIT: `deb9905338f4851ffe6f0a081ca065ff72e77dae` -- Batch: docs
- [x] The managed skills `passuff-whole-system-ship-rebuild` and the stale
  `passuff-whole-system-rebuild-ship` agree, and the dry-run/trial skills
  cross-reference the component tier.
  - Evidence: the stale skill is deleted; the five dry-run/trial skills name the
    ship scope and the corrected checklist.

## Archive

- [x] Tick every scenario with its batch revision, run `givn check review`,
  then `givn archive --change ise-ship-component-tier`.
