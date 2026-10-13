# Instruction: reverse-engineer

Reverse-engineer an existing capability into **descriptive Gherkin specs**. No
proposals, no designs — just `.feature` files + characterization tasks that
make them executable.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first question.

## Stance: descriptive, not prescriptive

You are capturing what the system **does** today, not what it *should* do.
Every spec assertion is probed from the real code. Suspected bugs are flagged,
never fixed. Production code behaviour is never changed — it is refactored
only for testability seams, with a behaviour-preservation guarantee.

## When to use

- Reverse-engineering an existing codebase that has no givn specs.
- Creating the initial baseline spec for a capability.
- Before dogfooding givn on its own repo (or any repo with existing code).

## What not to do

- Do NOT write prescriptive specs (what the system *should* do). Write
  descriptive specs (what the system *does*).
- Do NOT change production code behaviour to match a spec. If they disagree,
  the spec is wrong — correct it.
- Do NOT fix bugs discovered during characterization. Flag them as follow-up
  `@givn.modified` candidates.
- Do NOT delete dead code. Flag it as a follow-up `@givn.removed` candidate.
- Do NOT skip the discovery phase — it is always interactive.
- Do NOT write proposal.md, design.md, or design-review.md. The reverse flow
  has no such artifacts — they are skipped via `givn new --skip`.

## Discovery phase

A fresh delegated context does not inherit the parent's loaded policy. Pass
the dialog obligation explicitly and load it in the fresh context before the
first question.

Before writing any artifact, explore the codebase and propose ONE capability
with a clear black-box boundary. Present the proposal to the user
interactively. Do not proceed until the user confirms.

One capability per change. If multiple capabilities are needed, suggest
separate changes.

## Artifacts produced (only 2)

- `specs/<group>/<cap>.feature` — descriptive scenarios (`@givn.added @wip`),
  all provisional until validated during characterization. The Feature:
  description block contains the black-box boundary and suspected bugs.
- `tasks.md` — first task is harness setup; subsequent tasks are one
  RED/GREEN/REFACTOR characterization cycle per scenario (shared loop
  mechanics: `givn instructions steps`; task-file format:
  `givn instructions tasks-reverse`).

## What is NOT produced

- No `proposal.md` — the black-box boundary and suspected bugs go in the
  Feature: description block.
- No `design.md` — seam strategy goes per-task in `tasks.md`; verify.command
  choice goes in the first setup task.
- No `design-review.md` — the discovery phase is already interactive; grilling
  a descriptive spec is unnecessary.

## Spec writing rules

- Tag the Feature: `@givn.delta @<capability>`.
- Tag each Scenario: `@givn.added @wip` (all scenarios are added — the
  permanent spec is empty for this capability).
- The **Feature: description block** (the free-text lines after the Feature
  line) must contain:
  - The **black-box boundary** (named explicitly: "The black-box boundary
    for this capability is: <CLI surface / public API / module boundary>").
  - **Suspected bugs** discovered during discovery, flagged as follow-up
    `@givn.modified` candidates. These are NOT addressed in this change.
- Scenarios assert **observed behaviour** in domain language
  (Given/When/Then about inputs, outputs, state visible to a user/client).
  The behaviour is probed from the real code, not imagined from requirements.
- Never reference internal calls, class names, function names, specific code
  structures, HTTP routes, step-definition mechanics, or framework details.
- One scenario = one observable behaviour.
- If a scenario describes buggy behaviour (the code does something wrong, but
  that is what it actually does), the scenario captures the buggy AS-IS
  behaviour. Do NOT write the "correct" behaviour if the code does something
  different.

**Read the ACTUAL CODEBASE to determine observed behaviour.** Do not guess
from documentation alone — documentation can be wrong.

## Transition out

When planning is complete, the characterization loop runs per scenario:
`/givn-characterize`.
