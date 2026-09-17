# Proposal: adr-qualification-hardening

## Use-Case Context

- Use-case ID: none — cross-cutting guidance change; it hardens the
  architecture-decision qualification gate and the design-review challenge.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/05-adr-qualification-hardening.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters the Arc42
  addon guidance, the design-review challenge, and the maintainer's rule
  corpus.

## Problem / Opportunity

The qualification gate was already hardened from OR to strict AND semantics.
The remaining problem is that the gate language is stretchable: "reversal
would require migration, broad code or interface changes" can be honestly
claimed by any persisted entity, workflow gate, or shipped feature. There is
no falsification test. The ADR template ships prefilled with the passing
verdict, so the same agent drafts the verdict and the ADR. Every change must
audition an ADR candidate through a mandatory marker, and the chapter-selection
guidance normalizes "a single change routinely triggers row 9".

The result in a real project: of 23 decision records, several are misplaced —
a project identity key split, a terminal state value, an assumption-and-go-ahead
gate, a human evaluation gate, and a change-request entity — each passing the
gate only by stretching "authority boundary" or "reversal requires migration".
One change minted three records and another minted four.

## Proposed Solution

The qualification gate gains three fail-fast falsification tests, and the
decision process stops drafting before qualifying:

- **Single-change reversibility** — a decision fully implementable and
  reversible within one change, touching one component, fails. File count and
  interface breadth do not change this. A project identity key split is the
  demonstrated non-example: it is a data-model decision owned by the design
  and the glossary.
- **No multi-change constraint** — a decision that does not constrain at least
  two future changes or two components fails. A terminal state value plus its
  presentation filtering is the non-example: feature behavior owned by the
  capability spec and design.
- **Product-behavior exclusion** — a candidate that is observable workflow
  behavior ownable by a use case plus Gherkin (who authorizes what, in which
  order, with which state values) fails the lower-level-artifact dimension
  unless it also moves a component, authority, or deployment boundary, stated
  in the verdict. An assumption-and-go-ahead gate and a human evaluation gate
  are the non-examples; a change-request entity with crash-safety requirements
  is a domain entity, not a workflow gate.
- The tests classify by kind, not count; a foundational change may legitimately
  mint several genuine ADRs, and no numeric cap is introduced.
- The structured verdict (the five gates, supporting-only shared evidence,
  qualification, routing, and the route-specific target) is recorded in the
  change's Arc42 impact assessment before any MADR file is created; the ADR
  template ships blank, forcing authorship from that verdict.
- Design-review must confirm or downgrade every proposed new ADR with an
  explicit cheaper-home challenge: name the specific canonical artifact that
  would own the rationale instead. A missing challenge is a finding.
- The chapter-selection guidance no longer normalizes row 9 as a routine
  companion: correlated rows prompt a double-check, and an affected chapter
  does not by itself qualify an ADR.
- The design template states that technology-decision justification belongs in
  the design and is not promoted to ADR candidacy unless the falsification
  tests pass.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| adr-qualification-gate | EXTEND adr-qualification-gate (advisory) | `NEW in fragments` | The capability lives in the reusable support fragment; the routing check resolves only use-case-owned capabilities, so fragment deltas record `NEW in fragments`. |

## Out of Scope

- A lightweight decision log (rejected T4 option B); the cheaper-home
  challenge must name an existing canonical home.
- A one-ADR-per-change budget (rejected T4 option D).
- Migrating existing misplaced records in other projects; a historical-scope
  note is a project decision.
- Any runtime enforcement; the qualification gate stays authoring guidance.

## Open Questions

None. The exact wording of the tests and non-examples is a design decision;
the required observable outcome is that the guidance carries them, the verdict
precedes drafting, and the template ships blank.
