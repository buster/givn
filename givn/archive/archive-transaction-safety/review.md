# Review: archive-transaction-safety

## Fabrication audit

- **@e2e tag integrity:** no delta scenario carries `@e2e`; no permanent
  `@e2e` tag was removed or changed. The change adds no E2E action.
- **Empty step bodies:** 0 empty or no-op bodies across the three archive-proof
  step files (`archive_proof_steps.rs`, `archive_proof_e2e_steps.rs`,
  `archive_proof_support.rs`). Every new body contains a fixture action or an
  assertion.
- **Checked tasks and revisions:** all 11 tasks carry a unique revision.
  `git show --stat` confirms each revision touches production source:
  `src/proof.rs`, `src/gherkin/merge.rs`, `src/cli/archive.rs`, or
  `src/state.rs`. The shared implementation revision
  `a696477a5674f5f6eb0210d527f942ce17a032e2` contains the production change;
  the per-scenario revisions add the scenario-specific tests and invariant
  documentation.
- **Promised components:** `proof::read_status`/`candidate_matches_active`,
  the merge-progress receipt fields, `merge_file_retry`,
  `Backend::revision_file` on Git and Fossil, the archive transition helpers,
  and the status filter all exist in the tree.
- **Strict-mode proof:** a temporary undefined-step feature failed with
  `Step doesn't match any function` and `0 passed, 1 failed`; the proof is
  recorded in `tasks.md`.
- **E2E downgrade check:** the change adds no `@e2e` scenario. The existing
  archive E2E steps invoke the compiled `givn` binary as a subprocess and
  assert its stdout/stderr and receipt files; they are not repository-only.
- **E2E command identity:** `verify.command` is `./run-tests.sh --non-e2e`;
  `verify.e2e_command` is `./run-tests.sh --e2e`. The observed scopes were
  695 regular tests and 104 E2E tests, so the E2E scope is strictly smaller.
- **Design conformance:** the implementation uses the step files, runner,
  commands, and transition helpers named in `design.md`; the only added
  pieces are the committed-receipt `revision_file` seam and the narrowed
  witness, both named there.
- **Interaction coverage:**

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism in the step file |
|---|---|---|---|
| archive a completed change | present | Consumer archives a completed change and receives a proven archive | `common::run_givn` subprocess |
| repair a blocked archive | present | Consumer repairs a blocked archive and the rerun proves it | `common::run_givn` subprocess |
| inspect an unproven archive candidate | present | Consumer inspects an unproven archive candidate | `common::run_givn` running `status` |
| archive a new capability | present | Consumer archives a new capability and the merged spec lands in its declared use case | `common::run_givn` subprocess |
| archive without routing | present | Archive is blocked when a new capability has no declared routing decision | `common::run_givn` subprocess |
| archive a multi-file change | present | Nothing is merged when one delta file of a change has no routing decision | `common::run_givn` subprocess |
| archive a modified scenario | present | Consumer archives a modified scenario and the permanent spec keeps the docstring | `common::run_givn` subprocess |
| archive with coverage | present | Consumer archives a change with coverage enabled and the coverage summary appears in README | `common::run_givn` subprocess |
| archive an E2E delta | present | Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner | `common::run_givn` subprocess |

No inventory entry is missing, extra, or mapped to a repository-only
assertion.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Same-filesystem transition constraint | `02-architecture-constraints.md` TC-38 | Yes, row 2 | D2, failure table | Task 6 | `move_change_to_archive` refuses by name; no copy fallback | Yes |
| Atomic rename and reconciliation strategy | `04-solution-strategy.md` archive row | Yes, row 4 | D1/D2 | Tasks 3, 6 | `read_status`, `PendingWithActive`, `move_change_to_archive` | Yes |
| Archive/proof building blocks | `05-building-block-view.md` rows | Yes, row 5 | Impact table | Tasks 2-9 | `proof.rs`, `cli/archive.rs`, `state.rs` | Yes |
| Runtime transition and recovery stages | `06-runtime-view.md` 6.7 | Yes, row 6 | Transition section | Tasks 3-9 | `run`, `apply_merge_plan`, `recover_proven_finalization` | Yes |
| Provenance and evidence predicates | `08-crosscutting-concepts.md` | Yes, row 8 | D1, D6 | Tasks 2, 7 | `candidate_matches_active`, `is_evidence_path` | Yes |
| Interrupted-transition quality scenario | `10-quality-requirements.md` Q-104 | Yes, row 10 | Failure table | Tasks 3-9 | Scenario suite and unit tests | Yes |
| Rename and merge-progress risks | `11-risks-and-technical-debt.md` R-99, R-100 | Yes, row 11 | D4/D5, failure table | Tasks 4-6 | Per-file progress and atomic writes | Yes |
| Interrupted transition, archive candidate, merge progress | `12-glossary.md` | Yes, row 12 | D1/D4 | Tasks 2-5 | Terms used in code and specs | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The spec, design, and code use the glossary terms consistently: "active
change", "archive candidate", "pending archive candidate", "proven archive",
"interrupted archive transition", "merge progress", "evidence path", "tested
revision", and "forward repair". The new "Merge progress" term records
`merge_completed`, `merged_deltas`, and `merge_in_progress`. No competing
synonym appears in the change artifacts or the implementation.

## Coverage classification

- **Dead code (bucket 1):** the old `copy_dir_all` helper is removed. No other
  dead code was introduced.
- **Missing coverage (bucket 2):** none. The nine delta scenarios cover the
  no-receipt, conflict, interrupted move, interrupted merge, in-flight retry,
  blocked move, evidence, finalization rejection, and status paths. The
  non-scenario algorithms have unit tests for the comparison truth table,
  evidence prefixes, receipt progress, atomic receipt write, merge retry
  semantics, and retirement idempotence.
- **Hard to test (bucket 3):** none deferred. The finalization crash window is
  covered through the rejected-hook scenario plus the committed-receipt check;
  the remaining crash-timing windows are addressed by atomic receipt and spec
  writes and reserved temporary cleanup.
- **Redundant unit tests:** none. Unit tests cover internal algorithms that the
  Gherkin scenarios cannot observe directly; no scenario is duplicated by a
  module-level test of the same black-box behavior.

No coverage evidence has been measured for this change yet; the archive
verification receipt will carry the measured result. The classification above
is from inspection.

## E2E coverage

The inventory has nine entries, the design matrix has nine rows, and each row
has a matching existing `@e2e` scenario driven through the compiled CLI. The
change adds none. The opposite classification — a rendered browser page — does
not hold: the archive surface is a terminal command with text output and no
HTML, CSS, or JavaScript.

The observed counts are 695 regular tests and 104 E2E tests, so the E2E scope
is strictly smaller and the tag filter is real.

## Deferrals

None.

## Use-case and Persona conformance

The owning use case keeps one goal, the Developer actor, its preconditions,
main flow, extensions, rules, guarantees, and capability ownership. The delta
updates the main flow to move-then-merge, adds the interrupted-move and
interrupted-merge extensions, and keeps the no-receipt and conflict refusal
rules. No success outcome was added outside the flow.

Scope rejection criteria: the goal is a single verb phrase; the trigger names
the Developer; every capability passes the placement test; the guarantees
observe archive proof rather than a mutation; every extension names its flow
step; the preconditions are established by the flow; no guarantee is left
open; the goal names no mechanism; the capability name is goal-shaped; the
leaf names one actor goal; and no fragment/extends relationship is misused.

| Persona | Disposition | Evidence |
|---|---|---|
| developer--primary-user | Relevant: runs archive and repairs a blocked candidate | Conflict and retry scenarios; CLI messages |
| agent--technical-role | Relevant: invokes archive and follows the printed resume route | `status` and `instructions` routing |
| givn-maintainer--owner | Relevant: owns the receipt contract and state invariant | Receipt progress fields and unit tests |

No new Persona is introduced. The proposal's "confirmed Personas: none" means
no new Persona was confirmed for this change; the owning use case's existing
Personas remain the review lenses.

## Visual review

Not applicable. `design.md` declares `## Visual Design Contract` with
`Interface: none`; the change alters CLI state messages and file transitions,
adds no rendered screen, and adds no delta `@e2e` scenario. No screenshot or
transcript evidence is required.

## Deterministic gate dispositions

### Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A failed archive move keeps the change retryable | A rejected finalization publication leaves the archive unproven | boundary |

The pair shares a retry-shaped fixture but asserts different failure stages:
the first stops before the move and retries the rename; the second publishes
the candidate and rejects only the evidence-only finalization. They are not
duplicates or variants of one behavior.

### Split-or-keep

| Scenario | Decision |
|---|---|

No scenario exceeds the long-scenario threshold, so no split-or-keep row is
required.

## README impact decision

README-IMPACT: none

The change adds no command, flag, configuration key, or public output contract;
it strengthens archive recovery behind the existing interface.

## Sign-off checklist

- [x] Fabrication audit clean; every finding resolved.
- [x] Every checked task has a unique verified revision touching production
      source.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence is recorded per scope; the archive receipt
      will carry the measured result.
- [x] Coverage classified from inspection; no unresolved gap.
- [x] Dead code deleted; missing tests added; no hard-to-test gap deferred.
- [x] No redundant unit tests.
- [x] No `@wip` tags; no implementation-layer detail in the spec.
- [x] Canonical E2E policy applied; nine inventory actions map to nine existing
      E2E scenarios; no delta E2E scenario was added.
- [x] Every existing E2E scenario asserts the real CLI interface.
- [x] Local run command is `cargo run -- <command>`; no external services or
      digital twins.
- [x] `verify.e2e_command` invokes `./run-tests.sh --e2e` and is not identical
      to `verify.command`; the observed E2E count is strictly smaller.
- [x] Implementation matches the commands, file layout, and framework named in
      `design.md`.
- [x] Interaction coverage verified end to end.
- [x] No finding was excused outside the three coverage buckets.

REVIEW: PASS
