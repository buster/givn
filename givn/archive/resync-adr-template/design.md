# Design: resync-adr-template

## Overview

Restore the project-local ADR template from the 0.8.0 addon asset. The change
deletes one stale generated file, re-runs the addon scaffolder, and verifies
the restored content. No code changes.

## Exact file changes

| Path | Change |
|---|---|
| `docs/arc42/adr/adr-template.md` | Deleted, then re-created by `givn addons enable arc42` from `assets/addons/arc42-docs/chapters/adr/adr-template.md` |

## Procedure

1. `rm docs/arc42/adr/adr-template.md`.
2. `givn addons enable arc42` — the scaffolder writes only missing chapter
   files, so only the deleted template is restored.
3. Confirm the restored file contains neither `ADR-QUALIFICATION: PASS` nor
   `qualification: QUALIFIED` and matches the managed asset byte for byte.

## Verification

- `cmp docs/arc42/adr/adr-template.md assets/addons/arc42-docs/chapters/adr/adr-template.md`
  exits 0.
- `rg -c "ADR-QUALIFICATION: PASS|qualification: QUALIFIED" docs/arc42/adr/adr-template.md`
  reports no matches.
- The existing scenario "The ADR template ships with no pre-passed
  qualification values" (`givn/specs/fragments/adr-qualification-gate.feature`)
  continues to pass; it reads the managed asset.
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — the change touches a documentation file; no service, dependency, or
runtime surface.

## Internal Primitives

None — the change reuses the existing addon scaffolder.

## Use-case traceability

- Use-case ID: none (project-maintenance repair; `specs` skipped)
- Affected capability: none; the repair restores a generated project file
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
