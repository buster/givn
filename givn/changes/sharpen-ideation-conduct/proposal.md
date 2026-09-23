# Proposal: sharpen-ideation-conduct

## Use-Case Context

- Use-case ID: fragment `support` — capability `generate-ideation-guidance`
- Ideation topic: none — originates from user feedback after the
  `local-only-integrations` session (2026-09-23)
- Seed path: none
- Confirmed Personas: `developer--primary-user` (answers the questions),
  `agent--technical-role` (asks them and cuts the seeds),
  `givn-maintainer--owner` (owns the guidance corpus)

## Problem / Opportunity

Two failures surfaced while running one real ideation topic end to end.

First, the questions asked in the conversation were short and numerous. The
parked-question record already demands context, per-option meaning, an
in-practice example, gain, and cost — but the live question asked of the
human carried only a terse option list. The human had to reconstruct the
trade-offs from the files and sometimes returned a counter-question instead
of an answer. Decisions were also re-confirmed after being made: phase
reports re-listed already-agreed content for confirmation, and review
findings that followed mechanically from a confirmed decision came back as
questions.

Second, the handoff cut one change seed per use case. For one coherent
guarantee — "a project keeps Givn rules out of the repository by default" —
that produced five seeds. The first three would have shipped intermediate
states where the setting existed but `givn upgrade` still force-staged the
rules: a setting the tool does not honor. The human had to correct the cut.

Impact: the human spends attention re-deriving trade-offs and re-confirming
settled directions, and receives a change program shaped by the artifact
model instead of by shippable value.

## Proposed Solution

Observable outcomes for an ideation session:

1. Every live question carries what it blocks, the context that forces it,
   and one entry per option with its meaning, a concrete example of what the
   human sees, its gain, and its cost; the answer records the rejected
   alternatives with their reasons. A bare option list is a defect.
2. The session asks only what the current step cannot answer from confirmed
   decisions. It never asks again for a direction already decided: a review
   finding that follows mechanically from a confirmed decision is applied
   and reported; only a finding that changes a confirmed direction returns
   to the human. A phase report states what changed and invites correction;
   it is a question only when a new fork is unresolved.
3. Decisions that share one direction are asked as one question; independent
   decisions stay separate. Each decision is asked once.
4. At handoff, change seeds are cut by deliverable guarantee, not by artifact
   mapping: a seed is one change whose guarantee is true end to end and can
   ship alone. A seed may cover several use cases or fragments, and one use
   case may yield several seeds. No seed may create an intermediate state
   that makes a confirmed rule untrue — a setting no command honors, a
   decision no artifact applies. Seeds record their dependencies and a
   suggested start order.

## Capability Routing

`givn spec route --change sharpen-ideation-conduct` reports no signal (the
top candidates tie).

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `generate-ideation-guidance` | no signal | `NEW in fragments` | The capability is declared in `givn/specs/fragments/fragment.md` (support fragment); the delta lives under `specs/fragments/` per the fragment convention. |

## Out of Scope

- The parked-question record schema in the topic state; it already carries
  the full question shape and stays unchanged.
- Other dialog surfaces (explore, the grilling commands, proposal,
  design-review) beyond the reference to the canonical dialog policy they
  already consult.
- The use-case model and the definition of change size, which stay as they
  are; only the seed-cutting rule changes.
- Mechanical enforcement of question quality: no lint rule, parser, or gate
  is introduced.

## Open Questions

### D1 — Replace or remove the per-use-case seed rule

- Question: should the handoff rule become "cut seeds by deliverable
  guarantee, with the no-lying-intermediate-state test", or should the
  seed-cutting instruction be removed and left to the implementer?
- Options:
  1. **Replace the rule.** Meaning: the guidance states the deliverable cut
     and the intermediate-state test; gain: keeps the handoff artifact and
     fixes the failure mode directly; cost: one more rule to maintain; in
     practice: this session's five seeds collapse to two, and the setting
     and its enforcement ship together.
  2. **Remove the rule.** Meaning: the guidance stops telling agents how to
     cut; gain: less text; cost: the same failure recurs silently; in
     practice: an agent again returns one seed per use case.
  3. **Keep the rule.** Meaning: status quo; gain: no change; cost: the
     failure mode remains; in practice: the human corrects the cut each
     time.
- Disposition: answered 2026-09-23 (user): replace the rule with the
  deliverable-guarantee cut and the no-lying-intermediate-state test.

### D2 — Phase-boundary confirmations

- Question: what replaces "report the phase and let the user confirm before
  the next phase starts", given the demand not to reconfirm decided
  directions?
- Options:
  1. **Report and continue; ask only at unresolved forks.** Meaning: a phase
     boundary reports what was recorded, applies it, and proceeds; gain: no
     reconfirmation; cost: the human must correct a wrong recording instead
     of catching it upfront; in practice: the event chain and use-case cut
     proceed after a report unless the human corrects.
  2. **Keep explicit confirmation at every boundary.** Meaning: status quo;
     gain: catch errors early; cost: repeated confirmation of agreed
     content; in practice: every phase waits for "lgtm".
  3. **Confirm only the Frame, report the rest.** Meaning: the frame (goal,
     scope, stakeholders) is confirmed once because everything derives from
     it; later boundaries report; gain: one real gate and no noise; cost: a
     wrong later step is corrected after the fact; in practice: phases after
     the frame proceed on a report.
- Disposition: answered 2026-09-23 (user): confirm the Frame; later phase
  boundaries report what was recorded and proceed unless a new fork is
  unresolved.
