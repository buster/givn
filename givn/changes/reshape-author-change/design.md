# Design: reshape-author-change

## Overview

Split one leaf into a summary and two leaves, move five feature files, and
rename three capabilities. No behavior changes; `specs` is skipped and the
corpus is reorganized directly (the `group-specs-by-use-case` precedent).

## Exact file changes

| Path | Change |
|---|---|
| `givn/specs/author-change/usecase.md` | Becomes the `+` summary: union goal, no direct capabilities, Includes the two children |
| `givn/specs/initialize-project/usecase.md` | New `!` leaf narrative |
| `givn/specs/create-change/usecase.md` | New `!` leaf narrative |
| `givn/specs/author-change/new-command.feature` | Move to `givn/specs/create-change/create-a-change.feature`; Feature line renamed |
| `givn/specs/author-change/prose-consolidation.feature` | Move to `givn/specs/create-change/prose-consolidation.feature` |
| `givn/specs/author-change/visual-design-contract.feature` | Move to `givn/specs/create-change/visual-design-contract.feature` |
| `givn/specs/author-change/init-agent-target-selection.feature` | Move to `givn/specs/initialize-project/initialize-a-project.feature`; Feature line renamed |
| `givn/specs/author-change/install-extras.feature` | Move to `givn/specs/initialize-project/enable-optional-addons.feature`; Feature line renamed |

## Narrative shape

- The summary's Goal is the union; its Capabilities are `none`; its Includes
  are `usecase: initialize-project` and `usecase: create-change`.
- Each child is a `!` leaf with one goal, one primary actor (Developer),
  step-referenced extensions, and mechanism-free Goal, Trigger, Main flow,
  and Guarantees. Each includes `fragment: support`.
- The guidance capabilities live under `create-change`; the setup
  capabilities under `initialize-project`.

## Renames

| Current | New |
|---|---|
| `new-command` | `create-a-change` |
| `init-agent-target-selection` | `initialize-a-project` |
| `install-extras` | `enable-optional-addons` |

## Verification

- `givn spec tree` shows the summary with two children and no direct
  capabilities; the five capabilities resolve under their new homes.
- `givn lint` reports no new parse errors; the suite and e2e pass with the
  moved scenarios.
- No reference to the old capability names remains in `givn/specs/`.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps. The existing
step definitions keep serving the moved scenarios.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — specification reorganization only.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: `author-change` (promoted to `+`), `initialize-project` (new),
  `create-change` (new)
- Affected capabilities: the five listed above
- Confirmed Personas: the three existing Personas

## Open Technical Questions

None.
