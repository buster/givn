# Design Review: retire-lint-command-adr

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Destination | Where does the rationale live? | agreed — the verify-change Rules plus the existing lint capability spec |
| 2 | Disposition | Delete or archive the record? | agreed — delete; a failing record is not historical |
| 3 | Consistency | What happens to ADR-019's links? | agreed — they point at the spec; its status records the retirement |
| 4 | Consistency | What happens to the register? | agreed — count 24, ADR-027 row removed, relationship line updated |
| 5 | Architecture documentation | Is chapter 9 affected? | agreed — Yes, the register changes |

### Q1 — Destination

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: where the naming rationale lives after retirement.
- Context: the record fails the product-behavior test; the behavior is
  specified.
- Question: use case Rules, spec scenario, or both?
- Options:
  1. **Rules plus the existing spec.** The use case states the naming rule;
     the lint capability already owns the exit codes.
     Meaning: the rationale is reviewable where the behavior lives.
     In practice: one added rule.
     Gain: no duplicated spec.
     Cost: none.
  2. **A new scenario.** Add a spec scenario for the name.
     Meaning: the name is asserted in Gherkin.
     In practice: scenarios already invoke `givn lint`.
     Gain: explicit.
     Cost: redundant.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Disposition

- Status: closed 2026-09-18 (operator, migration decision)
- Blocks: the end state of the record.
- Context: the bundle says a failing record never enters the ADR archive.
- Question: delete or archive?
- Options:
  1. **Delete.** The rationale lives in the spec; the audit note stays in
     this change.
     Meaning: the register holds only qualified decisions.
     In practice: the file is removed.
     Gain: correct rule.
     Cost: the record's text is gone (its substance is in the spec).
  2. **Archive it.** Keep it as history.
     Meaning: a failing record becomes a historical ADR.
     In practice: contradicts the gate.
     Gain: text preserved.
     Cost: wrong rule.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — ADR-019

- Status: closed 2026-09-18 (operator)
- Blocks: whether the superseded record dangles.
- Context: ADR-019's status and body point at ADR-027.
- Question: how are its references repaired?
- Options:
  1. **Point the body at the spec; keep the historical status.** The status
     stays `superseded by ADR-027` because the register-history scenario
     asserts it and it is the historical fact; the body notes the retirement
     and references the current contract.
     Meaning: links resolve and the scenario stays true.
     In practice: five reference edits.
     Gain: integrity without rewriting the historical status.
     Cost: none.
  2. **Leave them.** Historical references stay.
     Meaning: broken links.
     In practice: a reader hits a missing file.
     Gain: no edits.
     Cost: dangling references.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Register

- Status: closed 2026-09-18 (operator)
- Blocks: the index state.
- Context: the README counts and lists active records.
- Question: what changes?
- Options:
  1. **Count 24, row removed, relationship line updated.** The archived row
     for ADR-019 notes the retirement.
     Meaning: the index matches the register.
     In practice: three edits.
     Gain: consistent.
     Cost: none.
  2. **Leave the count.** Only remove the row.
     Meaning: the index lies.
     In practice: drift.
     Gain: smaller diff.
     Cost: inconsistency.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Architecture documentation

- Status: closed 2026-09-18 (operator)
- Blocks: the arc42 assessment.
- Context: the register is chapter 9's content.
- Question: which rows are Yes?
- Options:
  1. **Row 9 only.** The register changes.
     Meaning: one Yes.
     In practice: truthful.
     Gain: accurate.
     Cost: none.
  2. **Rows 9 and 11.** Mention the retired record as debt.
     Meaning: no such risk entry.
     In practice: false claim.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | ADR-019 references ADR-027 in its status, two body sentences, a confirmation line, and a link. |
| Consistency | No other durable artifact references the ADR-027 file. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | The naming rule moves into `verify-change` | accepted |
| Typed Includes / Extends relationships | No relationship change | accepted |
| Confirmed Persona coverage | `developer--primary-user` serves the CLI surface | accepted |
| Actor and Persona distinction | No Gherkin actors introduced | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | The naming contract serves the developer's CLI | spec delta |

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
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — register count, row removal, ADR-019 references |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | No | No | Yes | — |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] The audit note records the record ID, the failing test, and the
      destination.
- [x] No MADR consequence requires a chapter 11 counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the retirement.
- [x] `specs` carries one narrative delta; no new step is needed.
- [x] `givn lint --change retire-lint-command-adr` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
