# Proposal: reshape-archive-change

## Use-Case Context

- Use-case ID: `archive-change`
- Ideation topic: `multi-scm-support`
- Seed path: `givn/ideation/multi-scm-support/changes/archive-change.md`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current `archive-change` document:

- **R1 goal conjunction** — the Goal joins "Publish … and prove …", and the
  Success guarantee names two outcomes.
- **R5 extension discipline** — all eight extensions lack the main-flow step
  reference and the protected guarantee.
- **R7 mechanism vocabulary** — "active backend" and "backend revision"
  appear in the Main flow and the Success guarantee.
- **R10 relationship misuse** — the leaf Includes `usecase: verify-change`;
  Includes is summary-to-child or leaf-to-fragment only.
- **R8 channel/mechanism name** — `preserve-docstrings-on-archive` names a
  code mechanism.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records these
deviations and names this follow-up.

## Proposed Solution

1. Rewrite the Goal as one phrase ("Archive a completed change with proven
   publication") and the Success guarantee as one outcome; archiving is one
   goal, so no split is needed.
2. Rewrite all eight extensions as `<step>: <condition> → <handling>;
   protects <guarantee>`.
3. Replace "backend" with the domain phrase "version-control system" in the
   Main flow and the Success guarantee; the confirmed glossary terms
   (`revision`, `candidate revision`, `tested revision`) stay.
4. Remove the `Includes: usecase: verify-change` relationship; the archive
   runs its own gates, so there is no unconditional reuse to model. The
   `fragment: support` include stays.
5. Rename `preserve-docstrings-on-archive` to
   `preserve-scenario-docstrings`; the feature file moves with it.

## Out of Scope

- Any behavior change; the scenarios keep their assertions.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
