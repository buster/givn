# Design Review: migrate-0-7-0-to-0-8-0

The design is the release-owned bundle plus this project's inventory. The
grilling therefore covers the bundle's applicability, the operator decisions
that shaped the inventory, and the disposition choices.

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Is the maintenance change the inventory or the repair site? | agreed — inventory and plan; repairs are named follow-ups |
| 2 | Operator decision | Which names does the R8 sweep record? | agreed — full list, 29 names |
| 3 | Operator decision | Where are the final new names decided? | agreed — inside each rename follow-up, one use case at a time |
| 4 | Operator decision | How is the canonical migration-plan ID freed? | agreed — the archived release record is renamed `release-0-8-0` |
| 5 | Operator decision | How is the consumer upgrade exercised? | agreed — marker set to 0.7.0; real commits A and B |
| 6 | ADR re-qualification | Do obsolete records go to the ADR archive? | agreed — superseded by ADR-069, never the failing-record rule |
| 7 | ADR re-qualification | Where does the ADR-027 rationale move? | agreed — the verify-change Rules and the lint capability spec |
| 8 | Term evidence | Where is the collision evidence recorded? | agreed — in the file's collision-check cells now (the one edit) |
| 9 | Follow-up granularity | One follow-up per rename or per use case? | agreed — per use case, per the bundle's rename rule |
| 10 | Persona coverage | Are `Personas: none` declarations on user-visible surfaces deviations? | agreed — rows; the user confirms the Persona or the reason |
| 11 | Completeness | Is the orphaned `config-overlay` capability handled? | agreed — folded into the fragment rename follow-up |
| 12 | Testability | Does the inventory satisfy the bundle's validation? | agreed — lint, check review, and the suite |

### Q1 — Inventory or repair

- Status: closed 2026-09-18 (operator, settled decisions)
- Blocks: whether the maintenance change edits the corpus.
- Context: the amended bundle directs the repairs to follow-ups.
- Question: record deviations only, or repair in place?
- Options:
  1. **Inventory and plan.** Every deviation gets a row and a named
     follow-up; the one exception is the term collision-check evidence.
     Meaning: the maintenance change archives when the inventory is complete.
     In practice: 18 follow-ups carry the repairs through the normal gates.
     Gain: every repair is reviewed and specified.
     Cost: the follow-ups are the bulk of the work.
  2. **Repair in place.** Fix the corpus in the maintenance change.
     Meaning: one large unreviewed corpus edit.
     In practice: the archive gate runs against a rewritten corpus.
     Gain: fewer changes.
     Cost: the exact failure the amended bundle exists to prevent.
- Resolution: **Option 1** (the seven settled decisions). Rejected: Option 2.

### Q2 — R8 sweep scope

- Status: closed 2026-09-18 (operator, after the before/after list)
- Blocks: how many naming deviations the inventory records.
- Context: the rule falsifies names that name a technical component,
  mechanism, or channel; 17 names fail a narrow reading, 29 a literal one.
- Question: narrow or full sweep?
- Options:
  1. **Full list (29).** Both channel/component names and mechanism/strategy
     names are recorded.
     Meaning: `template-conditional-rendering`, `config-overlay`,
     `readme-coverage-badge`, and similar become rename follow-ups.
     In practice: the follow-ups propose the final names with use-case
     context; the operator reviews them there.
     Gain: the rule is applied as written.
     Cost: the largest follow-up block.
  2. **Narrow list (17).** Only literal channel, deployment-component,
     removed-command, and one-off task names are recorded.
     Meaning: mechanism/strategy names stay.
     In practice: a later review can re-flag them.
     Gain: a smaller block.
     Cost: the literal rule text is knowingly unapplied.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Final names decided where

- Status: closed 2026-09-18 (operator)
- Blocks: whether this change must fix the final names.
- Context: the inventory records the deviation class, not the new name.
- Question: decide the names here or in the follow-up?
- Options:
  1. **In the follow-up.** The inventory records "rename" and the follow-up's
     proposal and design propose the concrete names; the design-review gate
     is where the operator approves them, one use case at a time.
     Meaning: the inventory stays an inventory.
     In practice: the operator sees each rename with its use-case context.
     Gain: no premature naming.
     Cost: none.
  2. **Here.** Fix all 29 names in the inventory.
     Meaning: the inventory becomes a design.
     In practice: names are decided without the follow-up's split context.
     Gain: fewer decisions later.
     Cost: rework when a split re-homes a capability.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Release-record rename

- Status: closed 2026-09-18 (operator)
- Blocks: whether `givn upgrade` can generate the canonical plan.
- Context: the archived release-development record occupied
  `migrate-0-7-0-to-0-8-0`; `migration_exists` refuses to create the plan and
  `givn archive` merges into an existing archive directory.
- Question: rename the release record or use a distinct plan ID?
- Options:
  1. **Rename the release record to `release-0-8-0`.** Its self-references
     (titles, receipt `change`, artifact paths) and the one cross-reference
     in the archived `migration-retrofit-plan` proposal are updated.
     Meaning: the canonical ID belongs to the project's real migration.
     In practice: `givn upgrade` prepared the plan exactly as the guide
     promises.
     Gain: the dogfood exercises the real generated-plan path.
     Cost: one archived record is renamed.
  2. **Distinct plan ID.** Keep the archive untouched; hand-create the plan.
     Meaning: the repo never exercises the generated-plan path.
     In practice: the plan diverges from every consumer's ID.
     Gain: no archive edit.
     Cost: a weaker dogfood.
- Resolution: **Option 1**. Rejected: Option 2.

### Q5 — Marker simulation

- Status: closed 2026-09-18 (operator instruction)
- Blocks: whether the migration is real or rehearsed.
- Context: the repo's marker was already 0.8.0 from the release change.
- Question: reset the marker and run the real upgrade, or simulate?
- Options:
  1. **Reset to 0.7.0 and run `givn upgrade`.** The binary produced Commit A
     (managed refresh) and Commit B (the plan) and the change is the real
     generated one.
     Meaning: this repo is a consumer of its own release.
     In practice: the inventory is the plan's phases worked in order.
     Gain: full dogfood.
     Cost: none.
  2. **Rehearse in a scratch copy.** The repo's marker and history stay.
     Meaning: the repo's own corpus is never migrated.
     In practice: the deviations stay.
     Gain: no marker edit.
     Cost: no real migration.
- Resolution: **Option 1**. Rejected: Option 2.

### Q6 — Obsolete ADR destination

- Status: closed 2026-09-18 (operator, bundle rules)
- Blocks: where ADR-044, ADR-045, and ADR-054 go.
- Context: they describe removed engine mechanisms and were missed by
  ADR-069's supersession list.
- Question: treat them as failing records or as superseded history?
- Options:
  1. **Superseded history.** Mark superseded by ADR-069 and move to
     `docs/arc42/adr/archive/`, matching the sixteen records ADR-069 already
     superseded.
     Meaning: they remain readable context.
     In practice: the register holds only current decisions.
     Gain: consistent with the register contract.
     Cost: ADR-069's supersession list grows.
  2. **Failing records.** Remove them and never archive them.
     Meaning: the bundle's failing-record rule.
     In practice: the removed-mechanism history is lost.
     Gain: none.
     Cost: wrong rule applied; the records did not fail the gate.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — ADR-027 rationale destination

- Status: closed 2026-09-18 (operator, bundle rules)
- Blocks: where the non-qualifying naming decision lives.
- Context: the record fails all three falsification tests; its behavior
  contract is the lint capability.
- Question: which canonical artifact owns the rationale?
- Options:
  1. **The verify-change Rules plus the lint capability spec.** The naming
     and exit-code rationale becomes a rule in the use case and the spec.
     Meaning: the observable contract keeps the rationale.
     In practice: the record leaves the register and README.
     Gain: the rule is reviewable where the behavior lives.
     Cost: none.
  2. **The instructions only.** Put the rationale in generated guidance.
     Meaning: guidance is a snapshot, not a contract.
     In practice: the rationale drifts from the spec.
     Gain: none.
     Cost: a weaker home.
- Resolution: **Option 1**. Rejected: Option 2.

### Q8 — Term evidence location

- Status: closed 2026-09-18 (operator, bundle rule)
- Blocks: whether the collision evidence is recorded now.
- Context: the term records are still the old schema; the follow-up migrates
  them.
- Question: record the evidence in the file now or defer it?
- Options:
  1. **Record it in the file's collision cells now.** The one permitted
     direct edit; the follow-up carries the cells into the new schema.
     Meaning: no "no collision" claim without commands.
     In practice: nine confirmed terms carry dated commands and counts.
     Gain: the bundle's exception is honored.
     Cost: one corpus edit in an inventory change.
  2. **Defer to the follow-up.** The inventory notes the evidence.
     Meaning: the follow-up re-runs the searches.
     In practice: the evidence is dated to the follow-up, not the migration.
     Gain: no corpus edit.
     Cost: contradicts the bundle's one exception.
- Resolution: **Option 1**. Rejected: Option 2.

### Q9 — Follow-up granularity

- Status: closed 2026-09-18 (operator, bundle rule)
- Blocks: the number of follow-ups.
- Context: the bundle says one use case per rename follow-up.
- Question: one follow-up per rename or per use case?
- Options:
  1. **Per use case.** Structural repair and renames for a use case ship in
     one follow-up; the support fragment gets its own.
     Meaning: 9 Phase 1 follow-ups.
     In practice: each follow-up owns its use case's spec deltas.
     Gain: the split and renames are designed together.
     Cost: none.
  2. **Per rename.** 29 small follow-ups.
     Meaning: each rename is isolated.
     In practice: the split context is lost.
     Gain: smaller diffs.
     Cost: 29 review cycles.
- Resolution: **Option 1**. Rejected: Option 2.

### Q10 — Persona coverage

- Status: closed 2026-09-18 (operator, bundle rules)
- Blocks: whether `Personas: none` is a deviation.
- Context: five use cases with user-visible CLI surfaces declare none.
- Question: rows or no action?
- Options:
  1. **Rows.** The follow-up confirms the relevant Persona with the user or
     records the reason the surface has none.
     Meaning: the review rule ("a user-visible surface requires at least one
     confirmed Persona") is applied.
     In practice: five rows and one follow-up.
     Gain: coverage is deliberate.
     Cost: one follow-up.
  2. **No action.** The declarations are intentional.
     Meaning: no persona review for those surfaces.
     In practice: the review rule stays unapplied.
     Gain: fewer rows.
     Cost: an unexamined gap.
- Resolution: **Option 1**. Rejected: Option 2.

### Q11 — Orphaned capability

- Status: closed 2026-09-18 (operator, bundle rules)
- Blocks: what happens to `config-overlay.feature`.
- Context: the file exists; `fragment.md` does not list it and no use case
  owns it.
- Question: fold it into the fragment rename follow-up or a separate repair?
- Options:
  1. **Fold into `rename-support-fragments`.** The follow-up restores
     ownership and renames.
     Meaning: one follow-up handles the fragment corpus.
     In practice: `fragment.md` lists the renamed capability.
     Gain: no orphan.
     Cost: none.
  2. **Separate follow-up.** One change for one file.
     Meaning: more ceremony.
     In practice: the same edit later.
     Gain: isolation.
     Cost: a review cycle.
- Resolution: **Option 1**. Rejected: Option 2.

### Q12 — Validation

- Status: closed 2026-09-18 (operator, bundle rules)
- Blocks: whether the change can archive.
- Context: the bundle names `givn lint`, `givn check review`, and the suite.
- Question: what evidence does the change record?
- Options:
  1. **All three.** `givn lint` (advisories recorded), `givn check review`,
     and `./run-tests.sh` plus `./run-tests.sh --e2e`.
     Meaning: the bundle's validation and the normal gates both run.
     In practice: the archive receipt carries the measured scopes.
     Gain: complete evidence.
     Cost: one suite run.
  2. **Lint only.** The change edits no code.
     Meaning: the archive gate still runs the suite at archive time.
     In practice: the same suite runs later.
     Gain: none.
     Cost: no early signal.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Completeness | The generated proposal wrapper has missing spaces at four Rust line continuations ("theinstalled", "inCommit", "workeach", "mustpreserve") — a product defect in `src/cli/migration.rs`, exposed by dogfooding. Recorded as a separate product fix after this migration; it does not change the inventory. |
| Completeness | The five use cases declaring `Personas: none` all have user-visible CLI surfaces, so the review rule applies. |
| Consistency | `config-overlay.feature` is unowned and unlisted; the other eighteen fragment files are listed in `fragment.md`. |
| Consistency | The three topic persona copies are byte-identical to the permanent files. |
| Testability | `givn lint` exits 0 with advisory findings only; the `[SEED ]` advisories match the seven seeds. |
| Consistency | ADR-069's supersession list omits ADR-044, ADR-045, and ADR-054 although their mechanisms were removed with the engine. |

## Open questions

None. The R8 final names are deliberately deferred to the rename follow-ups
(decision 3).

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Eight use-case documents inventoried; every permanent capability has an owner except `config-overlay` | accepted |
| Typed Includes / Extends relationships | `archive-change` Includes `usecase: verify-change`, which the relationship rules forbid | accepted |
| Confirmed Persona coverage | Five use cases declare none on user-visible surfaces | accepted |
| Actor and Persona distinction | No Gherkin actors change in this change | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | The developer's corpus is migrated; persona files are in scope | inventory rows |
| givn-maintainer--owner | Owns the register, vocabulary, and harness being migrated | inventory rows |
| agent--technical-role | Consumes the migrated specs and guidance | inventory rows |

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
| 9 | 09 architecture-decisions | No | No | Yes | — |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | No | No | Yes | — |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] ADR verdicts recorded in arc42.md: 24 qualified, 3 obsolete superseded
      by ADR-069, 1 NOT_QUALIFIED routed to lower-level artifacts.
- [x] No new MADR; no "Bad, because..." consequence requires a chapter 11
      counterpart.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/migrate-0-7-0-to-0-8-0/design.md` | Project inventory appended: Phase 1-5 tables, term evidence, follow-up register |
| `givn/ideation/multi-scm-support/domain-terms.md` | Collision-check evidence recorded for the nine confirmed terms (the one permitted edit) |
| `givn/changes/migrate-0-7-0-to-0-8-0/arc42.md` | Re-qualification verdicts and audit notes recorded |
| `givn/archive/release-0-8-0/` | Release record renamed from `migrate-0-7-0-to-0-8-0`; self-references updated |
| `givn/archive/migration-retrofit-plan/proposal.md` | Cross-reference annotated with the rename |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects the inventory and the operator decisions.
- [x] `specs` is intentionally skipped; the bundle names no scenario delta.
- [x] `givn lint --change migrate-0-7-0-to-0-8-0` has no parse errors.
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
