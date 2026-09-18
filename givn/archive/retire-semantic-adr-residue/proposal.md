# Proposal: retire-semantic-adr-residue

## Use-Case Context

- Use-case ID: fragment `adr-qualification-gate` (support fragment)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the repair edits the architecture register; no
  user-visible surface changes.

## Problem / Opportunity

Three accepted records in the architecture decision register describe
mechanisms removed with the semantic engine:

- **ADR-044** — the request-local current-change corpus for
  `givn spec explore --change`.
- **ADR-045** — project-relative scenario identities for the retrieval index.
- **ADR-054** — versioned E5/BGE/NLI evidence output.

ADR-069 removed the engine, its commands (`spec index`, `spec search`,
`spec explore`), and its output schemas, and superseded sixteen sibling
records. These three were missed, so the register still presents dead
decisions as current constraints. The register contract says only current
decisions stay active; superseded records are historical context in
`archive/`.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 3) records the three
as obsolete and names this follow-up.

## Proposed Solution

1. Mark each record `status: superseded by ADR-069` and move it to
   `docs/arc42/adr/archive/`.
2. Extend ADR-069's supersession list with the three records.
3. Update the register README: the active count drops from 28 to 25, the
   three rows move to the archived list, and the relationship line names
   them.
4. Update the permanent scenario "The migration preserves independent
   architecture history across three proposed decisions" from 28 to 25
   accepted decisions through a delta spec; it reads the live register, so
   every register-changing migration updates it.

## Out of Scope

- ADR-027 (`NOT_QUALIFIED`; separate follow-up).
- The stale project-local ADR template (repaired by
  `resync-adr-template`).
- Any change to deterministic retrieval (`spec find`, `spec tree`,
  `spec route`) or to ADR-037, which ADR-069 explicitly retains.

## Open Questions

None.
