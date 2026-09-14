# Design: verify-change

## Technical Decisions

- Pure wording change in `src/integrity_hook.rs`: the four integrity messages
  say "revision identifier" / "revision identifiers"; the placeholder and
  duplicate rules, the function shape, and the exit codes are unchanged. The
  check already treats the identifier as opaque (Q11): only emptiness,
  known placeholder tokens, and duplicates are rejected — no length or
  alphabet assumption — so the new opaque-identifier scenario needs no
  production change and pins the existing acceptance.
- The permanent `givn-workflow-enforcement.feature` placeholder scenario is
  aligned to the new wording in the same commit (its Then asserts the
  message text); the delta carries the same modification for the archive
  merge.

## Production Impact

- `src/integrity_hook.rs`: message wording; `check_hash_presence` renamed to
  `check_revision_identifier_presence`.
- `givn/specs/verify-change/givn-workflow-enforcement.feature`: permanent
  alignment of the placeholder scenario's Then.

## Test Design

| File | Binds |
|---|---|
| (none) | All delta steps reuse existing bindings (`a givn project with a change that has passed design-review`, the tasks.md-content Given, `stderr contains`, exit-code Thens). |

Single-scenario command:

```sh
GIVN_FEATURES=givn/changes/verify-change/specs cargo test --test features_runner -- --name "An opaque revision identifier passes the integrity check"
```

## Interaction Coverage Matrix

No new inventory row: the delta pins a variant of the existing integrity
check behavior; the use case's interactions are unchanged.

## Use-Case Traceability

| Rule | Decision | Evidence |
|---|---|---|
| The recorded revision identifier is opaque (Q11) | No shape validation; the new scenario pins a 40-character identifier | The opaque-identifier scenario |
| Neutral wording (Q10) | "revision identifier" messages | The placeholder scenario's assertion |

## Local Runnability

`./run-tests.sh`; no new executables or services.
