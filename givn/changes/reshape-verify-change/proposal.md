# Proposal: reshape-verify-change

## Use-Case Context

- Use-case ID: `verify-change`
- Ideation topic: `multi-scm-support`
- Seed path: `givn/ideation/multi-scm-support/changes/verify-change.md`
- Confirmed Personas: `developer--primary-user`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `verify-change` document:

- **R7 mechanism vocabulary** — the Trigger names "a Givn check" and the
  Success guarantee says "machine-readable"; the Main flow opens with
  "Parse".
- **R5 extension discipline** — both extensions lack the main-flow step
  reference and the protected guarantee.
- **R8 channel/mechanism names** — `lint-command` names a command,
  `deduplicate-change-dir-detection` names a directory mechanism and a
  one-off fix, and `validate-artifact-id-input` names an input mechanism.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

1. Rewrite the Trigger and Success guarantee in outcome language and the
   Main flow in observable terms.
2. Rewrite both extensions as `<step>: <condition> → <handling>; protects
   <guarantee>`.
3. Rename the three capabilities (final names approved in the inventory's
   before/after list, 2026-09-18):

| Current | New |
|---|---|
| `lint-command` | `check-specification-health` |
| `deduplicate-change-dir-detection` | `resolve-change-directories` |
| `validate-artifact-id-input` | `validate-artifact-ids` |

The feature files move with their capabilities.

## Out of Scope

- Any behavior change; the scenarios keep their assertions and the
  `givn lint` command keeps its name.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
