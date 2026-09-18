# Design: confirm-persona-coverage

## Overview

Five delta narratives replace the permanent use-case documents at archive.
Each carries the `## Personas` correction and the current permanent file's
base hash; the permanent files are untouched during the change.

## Exact file changes

| Delta path | Change | Base hash |
|---|---|---|
| `givn/changes/confirm-persona-coverage/specs/verify-change/usecase.md` | `- none` → `- developer--primary-user` | `ccd51a0e27bc2595d95b626576e3b653450ed76b9933b73feece814e5073372e` |
| `givn/changes/confirm-persona-coverage/specs/operate-product/usecase.md` | `- none` → `- developer--primary-user` | `b32bb42a81509cb9bcd3a75a5febf926c9e4dc1d68d0ce40efd54a7ddbdf1de9` |
| `givn/changes/confirm-persona-coverage/specs/inspect-change-state/usecase.md` | `- none` → `- developer--primary-user` | `897894303db86ea221ae9a0d9385bbeb371b59e1492585a23c940ea6f465ca98` |
| `givn/changes/confirm-persona-coverage/specs/discover-specifications/usecase.md` | `- none` → `- agent--technical-role` | `b7e7e2534bc1267474b62fa276368a7b8b09769d6213729c958063a32830e886` |
| `givn/changes/confirm-persona-coverage/specs/make-usecases-canonical/usecase.md` | `- none` → `- givn-maintainer--owner` | `6e5299c1a6f8b570536ea54d2b27113239d73c00f7444c5ad5041fdbb27af601` |

## Procedure

1. For each use case, copy the permanent document, replace the `- none`
   entry under `## Personas`, and set the trailing
   `<!-- givn:base-sha256:... -->` line to the hash of the current permanent
   file (the version the delta replaces).
2. Archive merges the narratives with the base-hash check; the permanent
   files change only at that point.

## Verification

- The archive merge accepts every narrative (base hash matches the
  permanent file).
- After archive, `rg -c "developer--primary-user|agent--technical-role|givn-maintainer--owner" givn/specs/<uc>/usecase.md`
  reports one match per file.
- `givn lint` reports no `[PERS ]` finding.
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.

## Step Definitions

N/A — the delta carries narrative documents only; no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — documentation-only change.

## Internal Primitives

None — the change reuses the narrative delta merge.

## Use-case traceability

- Use-case ID: the five use cases named above
- Affected capability: none; the change corrects persona coverage
- Confirmed Personas: the operator-approved actor Personas

## Open Technical Questions

None.
