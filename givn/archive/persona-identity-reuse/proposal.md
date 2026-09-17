# Proposal: persona-identity-reuse

## Use-Case Context

- Use-case ID: none — cross-cutting guidance and validation change; it
  extends the ideation support capability, the review gate, and the lint
  command.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/06-persona-identity-reuse.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters generated
  ideation guidance, the review gate, and the maintainer's rule corpus.

## Problem / Opportunity

The persona creation default is per-topic "create one or more personas" with
no inventory-existing-first step, no identity criteria, and a one-sentence
merge procedure. The schema mixes stable fields (role, vocabulary,
evaluation capability) with topic-flavored fields (interests, goals, pain
points), so every topic stance looks like a new segment. Review mechanics
reward minting: a user-visible surface requires a confirmed persona, and the
only review lens mechanism is a full persona file. Nothing validates persona
references, so dangling IDs and silent redefinition have zero cost.

The evidence is concrete: one human role was sliced into three persona files
by which port it implements, with bodies that are mostly port requirements;
one persona was rewritten with a topic stance and three divergent copies now
exist; five reused personas were copied in full and five new ones minted to
hit the cap exactly, with no canonical pointer; a change seed records a
confirmed stakeholder with "no persona file yet"; agents invented three
header dialects; and all permanent use cases record no personas at all while
active changes carry none either.

## Proposed Solution

Personas get stable identities, a reuse-first creation protocol, and
reference validation:

- A new persona exists only when it differs from an existing one in at least
  one of: its authority boundary (what it may do or decide), its evaluation
  capability (what it can judge), or its own vocabulary (terms it uses or
  refuses). The criteria are disjunctive. Topic-specific demands go into a
  dated topic-stances appendix, never into the stable core.
- A persona template carries a stable core (role, authority boundary,
  evaluation capability, own vocabulary, success criterion) and an
  append-only running record of dated entries per review or topic stance.
- Reuse comes first: inventory the project personas and every sibling topic's
  personas against the topic's stakeholder types, match by ID or record a
  one-sentence new-segment justification naming the distinguishing criterion,
  keep a reused persona's topic-local file to a header line plus new record
  entries, and append record entries to the project persona at handoff with a
  user-confirmed merged diff. One standardized header line records reuse or
  new-segment justification.
- `givn lint` reports an advisory warning when a `## Personas` entry in a
  change use-case document or an ideation use case matches no project or
  topic persona file. Forward references during an active ideation topic do
  not break anything; the warning never changes the exit code.
- `givn check review` requires a `PERSONAS:` line naming a persona that
  resolves to a file in the project persona collection; the reasoned `none`
  path for a change without a user-visible surface stays valid.
- The preserved behaviors are unchanged: user confirmation before use, the
  review duty, the cap (at most three segments per stakeholder type, about
  ten total), advisory findings with user arbitration, and handoff promotion
  with no auto-promotion. Actors stay distinct from persona biographies.
- givn's own glossary updates its persona segment and promotion rows to the
  identity criteria and the append-merge semantics.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| ideation-skill | not surfaced (route's ranking is lexical) | `NEW in fragments` | The capability owns the generated ideation guidance that carries the identity criteria, the schema, and the reuse protocol. |
| visual-review-gate | not surfaced (same ranking) | `EXTEND visual-review-gate` | The capability owns the review-time persona disposition check; the ID resolution extends it. |
| lint-command | not surfaced (same ranking) | `EXTEND lint-command` | The capability owns the static lint findings; the advisory dangling-ID warning extends it without changing the exit-code contract. |

## Out of Scope

- A lightweight persona review lens (rejected T5 option 3); no second
  persona-like artifact.
- Auto-promotion or automatic merging.
- Rewriting existing persona files in other projects.

## Open Questions

None. The exact identity-criteria wording and the resolution homes are design
decisions; the required observable outcome is that the guidance carries the
criteria and the protocol, the lint warning fires for a dangling ID, and the
review gate rejects an unresolved ID.
