# Proposal: inspect-change-state

## Use-Case Context

- Use-case ID: `inspect-change-state` (existing, modified; capability
  `givn-status-command`)
- Ideation topic: multi-scm-support
- Confirmed Personas: developer--primary-user, agent--technical-role,
  givn-maintainer--owner

## Problem / Opportunity

`givn status` never says which version-control backend owns the project, so
an agent or maintainer cannot see why archive or upgrade would refuse.

## Proposed Solution

`givn status` reports the active backend as `SCM: git`, `SCM: fossil`, or
`SCM: none`. A project without a backend still exits 0 and names archive and
managed upgrade as unavailable on the same line.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| givn-status-command | EXTEND givn-status-command | `EXTEND givn-status-command` | The existing capability owns the status output. |

## Out of Scope

JSON output shape; the unproven-candidate report (owned by `archive-change`).

## Open Questions

None.
