# Design: reshape-maintain-project

## Overview

Split one leaf into a summary and three leaves, split one feature file, and
move one feature. No behavior changes; `specs` is skipped and the corpus is
edited directly.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/maintain-project/usecase.md` | Becomes the `+` summary |
| `givn/specs/keep-documentation-current/usecase.md` | New `!` leaf narrative |
| `givn/specs/upgrade-a-project/usecase.md` | New `!` leaf narrative |
| `givn/specs/publish-a-release/usecase.md` | New `!` leaf narrative |
| `givn/specs/maintain-project/readme-enforcement.feature` | Move to `givn/specs/keep-documentation-current/` |
| `givn/specs/maintain-project/release-0-3-0.feature` | Split into `givn/specs/upgrade-a-project/apply-a-managed-upgrade.feature` (11 scenarios) and `givn/specs/publish-a-release/describe-the-release.feature` (5 scenarios) |

## Renames

| Current | New | Home |
|---|---|---|
| `release-0-3-0` | `apply-a-managed-upgrade` | upgrade-a-project |
| (release metadata scenarios) | `describe-the-release` | publish-a-release |

## Verification

- `givn spec tree` shows the summary and three children; the two new
  capabilities resolve.
- `givn lint` exits 0; no reference to `release-0-3-0` remains.
- The suite and e2e pass with the split scenarios.

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

- Use-case ID: `maintain-project` (promoted), `keep-documentation-current`,
  `upgrade-a-project`, `publish-a-release` (new)
- Affected capabilities: `apply-a-managed-upgrade`,
  `describe-the-release`, `readme-enforcement` (moved)
- Confirmed Personas: `developer--primary-user`, `givn-maintainer--owner`

## Open Technical Questions

None.
