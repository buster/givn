# Proposal: resync-adr-template

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the repair restores a generated template; no
  user-visible surface changes (operator-confirmed in the migration
  inventory).

## Problem / Opportunity

The project-local `docs/arc42/adr/adr-template.md` is a pre-0.8.0 copy. It
carries the old gate's pre-passed verdict (`ADR-QUALIFICATION: PASS`,
`qualification: QUALIFIED`), so an author copying it would inherit a passed
verdict before any qualification ran. The 0.8.0 managed asset ships blank
(`ADR-QUALIFICATION: <PASS | FAIL>`, `qualification: <blank until
recorded>`).

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 3) records this
deviation and names this follow-up.

## Proposed Solution

Delete the stale project-local copy and re-scaffold it from the 0.8.0 addon
with `givn addons enable arc42` (the scaffolder writes only missing chapter
files). Confirm the restored file is identical to
`assets/addons/arc42-docs/chapters/adr/adr-template.md` and contains neither
`ADR-QUALIFICATION: PASS` nor `qualification: QUALIFIED`.

## Out of Scope

- The other ADR re-qualification repairs (separate follow-ups).
- Any change to the managed asset, which is already blank.

## Open Questions

None.
