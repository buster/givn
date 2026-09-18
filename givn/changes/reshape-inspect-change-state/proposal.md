# Proposal: reshape-inspect-change-state

## Use-Case Context

- Use-case ID: `inspect-change-state`
- Ideation topic: `multi-scm-support`
- Seed path: `givn/ideation/multi-scm-support/changes/inspect-change-state.md`
- Confirmed Personas: `developer--primary-user`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `inspect-change-state`
document:

- **R5 extension discipline** — the extension lacks its main-flow step and
  the protected guarantee.
- **R8 channel and change-shaped names** — `givn-status-command` names the
  command, `validate-change-id-input` names an input mechanism, and
  `remove-dead-skip-list-code` and `fix-uppercase-checkbox-parsing` name
  one-off fixes rather than durable behavior.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

1. Anchor the extension to step 1; rewrite the Trigger in outcome language.
2. Rename the capabilities (final names approved in the inventory's
   before/after list, 2026-09-18):

| Current | New |
|---|---|
| `givn-status-command` | `report-change-status` |
| `validate-change-id-input` | `validate-change-ids` |
| `remove-dead-skip-list-code` | `show-skipped-artifacts` |
| `fix-uppercase-checkbox-parsing` | `read-task-progress` |

The feature files move with their capabilities.

## Out of Scope

- Any behavior change; the scenarios keep their assertions.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
