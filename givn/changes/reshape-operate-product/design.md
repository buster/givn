# Design: reshape-operate-product

## Overview

Rewrite one use-case narrative and rename two capabilities. No behavior
changes; `specs` is skipped and the corpus is edited directly.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/operate-product/usecase.md` | Goal, Trigger, Main flow, and Success guarantee rewritten; extension anchored; capabilities renamed |
| `givn/specs/operate-product/run-command.feature` | Move to `declare-the-product-start.feature`; title updated |
| `givn/specs/operate-product/run-execution.feature` | Move to `start-the-declared-product.feature`; title updated |

## Verification

- `givn lint` exits 0; no reference to the old capability names remains.
- `givn spec tree` shows the renamed capabilities under operate-product.
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

- Use-case ID: `operate-product`
- Affected capabilities: the two renamed ones
- Confirmed Personas: `developer--primary-user`

## Open Technical Questions

None.
