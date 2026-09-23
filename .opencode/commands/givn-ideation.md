---
description: Work a vague idea into change-ready use cases — structured, multi-session domain exploration
---

Work a vague idea into change-ready use cases.

**Never write givn change artifacts or code during ideation.** The outcome is
topic state and change seeds, produced at handoff.

**Input**: The argument after `/givn-ideation` is an optional topic id. A
topic id names one ideation effort; without one, derive it from the user's
description and confirm it with the user.

---

## Stance

- Every question carries the full option treatment; a direction already decided is never asked again.
- Follow the canonical dialog policy (`givn instructions dialog`): every
  question answerable standalone, every option with its meaning, gain, cost,
  and a concrete example.
- The user confirms every persona, every recorded decision, every update.
- Findings from reviews and personas are advisory; the user arbitrates.
- Park unknowns in the open question log instead of debating them.

---

## givn awareness

Check for context at the start:

```sh
givn status
```

Resume an existing topic by reading its state first (see the skill's topic
state layout); otherwise start a new topic.

---

## Full phases

Read the `givn-ideation` skill in full and follow it. The phases:

Frame → Event Storming → Use Case Definition → Use Case Elaboration → Handoff

---

## Guardrails

- Never write givn change artifacts — change seeds only, at handoff.
- Handoff never starts a change — the user runs the normal proposal flow.
- Never invent a persona the user has not confirmed.
- Reuse before minting: inventory the project personas and every sibling
  topic's personas, then match by ID or record a one-sentence justification
  naming the distinguishing criterion (authority boundary, evaluation
  capability, or own vocabulary). A reused topic file is record-only.
- Draft use cases with the permanent `usecase.md` schema and stable kebab-case
  use-case IDs. The same ID must survive topic state, change seed, change-side
  `usecase.md`, and permanent archive.
- Keep confirmed Persona IDs separate from Actors. Actors appear in Gherkin;
  Persona biographies remain human-reviewed Markdown under `givn/personas/`.
- Handoff presents the use-case mapping, Persona promotion decisions, source
  topic, open questions, and change seeds for explicit confirmation. It never
  silently promotes a Persona or performs Handoff as a side effect.
- Handoff writes every change seed with `Status: ready` and the seed schema
  from the `givn-ideation` skill.
- Cut seeds by deliverable guarantee; a confirmed rule must stay true.
- The Frame is confirmed once; later phase boundaries report and proceed.
