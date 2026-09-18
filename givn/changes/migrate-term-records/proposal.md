# Proposal: migrate-term-records

## Use-Case Context

- Use-case ID: none (project-maintenance repair; `specs` is skipped)
- Ideation topic: `multi-scm-support`
- Seed path: none
- Confirmed Personas: none — the repair edits the topic's term file; no
  user-visible surface changes.

## Problem / Opportunity

`givn/ideation/multi-scm-support/domain-terms.md` still uses the pre-0.8.0
two-column candidate table. The term schema the `term-introduction-discipline`
change introduced requires term, status, definition, contrast, first use, and
a recorded collision check. The nine terms merged into
`docs/arc42/12-glossary.md` are de-facto confirmed without statuses, and the
per-SCM terms carry no disposition.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 4) records this
deviation and names this follow-up; it also recorded the collision-check
evidence in the old cells.

## Proposed Solution

Rewrite the file to the term schema:

1. Nine confirmed terms (SCM backend, Revision, Checkout, Clean state,
   Candidate revision, Tested revision, Revision binding, Upgrade revision,
   Plan revision) carry the glossary definition, an anti-term contrast, the
   glossary line as first use, and the recorded collision evidence.
2. Repository root and Index / staging area are rejected with their
   definitions and reasons (superseded by Checkout; the domain must not
   require an index).
3. Hook and Check-in (Fossil) stay candidates: per-SCM terms used only
   qualified, staying in the backend seam; the Hook collision with the
   glossary's `Hook (post)` is recorded.

## Out of Scope

- Re-running the collision searches; the evidence was recorded by the
  migration change on 2026-09-18.
- Adding or removing terms.

## Open Questions

None.
