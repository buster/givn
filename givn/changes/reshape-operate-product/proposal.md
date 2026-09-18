# Proposal: reshape-operate-product

## Use-Case Context

- Use-case ID: `operate-product`
- Ideation topic: `multi-scm-support`
- Seed path: `givn/ideation/multi-scm-support/changes/operate-product.md`
- Confirmed Personas: `developer--primary-user`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `operate-product` document:

- **R1 goal conjunction** — the Goal joins "Start … and observe …", and the
  Success guarantee names two outcomes.
- **R7 mechanism vocabulary** — the Trigger names "the run command" and the
  Main flow names "the declared readiness signal".
- **R5 extension discipline** — the extension lacks its main-flow step and
  the protected guarantee.
- **R8 channel/mechanism names** — `run-command` and `run-execution` name
  the channel and the execution mechanism.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

1. Rewrite the Goal as one phrase ("Run the declared product to a reported
   outcome") and the Success guarantee as one outcome.
2. Rewrite the Trigger and Main flow in outcome language; the readiness
   vocabulary replaces "signal".
3. Anchor the extension to step 1.
4. Rename the capabilities (final names approved in the inventory's
   before/after list, 2026-09-18):

| Current | New |
|---|---|
| `run-command` | `declare-the-product-start` |
| `run-execution` | `start-the-declared-product` |

The feature files move with their capabilities.

## Out of Scope

- Any behavior change; the scenarios keep their assertions and the `givn run`
  command keeps its name.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
