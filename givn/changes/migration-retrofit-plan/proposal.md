# Proposal: migration-retrofit-plan

## Use-Case Context

- Use-case ID: none (maintenance change for a released migration bundle;
  `specs` is skipped)
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

The 0.8.0 migration bundle (`migrate-0-7-0-to-0-8-0`) shipped with an
execution model the operator did not confirm: it lands every retrofit — use
case splits, persona merges, ADR removals — inside the maintenance change
before that change archives. For a real corpus this means a split of a leaf
with eighteen capabilities and a batch of persona and ADR edits land
unreviewed by the normal change gates, and the maintenance change stays open
for the whole campaign.

The operator settled a different model in discussion, along with six further
decisions the bundle left implicit.

## Proposed Solution

Amend the 0.8.0 bundle (`assets/migrations/0.7.0-to-0.8.0/{proposal,design}.md`)
and the human guide (`docs/migrations/0.7.0-to-0.8.0.md`) to the decided
model:

1. **Full retrofit.** Nothing is grandfathered; every permanent use case,
   persona, and accepted ADR is brought into the model.
2. **Inventory and plan.** The maintenance change records every deviation
   with its disposition and the change that will fix it. The retrofits land as
   normal changes with their own spec deltas, review, and archive; the
   maintenance change does not carry them.
3. **Archive timing.** The maintenance change archives once the inventory is
   complete and every deviation names its follow-up change or an in-flight
   one. The inventory is the durable record; the follow-ups proceed as normal
   work.
4. **ADR scope.** Accepted records are re-qualified in the inventory; a
   `proposed` record is listed with its owning change as the follow-up that
   must qualify it before that change archives.
5. **Non-qualifying ADRs.** The rationale moves once to one canonical home,
   the record leaves the register and both indexes, and the migration change
   keeps the audit note (ID, failing test, destination). It is never copied
   into `docs/arc42/adr/archive/`.
6. **Renames.** Component- and channel-shaped capability and use-case names
   (R8) are renamed in the follow-up, which moves the `.feature` file and
   updates every reference; one use case per follow-up change.
7. **Term evidence.** Every existing `confirmed` term gets its search
   re-run and the commands and hit counts recorded in the record; a
   collision found becomes a named follow-up that renames or rejects the
   term.

The bundle keeps its phase content (rules, criteria, procedures); only the
execution model and the completion boundary change. No product behavior
changes and no version bump is needed: the 0.8.0 tag has not been created, so
the amended bundle is what 0.8.0 ships.

## Out of Scope

- Re-publishing or rewriting the already archived `migrate-0-7-0-to-0-8-0`
  release change; this change amends the bundle and the guide only.
- A version bump; the release is untagged.
- Performing any consumer project's retrofit.

## Open Questions

None.
