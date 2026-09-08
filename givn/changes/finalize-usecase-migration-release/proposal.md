# Proposal: finalize-usecase-migration-release

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: maintain-project
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

The use-case migration implementation and consumer dogfoods are complete, but
the release closure is not yet machine-checkable. The release bundle still names
an unreleased target, the Givn corpus migration has no retained before/after
receipt, and the migration gate's failure cases are not executable scenarios.
Maintainers cannot therefore distinguish a complete 0.6.0 release from a
locally successful migration implementation.

## Proposed Solution

The maintainer can inspect a concrete 0.5.0-to-0.6.0 migration bundle whose
semantic operations, evidence requirements, and archive protections are
explicit. The release checks report migration behavior, E2E, interaction, line,
and branch coverage regressions as blocking findings. The release documentation
records that the canonical model, coverage gate, and ideation/persona guidance
were delivered together, while Watn and Kiekste retain their committed
migration receipts. Historical archives and Kiekste's pending Handoff remain
unchanged.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| givn-workflow-enforcement | EXTEND givn-workflow-enforcement | EXTEND givn-workflow-enforcement | The existing verification capability owns deterministic review and archive protection; the release bundle remains the documented output. |

## Out of Scope

- No observable product behavior is changed.
- Historical content under `givn/archive/` is not rewritten.
- Kiekste Handoff, Persona promotion, and unresolved ideation questions are not performed.
- No compatibility reader for the retired group model is added.

## Open Questions

- The target version is fixed at `0.6.0` for this release bundle.
