# Proposal: convert-topic-persona-copies

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: `multi-scm-support`
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — the topic copies are the subjects of the repair.

## Problem / Opportunity

`givn/ideation/multi-scm-support/personas/` holds three files that are
byte-identical biography copies of the project personas. The persona
identity and reuse rule (`persona-identity-reuse`) says a reused persona's
topic-local file contains a reuse header plus new record entries only —
never a biography copy; the biography lives in exactly one place. Copies
drift and invite contradictory edits.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 2) records this
deviation and names this follow-up.

## Proposed Solution

Replace each topic copy with the reuse record:

- a title marked as the topic record;
- the schema's `Reuse: <persona-id>, base <short-hash>; this file adds record
  entries only` header, with the base being the current permanent file's
  content hash;
- an honest note that the topic added no entries, with the biography path.

## Out of Scope

- Changing the permanent persona files (their schema migration is the
  `migrate-personas-to-stable-core` follow-up).
- Adding topic stances or record entries that were never recorded.

## Open Questions

None.
