# Proposal: confirm-persona-coverage

## Use-Case Context

- Use-case ID: `verify-change`, `operate-product`,
  `inspect-change-state`, `discover-specifications`,
  `make-usecases-canonical`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user` (verify-change,
  operate-product, inspect-change-state), `agent--technical-role`
  (discover-specifications), `givn-maintainer--owner`
  (make-usecases-canonical) — operator-confirmed 2026-09-18

## Problem / Opportunity

Five use cases declare `## Personas: none` although each owns a
user-visible surface (checks, `givn run`, `givn status`, spec discovery, and
the use-case tree). The review rule requires at least one confirmed Persona
on a user-visible surface; `none` without a recorded reason leaves those
surfaces without a persona lens.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 2) records this
deviation and names this follow-up.

## Proposed Solution

Add the actor's confirmed Persona to each use case's `## Personas` section
through delta narratives carrying the current base hash:

| Use case | Actor | Confirmed Persona |
|---|---|---|
| verify-change | Developer | `developer--primary-user` |
| operate-product | Product operator | `developer--primary-user` |
| inspect-change-state | Developer | `developer--primary-user` |
| discover-specifications | Authoring agent | `agent--technical-role` |
| make-usecases-canonical | Givn maintainer | `givn-maintainer--owner` |

The operator chose the actor's Persona per use case over broader lists.

## Out of Scope

- Use cases that already list Personas.
- Inventing Personas or promoting new segments.
- The permanent persona schema migration
  (`migrate-personas-to-stable-core`).

## Open Questions

None.
