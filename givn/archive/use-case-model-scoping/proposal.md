# Proposal: use-case-model-scoping

## Use-Case Context

- Use-case ID: none — cross-cutting guidance change; it extends the canonical
  use-case contract capability and the ideation and instruction surfaces.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/04-use-case-model-scoping.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters generated
  guidance, the review criteria, and the maintainer's rule corpus.

## Problem / Opportunity

givn's formal use-case definition is one sentence in a historical release
plan, and the ideation cutting guidance is four lines. There is no oversize
signal, and the only size signal that exists — the one-capability-leaf warning
— punishes small leaves, so the tool statistically rewards absorbing
capabilities into mega-leaves. Level discipline is dead: the parser permits a
summary level with direct capabilities, and no summary document exists. The
review gate has no rejection criteria for scope.

The result in a real project: one use case with 18 capabilities, 23
unnumbered extensions, and a goal that is a conjunction of four goals; another
use case promises observation while owning a rename mutation and the entire
web server. The ideation phase had already defined six properly-sized goals;
all shipped as capabilities absorbed into the mega-leaf.

## Proposed Solution

The use-case model becomes explicit and enforceable at authoring and review
time:

- A canonical definition: a use case is a contract between one primary actor
  and the system — the actor's single goal as an observable outcome, with the
  success and minimal guarantees the system makes while pursuing it. A use
  case stays true if the system is rebuilt with different technology.
- Eight scoping rules with falsification tests: one goal and one primary
  actor; goal-shaped rather than component-shaped names; level discipline
  (leaf owns capabilities directly, summary owns only child leaves, a leaf
  with three or more actor goals must be promoted and split); mutation
  containment (observing cases own no mutating interactions); extension
  discipline (every extension names its main-flow step and the guarantee it
  protects, never a new outcome); precondition and flow consistency; and
  implementation independence.
- Authoring requirements for the generated use-case document: the primary
  actor first with supporting actors marked, numbered main-flow steps that
  pair actor intention with system responsibility, the extension format, and
  guarantees that are established rather than assumed.
- Ten review rejection criteria (R1–R10) in the review guidance and the
  design-review use-case branch, each tied to an observable document section.
- A durable document, `docs/use-case-model.md`, holds the definition, the
  rules, the authoring requirements, the rejection criteria, and the worked
  before/after example from the real project (splitting an observing mega-leaf
  into focused leaves and a substrate fragment; promoting and splitting a
  conjunction-goal leaf into the goals ideation already named). The workflow
  and reference documents point at it.
- Generated use-case document templates carry the authoring guidance, and the
  ideation Handoff maps each elaborated leaf to a permanent leaf document,
  never to a capability inside another goal.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| canonical-usecase-contract | not surfaced (route's ranking is lexical) | `EXTEND canonical-usecase-contract` | The capability owns the canonical use-case contract; the definition, the durable document, and the template guidance extend it. |
| ideation-skill | not surfaced (same ranking) | `NEW in fragments` | The capability lives in the reusable support fragment and owns the generated ideation guidance that carries the definition, the rules, and the Handoff mapping. |
| instruction-integrity | not surfaced (same ranking) | `NEW in fragments` | The capability owns the instruction and generated-guidance content checks, including the review rejection criteria and the specs placement test. |

## Out of Scope

- Parser oversize warnings, unreferenced-extension warnings, summary-with-
  capabilities errors, and demoting the one-capability-leaf warning (rejected
  T3 option B).
- Router anti-absorption, split/promote decision verbs, and archive-time
  capability moves (rejected T3 option C).
- Re-cutting existing corpora in other projects.
- New parser warnings; lint behavior for existing corpora is unchanged.

## Open Questions

None. The exact wording of the rules and criteria is a design decision; the
required observable outcome is that the generated guidance carries them, the
durable document exists and is referenced, and the template carries the
authoring guidance.
