# Design: reshape-inspect-change-state

## Overview

Rewrite one use-case narrative and rename four capabilities. No behavior
changes; `specs` is skipped.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/inspect-change-state/usecase.md` | Trigger rewritten; extension anchored; capabilities renamed |
| `givn/specs/inspect-change-state/givn-status-command.feature` | Move to `report-change-status.feature`; title updated |
| `givn/specs/inspect-change-state/validate-change-id-input.feature` | Move to `validate-change-ids.feature`; title updated |
| `givn/specs/inspect-change-state/remove-dead-skip-list-code.feature` | Move to `show-skipped-artifacts.feature`; title updated |
| `givn/specs/inspect-change-state/fix-uppercase-checkbox-parsing.feature` | Move to `read-task-progress.feature`; title updated |

## Verification

- `givn lint` exits 0; no reference to the old capability names remains.
- The suite and e2e pass with the moved scenarios.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — specification reorganization only.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: `inspect-change-state`
- Affected capabilities: the four renamed ones
- Confirmed Personas: `developer--primary-user`

## Open Technical Questions

None.
