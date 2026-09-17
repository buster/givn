# Design Review: migration-retrofit-plan

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Consistency | Do the generated wrapper strings contradict the inventory model? | agreed — wrapper defers to each bundle's completion rules |
| 2 | Completeness | Are the guide prompts and the proposal wording fully aligned? | agreed — prompts rewritten; proposal sentence corrected |
| 3 | Completeness | Is the inventory table shape pinned? | agreed — columns, id format, in-flight definition |
| 4 | Completeness | Are seeds, qualifiers, and overrides inventoried? | agreed — rows with follow-ups; readings stay in-change |
| 5 | Completeness | Where does the term evidence live? | agreed — direct edit as the one exception; collisions become rows |
| 6 | Completeness | What if a proposed ADR's owner archived? | agreed — re-qualify here or a new follow-up; never name an archived change |
| 7 | Consistency | Which ADR steps stay and which move? | agreed — verdict and audit note stay; removal, rationale move, and template replacement are follow-ups |
| 8 | Testability | Is the smoke test specific to the amendment? | agreed — unique strings named; wrapper fixtures unaffected |
| 9 | Risk | What prevents a half-applied bundle? | agreed — execution-model block per phase plus the no-corpus-edit check |
| 10 | Consistency | Do the README and changelog describe the amended model? | agreed — both updated; 0.8.0 is untagged |
| 11 | Architecture documentation | Do durable chapters describe the old model? | agreed — chapters 02, 06, 07 reworded; rows 2, 6, 7 Yes |

### Q1 — Wrapper strings

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the generated plan tells the consumer to complete the
  phases in-change.
- Context: `src/cli/migration.rs` says "complete each phase" and "do not
  create independent parallel migration changes".
- Question: reword the wrapper or override in the bundle?
- Options:
  1. **Reword to defer to each bundle.**
     Meaning: "work each phase in order and follow the completion rules in
     its bundle"; the design adds "a bundle may direct that the repairs run
     as named follow-up changes".
     In practice: old bundles keep their in-change completion; the 0.8.0
     bundle's model is honored.
     Gain: no contradiction, no per-bundle conditional.
     Cost: one production string change.
  2. **Override in the bundle only.**
     Meaning: the wrapper keeps saying complete-in-change; the bundle adds a
     counter-sentence.
     In practice: contradictory instructions.
     Gain: no Rust change.
     Cost: a misleading wrapper.
- Resolution: **Option 1** because the wrapper is generic and the old bundles
  still complete in-change. Rejected: Option 2.

### Q2 — Guide and proposal alignment

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the guide still instructs in-change repairs.
- Context: Prompts 2-5 were repair prompts; the proposal said the bundle
  "keeps its phase content".
- Question: rewrite them?
- Options:
  1. **Rewrite prompts to inventory and follow-ups; correct the sentence.**
     Meaning: every prompt records rows and names follow-ups; the completion
     boundary becomes two-level.
     In practice: the guide matches the bundle.
     Gain: one model everywhere.
     Cost: a guide rewrite.
  2. **Leave the prompts.**
     Meaning: the guide contradicts the bundle.
     In practice: a consumer follows the detailed prompts.
     Gain: none.
     Cost: the exact failure the amendment fixes.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Table shape

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether two runs produce the same inventory.
- Context: the plan said "build a disposition table" without columns.
- Question: pin the shape?
- Options:
  1. **Pin columns, id format, and in-flight definition.**
     Meaning: `| Artifact | Deviation (criterion or test) | Disposition |
     Follow-up change id |`; kebab-case ids; in-flight means the change
     directory exists.
     In practice: deterministic tables.
     Gain: reproducible.
     Cost: none.
  2. **Describe loosely.**
     Meaning: free-form tables.
     In practice: inconsistent.
     Gain: none.
     Cost: ambiguity.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Seeds, qualifiers, overrides

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether those adoptions fall out of the model.
- Context: the amended inventory list omitted them.
- Question: rows, in-change work, or dropped?
- Options:
  1. **Rows with follow-ups; readings stay in-change.**
     Meaning: seeds, qualifiers/ledger, and overrides become rows; reading
     the dialog policy and re-reading lint/route output stay in-change.
     In practice: nothing is lost.
     Gain: complete model.
     Cost: none.
  2. **Drop them.**
     Meaning: silent gaps.
     In practice: unadopted rules.
     Gain: shorter.
     Cost: incomplete retrofit.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Term evidence location

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the evidence edit contradicts "no repairs".
- Context: decision 7 requires recording evidence in the record.
- Question: direct edit or follow-up?
- Options:
  1. **Direct edit as the one exception.**
     Meaning: re-run the searches and write the cells; collisions become
     rows.
     In practice: the record is honest immediately.
     Gain: matches the decision.
     Cost: one documented exception.
  2. **Evidence in the inventory only.**
     Meaning: the term file stays stale until a follow-up.
     In practice: the record still lies.
     Gain: strict no-edit rule.
     Cost: contradicts decision 7.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Proposed ADR with no live owner

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether a dead owner can be named.
- Context: decision 4 names the owning change; an archived owner cannot act.
- Question: what is the disposition?
- Options:
  1. **Re-qualify here or create a new follow-up.**
     Meaning: accepted or rejected with a canonical destination; never name
     an archived change as pending.
     In practice: every record resolves.
     Gain: no dead references.
     Cost: one rule.
  2. **Name the archived change anyway.**
     Meaning: the row can never complete.
     In practice: a permanent pending item.
     Gain: none.
     Cost: a broken inventory.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — ADR step split

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: which ADR steps stay in the maintenance change.
- Context: the old phase 3 assigned removal and template replacement there.
- Question: what stays?
- Options:
  1. **Verdict and audit note stay; removal, rationale move, template
     replacement are follow-ups.**
     Meaning: the maintenance change re-qualifies and records; the follow-up
     executes.
     In practice: repairs pass the normal gates.
     Gain: consistent with the model.
     Cost: none.
  2. **Everything stays.**
     Meaning: removals happen in-change.
     In practice: unreviewed corpus edits.
     Gain: none.
     Cost: the rejected model.
- Resolution: **Option 1**. Rejected: Option 2.

### Q8 — Smoke-test specificity

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the test distinguishes old from amended.
- Context: `accepted`, `proposed by`, and `Re-qualify` already occur in the
  old bundle.
- Question: which strings?
- Options:
  1. **Unique strings only.**
     Meaning: `## Execution model`, `## Inventory and dispositions`,
     `follow-up change`, and the wrapper sentence.
     In practice: the old bundle fails the assertion.
     Gain: a real regression check.
     Cost: none.
  2. **Reuse existing words.**
     Meaning: assertions pass on both versions.
     In practice: no signal.
     Gain: none.
     Cost: a fake check.
- Resolution: **Option 1**. Rejected: Option 2.

### Q9 — Half-applied bundle

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether a consumer can follow repair procedures by mistake.
- Context: the phases still contain the detailed rules.
- Question: what makes the failure visible?
- Options:
  1. **Per-phase execution-model line plus the no-corpus-edit check.**
     Meaning: each phase opens with "these are inventory criteria; the
     repairs run in the named follow-up"; validation checks that no corpus
     artifact was edited (except term cells).
     In practice: the failure is caught before archive.
     Gain: mechanical visibility.
     Cost: a line per phase.
  2. **Rely on the top-level block.**
     Meaning: a long document buries it.
     In practice: the likely failure.
     Gain: none.
     Cost: half-applied bundles.
- Resolution: **Option 1**. Rejected: Option 2.

### Q10 — README and changelog

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the contract docs match.
- Context: both describe in-change completion.
- Question: update them?
- Options:
  1. **Update both.**
     Meaning: the README defers to each bundle's completion rules; the
     changelog describes the inventory-and-follow-up model.
     In practice: 0.8.0's documentation is truthful.
     Gain: consistency.
     Cost: two edits.
  2. **Leave them.**
     Meaning: stale contract text.
     In practice: confusion.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q11 — Durable chapters

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the arc42 text describes the old model.
- Context: chapters 02, 06, and 07 carry in-change completion sentences.
- Question: reword them?
- Options:
  1. **Reword and mark rows 2, 6, 7 Yes.**
     Meaning: project-owned artifacts change through LLM-guided project
     changes; the plan archives once the inventory is complete; phases are
     inventoried in one aggregate change.
     In practice: durable docs match the model.
     Gain: conformance.
     Cost: three sentences.
  2. **Leave them.**
     Meaning: contradictory durable docs.
     In practice: a design-review finding.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Consistency | The complete stale-sentence inventory is captured in the amended files; no wrapper string is asserted by any spec or test. |
| Testability | Placeholder substitution is a fixed four-key replace; the amendment introduces no new placeholder, so the no-`{{` assertion is safe. |
| Testability | No fixture builds a 0.7.0 project; the smoke test stays manual with exact strings, matching the release change's precedent. |
| Consistency | Version and catalog claims verified: 0.8.0 in Cargo.toml and the config marker; the catalog entry and asset existence are validated by the loader. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Maintenance change for a released bundle; `specs` is skipped. | accepted |
| Typed Includes / Extends relationships | No relationship changes. | accepted |
| Confirmed Persona coverage | `none` — no user-visible product surface. | accepted |
| Actor and Persona distinction | No Gherkin actors introduced. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | No user-visible product surface; the amendment changes consumer-facing guidance. | accepted |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | Yes — ownership sentence reworded |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | No | No | Yes | — |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.13 archive timing |
| 7 | 07 deployment-view | Yes | Yes | Yes | Yes — bundle delivery sentence |
| 8 | 08 crosscutting-concepts | No | No | Yes | — |
| 9 | 09 architecture-decisions | No | No | Yes | — |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | No | No | Yes | — |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the change amends consumer guidance.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/migration-retrofit-plan/design.md` | Wrapper change added; table shape pinned; seeds/qualifiers/overrides rows; term-evidence exception; proposed-ADR owner rule; ADR step split; unique smoke-test strings; per-phase execution-model line; README/changelog and chapter edits added. |
| `givn/changes/migration-retrofit-plan/arc42.md` | Rows 2, 6, 7 marked Yes with the chapter edits. |
| `assets/migrations/0.7.0-to-0.8.0/{proposal,design}.md` | Rewritten to the inventory-and-plan model. |
| `docs/migrations/0.7.0-to-0.8.0.md` | Contract and prompts rewritten; two-level completion boundary. |
| `docs/migrations/README.md`, `CHANGELOG.md` | Amended-model wording. |
| `src/cli/migration.rs` | Wrapper strings defer to each bundle's completion rules. |
| `docs/arc42/02-architecture-constraints.md`, `06-runtime-view.md`, `07-deployment-view.md` | Old-model sentences reworded. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs` is intentionally skipped; no scenario updates were identified.
- [x] `givn lint --change migration-retrofit-plan` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
