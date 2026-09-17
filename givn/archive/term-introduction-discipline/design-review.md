# Design Review: term-introduction-discipline

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Missing scenarios | Are the four ideation-flow integration behaviours asserted? | agreed — four assertions added |
| 2 | Scope | Does the scenario assert the full entry schema? | agreed — Contrast, First use, and the candidate rule asserted |
| 3 | Tech choices | Is `confirmed` reachable without the domain-modeling skill? | agreed — one-line criteria in the ideation bullet |
| 4 | Testability | Are the asserted phrases pinned and schema-distinctive? | agreed — bare status words dropped; per-step phrase mapping recorded |
| 5 | Risk | Can a piped grep command break the table cell? | agreed — escaping and pipe-free command rule added |

### Q1 — Ideation-flow integration assertions

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the quality-review branch, the Handoff rule, the
  rejected-term retention, and the domain-modeling reference are executable.
- Context: the handover's §4 mandates those integration behaviours, and the
  scenario asserted only the schema and the collision procedure.
- Question: assert the four behaviours?
- Options:
  1. **Add four assertions to the existing scenario.**
     Meaning: same `givn skills sync` driver, no new steps.
     In practice: assertions for "recorded collision check",
     "only `confirmed` terms", "never deleted", and "givn-domain-modeling".
     Gain: the integration behaviours fail RED when dropped.
     Cost: four more assertions.
  2. **Leave them to review.**
     Meaning: prose-only verification.
     In practice: a template that drops the review branch passes the suite.
     Gain: smaller scenario.
     Cost: mandatory behaviours without guards.
- Resolution: **Option 1** because the behaviours are handover requirements
  and the assertions are free. Rejected: Option 2 because it leaves them
  unverified. The workflow-document pointer stays a review-checked deliverable:
  no test reads a project document, and asserting its wording would not be a
  product behaviour.

### Q2 — Full schema coverage

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether a template that drops the Contrast or First use column can
  pass.
- Context: the scenario asserted only status words and the collision column,
  so the schema's distinctive columns were unguarded.
- Question: assert the full schema?
- Options:
  1. **Assert Contrast, First use, and the candidate rule.**
     Meaning: three more phrases from the schema table and rules.
     In practice: a template missing the contrast column fails RED.
     Gain: the schema is complete in the spec.
     Cost: three assertions.
  2. **Keep the status-and-collision surface.**
     Meaning: the columns stay unasserted.
     In practice: a partial schema passes.
     Gain: fewer assertions.
     Cost: the schema can silently shrink.
- Resolution: **Option 1** because the schema is the change's contract.
  Rejected: Option 2 because it permits a partial record.

### Q3 — `confirmed` without the domain-modeling skill

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the confirmed status is reachable in an addon-free project.
- Context: the domain-modeling skill is generated only with the
  `domain_modeling` and `arc42` addons enabled; the ideation skill ships with
  no addons. A bare reference would dangle in an addon-free project.
- Question: keep the split or add a one-line fallback?
- Options:
  1. **One-line criteria in the ideation bullet.**
     Meaning: the bullet names genus-and-differentia, contrast pair, and one
     concrete sentence, and points at the skill for the full test.
     In practice: `confirmed` stays reachable without the skill; the full test
     is not duplicated.
     Gain: no dangling reference; the schema stays self-contained.
     Cost: one sentence.
  2. **Bare reference to the skill.**
     Meaning: the criteria live only in the domain-modeling skill.
     In practice: an addon-free project cannot apply the test.
     Gain: single source for the test.
     Cost: a dangling reference in the common addon-free case.
- Resolution: **Option 1** because the handover requires the reference and the
  addon-free case is the default. Rejected: Option 2 because it breaks the
  schema in projects without the addon.

### Q4 — Pinned, schema-distinctive phrases

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether an assertion can pass by accident.
- Context: "confirmed" and "rejected" already occur in the current template as
  unrelated substrings, and the domain-modeling steps need exact phrases.
- Question: pin the phrases?
- Options:
  1. **Drop bare status words; record a per-step phrase map.**
     Meaning: assert the status list and distinctive schema phrases; pin each
     domain-modeling step's `contains` strings in design.md.
     In practice: "`candidate`, `confirmed`, or `rejected`" replaces the bare
     words; each step's phrases are documented.
     Gain: RED cannot pass by substring accident.
     Cost: none.
  2. **Keep the bare status words.**
     Meaning: two assertions are already green before the change.
     In practice: the scenario still fails RED on the other phrases.
     Gain: none.
     Cost: misleading evidence.
- Resolution: **Option 1** because the assertion must prove the schema, not a
  substring. Rejected: Option 2 because it weakens the guard.

### Q5 — Pipes in the recorded collision check

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether a recorded command can break the term-record table.
- Context: a pipeline such as `rg -n "Port" docs/ | wc -l` terminates a
  Markdown table cell, splitting the record into extra columns.
- Question: add an escaping or recording rule?
- Options:
  1. **Escape `|` and prefer pipe-free commands.**
     Meaning: one line in the schema guidance.
     In practice: `rg -c "Port" docs/arc42/12-glossary.md` is recorded with
     its count; any remaining pipe is escaped as `\|`.
     Gain: the record stays a readable table row.
     Cost: one line.
  2. **Leave it to the author.**
     Meaning: no rule; a piped command may break the row.
     In practice: the term record becomes unreadable.
     Gain: none.
     Cost: a known formatting failure.
- Resolution: **Option 1** because the collision cell is the record's most
  failure-prone field. Rejected: Option 2 because it leaves a predictable
  breakage.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| E2E fidelity | No new `@e2e` scenario is required: both capabilities are fragment-owned with no interaction inventory, and their existing `@e2e` scenarios drive the real `givn skills sync` subprocess; the new scenarios are content variants. |
| Testability | Strict mode is `.fail_on_skipped()`, `GIVN_FEATURES` selects the delta, and the single-scenario command is valid; the scenario is genuinely RED against the current templates. |
| Scope | `skill-rewrites.feature` asserts no phrase from the edited ideation or domain-modeling sections, so it needs no update. |
| Architecture documentation | The independent 12-row walk matches arc42.md; section 8.20, R-68, and the `Term record` glossary row contain real content; no ADR qualifies. |
| Architecture documentation | R-67 and R-68 sat between R-63 and R-64; both were moved to the end of the risk table to restore numeric order. |
| Use-case and Persona context | Both capabilities are fragment-owned; the fragment declares no interactions, so no use-case document change is needed. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `ideation-skill` and `domain-skill-trigger` keep fragment ownership. | accepted |
| Typed Includes / Extends relationships | The support fragment is included by the use cases; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (generated guidance) and `givn-maintainer--owner` (rule corpus, term convention) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps the Developer and the Authoring agent as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the generated ideation and domain-modeling guidance that carries the schema, procedure, and test. | `The ideation guidance carries the term schema and the collision procedure`; `The domain-modeling guidance carries the term introduction test` |
| givn-maintainer--owner | Owns the term-record convention and the guidance corpus. | Same scenarios; chapter 8.20 |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | No | No | Yes | — |
| 6 | 06 runtime-view | No | No | Yes | — |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.20 term introduction discipline |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — no qualified candidate; destination 8.20 |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-68 (moved to numeric order) |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Term record` row |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the canonical destination is chapter 8.20.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/term-introduction-discipline/specs/fragments/ideation-skill.feature` | Added the four integration assertions and the schema assertions; dropped the bare status words for the status list. |
| `givn/changes/term-introduction-discipline/design.md` | Status list, one-line criteria, escaping rule, full assertion list, per-step phrase mapping. |
| `givn/changes/term-introduction-discipline/arc42.md` | Chapter 11 wording unaffected; R-68 reference remains correct. |
| `docs/arc42/11-risks-and-technical-debt.md` | Moved R-67 and R-68 to the end of the table to restore numeric order. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change term-introduction-discipline` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
