# Proposal: term-introduction-discipline

## Use-Case Context

- Use-case ID: none — cross-cutting guidance change; it extends the ideation
  support capability and the domain-modeling skill.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/03-term-introduction-discipline.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters generated
  ideation and domain-modeling guidance and the maintainer's rule corpus.

## Problem / Opportunity

Agents coin domain terms freely during ideation. The terms are often abstract
and not self-explanatory, so questions that use them cannot be answered. There
is no term-introduction gate, no collision check, and no positive naming
guidance. The existing vocabulary rules are prose asking the model to judge its
own compliance, and the only deterministic hook verifies that the glossary
file is not byte-identical to its template.

The evidence is concrete: terms such as Substrate and Seam were coined and
rejected only after a user challenge, and Seam collided with the codebase's
existing test/crash-injection meaning; a Backend Port to Candidate Port rename
was proposed in a seed and never applied because no signal exists anywhere;
collisions such as Operator authority, Channel, Capability, and Delivery were
discovered mid-ideation rather than at introduction; and terms such as End
User, External Partner, and Requester are referenced everywhere but defined
nowhere.

## Proposed Solution

Ideation keeps a term record with a fixed entry schema, and a term reaches
`confirmed` only after passing a term introduction test and a recorded
collision check:

- Every entry in `domain-terms.md` records the term, its status (`candidate`,
  `confirmed`, or `rejected`), a one-sentence definition, a contrast
  ("X, not Y"), its first use (file and line), and the collision check that
  was run. A `candidate` needs only the definition and first use; a
  `confirmed` term needs the full test and the recorded check; a `rejected`
  term keeps its definition and the rejection reason and is never deleted.
- Before a term reaches `confirmed`, the author runs and records a
  search-before-coin procedure: grep the durable glossary for the term and its
  synonyms, grep every sibling topic's term record and the repository, and
  record the commands and hit counts. "No collision" without recorded commands
  is not a record.
- A term introduction test gates `confirmed`: a genus and differentia
  definition ("a Port is the interface contract at which one implementation
  can be replaced"), a contrast pair ("Port, not Boundary — a boundary is a
  limit that must not be crossed"), and one concrete domain sentence using the
  term. When the genus slot cannot be filled, the term is not ready; the
  Substrate failure ("a Substrate is a kind of ___" has no referent) is the
  demonstrated non-example.
- The ideation flow integrates the record: the Event Storming collision step
  points to the procedure, the quality review verifies that every collision
  entry has a recorded check, and Handoff proposes only `confirmed` terms for
  the durable term list while reporting `rejected` terms with their reason.
- The human-facing workflow documentation names the term record and points at
  the skill as the canonical schema; the schema is not duplicated.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| ideation-skill | not surfaced (same inconclusive ranking as earlier changes) | `NEW in fragments` | The capability lives in the reusable support fragment and owns the generated ideation guidance that carries the term schema, the collision procedure, and the review branch. |
| domain-skill-trigger | not surfaced (same inconclusive ranking) | `NEW in fragments` | The capability owns the generated domain-modeling guidance that carries the term introduction test. |

## Out of Scope

- Machine-checkable `TERMS:` declarations and a parser (rejected T2 option 1).
- A glossary scaffold upgrade and an anti-term lint (rejected T2 option 3);
  the scaffold/instruction column mismatch remains accepted debt.
- Migrating existing term records in other projects.
- Any deterministic term checker; this stays authoring guidance.

## Open Questions

None. The exact schema wording and the review branch are design decisions; the
required observable outcome is that the generated guidance carries the schema,
the collision procedure, and the test.
