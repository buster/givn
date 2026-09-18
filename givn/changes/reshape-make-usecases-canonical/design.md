# Design: reshape-make-usecases-canonical

## Overview

One narrative edit; `specs` is skipped.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/make-usecases-canonical/usecase.md` | Extension anchored to step 1 |

## Verification

- `givn lint` exits 0.
- The suite and e2e pass.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — narrative edit only.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: `make-usecases-canonical`
- Affected capability: `canonical-usecase-contract` (unchanged)
- Confirmed Personas: `givn-maintainer--owner`

## Open Technical Questions

None.
