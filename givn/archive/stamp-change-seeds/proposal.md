# Proposal: stamp-change-seeds

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: `multi-scm-support`
- Seed path: the seven seeds under
  `givn/ideation/multi-scm-support/changes/`
- Confirmed Personas: none — the repair edits ideation provenance; no
  user-visible surface changes.

## Problem / Opportunity

The seven `multi-scm-support` change seeds predate the seed schema the
`seed-artifact-lifecycle` change introduced. `givn lint` reports every one
with `[SEED ] change seed has no valid status line`, and the seeds also lack
the schema's fields (`Suggested start`) and the accepted decisions' option
treatment. All seven were consumed and implemented by the archived changes of
the same ids, so their provenance is stale and machine-invisible.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 4) records this
deviation and names this follow-up.

## Proposed Solution

Convert each seed to the schema:

1. Add `Status: implemented in <change-id>` as the line after the title.
2. Normalize the fields to `Use-case ID`, `Topic`, `Personas`,
   `Suggested start`, `Handoff`; derive `Suggested start` from the recorded
   dependencies (the `scm-backend` seed first, then the seeds that depend on
   it).
3. Convert the accepted decisions to `###` sections with Context, Options,
   and Chosen; the options come from the recorded questions log and the
   seed's own decision bullets.
4. Carry the open questions with their recorded resolutions.
5. Keep the scope note as provenance.

The retrospective conversion names each decision's options and outcome; it
does not reconstruct the full four-part dialog treatment, which lives in the
topic's session record.

## Out of Scope

- Reopening any decision; the status is `implemented in`, not `proposed by`.
- Changing the product guidance or the lint check.

## Open Questions

None.
