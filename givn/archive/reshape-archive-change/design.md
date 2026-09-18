# Design: reshape-archive-change

## Overview

Rewrite one use-case narrative, remove one include, and rename one
capability. No behavior changes; `specs` is skipped and the corpus is edited
directly.

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/archive-change/usecase.md` | Goal and Success guarantee rewritten; extensions step-referenced; mechanism vocabulary replaced; `usecase: verify-change` include removed; capability renamed |
| `givn/specs/archive-change/preserve-docstrings-on-archive.feature` | Move to `preserve-scenario-docstrings.feature`; title updated |

## Extension mapping

| Current | Rewritten as |
|---|---|
| Uncommitted work outside the resolved evidence blocks the archive | `1: … → blocked, names the offending files; protects the minimal guarantee` |
| A failed scope leaves an unproven candidate | `5: … → the candidate stays published and repairable; protects the minimal guarantee` |
| Finalization refuses a changed tree | `7: … → finalization refuses; protects the success guarantee` |
| A backend hook rejects the candidate | `4: … → unproven and repairable, state is version-control-specific; protects the minimal guarantee` |
| No SCM backend is selected | `1: … → refused by name before any mutation; protects the minimal guarantee` |
| An already proven archive refuses another run | `1: … → another run is refused; protects the success guarantee` |
| A legacy archive without a receipt refuses another run | `1: … → another run is refused; protects the success guarantee` |
| Active and archived at once is corrupt | `1: … → refused as corrupt; protects the success guarantee` |

## Verification

- `givn lint` exits 0; no reference to the old capability name remains.
- `givn spec tree` shows `preserve-scenario-docstrings` under archive-change.
- The suite and e2e pass with the moved scenario.

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

- Use-case ID: `archive-change`
- Affected capabilities: `preserve-scenario-docstrings` (renamed)
- Confirmed Personas: the three existing Personas

## Open Technical Questions

None.
