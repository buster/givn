# Design: migrate-term-records

## Overview

Convert `domain-terms.md` from the two-column candidate table to the term
schema. One file changes; no product code.

## Exact file changes

| Path | Change |
|---|---|
| `givn/ideation/multi-scm-support/domain-terms.md` | Term schema: Term, Status, Definition, Contrast, First use, Collision check; 13 rows |

## Status assignment

| Status | Terms | Basis |
|---|---|---|
| confirmed | SCM backend, Revision, Checkout, Clean state, Candidate revision, Tested revision, Revision binding, Upgrade revision, Plan revision | merged into `docs/arc42/12-glossary.md` by Q10 (2026-09-14) |
| rejected | Repository root, Index / staging area | superseded by Checkout; Git-only concept replaced by "publish a subset" |
| candidate | Hook, Check-in (Fossil) | per-SCM terms used only qualified; they stay in the backend seam |

## Verification

- The file parses as a six-column markdown table with 13 data rows.
- The collision-check cells carry the migration's recorded commands and
  counts.
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.

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
- Affected capability: none; the change repairs the topic's term records
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
