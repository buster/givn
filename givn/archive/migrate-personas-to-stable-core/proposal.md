# Proposal: migrate-personas-to-stable-core

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: `multi-scm-support`
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — the subjects of the repair.

## Problem / Opportunity

The three permanent personas predate the persona schema the
`persona-identity-reuse` change introduced. They use pre-0.8.0 sections
(`Role`, `Interests`, `Goals`, `Pain points`, `Success criterion`,
`Vocabulary`, an undated log) instead of the stable core, the running record,
and topic stances. They also merge two segments each without stating which
identity criterion (authority boundary, evaluation capability, or own
vocabulary) the segments share, so the merge is justified by narrative
rather than the disjunctive criteria.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 2) records this
deviation and names this follow-up.

## Proposed Solution

Convert each persona to the schema:

1. Stable core with Role, Authority boundary, Evaluation capability, Own
   vocabulary, and Success criterion. The Role line states the shared
   identity criterion for the merged segments; the old interests, goals, and
   pain points are folded into the boundary, evaluation, and vocabulary
   fields.
2. Running record carrying the existing dated log entries.
3. Topic stances carrying the recorded topic-specific demands.

## Out of Scope

- Splitting a persona; each merge is justified by a shared criterion.
- Inventing reviews, needs, or stances that were never recorded.
- The topic-local reuse records (repaired by
  `convert-topic-persona-copies`).

## Open Questions

None.
