# Design: operate-product

## Technical Decisions

- Pure wording: the two upgrade-backfill scenario steps and their step
  bindings rename "commit" to "revision" (`latest_revision_contains`,
  `latest_revision_unchanged`); the assertions are unchanged and remain
  Git-fixture based (the step mechanics are backend-neutral in wording; the
  fixture backend is not part of this delta).
- The permanent feature is aligned in the same commit; the delta carries the
  same modifications for the merge.

## Production Impact

- `givn/specs/operate-product/run-command.feature`: permanent alignment.
- `tests/steps/run_execution_e2e_steps.rs`: the two bindings renamed.

## Test Design

All delta steps reuse existing bindings apart from the renamed texts, which
the same file binds. Single-scenario command:

```sh
GIVN_FEATURES=givn/changes/operate-product/specs cargo test --test features_runner -- --name "The upgrade commit includes the backfilled run declaration"
```

## Interaction Coverage Matrix

No inventory change: the scenarios' titles are unchanged and the rows stay.

## Use-Case Traceability

Q10 neutral wording for the upgrade-revision content assertions.

## Local Runnability

`./run-tests.sh`; nothing new.
