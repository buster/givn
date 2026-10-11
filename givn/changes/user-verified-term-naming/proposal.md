# Proposal: user-verified-term-naming

## Release note

Givn asks the user to verify every newly coined domain term — failed names
immediately, with the concept's description and concrete candidate names, the
rest at the phase's term check — before any canvas, use case, or change seed
relies on it.

## Use-Case Context

- Use-case ID: fragment `support` — capabilities `generate-ideation-guidance`
  and `guide-domain-modeling`.
- Ideation topic: none — originates from direct operator feedback in the
  term-naming design discussion (2026-10-11).
- Seed path: none.
- Confirmed Personas: `developer--primary-user` (answers the naming question
  during ideation), `agent--technical-role` (runs the naming dialog and the
  phase term check), `givn-maintainer--owner` (owns the guidance and
  vocabulary corpus). Reuse the existing confirmed project records without
  creating or promoting personas.

## Problem / Opportunity

Agents coin domain terms during ideation and often settle on names that do
not fit: generic, overloaded, or a boundary choice the user would decide
differently. The user is not asked to verify the name. The guidance's current
remedy is "Propose it and justify it" — the agent still chooses alone, offers
no alternatives, and records no user decision. A term becomes `confirmed`
when its introduction test and collision check pass; there is no user gate,
and the first explicit user decision point is Handoff, when canvases, use
cases, and change seeds already build on the name and a rename ripples
through every artifact.

The evidence is concrete. In a two-person connection app, ideation coined
**Meeting** for the moment two people who met in person decide to connect;
the user found the name unfitting and **Handshake** was the better one. The
correction depended on the user spotting the problem — the agent had no
dialog to offer alternatives and no place to record why the first name was
replaced. The choice was also a boundary decision, not a wording preference:
the occasion that contains the mutual acceptance and the mutual acceptance
itself are different invariants, and only the domain owner can settle which
one the term names.

Detectable failures follow the same path: a term with no fillable genus (the
Substrate example) or a generic word (`list`, `item`) is coinable today and
reaches `confirmed` without the user ever seeing the name.

## Proposed Solution

### Requirements (observable outcomes)

1. **Immediate naming dialog on a failed name.** When a newly coined term is
   generic, overloaded, cannot fill its genus slot, or genuinely competing
   candidate names exist, the agent asks the user one naming question
   immediately: it describes the concept in the domain's language, proposes
   up to three candidate names — each with what it evokes and any collision
   it carries — and accepts the user's own name.
2. **Phase term check for the rest.** Every other new core term — a new
   aggregate root, a use-case goal noun, or an event name that introduces a
   noun — rides the phase's closing checkpoint as one question: "Term check —
   <phase>: <N> new core terms. Confirm all, or name the terms to revisit."
   Each listed term carries its one-sentence description and its recorded
   collision check. Confirming all makes the terms `confirmed` and the next
   phase builds on them; naming terms to revisit opens only those terms'
   naming dialogs, one at a time, before confirmation — the other terms
   stand and nothing else re-opens.
3. **The user verifies every `confirmed` term.** A term reaches `confirmed`
   only after the user's name verification, through the phase term check or
   the term's own naming dialog. Handoff is no longer the first user
   decision point for a term.
4. **A user-supplied name is checked like any other.** When the user answers
   with their own name, it gets the same collision check and term
   introduction test as a proposed candidate before the term can be
   confirmed.
5. **Replaced names are recorded.** A name the user replaces becomes an
   anti-term of the confirmed term (feeding the Contrast column and the
   glossary's anti-terms); a rejected candidate is recorded `rejected` with
   its reason and is never deleted.
6. **Outside ideation the dialog still fires.** When the agent coins or
   challenges a term during proposal, specs, or design, the immediate naming
   dialog applies; there is no batch list there because no phase checkpoint
   exists to attach one to.
7. **The gate survives without the addon.** The one-line requirement that a
   `confirmed` term carries the user's verified name stays unconditional in
   the term record, so the gate works with the domain-modeling addon off;
   the dialog's full wording lives in the domain-modeling skill.
8. **The command guardrails repeat the rule in one line**, so an agent that
   reads only the ideation command still asks the user before building on a
   term.

### The rules the generated guidance must carry

Draft wording, for the implementer — the exact strings are pinned in the
specs and placed by the design.

The domain-modeling skill's challenge bullet (replacing "Propose it and
justify it"):

```text
- **Challenge generic or overloaded terms.** When conversation or the specs use
  a word that could mean many things ("list", "item", "user", "trip"), force a
  precise domain noun. When the name matters, do not choose it alone: describe
  the concept in one sentence, propose up to three candidate names with what
  each evokes and any collision it carries, and accept the user's own name.
  Example: "You wrote 'list'. The concept is 'the set of items one household
  plans to buy together'. Candidates: ShoppingList (the household's shared
  plan), Basket (short, but collides with the store's checkout basket). Which
  name should the record use, or what is yours?"
```

The ideation term record: the `confirmed` requirement line gains the user's
verified name:

```text
- `confirmed` — requires a genus-and-differentia definition, a contrast pair,
  one concrete domain sentence, the user's verified name, and a recorded
  collision check. The full term introduction test lives in
  `givn-domain-modeling`.
```

The ideation term check (a new subsection or bullet; placement is a design
decision):

```text
- The user verifies every new core term before it is used. When a coined term
  is generic, overloaded, cannot fill its genus slot, or has competing
  candidates, ask the naming question immediately. Every other new core term
  rides the phase's closing checkpoint as one question: "Term check — <phase>:
  <N> new core terms. Confirm all, or name the terms to revisit." Each row
  gives the term, its one-sentence description, and the recorded collision
  check. Confirm all makes them `confirmed`; a term named for revisit gets its
  naming dialog, one at a time, before confirmation. A name the user supplies
  is collision-checked like any other; a replaced name becomes an anti-term.
```

The ideation command guardrail line:

```text
- The user verifies every new core term — by the phase term check or the
  term's naming dialog — before the canvas, use cases, or seeds rely on it.
```

### Example — the term check that would have caught Meeting

At the Event Storming closing checkpoint the question reads:

```text
Term check — Event Storming: 2 new core terms. Confirm all, or name the terms
to revisit.

| Term    | Description                                                              | Collision check |
|---------|--------------------------------------------------------------------------|-----------------|
| Meeting | the occasion on which two people who met in person decide to connect     | no hits in the glossary, sibling topics, or the repository |
| Vouch   | the one-sided statement by which an existing member stands behind a newcomer | no hits |
```

The user answers "Revisit Meeting". The naming dialog describes the concept —
"the two-sided acceptance by which two people who met in person become
connected; both sides must agree" — and proposes: Meeting (the occasion that
contains the acceptance), Handshake (the acceptance itself), Connection
(short, but overloads with the network meaning in the codebase), or the
user's own name. The user picks Handshake. Meeting is recorded as an
anti-term of Handshake and as a `rejected` candidate with the reason ("names
the occasion, not the mutual acceptance"); Vouch was confirmed with the rest.
The same dialog fires immediately, without waiting for a checkpoint, when the
coined name fails outright — a genus that cannot be filled or a word from
the generic list.

## Capability Routing

| Proposed capability | Decision | Rationale |
|---|---|---|
| `generate-ideation-guidance` | `EXTEND generate-ideation-guidance` | Route reports no signal. The capability is declared in `givn/specs/fragments/fragment.md`; this change adds the user-verification gate, the phase term check, and the command guardrail line to the generated ideation guidance it owns, so the delta lives under `specs/fragments/` per the fragment convention. |
| `guide-domain-modeling` | `EXTEND guide-domain-modeling` | Route reports no signal. The capability owns the generated domain-modeling guidance; this change replaces the challenge bullet's remedy with the naming dialog, so the delta lives under `specs/fragments/` per the fragment convention. |

## Out of Scope

- Machine enforcement: no lint rule, parser, or gate inspects names or blocks
  a status flip; the fix is the dialog and the user's answer, exercised in
  conversation (accepted risk, consistent with the guidance-only posture of
  the term-introduction-discipline change).
- Changing the term introduction test or the search-before-coin procedure;
  the user's verification binds to their outcome, it does not replace them.
- A new term-record column, artifact type, or generated file; the schema
  stays as the term-introduction-discipline change set it.
- Retroactive renames in consumer projects and archived topics.
- The Handoff durable-term-list decision; it still exists, now with names
  already verified.
- Persona confirmation rules and the addon trigger routing.

## Open Questions

No unresolved operator decision blocks specification; the mechanism was
settled with the user on 2026-10-11. The entries below record the settled
decisions; they are not live questions to ask again.

### D1 — How the user verifies term names

- Context: terms are coined throughout ideation; asking about every name at
  coining time interrupts the flow, asking only at Handoff is too late.
- Question: at which point does the user verify a newly coined term's name?
- Options:
  1. **Immediate naming question for every new term** — meaning: the agent
     asks at each coining; gain: the earliest possible verification; cost:
     a question interrupts the flow at every new noun. In practice: each new
     event noun prompts a question before the canvas can continue.
  2. **One verification list at phase closing** — meaning: the agent
     collects names and asks once per phase; gain: minimal interruption;
     cost: a bad name shapes the whole phase's thinking before the user
     sees it. In practice: the canvas is drafted with Meeting and corrected
     only at the checkpoint.
  3. **Immediate dialog for names that fail the test, phase check for the
     rest** — meaning: generic, genus-less, or competing names are asked at
     coining; every other core term rides the phase closing question;
     flagged terms get individual dialogs, one at a time; gain: failed
     names never propagate and sound names do not interrupt; cost: two
     paths to keep consistent. In practice: Meeting rides the check,
     a genus-less Substrate is asked immediately.
- Disposition: answered — the user chose option 3 in the term-naming design
  discussion (2026-10-11), including the request that the question contain
  the concept's description and offer candidate names plus the user's own.

### D2 — Where the verification guidance lives

- Context: the naming dialog must be available whenever terms are named
  (ideation and later phases), while the term record's status machine only
  exists in ideation.
- Question: does the naming dialog and its triggers live in the
  domain-modeling skill, in the ideation rules, or in both?
- Options:
  1. **All in the ideation rules** — meaning: the dialog exists only in
     ideation; gain: one owner; cost: proposal, specs, and design names
     stay agent-decided, and ideation restates the term rules. In practice:
     a design-time generic noun goes unchallenged.
  2. **All in the domain-modeling skill** — meaning: the dialog lives with
     the discipline and the term record points at it; gain: one dialog
     definition; cost: the skill only routes at specs and design (and only
     with the addon), so ideation — where most terms are coined — would not
     see it and the user gate would depend on the addon. In practice: a
     project without the addon coins terms unverified.
  3. **Dialog in the domain-modeling skill, binding in the ideation
     rules** — meaning: the skill defines when and how to ask, covering
     proposal, specs, design, and review; the ideation guidance binds the
     user's answer to the term record's `confirmed` transition and keeps a
     one-line unconditional gate; gain: every phase covered, ideation
     covered without the addon; cost: the ideation text references the
     skill and must stay consistent with it. In practice: the skill carries
     the dialog; the term record line names the user's verified name.
- Disposition: answered — the user accepted option 3 in the term-naming
  design discussion (2026-10-11), after asking whether verification belongs
  in the domain-modeling skill or the ideation rules.
