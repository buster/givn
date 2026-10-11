# Design: user-verified-term-naming

## Overview

The change makes the user the decider of every newly coined core domain term:
the naming question fires immediately when a coined name fails, every other new
core term rides the phase's term check, and `confirmed` now requires the user's
verified name. The WHAT lives in `proposal.md`; the observable behaviour is
locked by the two delta features under `specs/fragments/`.

The HOW is text: three generated-guidance assets carry the new wording — the
`givn-domain-modeling` skill template (the naming-dialog shape), the
`givn-ideation` skill template (the `confirmed` clause and the term-check
trigger), and the `givn-ideation` command guardrails (the one-line binding).
No Rust code path changes; the scenarios run the real `givn skills sync` and
assert on the rendered files, so they test exactly what consumers receive.

## Decisions

- Decides D1: The user verifies term names in two tiers — an immediate naming
  question when a coined name fails, and one combined term check at each
  phase's closing checkpoint for every other new core term — realized in the
  term-check text of `assets/skills/givn-ideation/SKILL.md.tmpl` and the
  naming-dialog wording of `assets/skills/givn-domain-modeling/SKILL.md.tmpl`.
- Decides D2: The dialog shape (concept description, up to three candidate
  names with what each evokes and any collision, the user's own name accepted)
  is single-sourced in the `givn-domain-modeling` skill; the binding (when the
  question fires, what `confirmed` requires, how a revisit resolves) lives in
  the `givn-ideation` rules and command guardrails — realized in all three
  assets.
- Decides (design): the `confirmed` verified-name clause and the term-check
  bullets ship unconditionally; only the existing "full term introduction test
  lives in `givn-domain-modeling`" sentence keeps its
  `{{#if addons.domain_modeling}}` wrap. Ideation coins terms with or without
  the addon, and the command guardrail is already unconditional; the
  addon-free delta scenario pins the unconditional `confirmed` clause.
- Decides (design): the naming-dialog shape is stated once, in the
  domain-modeling challenge bullet; the ideation text names the "naming
  dialog" without restating its shape.
- Decides (design): the two human-facing records — the term-record paragraph
  in `docs/workflow.md` and the term-record row in
  `docs/arc42/12-glossary.md` — are amended to match. No step reads either
  file; review verifies them against the exact texts in this document.

## Exact file changes

| Path | Change |
|---|---|
| `assets/skills/givn-domain-modeling/SKILL.md.tmpl` | Replace the challenge bullet's remedy sentence with the naming-dialog procedure and worked example. |
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Add "the user's verified name" to the `confirmed` bullet; add the unconditional term-check section after the `rejected` bullet. |
| `assets/commands/givn-ideation.md` | Add one guardrail bullet after the persona-confirmation bullet. |
| `docs/workflow.md` | Amend the term-record paragraph (durable doc, review-verified). |
| `docs/arc42/08-crosscutting-concepts.md` | Amend section 8.20's first paragraph (durable doc, review-verified). |
| `docs/arc42/12-glossary.md` | Amend the `Term record` row (durable doc, review-verified). |

### New wording, for the implementer

Challenge bullet, `assets/skills/givn-domain-modeling/SKILL.md.tmpl` (replaces
the current bullet; the first three lines keep today's text):

```markdown
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

The `confirmed` bullet in `assets/skills/givn-ideation/SKILL.md.tmpl` becomes:

```markdown
- `confirmed` — requires a genus-and-differentia definition, a contrast pair,
  one concrete domain sentence, the user's verified name, and a
  recorded collision check.
```

After that bullet the addon-gated sentence
(`{{#if addons.domain_modeling}} The full term introduction test lives in
`givn-domain-modeling`. {{/if}}`) stays exactly as it is; the `rejected` bullet
stays exactly as it is. After the `rejected` bullet, add the unconditional
section:

```markdown
The user verifies every new core term before it is used:

- When a coined term is generic, overloaded, cannot fill its genus slot, or
  has competing candidates, ask the naming question immediately.
- Every other new core term — a new aggregate root, a use-case goal noun, or
  an event name introducing a noun — rides the phase's closing checkpoint as
  one question. It reads: "Term check — <phase>: <N> new core terms.
  Confirm all, or name the terms to revisit." Each row gives the term, its
  one-sentence description, and the recorded collision check.
- Confirm all makes them `confirmed`; a term named for revisit gets its
  naming dialog, one at a time, before confirmation; the other terms stand
  and nothing else re-opens. A name the user supplies
  is collision-checked like any other; a replaced name becomes an anti-term.
```

`assets/commands/givn-ideation.md`, one new Guardrails bullet after "Never
invent a persona the user has not confirmed.":

```markdown
- The user verifies every new core term — by the phase term check or the
  term's naming dialog — before the canvas, use cases, or seeds rely on it.
```

`docs/workflow.md`, term-record paragraph: add one sentence naming the
verified-name requirement — a `confirmed` term carries the user's verified
name, asked immediately when the name fails and otherwise at the phase's term
check.

`docs/arc42/12-glossary.md`, `Term record` row: same amendment in the row's
definition — a `confirmed` term has passed the term introduction test, carries
a recorded collision check, and carries the user's verified name (asked
immediately when the name fails, otherwise at the phase's term check).

`docs/arc42/08-crosscutting-concepts.md`, section 8.20 first paragraph: a
`confirmed` term requires the term introduction test, a recorded
search-before-coin check, and the user's verified name (asked immediately when
the name fails, otherwise at the phase's term check).

Durable-doc verification note: no step reads `docs/workflow.md` or
`docs/arc42/12-glossary.md`; design-review and review verify them against the
exact texts above.

## Step definitions

One file per capability. The `guide-domain-modeling` capability — the only one
with new steps — extends `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`,
which already loads the template (`Given the domain-modeling skill template is
inspected`) and exposes the body via `skill_template(world)`. Eight new `Then`
steps:

| Step | Assertion on the template body |
|---|---|
| `the body no longer tells the agent to choose the name alone` | no "Propose it and justify it"; contains "do not choose it alone" |
| `the body describes the concept before proposing names` | contains "the concept in one sentence" |
| `the body offers up to three candidate names` | contains "up to three candidate names" |
| `the body states what each candidate evokes and any collision it carries` | contains "any collision it carries" |
| `the body accepts the user's own name` | contains "accept the user's own name" |
| `the body shows the naming question for "<term>"` (regex) | contains the term and "Candidates:" |
| `the body shows the colliding candidate "<candidate>"` (regex) | contains the candidate and "collides" |
| `the body asks which name the record should use` | contains "should the record use, or what is yours" |

Both delta scenarios in `specs/fragments/guide-domain-modeling.feature` carry
`@wip` today; implementing these steps and removing the tag is part of the
implementation. The `generate-ideation-guidance` delta needs no new steps: it
reuses the existing generic guidance/command steps and the two fixtures
(addons on via `givn init`; addons off via `givn init --no-addons`).

## Runner and strict mode

- Verify command (`givn/commands.yaml`): `./run-tests.sh`; E2E smoke:
  `./run-tests.sh --e2e`.
- Strict mode: `tests/features_runner.rs` uses `.fail_on_skipped()` and exits
  non-zero when `stats.failed > 0 || stats.skipped > 0`, so undefined or
  pending steps fail the run. The `@wip` tag marks not-yet-implemented step
  definitions and is reported by `givn lint`; archive and `check tasks` block
  while a non-removed `@wip` scenario exists in the delta.
- Change-scope run:
  `GIVN_FEATURES=givn/changes/user-verified-term-naming/specs ./run-tests.sh`
- Single scenario:
  `GIVN_FEATURES=givn/changes/user-verified-term-naming/specs cargo test --locked --test features_runner -- --exact "fragments/guide-domain-modeling.feature::The challenge bullet hands the name choice to the user"`

## Use-case traceability

| Use-case element | Technical realization | E2E evidence |
|---|---|---|
| Capability `generate-ideation-guidance` (fragment `support`) | `confirmed` clause + term-check section in `assets/skills/givn-ideation/SKILL.md.tmpl`; guardrail bullet in `assets/commands/givn-ideation.md` | "A consumer synchronizes skills and every agent target receives the ideation facilitation skill" |
| Capability `guide-domain-modeling` (fragment `support`) | naming-dialog procedure + example in the challenge bullet of `assets/skills/givn-domain-modeling/SKILL.md.tmpl` | "An agent synchronizes skills and the domain skill is routed" |
| Rule: the user verifies every new core term before it is used | immediate-question trigger, phase term check, verified-name `confirmed` clause, anti-term/revisit handling | The five `generate-ideation-guidance` delta scenarios assert the rendered guidance |
| Confirmed Personas: `developer--primary-user` (answers the naming question), `agent--technical-role` (asks it and runs the term check), `givn-maintainer--owner` (owns the guidance) | The guidance text addresses each role in its own terms | The capability has CLI text output, so Personas are named |

Actors in the scenarios (`the user`, `an agent`) stay distinct from Personas;
the Persona rows record review context only.

## Visual Design Contract

- Interface: terminal.
- Viewports: not applicable — no Web UI; nothing renders HTML, CSS, or
  JavaScript. The opposite classification was tested: `givn skills sync`
  writes Markdown guidance files and the CLI stdout contract is unchanged, so
  a browser driver cannot apply.
- Surfaces covered: the generated skill and command files consumed in a
  terminal.
- Evidence: no new `@e2e` scenario; the existing synchronize-skills
  transcripts remain the evidence and are unchanged.

## Interaction Coverage Matrix

The change adds no new consumer action, so the fragment's interaction
inventory (`givn/specs/fragments/fragment.md`) is unchanged and no new
inventory entry exists to map. The existing `@e2e` scenarios below cover the
unchanged `givn skills sync` action; the delta scenarios assert the new
content through the same real sync.

| Existing coverage (no new action) | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `givn skills sync` renders the ideation guidance | "A consumer synchronizes skills and every agent target receives the ideation facilitation skill" | CLI | Real `givn skills sync` subprocess; content asserted on the rendered files |
| `givn skills sync` renders the domain-modeling guidance | "An agent synchronizes skills and the domain skill is routed" | CLI | Real `givn skills sync` subprocess; content asserted on the rendered files |

## Version freshness

No language, framework, library, or image version is introduced. The change
edits embedded text assets consumed by the existing Rust binary; the lockfile
is untouched.

## Local runnability & digital twins

No external or third-party dependency is touched. Local run and test command:
`./run-tests.sh`; E2E smoke via the configured `verify.e2e_command`. No digital
twin is required.

## Coverage process boundaries

N/A — the change adds no instrumented process and no measurement command.

## Arc42 impact

Chapter 8 (crosscutting concepts): section 8.20 Term Introduction Discipline
defines `confirmed`; its first paragraph gains the user's verified name.
Chapter 12 (glossary): the `Term record` row gains the same requirement.
Outside arc42, `docs/workflow.md` records the same rule. All edits are
review-verified against the texts in this document; the change's `arc42.md`
records the delta.

## Internal Primitives

None — the change adds no module boundary, no struct, and no function to the
production code.
