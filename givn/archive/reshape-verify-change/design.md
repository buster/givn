# Design: reshape-verify-change

## Overview

Rewrite one use-case narrative and rename three capabilities. No behavior
changes; `specs` is skipped and the corpus is edited directly.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/verify-change/usecase.md` | Trigger, Main flow, and Success guarantee rewritten; extensions step-anchored; capabilities renamed |
| `givn/specs/verify-change/lint-command.feature` | Move to `check-specification-health.feature`; title updated |
| `givn/specs/verify-change/deduplicate-change-dir-detection.feature` | Move to `resolve-change-directories.feature`; title updated |
| `givn/specs/verify-change/validate-artifact-id-input.feature` | Move to `validate-artifact-ids.feature`; title updated |

## Verification

- `givn lint` exits 0; no reference to the old capability names remains.
- `givn spec tree` shows the renamed capabilities under verify-change.
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

- Use-case ID: `verify-change`
- Affected capabilities: the three renamed ones
- Confirmed Personas: `developer--primary-user`

## Open Technical Questions

None.
