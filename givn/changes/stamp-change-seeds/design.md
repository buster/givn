# Design: stamp-change-seeds

## Overview

Convert the seven consumed `multi-scm-support` seeds to the seed schema:
status line, normalized fields, decisions with option treatment, open
questions with their resolutions, and the scope note. No product code
changes; `givn lint` stops reporting `[SEED ]` for these files.

## Exact file changes

| Path | Change |
|---|---|
| `givn/ideation/multi-scm-support/changes/scm-backend.md` | Schema conversion; `Status: implemented in scm-backend` |
| `givn/ideation/multi-scm-support/changes/author-change.md` | Schema conversion; `Status: implemented in author-change` |
| `givn/ideation/multi-scm-support/changes/archive-change.md` | Schema conversion; `Status: implemented in archive-change` |
| `givn/ideation/multi-scm-support/changes/inspect-change-state.md` | Schema conversion; `Status: implemented in inspect-change-state` |
| `givn/ideation/multi-scm-support/changes/maintain-project.md` | Schema conversion; `Status: implemented in maintain-project` |
| `givn/ideation/multi-scm-support/changes/operate-product.md` | Schema conversion; `Status: implemented in operate-product` |
| `givn/ideation/multi-scm-support/changes/verify-change.md` | Schema conversion; `Status: implemented in verify-change` |

## Procedure

1. For each seed: title, then `Status: implemented in <change-id>`, then the
   five fields (`Use-case ID`, `Topic`, `Personas`, `Suggested start`,
   `Handoff`).
2. Convert each accepted decision to a `### <title>` section with Context,
   Options (meaning, in practice, gain, cost where the recorded material
   supports them), and Chosen/Rejected.
3. Carry the open questions with their recorded resolutions from
   `questions.md`.
4. Keep the scope note as provenance.

## Verification

- `givn lint` reports no `[SEED ]` finding for the seven files (previously
  seven).
- The suite passes: `./run-tests.sh` and `./run-tests.sh --e2e`.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — documentation-only change.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: none (project-maintenance repair; `specs` skipped)
- Affected capability: none; the change repairs ideation provenance
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
