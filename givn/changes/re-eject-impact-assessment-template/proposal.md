# Proposal: re-eject-impact-assessment-template

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the repair restores an ejected template; no
  user-visible surface changes.

## Problem / Opportunity

The project owns an ejected copy of the arc42 impact-assessment template at
`givn/artifacts/arc42-docs/impact-assessment-template.md`. It predates
0.8.0: it is missing the structured ADR verdict block
(`qualification:` … `replacement_adr:`) that the `adr-qualification-gate`
work added to the managed template. An author using the project copy would
not record the structured verdict the design-review and review gates now
expect. The copy contains no genuine project-specific edits.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 5) records this
deviation and names this follow-up.

## Proposed Solution

Re-eject the facet from the 0.8.0 binary with
`givn eject arc42-docs:template --force`, then confirm the file is
byte-identical to `assets/addons/arc42-docs/impact-assessment-template.md`
and contains the structured verdict block.

## Out of Scope

- Customizing the template; the project has no genuine edits to re-apply.
- Any other ejected facet; `givn/skills/` is empty and `givn/config.yaml`
  carries settings only.

## Open Questions

None.
