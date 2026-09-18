# Proposal: retire-lint-command-adr

## Use-Case Context

- Use-case ID: `verify-change`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user` — the naming contract serves
  the developer's CLI surface.

## Problem / Opportunity

ADR-027 ("Name the static check `givn lint`") fails the 0.8.0 falsification
tests:

- single-change reversibility: the rename is one change, one component;
- no multi-change constraint: it constrains no two future changes;
- product-behavior exclusion: the command name and exit codes are observable
  behavior ownable by the `verify-change` use case and the lint capability
  specification.

It stays in the active register only because the old gate accepted it. The
0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 3) records it as
`NOT_QUALIFIED` and names this follow-up.

## Proposed Solution

1. Move the rationale into the canonical home: the `verify-change` use-case
   Rules gain "The static check is `givn lint`; `verify.command` names the
   test runner, and lint never runs tests." The lint capability specification
   already owns the exit-code behavior.
2. Remove `docs/arc42/adr/ADR-027-lint-command-name.md`; it does not enter
   the ADR archive because a record that fails the gate is not a historical
   ADR.
3. Update ADR-019's archive record and the register README so no link points
   at the retired record; the active count drops from 25 to 24.

## Out of Scope

- The other register repairs (separate follow-ups).
- Any behavior change to `givn lint`.

## Open Questions

None.
