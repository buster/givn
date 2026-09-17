# Proposal: guarantee-question-traceability

## Use-Case Context

- Use-case ID: none — cross-cutting guidance change; it extends the ideation
  support capability and the proposal, design, and design-review surfaces.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/07-guarantee-question-traceability.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters generated
  ideation and workflow guidance and the maintainer's rule corpus.

## Problem / Opportunity

Use cases assert unconditional guarantees while their own open questions
admit the mechanism is undecided. A use case promises that a branch changes
only by compare-and-swap while another rule concedes that how the tool
implements it is open; another promises at-most-once delivery while
idempotency is an open question. Proposals systematically defer every
question with "None blocking… resolved in `design.md`", which is
uncheckable. No artifact owns a ledger linking question to decision to
location, so the design becomes an unindexed decision log and the proposal
gate's purpose — surface unknowns before specs — is short-circuited.

## Proposed Solution

Open prerequisites and proposal questions become traceable:

- A rule or guarantee that depends on an unresolved question carries an
  `(open: Q<N>)` qualifier naming the question in the ideation question log.
  The qualifier stays until the question closes; closing the question removes
  the qualifier in the same edit. An unqualified guarantee whose prerequisite
  is open is a review finding.
- When a change comes from an ideation seed, every seed question that
  intersects the proposal scope is dispositioned in the proposal before
  specs: **answered** with its source, **carried** with an owner and the
  artifact that will decide it, or **out of scope** with a reason. An
  undispositioned intersecting question blocks the specs step; for changes
  without an ideation source the same structure is advisory. "Resolved in
  `design.md`" alone is not a disposition.
- The proposal's open questions get stable IDs (`D1`, `D2`, …) and carry the
  full question with the dialog policy's option treatment plus the
  disposition.
- The design records a decisions ledger: `Decides D1: <decision> — <where it
  is realized>`. Every `D<n>` either appears there or is explicitly carried
  with a named owner.
- Design-review verifies that every `D<n>` resolves or is carried; an
  unresolved, unrecorded item is a finding.
- The proposal, design, and design-review templates show the shapes; the
  proposal and design instructions describe the contract.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| ideation-skill | not surfaced (route's ranking is lexical) | `NEW in fragments` | The capability owns the generated ideation guidance that carries the qualifier rule. |
| instruction-integrity | not surfaced (same ranking) | `NEW in fragments` | The capability owns the instruction-content checks; the proposal disposition rule, the design ledger contract, and the design-review verification are asserted there. |

## Out of Scope

- Runtime enforcement of the qualifier or the ledger; both are authoring
  contracts checked by review.
- Automatic question answering or inferred dispositions.
- Retro-fitting existing proposals and use cases in other projects.
- A parser or a second numbering scheme; the qualifier and the `D<n>` IDs stay
  grep-able text.

## Open Questions

None. The exact wording is a design decision; the required observable outcome
is that the generated guidance carries the qualifier rule, the disposition
rule, and the ledger contract.
