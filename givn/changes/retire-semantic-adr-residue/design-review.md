# Design Review: retire-semantic-adr-residue

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Rule choice | Supersede-and-archive or delete as failing records? | agreed — supersede; the records did not fail the gate |
| 2 | Consistency | Does ADR-069's supersession list grow? | agreed — yes, the three links are appended |
| 3 | Consistency | Which indexes need updating? | agreed — README count, active rows, archived rows, relationship line |
| 4 | Risk | Do the moved records' internal links break? | no — their relative archive links stay valid |
| 5 | Architecture documentation | Is chapter 9 affected? | agreed — Yes; the register is chapter 9's content |
| 6 | Testability | How is the end state proven? | agreed — accepted-count check, README check, suite |
| 7 | Completeness | Does any scenario read the live register count? | agreed — the register-history scenario is modified to 25 through a delta |

### Q1 — Supersede or delete

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: the end state of the three records.
- Context: ADR-069 removed their subject; the register keeps only current
  decisions and archives superseded ones.
- Question: supersede-and-archive, or delete as failing records?
- Options:
  1. **Supersede and archive.** `status: superseded by ADR-069`, moved to
     `archive/`, exactly like the sixteen siblings.
     Meaning: the history stays readable.
     In practice: the active register holds 25 records.
     Gain: consistent with the register contract.
     Cost: none.
  2. **Delete as failing.** Treat them as failed qualification.
     Meaning: the bundle's failing-record rule (never archive a failed
     record).
     In practice: history is lost for records that did not fail.
     Gain: none.
     Cost: wrong rule; lost context.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — ADR-069 list

- Status: closed 2026-09-18 (operator)
- Blocks: whether the supersession is traceable from ADR-069.
- Context: ADR-069 lists sixteen superseded records.
- Question: append the three or leave ADR-069 untouched?
- Options:
  1. **Append.** The list becomes the complete supersession record.
     Meaning: one edit.
     In practice: a reader sees the full set.
     Gain: traceability.
     Cost: none.
  2. **Leave.** The moved records' status names ADR-069.
     Meaning: the reverse link only.
     In practice: ADR-069's list is incomplete.
     Gain: no edit.
     Cost: an inconsistent register.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Indexes

- Status: closed 2026-09-18 (codebase inspection)
- Blocks: which files name the three records.
- Context: the README holds the active and archived registers and the
  relationship table; chapter 9 and chapter 11 do not name them.
- Question: which indexes change?
- Options:
  1. **The README only.** Count 28 → 25, rows moved, relationship line
     extended.
     Meaning: three README edits.
     In practice: no other index drifts.
     Gain: complete.
     Cost: none.
  2. **Also chapter 9.** Add register prose.
     Meaning: duplication of the README.
     In practice: drift risk.
     Gain: none.
     Cost: maintenance.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Link safety

- Status: closed 2026-09-18 (inspection)
- Blocks: whether moving the records breaks links.
- Context: the records link to sibling records and to `archive/` paths.
- Question: any link rewrite needed?
- Options:
  1. **No rewrite.** Sibling links inside `archive/` resolve within the
     same directory; links that already used `archive/` prefixes are
     relative to `adr/`.
     Meaning: no textual link change.
     In practice: the moved files render correctly.
     Gain: a pure move.
     Cost: none.
  2. **Rewrite links.** Rebase every relative link.
     Meaning: churn without benefit.
     In practice: risk of typos.
     Gain: none.
     Cost: churn.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: the register is chapter 9's content.
- Question: which rows are Yes?
- Options:
  1. **Row 9 only.** The register changes; nothing else does.
     Meaning: one Yes with the summary.
     In practice: the assessment matches the edit.
     Gain: truthful.
     Cost: none.
  2. **Rows 9 and 11.** Mention the residue as debt.
     Meaning: chapter 11 has no such entry.
     In practice: a false claim.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — Register-count scenario

- Status: closed 2026-09-18 (suite evidence)
- Blocks: whether the change can be `specs`-free.
- Context: the permanent scenario "The migration preserves independent
  architecture history across three proposed decisions" reads the live
  register and pins 28 accepted decisions; the suite fails with 25.
- Question: update the scenario through a delta or leave it failing?
- Options:
  1. **Delta the scenario to 25.** The scenario is the live register's
     current-state assertion; every register-changing migration updates it
     (25 → 26 → … → 28 historically).
     Meaning: one `@givn.modified` scenario; no Rust change because the
     25-count step already exists.
     In practice: the suite passes and the spec states the current register.
     Gain: the dependency is explicit.
     Cost: none.
  2. **Leave the scenario and change the step.** Point the 28-step at a
     fixture.
     Meaning: the scenario stops reading the live register.
     In practice: it no longer proves the register's state.
     Gain: no count updates later.
     Cost: loses the live-register check.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Testability

- Status: closed 2026-09-18 (operator)
- Blocks: the evidence.
- Context: no spec delta covers the register.
- Question: what proves the end state?
- Options:
  1. **Count and README checks plus the suite.** `rg` proves 25 accepted
     records and the README's references; the suites prove no regression.
     Meaning: reproducible evidence.
     In practice: recorded in tasks.md.
     Gain: verifiable.
     Cost: none.
  2. **Inspection only.** Read the files.
     Meaning: weaker evidence.
     In practice: no command trail.
     Gain: none.
     Cost: an unproven claim.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | Chapter 9 and chapter 11 do not name the three records; only the README indexes them. |
| Risk | The records' links already use archive-relative paths, so the move is safe. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | No use-case delta; the repair maintains the register | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | No user-visible surface; `none` with the reason | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| givn-maintainer--owner | Owns the register being corrected | accepted |

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
| 8 | 08 crosscutting-concepts | No | No | Yes | — |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — register count, rows, relationship line, ADR-069 list |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | No | No | Yes | — |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the change retires records.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the move.
- [x] `specs` carries one modified scenario; no new step is needed.
- [x] `givn lint --change retire-semantic-adr-residue` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
