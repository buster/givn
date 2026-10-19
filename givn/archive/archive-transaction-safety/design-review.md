# Design review: archive-transaction-safety

Two independent fresh-context reviews were run against the proposal, delta
specs, design, arc42 assessment, permanent contract, and source. Questions that
the casebook or the proposal already settles were resolved from the artifacts;
no user preference was required.

## Round 1 — grilling findings and resolutions

| ID | Finding | Resolution |
|---|---|---|
| B1 | Move-then-merge re-points only half the post-rename reads: narrative merge and new-capability routing still read `givn/changes/<id>`. | Resolved: the merge plan (relative delta, parsed feature, permanent target, retirement directory) is resolved before the move; the merge reads the archived candidate, and routing reads the archived proposal. Recorded in design D4 and the architecture-impact table. |
| B2 | PendingWithActive with a tracked/staged duplicate has no candidate-revision contract. | Resolved: remove the duplicate, stage the change path, and publish once after any merge only when the revision changed. A staged/untracked restore returns the index to the candidate and needs no publication. Recorded in design "Witness scopes" and the transition sequence. |
| B3 | `merge_completed` plus retry cannot resume retirement/modified/added safely. | Resolved: per-file progress (`merged_deltas`, `merge_in_progress`), prevalidation dry run, parsed-scenario retry comparison, idempotent retirement cleanup and de-duplicated tombstone, atomic tombstone writes. Recorded in D4/D5 and R-100. |
| B4 | The both-directory comparison is underspecified. | Resolved: an active-subset rule with byte comparison; evidence paths are ignored on both sides; the active duplicate is not authoritative. Recorded in D1 and the comparison unit tests. |
| H1 | Use-case main flow keeps merge before move. | Resolved: main flow is gates, write receipt and move, merge, evidence, publish, verify, coverage, finalize. |
| H2 | Glossary says a resume never repeats the merge. | Resolved: repeated only when `merge_completed` is false. |
| H3 | Missing scenarios for interrupted merge, rename failure, no-receipt, status route, idempotence. | Resolved: scenarios added for each, plus an in-flight delta retry. |
| H4 | The modified scenario's unproven assertion is a fixture tautology. | Resolved: it asserts the conflict message, an unchanged receipt, and the surviving active directory. |
| H5 | Single-scenario command does not target the delta. | Resolved: command includes `GIVN_FEATURES=.../specs/archive-change`. |
| H6 | A rejected finalization publication leaves a proven-on-disk archive. | Resolved: Proven is final only when the committed receipt is proven; otherwise archive republishes finalization when the tested revision matches HEAD, or restores pending and re-verifies. The scenario's second run exercises the published-history check. |
| H7 | Transition resume skips all gates. | Resolved: a rerun of an interrupted move repeats the pre-publication gates before retrying the rename. |
| H8 | Receipt writes are not atomic. | Resolved: temp file plus rename; startup removes reserved `*.givn-tmp` files. |
| H9 | One Corrupt variant cannot carry distinct messages. | Resolved: `Corrupt { both_dirs, pending_conflict }` with three messages. |
| H10 | Persona disposition contradicts the confirmed collection. | Resolved: design records the three existing personas as review lenses and states that the proposal's "none" means no new Persona was confirmed. |
| H11 | Duplicate Q-102. | Resolved: the new quality scenario is Q-104; `arc42.md` cites Q-104. |
| H12 | Chapter 01 marked Yes but empty. | Resolved: row 1 is No; the quality scenario lives in chapter 10. |
| H13 | Same-filesystem constraint missing from chapter 02. | Resolved: TC-38 added; `arc42.md` row 2 is Yes. |
| H14 | Chapter 06 state table and wording are incomplete. | Resolved: rows added for interrupted move, no receipt/proven, and unreadable receipt; the failed-rename state names the pending receipt. |
| H15 | "Archive candidate" conflicts with the pre-merge state. | Resolved: the glossary definition starts the candidate at the transition and includes `merge_completed: false`. |
| M2 | Fossil publication accepts a sibling change by prefix. | Rejected as incorrect: `Path::starts_with` is component-aware, so `givn/changes/x-other` does not match `givn/changes/x`. |
| M3 | No RED stub pattern or unit-test rows. | Resolved: panic stub, single-scenario command, unit-test table, and scenario RED table added. |
| M4 | Evidence prefix semantics are underspecified. | Resolved: trailing-separator prefixes, exact receipt path, positive/negative unit tests. |
| M5 | ADR check lacks a canonical destination. | Resolved: existing ADRs checked; `NOT_QUALIFIED`; canonical destination is `design.md`. |
| M6 | Visual contract heading is not machine-readable. | Resolved: exact `## Visual Design Contract` heading with `Interface: none`. |
| M7 | New scenarios lack `@wip`. | Resolved: all delta scenarios are `@wip`; tasks remove them before archive. |

## Round 2 — hardening findings and resolutions

| ID | Finding | Resolution |
|---|---|---|
| N1 | PendingWithActive has no defined clean-witness scope. | Resolved: a narrowed witness allows exactly the change path and merge targets; unrelated dirty paths still block. Staging returns a staged-only restore to the candidate. |
| N2 | A staged restore cannot be published and can dead-end. | Resolved: stage the change path, use `has_staged_changes`; skip publication when the index returns to the candidate, publish once when a committed restore leaves a deletion. |
| N3 | Merge resume has no merge-aware clean witness. | Resolved: narrowed witness allows the exact merge-plan targets plus `.retired.yaml`; unrelated specs and files block. |
| N4 | D7 does not survive a crash or a failed receipt restore. | Resolved: provenance is decided from the committed receipt via `revision_file`, not only the working tree; republish or re-verify. |
| N5 | Use-case step 8 records the receipt before the README refresh. | Resolved: refresh before `write_proven`. |
| N6 | Retirement retry can duplicate the tombstone or orphan the narrative. | Resolved: cached retirement target, repeated last-capability cleanup, idempotent de-duplicated atomic tombstone write. |
| N7 | The idempotence scenario does not exercise idempotence. | Resolved: new "in-flight delta file is retried" scenario sets `merge_in_progress` against an already-merged spec. |
| N8 | Pre-merge duplicate-deletion publication can create a candidate without the merged corpus. | Resolved: remove, stage, merge, then publish once only when the revision changed. |
| N9 | The active-subset rule discards active-only evidence and active-side deletions. | Accepted trade-off: the active duplicate is not authoritative; evidence is output and deletions there are not copied back. Recorded in D1. |
| N10 | Atomic-write temp files block a resume after a crash. | Resolved: reserved `*.givn-tmp` names are removed at archive startup before the witness. |
| N11 | Chapter 06 table omits rows. | Resolved: rows added. |
| N12 | The use-case delta drops the generic both-directory extension. | Resolved: extension added for the no-receipt pair. |
| N13 | The no-receipt scenario does not pin its message. | Resolved: it asserts "corrupt and both directories exist". |
| N14 | Glossary lacks merge-progress terms. | Resolved: "Merge progress" term added. |
| N15 | Chapter 08 is narrower than D1. | Resolved: evidence on both sides is named. |
| N16 | ADR canonical destination names two artifacts. | Resolved: exactly `design.md`. |
| N17 | The E2E-runner check is classified fresh-only. | Resolved: stage 3 says pending candidates re-check it. |
| N18 | Candidate messages do not name a deletion-only publication. | Resolved: messages say "candidate state"; the failure table covers the deletion-only revision. |
| N19 | Corrupt both-directory state is not surfaced by `givn status`. | Accepted boundary: `givn archive` reports the specific conflict; status lists resumable candidates. The proposal asks for a precise refusal at archive time, which this provides. |
| N20 | One SHAPE advisory between the retry and finalization scenarios. | Accepted: the shared shape is a fixture scaffold; the scenarios assert different failure stages (move versus finalization publication). Kept separate for distinct RED evidence. |
| N21 | `instructions.rs` PendingWithActive handling is unreachable. | Resolved: instructions.rs is removed from the architecture-impact table; the zero-active path is covered by `state::list_unproven_archives`. |
| N22 | `copy_dir_all` becomes dead code. | Resolved: the impact table names its removal. |
| N23 | "Archive candidate" wording reads as the archive location. | Resolved: reworded to merge "from that location into the permanent corpus". |
| N24 | Use-case diagram order reverses write/move. | Resolved: "Write pending receipt and move change". |
| N25 | Retirement test unit is named under `merge.rs`. | Resolved: named `archive::tests::retirement_retry_is_idempotent_and_cleans_up`. |
| N26 | The use-case rule lacks the evidence exception. | Resolved: "outside the resolved evidence paths". |

## Branch dispositions

| Branch | Disposition |
|---|---|
| Scope | The proposal's states, recovery paths, evidence paths, and roll-forward repair are covered; interrupted merge and finalization rejection were added because they are required by the minimal guarantee. |
| Tech choices | Atomic rename, receipt-driven progress, narrowed witness, and committed-receipt finalization are the simplest designs that satisfy the scenarios. |
| Missing scenarios | None after the added interrupted-move, interrupted-merge, in-flight retry, no-receipt, blocked-move, evidence, finalization-rejection, and status scenarios. |
| Testability | Every scenario has a RED table row, a stub pattern, a single-scenario command, and either a real fixture or a unit test. |
| E2E fidelity | No new `@e2e`; the existing CLI subprocess driver remains the real interface. |
| Interaction coverage | 9/9 inventory rows match the design matrix; the delta adds no action. |
| Risk | The dominant risk is the recovery witness; it is narrowed rather than skipped, and the unit/scenario coverage targets it. |
| Use-case and Persona context | Main flow, extensions, rules, and glossary align; Persona dispositions are recorded. |
| Decision ledger | D1, D2, D3 resolve; D4-D7 are added with realization sites. |
| Visual design contract | `Interface: none` with the rendered-screen rationale. |
| Deferrals | None. |
| Persona disposition | Three existing personas recorded; no new Persona. |
| Interface classification | CLI; the opposite browser classification does not hold. |
| ADR qualification | `NOT_QUALIFIED`; ADR-070/072 checked; canonical destination `design.md`. |
| Architecture documentation | Independent 12-row check matches `arc42.md`; all 12 chapters exist and contain content; no ASCII art. |
| Ubiquitous language | "Interrupted archive transition", "Archive candidate", and "Merge progress" are recorded and used consistently. |

## Residual boundaries

- An active duplicate is not authoritative. Evidence created only in the
  duplicate is discarded when the duplicate is removed; evidence is output, not
  a contract input.
- `givn status` lists resumable candidates, not corrupt both-directory states.
  `givn archive` reports the specific conflict.
- The user may still choose to delete one side of a conflicting pair; that is
  the explicit conflict path, not the normal recovery path.

## Status

DESIGN-REVIEW: PASS
