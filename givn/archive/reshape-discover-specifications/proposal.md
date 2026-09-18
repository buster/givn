# Proposal: reshape-discover-specifications

## Use-Case Context

- Use-case ID: `discover-specifications`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `agent--technical-role`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `discover-specifications`
document:

- **R5 extension discipline** — the extension lacks its main-flow step and
  the protected guarantee.
- **R8 mechanism name** — `spec-progress-output` names an output mechanism.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

1. Anchor the extension to step 1; rewrite the trigger in outcome language.
2. Rename `spec-progress-output` to `keep-audit-output-clean` (final name
   approved in the inventory's before/after list, 2026-09-18); the feature
   file moves with it.

## Out of Scope

- Any behavior change; the scenarios keep their assertions.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
