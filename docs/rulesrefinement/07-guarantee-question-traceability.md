# 07 — guarantee-question-traceability

Status: ready for `/givn-propose`
Source: guarantees over open prerequisites (M5 options A and B)
Depends on: 02 (seed schema), 04 (use-case model)

## Problem

Use cases assert unconditional guarantees while their own open questions
admit the mechanism is undecided. Proposals systematically defer every
question with "None blocking… resolved in `design.md`", which is
uncheckable. No artifact owns a ledger linking question to decision to
location, so `design.md` becomes an unindexed decision log and the proposal
gate's purpose — surface unknowns before specs — is short-circuited.

## Evidence

kiekste side:

- `givn/ideation/fossil-scm-integration/use-cases/publish-fossil-change.md`:
  asserts R2 ("`main` changes only by compare-and-swap") and R4
  ("at-most-once across crashes") while R8 concedes "How Fossil implements
  the compare-and-swap guarantee … is open (Q5)".
- `givn/ideation/fossil-scm-integration/use-cases/file-ticket-and-queue-work.md`:
  asserts R8/R10/R11 (agent ticket updates, ticket question-loop, no silent
  loss) while mechanism, authority, and ref placement are open (Q7, gap 5).
- `givn/ideation/interchangeable-building-blocks/use-cases/act-from-a-port.md`:
  minimal guarantee "recorded exactly once" while idempotency (Q5) and the
  observation mechanism (Q16) are open.
- `givn/ideation/interchangeable-building-blocks/use-cases/read-from-a-port.md`:
  promises "the Viewer's scope explicit" while the grant question is parked.
- All 18 active kiekste changes: proposal Open Questions read "None
  blocking… resolved in `design.md`".
- `docs/architecture/target-architecture.md` migration step 1 independently
  rediscovers the rule: "no use case promises a guarantee while the
  semantics required to provide it remain open or outside its scope".

givn side:

- `assets/skills/givn-ideation/SKILL.md.tmpl` lines 98–127: use-case schema
  with Rules and guarantees sections; no qualifier for open dependencies.
- `assets/instructions/proposal.md` lines 28–32: Open Questions listed with
  no disposition rule (blocking vs carried vs dropped).
- `assets/templates/proposal.md` lines 39–41: one-line prompt.
- `assets/templates/design.md`: no section mapping decisions back to
  questions.
- `assets/instructions/design-review.md`: no ledger check.

## Decided scope

1. `(open: Qn)` qualifiers on use-case rules and guarantees, with a
   fail-closed propose step for ideation-sourced changes. (M5 A)
2. A decision ledger: proposal questions get `D1…` IDs; design records
   `Decides D1: …`; design-review verifies every item resolves. (M5 B)

## Required changes

### 1. Guarantee and rule qualifiers

- Ideation skill: a rule or guarantee that depends on an unresolved question
  carries `(open: Q<N>)` after the statement, naming the question in
  `questions.md`.
- A rule or guarantee with an open qualifier must not be presented as
  unconditional; the review criteria (change 04) treat an unqualified
  guarantee whose prerequisite is open as a finding.
- The qualifier stays until the question closes; closing the question removes
  the qualifier in the same edit.

### 2. Fail-closed propose for ideation-sourced changes

- `assets/commands/givn-propose.md` and `assets/instructions/proposal.md`:
  when a change comes from an ideation seed, every open question in the seed
  that intersects the proposal scope must be dispositioned in the proposal
  before specs:
  - **answered** — the answer is recorded, with its source;
  - **carried** — kept open with an owner and the artifact that will decide
    it;
  - **out of scope** — explicitly excluded with a reason.
- An undispositioned intersecting question blocks the specs step. For
  changes without an ideation source, the same structure is advisory.
- The proposal records the disposition next to each question; "resolved in
  `design.md`" alone is not a disposition.

### 3. Decision ledger

- Proposal `## Open Questions`: each entry gets an ID `D1`, `D2`, … plus the
  full question with the option treatment from the dialog policy (context,
  per-option meaning, gain, cost, concrete example) and its disposition from
  (2).
- `design.md`: a `## Decisions` section records `Decides D1: <decision> —
  <where it is realized>`. Every `D<n>` either appears here or is explicitly
  carried with a named owner.
- Design-review: a checklist item verifies every `D<n>` resolves or is
  carried; an unresolved, unrecorded item is a finding.

### 4. Templates

- `assets/templates/proposal.md`: Open Questions section shows the
  `D<n>` + disposition shape.
- `assets/templates/design.md`: add the `## Decisions` section with the
  `Decides D<n>` shape.
- `assets/templates/design-review.md`: add the ledger checklist item.
- `assets/instructions/design.md`: describe the ledger contract.

## Files to change

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | `(open: Q<n>)` qualifier rule |
| `assets/commands/givn-propose.md` | Fail-closed disposition for ideation-sourced changes |
| `assets/instructions/proposal.md` | Disposition rule; `D<n>` IDs |
| `assets/templates/proposal.md` | Question and disposition shape |
| `assets/instructions/design.md` | Ledger contract |
| `assets/templates/design.md` | `## Decisions` section |
| `assets/instructions/design-review.md` | Ledger verification requirement |
| `assets/commands/givn-design-review.md` | Same, in the grilling checklist |
| `assets/templates/design-review.md` | Ledger checklist item |

## Executable-spec impact

- `givn/specs/fragments/ideation-skill.feature`: assert the generated
  guidance contains the `(open:` qualifier rule.
- `givn/specs/fragments/instruction-integrity.feature`: assert the proposal
  instruction carries the disposition rule and the design instruction
  carries the ledger contract.
- `givn/specs/verify-change/review-disposition-gates.feature`: scenario for
  an unresolved `D<n>` producing a design-review finding.
- Check `givn/specs/fragments/skill-rewrites.feature` for phrase assertions
  on the edited artifacts.

## Acceptance criteria

- [ ] A guarantee with `(open: Q5)` cannot pass propose for an
      ideation-sourced change until Q5 is answered, carried with an owner,
      or excluded.
- [ ] A proposal whose Open Questions say only "resolved in `design.md`"
      fails the disposition rule.
- [ ] Design-review reports an unresolved, unrecorded `D<n>` as a finding.
- [ ] Changes without an ideation source get the structure advisory, not
      blocking.

## Out of scope

- Runtime enforcement of the qualifier or the ledger; both are authoring
  contracts checked by review.
- Automatic question answering or inference of dispositions.
- Retro-fitting kiekste's existing proposals and use cases.

## Risks and notes

- Bureaucratic overhead on small changes: scope the hard requirement to
  ideation-sourced changes only; everything else is advisory.
- The qualifier and the `D<n>` IDs must stay grep-able and simple; do not
  introduce a parser or a second numbering scheme.
- Interaction with change 04: the rejection criteria treat an unqualified
  guarantee with an open prerequisite as a finding. Keep the wording in one
  canonical place (the ideation skill) and reference it from the review
  instruction.
