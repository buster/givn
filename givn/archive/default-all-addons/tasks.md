# Tasks: default-all-addons

Owning use cases: `initialize-project` (capability `enable-optional-addons`)
and `assess-code-structure` (capability `outline-report`). Personas:
`developer--primary-user`, `agent--technical-role`.

The default is enabled for every add-on. An explicit `false` is the only
opt-out; `givn init --no-addons` writes it for every known add-on, and a
partial `addons:` block changes only the keys it names.

## Setup

- [x] Runner and strict mode inherited unchanged:
      `tests/features_runner.rs` calls `.fail_on_skipped()`, and
      `givn/commands.yaml` keeps `./run-tests.sh --non-e2e` / `--e2e`.
- [x] Step definitions: `tests/steps/default_all_addons_steps.rs` provides the
      partial-block writer; existing `givn addon` and outline steps are reused.
- [x] No new E2E inventory action; the existing init and outline-addon E2E
      scenarios cover the real CLI.

## Scenario: givn init enables every addon by default

- [x] RED against the parent checkout: `Then stdout contains "[enabled ]  outline"` failed; outline and architecture_principles were default-off.
- [x] GREEN: `1 scenario (1 passed)`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `9a0115c9` -- `feat(enable-optional-addons): all add-ons enabled by default` (Batch: all-addons-default)

## Scenario: givn addons list reports every addon enabled by default

- [x] RED against the parent checkout: `stdout contains "[enabled ]  outline"` failed because the list used the config-only false default.
- [x] GREEN: `1 scenario (1 passed)`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `9a0115c9` -- `feat(enable-optional-addons): all add-ons enabled by default` (Batch: all-addons-default)

## Scenario: givn init --no-addons writes an explicit opt-out for every addon

- [x] RED against the parent checkout: `givn addon "outline" is "false"` failed because `--no-addons` relied on the embedded false default.
- [x] GREEN: `1 scenario (1 passed)`; the config carries explicit false for the named add-ons.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `9a0115c9` -- `feat(enable-optional-addons): all add-ons enabled by default` (Batch: all-addons-default)

## Scenario: a partial addons block keeps unnamed addons enabled

- [x] RED against the parent checkout: `stdout contains "[enabled ]  domain_modeling"` failed because a missing key defaulted to false.
- [x] GREEN: `1 scenario (1 passed)`; only the named outline key changes.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `9a0115c9` -- `feat(enable-optional-addons): all add-ons enabled by default` (Batch: all-addons-default)

## Scenario: The outline experiment is enabled by default

- [x] RED against the parent checkout: `stdout contains "[enabled ]  outline"` failed.
- [x] GREEN: `1 scenario (1 passed)`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `9a0115c9` -- `feat(enable-optional-addons): all add-ons enabled by default` (Batch: all-addons-default)

## Scenario: A project can explicitly disable the outline experiment

- [x] RED: passed against the parent checkout because the outline add-on was already disabled; kept as regression protection for the explicit opt-out.
- [x] GREEN: `1 scenario (1 passed)`.
- [x] REFACTOR: `1 scenario (1 passed)`.
- [x] COMMIT: `9a0115c9` -- `feat(enable-optional-addons): all add-ons enabled by default` (Batch: all-addons-default)

## Closeout

- [x] `GIVN_FEATURES=givn/changes/default-all-addons/specs ./run-tests.sh --non-e2e`
      exited 0; `6 scenarios (6 passed)`.
- [x] `./run-tests.sh --non-e2e` exited 0; `728 tests passed`.
- [x] `./run-tests.sh --e2e` exited 0; `112 tests passed`.
- [x] `givn init --no-addons` writes explicit false for every known add-on;
      `givn addons list` shows the effective default.
- [x] `givn check review --change default-all-addons` passed: integrity,
      overlap dispositions, routing, receipt, and `outline review: PASS`.
- [x] `givn archive --change default-all-addons` runs in the same pass; the
      archive gate reuses the current receipt and re-runs the merged suite.
