# Design: inspect-change-state

## Technical Decisions

- `src/cli/status.rs` gains `print_scm_line`, printed once at the top of every
  human-readable `givn status` path (list, auto-detail, detail, and the
  no-changes path); JSON output is untouched. Git prints `SCM: git`, Fossil
  `SCM: fossil`; any selection failure prints
  `SCM: none (archive and managed upgrade are unavailable)` (A19: the
  specific reason surfaces from the command that needs the backend).
- The SCM line shifts every human-readable line number by one, so the
  permanent "Multiple active changes listed alphabetically — text mode"
  scenario's positional assertions move from lines 3/4 to 4/5; the permanent
  feature is aligned in the implementation commit and the delta carries the
  same modification.

## Production Impact

- `src/cli/status.rs`: `print_scm_line` + the call site.
- `givn/specs/inspect-change-state/givn-status-command.feature`: permanent
  line-number alignment.

## Test Design

| File | Binds |
|---|---|
| `tests/steps/init_agent_target_selection_steps.rs` | `a (Git|Fossil) project` Given (backend fixtures), `stdout reports the active backend as <scm>`, and the unavailable-phrase Then. The no-backend Given reuses the seed-1 fixture. |

Single-scenario command:

```sh
GIVN_FEATURES=givn/changes/inspect-change-state/specs cargo test --test features_runner -- --name "Status names the active SCM backend"
```

## Interaction Coverage Matrix

No new inventory row: the delta extends the existing inspect-project-state
action's output; the use case's interactions are unchanged.

## Use-Case Traceability

| Decision | Evidence |
|---|---|
| Status reports `SCM: git|fossil|none` | The outline's two examples |
| No backend still exits 0 and names the unavailable commands | The no-backend scenario |

## Local Runnability

`./run-tests.sh`; no new executables or services.
