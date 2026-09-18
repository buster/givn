# Design: reshape-discover-specifications

## Overview

Rewrite one use-case narrative and rename one capability. No behavior
changes; `specs` is skipped.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/discover-specifications/usecase.md` | Trigger and extension rewritten; capability renamed |
| `givn/specs/discover-specifications/spec-progress-output.feature` | Move to `keep-audit-output-clean.feature`; title updated |

## Verification

- `givn lint` exits 0; no reference to the old capability name remains.
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

- Use-case ID: `discover-specifications`
- Affected capability: `keep-audit-output-clean`
- Confirmed Personas: `agent--technical-role`

## Open Technical Questions

None.
