# Design Review: {{CHANGE_ID}}

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | | | | |

## Resolved by codebase exploration

Items answered without user input by exploring the existing codebase.

| Branch | Finding |
|---|---|
| | |

## Open questions

Questions that could not be resolved and were flagged explicitly.

| # | Question |
|---|---|---|
| | | |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | | |
| Typed Includes / Extends relationships | | |
| Confirmed Persona coverage | | |
| Actor and Persona distinction | | |

### Persona disposition

One row per confirmed persona, recording the relevance and the evidence; when
no persona is confirmed, record the reason and the operator confirmation.

| Persona | Relevance | Evidence |
|---|---|---|
| | | |

## Architecture documentation (arc42) check

N/A if the arc42 addon is not enabled (`addons.arc42` in `givn/config.yaml`).
Otherwise, fill in **all 12 rows** — this is an independent audit, not a
transcription of `arc42.md`'s own table. Assess each chapter against
proposal.md/design.md yourself before reading what `arc42.md` claims, then
record both:

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | | | | |
| 2 | 02 architecture-constraints | | | | |
| 3 | 03 context-and-scope | | | | |
| 4 | 04 solution-strategy | | | | |
| 5 | 05 building-block-view | | | | |
| 6 | 06 runtime-view | | | | |
| 7 | 07 deployment-view | | | | |
| 8 | 08 crosscutting-concepts | | | | |
| 9 | 09 architecture-decisions | | | | |
| 10 | 10 quality-requirements | | | | |
| 11 | 11 risks-and-technical-debt | | | | |
| 12 | 12 glossary | | | | |

- [ ] All 12 rows assessed independently against proposal.md/design.md
      before reading arc42.md's own claims.
- [ ] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md
      saying "No" or omitting the row — any such row is a finding,
      not a note.
- [ ] Every qualified ADR candidate in design.md has a corresponding MADR entry
      in chapter 09; non-qualifying decisions name exactly one canonical target.
- [ ] Every MADR "Bad, because..." consequence has a counterpart entry in
      chapter 11 (risks-and-technical-debt).
- [ ] Every ADR candidate records the structured qualification verdict and its
      route-specific target; `Must be shared` is supporting evidence only.
- [ ] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

Edits applied to existing artifacts as a result of grilling.

| Artifact | Change summary |
|---|---|
| | |

## Sign-off

- [ ] Every branch of the design tree has been walked.
- [ ] All questions are resolved.
- [ ] `design.md` reflects any decisions reached during grilling.
- [ ] `specs/**/*.feature` updated if any missing scenarios were identified.
- [ ] `givn lint --change {{CHANGE_ID}}` exits 0 or 2 (no parse errors).
- [ ] Architecture documentation (arc42) check completed above (or marked
      N/A because the addon is not enabled) — never left blank.

**DESIGN-REVIEW: PASS** <!-- change to FAIL if any question remains unresolved -->
