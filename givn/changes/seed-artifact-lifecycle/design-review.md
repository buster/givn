# Design Review: seed-artifact-lifecycle

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Risk | What is the stdout/stderr contract of the no-feature early return with a seed advisory? | agreed — findings on stderr, no-features line on stdout, exit 0 |
| 2 | Testability | How does the clean-seed guard enter RED? | agreed — folded into the first advisory scenario |
| 3 | Missing scenarios | Is the valid `proposed by` active-change case covered? | agreed — scenario added |
| 4 | Missing scenarios | Are unrecognised status and prose-preceded status pinned? | agreed — combined scenario added; empty reference is unrecognised |
| 5 | Tech choices | Is the count-in-summary contract for advisories asserted? | agreed — mixed-run scenario added |
| 6 | Scope | Should the propose/archive stamping criteria become executable? | agreed (operator) — phrase scenario added; no @e2e tag |
| 7 | E2E fidelity | Does lint-command need its own `@e2e` scenario? | agreed (operator) — sibling `spec-overlap-gates` coverage recorded instead |
| 8 | Scope | Should `docs/workflow.md` record the seed scan and status? | agreed — handoff row and advisory table updated |
| 9 | Missing scenarios | Is the whole-tree-only scan scope pinned by a scenario? | agreed — keep scope; no negative-only scenario |
| 10 | Architecture documentation | Is an ADR required for the seed convention or advisory check? | agreed — no; canonical destination chapters 6.4 and 8.19 |

### Q1 — No-feature early return with a seed advisory

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: the observable contract of `givn lint` in a project with seeds but
  no feature files.
- Context: the three advisory scenarios run lint in a project that may have no
  `.feature` file, which currently prints "no .feature files found under
  givn/" and exits 0 before any finding is rendered. Without a decided
  contract the seed finding could vanish or replace the existing line.
- Question: what should that path print?
- Options:
  1. **Findings on stderr, keep the no-features line, exit 0.**
     Meaning: the early return renders seed findings first, keeps its existing
     stdout line, and the JSON path emits the findings instead of `[]`.
     In practice: `[SEED ] … no valid status line` on stderr and `givn lint:
     no .feature files found under givn/` on stdout, exit 0.
     Gain: the existing no-features contract is preserved and seeds are still
     reported.
     Cost: the finding-count summary does not apply on this path.
  2. **Replace the line with a summary.**
     Meaning: print `givn lint: 0 file(s) checked, 1 finding(s)`.
     In practice: a permanent scenario asserting the no-features line breaks.
     Gain: one consistent summary format.
     Cost: changes a documented permanent contract for an advisory.
- Resolution: **Option 1** because it preserves the permanent no-features
  contract and still reports the advisory. Rejected: Option 2 because an
  advisory must not rewrite an existing contract.

### Q2 — Clean-seed guard in the RED loop

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: how the "clean seed is not reported" guard enters the TDD loop.
- Context: the clean-seed scenario asserts only exit 0 and a clean summary,
  both already true before any seed check exists, so it cannot fail RED and
  would violate the "exit 0 = hard stop" rule.
- Question: how should the guard enter the loop?
- Options:
  1. **Fold it into the first advisory scenario.**
     Meaning: add a `Status: ready` seed to the missing-status scenario and
     assert its file is not named on stderr.
     In practice: the scenario fails RED on the `[SEED ]` assertion and the
     guard rides in the same commit.
     Gain: a RED-able scenario carries both the positive and negative cases.
     Cost: one extra fixture and one negative assertion.
  2. **Keep it separate as an immediate-GREEN guard.**
     Meaning: document it as a guard that starts green.
     In practice: a scenario that never fails RED enters tasks.md as a
     zero-exit stop.
     Gain: one scenario per behavior.
     Cost: violates the RED contract.
- Resolution: **Option 1** because every scenario must be able to fail RED.
  Rejected: Option 2 because an immediate-GREEN scenario cannot prove the
  check exists.

### Q3 — Valid `proposed by` active-change case

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether an implementation that flags every `proposed by` seed can
  pass.
- Context: the ghost and archived scenarios both expect a finding; without the
  active case, a check that reports every `proposed by` seed passes both.
- Question: should the active case be covered?
- Options:
  1. **Add the scenario.**
     Meaning: a seed proposed by an existing active change yields a clean run.
     In practice: `an existing change named "active-change"` plus a seed with
     `Status: proposed by active-change` prints the clean summary.
     Gain: the active/archived distinction is proven.
     Cost: one scenario.
  2. **Rely on the parse rule.**
     Meaning: no scenario; the design documents the rule.
     In practice: the valid branch is untested.
     Gain: none.
     Cost: an over-reporting implementation ships.
- Resolution: **Option 1** because the branch is the check's whole point.
  Rejected: Option 2 because it leaves the valid path unverified.

### Q4 — Unrecognised status and prose-preceded status

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the tolerance rule and the unrecognised-value case are
  pinned.
- Context: the parse rule reads the first `Status:` line anywhere in the file
  and treats an unrecognised value as missing. Neither behavior had a
  scenario, and an empty reference (`Status: implemented in`) was
  unclassified.
- Question: should these be covered and how is an empty reference classified?
- Options:
  1. **One combined scenario; empty reference is unrecognised.**
     Meaning: one seed with `Status: done` after prose is reported; one seed
     whose prose precedes `Status: ready` is not; an empty reference counts as
     unrecognised.
     In practice: two fixtures, two assertions, one RED failure.
     Gain: both design behaviours are pinned in one scenario.
     Cost: two fixtures in one scenario.
  2. **Unrecognised value only.**
     Meaning: skip the tolerance case.
     In practice: a parser that rejects prose-preceded status ships.
     Gain: fewer fixtures.
     Cost: the tolerance rule is unverified.
- Resolution: **Option 1** because both behaviors are design decisions that
  need executable cover. Rejected: Option 2 because the tolerance rule is the
  risk the handover names.

### Q5 — Count-in-summary contract for advisories

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether `N finding(s)` may coexist with exit 0, and whether a
  scenario proves it.
- Context: advisory findings are counted in the summary but never affect the
  exit code; all three advisory scenarios hit the no-features path, so the
  mixed run (feature plus advisory) was unasserted.
- Question: keep the count-in-summary contract and assert it?
- Options:
  1. **Keep it; add the mixed-run scenario.**
     Meaning: one clean feature plus a dangling seed prints `givn lint: 1
     file(s) checked, 1 finding(s)` on stderr, exit 0.
     In practice: the summary stays truthful while the contract stays
     advisory.
     Gain: the count contract is proven.
     Cost: one scenario.
  2. **Say "1 advisory finding(s)".**
     Meaning: a distinct summary wording for advisories.
     In practice: scripts parsing the summary see a new format.
     Gain: clearer output.
     Cost: a new permanent summary format for one finding family.
- Resolution: **Option 1** because the existing summary format stays stable
  and the advisory nature is carried by the `[SEED ]` label. Rejected:
  Option 2 because it changes a stable summary format.

### Q6 — Executable propose/archive stamping

- Status: closed 2026-09-17 (operator decision)
- Blocks: whether the acceptance criteria "the propose flow stamps a seed and
  records its path" and "the archive guidance stamps implemented/withdrawn"
  have executable evidence.
- Context: the handover leaves propose/archive stamping as prose and makes the
  archive scenario conditional on an existing spec asserting archive-step
  prose (none does). Without assertions the criteria cannot fail RED.
- Question: add the phrase scenarios or keep them prose-only?
- Options:
  1. **Add one phrase scenario for both commands.**
     Meaning: assert the generated propose command contains "proposed by" and
     the generated archive command contains "implemented in".
     In practice: two existing generic command steps, one new scenario.
     Gain: both acceptance criteria become executable.
     Cost: one scenario and one routing row beyond the handover's list.
  2. **Keep prose-only.**
     Meaning: review verifies the guidance by inspection.
     In practice: the criteria have no executable guard.
     Gain: handover scope preserved exactly.
     Cost: an acceptance criterion that cannot fail.
- Resolution: **Option 1** (operator choice) because the acceptance criteria
  deserve executable evidence and the step machinery already exists. The
  `@e2e` half of the recommendation was declined: running lint is already
  proven end-to-end by sibling scenarios (see Q7).

### Q7 — Own `@e2e` scenario for lint-command

- Status: closed 2026-09-17 (operator decision)
- Blocks: whether this change adds an `@e2e` tag and an inventory row for the
  lint capability.
- Context: `lint-command` has no inventory row and no `@e2e` scenario, but the
  static lint action is already driven end-to-end by two `@e2e` scenarios in
  the sibling `spec-overlap-gates` capability.
- Question: add the tag and inventory row, or record sibling coverage?
- Options:
  1. **Add the `@e2e` tag and inventory row.**
     Meaning: the advisory scenario runs in the e2e scope and the use case
     gains a lint-command inventory row.
     In practice: one more e2e scenario and an e2e task in this change.
     Gain: per-capability e2e self-sufficiency.
     Cost: a third e2e scenario for the same command action.
  2. **Record sibling coverage in the matrix.**
     Meaning: the two `spec-overlap-gates` `@e2e` scenarios are the lint
     action's end-to-end evidence; the design matrix names them.
     In practice: no new tag, no inventory row.
     Gain: no duplicate e2e coverage of one action.
     Cost: lint-command relies on sibling evidence.
- Resolution: **Option 2** (operator choice) because running the static lint
  check is one action already proven end-to-end, and the canonical policy
  normalizes one `@e2e` per action. Rejected: Option 1 because it duplicates
  the action's e2e evidence.

### Q8 — Workflow documentation

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether `docs/workflow.md` records the seed scan and status.
- Context: the handover asks for a workflow-doc update when the scan needs a
  documented scan root; the advisory table lists only overlap findings and
  the handoff row does not mention the status.
- Question: update the workflow document?
- Options:
  1. **Update both spots.**
     Meaning: one advisory-table row for seed status and `Status: ready` in
     the handoff row.
     In practice: two small edits.
     Gain: the user-facing workflow stays truthful.
     Cost: none.
  2. **Rely on the runtime view and the seed section.**
     Meaning: leave workflow.md unchanged.
     In practice: the advisory table omits the new finding family.
     Gain: smaller diff.
     Cost: a stale workflow document.
- Resolution: **Option 1** because the document is the user-facing workflow
  and the edits are small. Rejected: Option 2 because it leaves a known gap.

### Q9 — Whole-tree-only scan scope

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether `givn lint --change` should report seed advisories.
- Context: the design restricts advisories to the default scope so a change's
  lint stays about its delta; no scenario can fail RED on a negative-only
  exclusion.
- Question: keep the restriction or broaden the scan?
- Options:
  1. **Keep it, no negative scenario.**
     Meaning: `--change` and explicit paths do not scan seeds.
     In practice: the scope is recorded in design, 6.4, and 8.19.
     Gain: change-scoped lint stays focused.
     Cost: the exclusion has no executable guard.
  2. **Broaden to `--change`.**
     Meaning: the authoring loop sees stale seeds too.
     In practice: every change lint reports seeds unrelated to the change.
     Gain: earlier visibility.
     Cost: noise in the change loop.
- Resolution: **Option 1** because the scan is project hygiene, not a change
  gate. Rejected: Option 2 because unrelated advisories would dilute the
  authoring signal.

### Q10 — ADR requirement

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the seed convention or advisory finding needs a decision
  record.
- Context: the qualification gate requires real alternatives, architectural
  impact, durable consequence, a canonical lower-level home, and an
  existing-ADR check. The advisory check changes no command boundary or exit
  contract; ADR-027 fixes the lint exit precedence and ADR-028 the
  mandatory-check boundary.
- Question: create or amend a record?
- Options:
  1. **No record; canonical destination chapters 6.4 and 8.19.**
     Meaning: the convention lives in the seed-lifecycle section and the
     runtime view; no ADR.
     In practice: chapter 9 unchanged.
     Gain: the ADR register stays meaningful.
     Cost: none.
  2. **Amend the lint record (ADR-027).**
     Meaning: note the advisory finding family and its exit neutrality.
     In practice: an amendment for a non-qualifying choice.
     Gain: the register mentions the family.
     Cost: an amendment the gate does not require.
- Resolution: **Option 1** because the choice fails architectural impact and
  durable consequence and neither existing boundary changes. Rejected:
  Option 2 because it inflates the register.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Testability | The single-scenario command is valid and identical in form to change 1's; strict mode is `.fail_on_skipped()`; the stub is `unimplemented!()`. |
| Risk | Exit-code neutrality is mechanically safe: the exit code derives from boolean flags while the summary count is `total_errors`; `render_prose` needs one `seed_status` arm for the `[SEED ]` label. |
| Risk | No glob dependency is needed; the scan walks `read_dir` like `collect_feature_files`. The new fixture Given needs no use-case fixture because seeds live outside `givn/specs/`. |
| Scope | The existing `the generated givn-<command> command contains` step already covers propose and archive commands, making the Q6 scenario cheap and RED-able. |
| Scope | No permanent spec asserts archive-step prose, so the handover's conditional archive scenario is not triggered; the Q6 scenario is the operator-approved addition. |
| E2E fidelity | Two `spec-overlap-gates` `@e2e` scenarios drive `givn lint` end-to-end; `lint-command` has zero `@e2e` scenarios and no inventory row; the fragment declares `Interactions: none`. |
| Missing scenarios | A project without `givn/ideation/` is covered by the permanent "No .feature files under givn/" scenario, which forces the no-op path. Seeds outside `givn/ideation/<topic>/changes/` are deliberately out of the schema. |
| Architecture documentation | The independent 12-row walk matches arc42.md; claimed chapters contain real content; no ADR covers the seed lifecycle and no amendment is forced. |
| Architecture documentation | arc42.md row 12 wording corrected: the lifecycle is recorded under the new `Seed status` row, not by changing `Change seed`. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `lint-command` extends the verify-change use case; `ideation-skill` keeps fragment ownership; `prose-consolidation` extends author-change. | accepted |
| Typed Includes / Extends relationships | `verify-change` and `author-change` include the `support` fragment; no relationship changes. | accepted |
| Confirmed Persona coverage | `agent--technical-role` (generated guidance, lint output) and `givn-maintainer--owner` (seed convention, rule corpus) are confirmed permanent Personas. | accepted |
| Actor and Persona distinction | Gherkin keeps Developer and Authoring agent as actors; Personas stay review lenses. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| agent--technical-role | Consumes the generated ideation guidance, the propose/archive command instructions, and the lint advisories. | `The ideation guidance carries the seed schema and its status values`; `The propose and archive commands carry the seed stamping instructions`; lint advisory scenarios |
| givn-maintainer--owner | Owns the seed lifecycle convention, the rule corpus, and the advisory posture. | `A change seed without a status line is reported as an advisory`; chapter 8.19 |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes — `cli::lint` row updated |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — 6.4 seed scan paragraph |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — 8.19 seed lifecycle |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — no qualified candidate; destination 6.4/8.19 |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes — R-46 mitigation amended |
| 12 | 12 glossary | Yes | Yes | Yes | Yes — `Seed status` row (arc42.md wording corrected) |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the canonical destination is chapters 6.4
      and 8.19, and the ADR-027/ADR-028 boundary check is recorded.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/seed-artifact-lifecycle/design.md` | Early-return contract, empty-reference classification, count-in-summary scenario, propose/archive phrase scenario, sibling e2e coverage, workflow update, step-definition rows. |
| `givn/changes/seed-artifact-lifecycle/specs/verify-change/lint-command.feature` | Rewritten to six scenarios: missing status (with clean-seed guard), dangling, stale, active, unrecognised/prose, mixed-run count. |
| `givn/changes/seed-artifact-lifecycle/specs/author-change/prose-consolidation.feature` | New delta: propose/archive stamping phrases. |
| `givn/changes/seed-artifact-lifecycle/proposal.md` | Added the `prose-consolidation` routing row. |
| `givn/changes/seed-artifact-lifecycle/arc42.md` | Corrected the chapter 12 wording. |
| `docs/workflow.md` | Handoff row names `Status: ready`; advisory table gains the seed-status row. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios identified.
- [x] `givn lint --change seed-artifact-lifecycle` exits 2 (WIP, no parse errors).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
