# Design Review: archive-proof

## Method

Fresh-context grilling subagent (general-purpose, high effort) read the
proposal, specs, design, arc42 assessment, and the relevant codebase, then
walked every required branch: scope, tech choices, missing scenarios,
testability, E2E fidelity, interaction coverage, risk, use-case/Persona
context, ADR qualification, arc42 re-derivation, fixture blast radius, and the
`GIVN_RESULT_FILE` mechanism. The orchestrator resolved every question with
the recommended decision and applied the hardening edits below.

## Questions resolved

| # | Finding | Decision | Applied |
|---|---|---|---|
| Q1 | The permanent fragment scenario "Pre-merge archive failure preserves the permanent specification tree" contradicts forward repair and is not in the delta. | Remove it in a new fragment delta and add "A failed archive preserves the merged candidate unproven". | `specs/fragments/repository-ownership-fixtures.feature` (new), design Step Definitions/fixture notes |
| Q2 | The delta E2E scenario claimed a preserved unproven candidate while the design blocks unconfigured E2E pre-merge. | Keep the pre-merge config-gap block; the E2E scenario now asserts the change stays active and no candidate exists. | `enforce-post-merge-e2e-gate.feature` modified scenario |
| Q3 | Finalization wrote the proven receipt before the guards, so a guard failure left a false `proven` state. | Guards run first; the proven receipt is written only after both pass. | design finalization step 7, `06-runtime-view.md` 6.7 |
| Q4 | The pending receipt cannot name its own candidate commit hash. | Pending receipt has `candidate_commit: null`; finalization sets both ids to the verified HEAD. | design data model, step 5 |
| Q5 | "Malformed" result was promised but not covered. | Added `malformed` to the result-contract outline. | `archive-proof.feature` |
| Q6 | Fixture blast radius under-enumerated; Git identity and coverage handling unspecified. | Added the explicit file list, `git_init` identity setup, `disable_coverage_addon` default, and the `emit-result.sh` helper to the design. | design fixture migration |
| Q7 | Legacy and corrupt archive states had behavior with no scenario; `read_status` could not see the both-dirs state. | Added both refusal scenarios; signature is now `read_status(project_root, id)`. | `archive-proof.feature`, design |
| Q8 | `run-tests.sh` zero-scenario early exit conflicts with fail-closed results. | When `GIVN_RESULT_FILE` is set, the early exit writes a `total: 0` result so Givn reports the zero-scenario contract failure. | design `GIVN_RESULT_FILE` support |
| Q9 | The modified E2E Given named a `"true"` command that no longer reports counts. | Given now says the e2e command reports scenario counts. | `enforce-post-merge-e2e-gate.feature` |
| Q10 | `local_timestamp()` cannot produce the receipt's RFC3339 timestamps and no date crate is allowed. | Receipt uses Unix epoch seconds (`started_at_unix`, `finished_at_unix`). | design tech decisions and data model |
| Q11 | The Interaction Coverage Matrix covered only the 3 new inventory entries. | Matrix now covers all 9 durable inventory entries with CLI mechanisms. | design matrix |
| Q12 | A crash between moving the change and writing the pending receipt left an unrecoverable legacy-looking state. | The pending receipt is written before the active directory is removed. | design step 5 |
| Q13 | Dirty evidence paths are excluded before verification while the receipt binds a commit. | Documented: evidence paths are outputs, not inputs; the receipt binds the code/spec commit and the finalization commits derived evidence; leftover evidence is the resume path. | design finalization guard |
| Q14 | Missing scenarios for commit rejection, coverage-failure resume, wrong-scope results. | Added "A rejected candidate commit leaves the merge staged and unproven", "A coverage failure proves on rerun after the repair", and the `declaring the wrong scope` outline row. | specs |
| Q15 | New modules were registered in `lib.rs` only. | `src/main.rs` added to the changed-modules list. | design |
| Q16 | Repository-root-relative status/add breaks for a project nested in a monorepo. | Explicit product decision: the project root must be the repository root; nested projects are refused with a named constraint and a scenario. | `git.rs` design, TC-32, `archive-proof.feature` |
| Q17 | Instructions routing was attributed only to `report_archived_change`. | Both `report_no_active_changes` (bare command) and `report_archived_change` (`--change`) surface pending archives. | design module map |
| Q18 | ADR-070's Confirmation referenced a nonexistent review gate. | Reworded to the archive gate plus the independent change review. | ADR-070 |

## Confirmed clean branches

- All `@givn.modified`/`@givn.removed` titles match permanent scenario titles
  byte-for-byte.
- The delta use-case base hash matches the current permanent document.
- New interaction inventory rows match the new `@e2e` titles exactly.
- ADR-070 qualifies, is `proposed`, registered, and its Bad consequences map
  to chapter 11 risks.
- The independent 12-row arc42 re-derivation matches `arc42.md`; chapter
  content matches the design; diagrams are Mermaid.
- The cucumber-rs 0.23 mechanism (`run` returns the default `Summarize`
  writer; `scenarios_stats()` exists) is implementable, and forcing the cargo
  path under `GIVN_RESULT_FILE` is necessary because nextest runs one process
  per scenario.

## Persona and use-case disposition

No Persona is confirmed for `archive-change`; Actors remain Developer and
Reviewer. No Persona was invented, promoted, or used as a Gherkin actor. The
use-case contract change (minimal guarantee to forward repair) is recorded in
the delta `usecase.md` and mirrored in the specs.

## Hardening verification

`givn lint --change archive-proof` exits 2 with WIP findings only (every new
and modified scenario is intentionally `@wip` until its step definitions and
implementation exist).

DESIGN-REVIEW: PASS

## Re-review: scope additions (implementation phase)

A second fresh-context review covered the three permanent scenarios outside
the routed capabilities that required deltas (`coverage-script-workflow`,
`deduplicate-change-dir-detection`, `adr-qualification-gate`). Findings and
resolutions:

- **ADR-070 acceptance timing** — the register scenario is archive-verified
  and asserts no `proposed` records, so accepting ADR-070 before the archive
  is forced. Chapter 09 now records the deliberate departure from the
  ADR-067 precedent and the design owns the decision. Resolved.
- **tasks.md coverage** — RED/GREEN/REFACTOR/COMMIT sections added for all
  three scope additions; the flip task now records the forced timing.
- **Coverage scenario title** — the retained `@givn.modified` title claimed
  the project was preserved while the body asserted mutation; converted to
  `@givn.removed` + `@givn.added` with a truthful title.
- **Dead rollback/backup step definitions** — all removed from
  `workflow_enforcement_steps.rs`, `coverage_script_workflow_steps.rs`, and
  `enforce_post_merge_e2e_gate_steps.rs`; the new archive-proof binding that
  asserts a missing backup is kept.
- **Stale arc42 8.9a and manifest comment** — updated to finalization
  ordering and the no-restore contract.
- **Review assets** — `assets/instructions/review.md` and
  `assets/templates/review.md` no longer require suite execution or
  instrumented coverage in review; they consume the archive receipt.

All deltas bind, titles match byte-for-byte, and the delta-scope suite
(`GIVN_FEATURES=givn/changes/archive-proof/specs`) passes 43/43 scenarios
plus 4/4 e2e scenarios. No blocker remains.

DESIGN-REVIEW: PASS
