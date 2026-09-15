# Proposal: verify-change

## Use-Case Context

- Use-case ID: `verify-change` (existing, modified; capability
  `givn-workflow-enforcement`)
- Ideation topic: multi-scm-support
- Confirmed Personas: agent--technical-role, givn-maintainer--owner

## Problem / Opportunity

The integrity check's failure wording says "commit hash", which is Git
vocabulary, and no scenario pins that a revision identifier recorded on
another backend passes. Q11 records the identifier as opaque: non-empty,
non-placeholder, non-duplicate — no length or alphabet assumption.

## Proposed Solution

- The integrity check's messages say "revision identifier" instead of
  "commit hash"; placeholder and duplicate rejection is unchanged.
- A new scenario pins that an opaque 40-character identifier (a Fossil-style
  check-in) passes the integrity check.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| givn-workflow-enforcement | EXTEND givn-workflow-enforcement | `EXTEND givn-workflow-enforcement` | The existing capability owns the integrity scenarios. |

## Out of Scope

Backend-specific validation of identifier shape (never needed, per Q11).

## Open Questions

None.
