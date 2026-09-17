# Proposal: dialog-question-quality

## Use-Case Context

- Use-case ID: none — cross-cutting guidance change; it extends support
  capabilities consumed by the authoring and review use cases.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/01-dialog-question-quality.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters generated
  agent guidance and the maintainer's rule corpus.

## Problem / Opportunity

Every interactive step in the lifecycle asks the user questions: ideation,
exploration, the two grilling commands, the ambiguity check during proposal
planning, the design-review grilling, review dispositions, and archive
confirmations. Exactly one of those steps carries a rule about what a question
must contain. Everywhere else an agent is free to compress a question into
codes the reader cannot decode: lettered options whose meanings exist nowhere
("A now, C later, B possibly in between"), bare cross-references ("(Q16)",
"(R-033/R-035/R-052 family)"), and outcome-only resolutions ("Rule settled: B").

The records that carry questions compress the context away too. The ideation
question log has no fixed entry shape, so a parked unknown can be a single
line. The design-review grilling log is one table with a single `Question`
cell, so a decision's context, options, and rejected alternatives have no home.
A reader must open several other files and reconstruct the question from memory
to answer it — and some questions cannot be answered at all.

## Proposed Solution

One canonical dialog policy governs how every interactive step phrases and
records a question. An author can request it through the Givn CLI as the
`dialog` instruction. The policy states, in one page:

- Name every artifact in domain language with its identifier in parentheses.
- Every option states what it means, what it gains, what it costs, and one
  concrete example of what the user sees or what the system does. A bare
  letter or abstract label is never an option.
- Every cross-reference carries a one-line gloss on first use per file or
  turn; the reader must not need to open another file.
- A question is answerable standalone: it names the decision it blocks, the
  context that forces it, the options with gains and costs, and a concrete
  example of each.
- The rejected alternatives are recorded with the resolution, not only the
  chosen outcome.
- One question at a time; wait for the answer.
- Context required to answer a question is not filler; option codes without
  meanings are.

Every interactive step's generated guidance references this policy instead of
restating it; worked examples may stay where they are.

The ideation question log gains a fixed entry shape: title, status, what the
question blocks, context, the standalone question, one entry per option with
its meaning, concrete example, gain, and cost, and a resolution that records
the chosen option with its reason and the rejected alternatives with theirs. A
closed question keeps its full record; an outcome-only resolution is not a
valid record.

The design-review grilling record keeps its scannable index table and adds the
full per-question record below it, using the same entry shape. Operator
decisions presented during grilling carry the full option treatment, and the
record carries the rejected alternatives.

The Absolute Mode policy gains the "context is not filler" line without
weakening its compression rules.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| instruction-integrity | not surfaced (top: release-0-3-0, 6.00 — matches only on the word "spec") | `NEW in fragments` | The capability lives in the reusable support fragment; the routing check resolves only use-case-owned capabilities, so fragment deltas record `NEW in fragments`. It owns the canonical instruction retrieval contract and the policy-content scenarios. |
| ideation-skill | not surfaced (same inconclusive ranking) | `NEW in fragments` | Same fragment rule; the capability owns the generated ideation guidance that carries the question-entry schema and the policy reference. |
| prose-consolidation | not surfaced (same inconclusive ranking) | `EXTEND prose-consolidation` | The capability owns the generated-guidance-points-to-the-canonical-policy scenario; the dialog policy pointer extends that behavior. |
| install-extras | not surfaced (same inconclusive ranking) | `EXTEND install-extras` | The capability owns addon enable behavior; the absolute-mode policy block's new dialog line is asserted there. |

## Out of Scope

- A separate answerability checklist gate at phase reviews (rejected T1 D).
- Runtime validation of question text; this remains authoring guidance.
- Rewriting question records in projects other than givn.
- The design-review gate policy itself, which change 5 owns; only the
  dialog-discipline wording moves out of it.
- The seed lifecycle, term discipline, use-case scoping, ADR qualification,
  persona reuse, guarantee traceability, and finding-severity work.

## Open Questions

None. Whether the canonical policy is reachable through the existing
instruction lookup without new wiring is a design question, not a scope
question; the required observable outcome is that the CLI serves it.
