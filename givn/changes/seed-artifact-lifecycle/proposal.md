# Proposal: seed-artifact-lifecycle

## Use-Case Context

- Use-case ID: none — cross-cutting guidance and tooling change; it extends
  the ideation support capability and the verification use case.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/02-seed-artifact-lifecycle.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters generated
  ideation guidance, the proposal and archive guidance, and the lint output.

## Problem / Opportunity

A change seed is the handoff record between ideation and proposal planning. It
has no content schema and no lifecycle state. A consumed seed still reads
"ready for propose"; a seed contradicted by a later decision keeps its
original instructions with no pointer to the contradiction; the proposal and
archive steps never write back. Tooling is blind: `givn new`, `givn status`,
and `givn instructions` contain no seed handling, and `givn lint` reports
nothing about seeds.

The evidence is concrete: a seed that was superseded by another topic's seed
carries no `superseded` reference, and a grep for the superseding topic across
its own topic returns zero hits; consumed seeds name no consuming change; open
questions in seeds are bare references such as "(Q16)"; and one topic invented
ad-hoc `Status — Superseded` headers because no convention exists.

A reader cannot tell whether a seed is still actionable, already consumed, or
contradicted without reconstructing the whole topic history by hand.

## Proposed Solution

Change seeds gain a fixed content schema and a lifecycle status, and the
lifecycle steps keep that status current:

- Every seed carries a status line as the first line after its title, with one
  of: `ready`, `proposed by <change-id>`, `implemented in <change-id>`,
  `superseded by <reference>`, `withdrawn`.
- The ideation Handoff writes every seed with `Status: ready`, and the seed
  schema requires the stable use-case ID, topic source, confirmed Persona IDs,
  accepted decisions with their full option treatment and rationale, open
  questions as full standalone text, and the exact handoff status and
  suggested start order.
- Proposal planning reads the seed when a change originates from one, records
  the seed path in the proposal's Use-Case Context, and stamps the seed
  `proposed by <change-id>`.
- Archiving stamps the originating seed `implemented in <change-id>` or
  `withdrawn` with a one-line reason, or records "no seed" explicitly.
- A superseding decision updates the seed to `superseded by <reference>`
  without rewriting the seed's original text; the status line is the only
  edit.
- `givn lint` reports advisory seed-status findings for seeds under
  `givn/ideation/*/changes/*.md`: a missing status line, a `proposed by` change
  that does not exist, and a `proposed by` change that is already archived. The
  findings are advisory; the lint exit-code contract is unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| ideation-skill | not surfaced (same inconclusive ranking as change 1) | `NEW in fragments` | The capability lives in the reusable support fragment and owns the generated ideation guidance that carries the seed schema and status values. |
| lint-command | not surfaced (same inconclusive ranking) | `EXTEND lint-command` | The capability owns the static lint command's findings; the advisory seed-status check extends it without changing its exit-code contract. |
| prose-consolidation | not surfaced (same inconclusive ranking) | `EXTEND prose-consolidation` | The capability owns the generated-command guidance assertions; the propose and archive seed-stamping phrases are asserted there. |

## Out of Scope

- `givn new --from-seed` scaffolding and atomic stamping (rejected M2 B).
- Automatic status transitions beyond the propose and archive stamps.
- Rewriting existing seeds or ideation topic artifacts in other projects.
- Any new runtime artifact or manifest entry for seeds.
- A hard failure on ideation state; the check stays advisory.

## Open Questions

None. The scan root and the advisory rendering are design decisions; the
required observable outcome is that the three cases are reported without
changing the exit-code contract.
