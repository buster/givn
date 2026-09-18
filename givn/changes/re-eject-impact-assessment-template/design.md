# Design: re-eject-impact-assessment-template

## Overview

Replace the stale ejected template with the 0.8.0 managed asset through the
eject command. One file changes; no code.

## Exact file changes

| Path | Change |
|---|---|
| `givn/artifacts/arc42-docs/impact-assessment-template.md` | Re-ejected from the 0.8.0 asset; gains the structured verdict block |

## Procedure

1. `givn eject arc42-docs:template --force`.
2. `cmp` the result against the managed asset.
3. Confirm the structured verdict block is present.

## Verification

- `cmp givn/artifacts/arc42-docs/impact-assessment-template.md assets/addons/arc42-docs/impact-assessment-template.md`
  exits 0.
- `rg -c "qualification: <blank until recorded>" givn/artifacts/arc42-docs/impact-assessment-template.md`
  reports one match.
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — documentation-only change.

## Internal Primitives

None — the change reuses the existing eject command.

## Use-case traceability

- Use-case ID: none (project-maintenance repair; `specs` skipped)
- Affected capability: none; the change restores an ejected template
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
