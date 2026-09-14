# Proposal: operate-product

## Use-Case Context

- Use-case ID: `operate-product` (existing, modified; capability `run-command`)
- Ideation topic: multi-scm-support
- Confirmed Personas: agent--technical-role, developer--primary-user

## Problem / Opportunity

The upgrade-backfill scenarios say "the latest commit", Git vocabulary, for a
fact the seam already guarantees for any backend: the latest revision
contains the backfilled run declaration, and a repeated upgrade creates no
additional revision.

## Proposed Solution

Reword the two scenarios to "the latest revision contains ..." / "the latest
revision is unchanged by the second upgrade". No runtime behavior and no new
scenario (the topic's recorded decision).

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| run-command | EXTEND run-command | `EXTEND run-command` | The existing capability owns the scenarios. |

## Out of Scope

Anything but wording.

## Open Questions

None.
