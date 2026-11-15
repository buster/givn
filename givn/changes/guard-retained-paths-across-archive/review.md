# Review: guard-retained-paths-across-archive

Review kind: independent
Author: guard-retained-paths-across-archive hardening author (omp, deepseek-flash)
Reviewer context: independent grilling session whose decisions Q1–Q5 the operator recorded on 2026-11-15; this authoring context applied them and owns the artifact edits

## Scope reviewed

The full change range `23ba7139..HEAD` (the prompt's `d160e7e..HEAD` starts at
the design/instruction commit, which is itself part of this change): the
design, design-review, arc42 assessment, tasks, both delta features, the delta
use case, the guard in `src/cli/archive.rs`, the matcher move to
`src/e2e_boundary/paths.rs`, the three step files, the served `ise` and
`archive` instructions, the arc42 chapters 5/6/8/11, `docs/reference.md`,
`docs/glossary.md`, and both committed transcripts. All six batch commits were
read.

## Implementation Evidence Check

**Step definition scan** — every file touched by this change:

```
$ grep -nE "fn [A-Za-z0-9_]+\([^)]*\) *\{\s*\}" \
    tests/steps/require_durable_retained_paths_steps.rs \
    tests/steps/require_durable_retained_paths_e2e_steps.rs \
    tests/steps/instruction_integrity_steps.rs
(no matches)
$ grep -nE "todo!|unimplemented!|^\s*pass\s*$|^\s*return;\s*$" \
    tests/steps/require_durable_retained_paths_steps.rs \
    tests/steps/require_durable_retained_paths_e2e_steps.rs
(none)
0 empty step bodies found across 3 files.
```

The refusal steps assert exit status, stderr content, the change directory,
the pending-receipt absence, and the candidate absence. The guidance steps
collapse whitespace and assert the served text. The `@e2e` step asserts the
moved change and the archived file from the real subprocess output.

**Commit cross-check**

| Scenario | Batch commit | Touches production code or its regression test? |
|---|---|---|
| Archive is blocked when a declared retained path moves with the change (2 rows) | `710e0a7c` | Yes — `src/cli/archive.rs` guard, `src/e2e_boundary/*` matcher move, `src/ise/partition.rs`, step file |
| The refusal reports at most twenty isolated retained paths | `710e0a7c` | Yes — same guard and bounded report |
| Archive is blocked when the declaration cannot be loaded | `710e0a7c` | Yes — same guard's loader branch |
| A retained root that also covers the archive location is unaffected | `710e0a7c` | Yes — survival predicate and its probe |
| A project with ise off archives without reading the declaration | `710e0a7c` | Yes — `is_effective` early return and its probe |
| The ise instruction says declared retained paths survive archiving | `a641ed61` | Yes — new step bindings; served text landed in `d160e7e` and is discriminated by the bound scenario |
| Consumer archives a change whose declared retained paths survive the move | `d8fbea44` | Yes — `@e2e` step and transcript rewrite |

All commits carry `Batch: <name>` and the scenario titles. The unit-test task
is in the same `710e0a7c` batch; `cargo test --lib -- cli::archive::tests`
reports 26 passed on the unmodified tree.

**Promised-architecture cross-check** — design.md declares no new component or
contract; it names six helper primitives plus `MAX_REPORTED_RETAINED_PATHS`.
All exist in `src/cli/archive.rs`:
`ensure_declared_paths_survive_archive`, `declared_retained_path_offenders`,
`partition_content_paths`, `archived_counterpart`, `reported_moved_path`,
`archive_advice`, `retained_path_refusal`, and the 20-line constant. The moved
matcher is present in `src/e2e_boundary/paths.rs` and re-exported from
`src/e2e_boundary/mod.rs`. `ise::partition` is the only other consumer and
imports the shared function.

**Strict-mode proof** — tasks.md `## Setup` records the expected fail-closed
behaviour: an unbound scratch step exits non-zero (`Step doesn't match any
function`), the scratch feature is removed, and the `@wip`-excluded delta run
reports 0 scenarios at exit 0.

**Downgraded e2e scan**

| `@e2e` Scenario | Real-interface assertion | Driven through the real interface? | Downgraded? |
|---|---|---|---|
| Consumer archives a change whose declared retained paths survive the move | Exit status 0, the change moved to `givn/archive/`, `design.md` present in the archived change | `CARGO_BIN_EXE_givn archive --change durable-paths` through `tests/common` (subprocess CLI) | No |

The capability is a CLI, not a browser UI; the opposite classification (Web UI)
does not hold because the change emits terminal text and an exit code and
renders no page.

**E2E scope**

| Capability | `@e2e` scenarios | Distinct actions | Over-produced? |
|---|---|---|---|
| `require-durable-retained-paths` | 1 | 1 (archive a change whose declared retained paths survive the move) | No — the refusal rows, bounded report, malformed declaration, broad root, and `ise` off are variants and stay regular |

**Duplicate-implementation check**

`givn/commands.yaml` names `verify.e2e_command: ./run-tests.sh --e2e`; the
script runs `cargo nextest … --test features_runner` with
`GIVN_FEATURES=${GIVN_FEATURES:-givn/specs}` and the tag expression
`@e2e and not @givn.removed`. The only binding of
`the archived change contains "<name>"` is
`tests/steps/require_durable_retained_paths_e2e_steps.rs`; the tree (including
untracked paths) contains no second implementation of this capability. The
weaker HTTP/fetch substitution does not exist here.

**Local environment check** — the scenarios run the compiled binary against
temporary Git fixtures; no service, port, database, or digital twin is
involved. The delta run (7 scenarios / 51 steps) completed in 3.4 s on this
worktree; the tasks record the full `./run-tests.sh --non-e2e` (1054 passed,
113 skipped) and `./run-tests.sh --e2e` (113 passed, 1054 skipped), both exit 0.

**E2E command and scope selection**

| `verify.command` | `verify.e2e_command` | Explicit scope selection | Regular evidence | E2e evidence | Zero failed/skipped? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | `run-tests.sh` exports `GIVN_RESULT_SCOPE` (`e2e` for `--e2e`, `regular` otherwise) and the runner writes it to `GIVN_RESULT_FILE` | change receipt: 7 total / 7 passed / 0 failed / 0 skipped | tasks record: 113 passed / 1054 skipped in the e2e scope | Yes for the change scope; the e2e scope records its skips as the regular-tagged complement |

**Design.md conformance**

| Decision (design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Guard call site | `run()`, fresh block, after `ensure_change_receipt`, before `ensure_artifact_gates` | Exactly there (`src/cli/archive.rs:138`) | Yes |
| Survival predicate | One predicate over `Backend::content_paths`; entry survives via a surviving content path or an archived counterpart | `declared_retained_path_offenders` + `partition_content_paths` + `archived_counterpart` | Yes |
| Advice shape | Leading `givn/changes` segments replaced by `givn/archive`; `None` when the entry reaches the tree through another prefix | `archive_advice` | Yes |
| Bound | `MAX_REPORTED_RETAINED_PATHS = 20` plus `+N more` | `src/cli/archive.rs:1403` and `retained_path_refusal` | Yes |
| Step files | `require_durable_retained_paths_steps.rs` and `..._e2e_steps.rs`; `instruction_integrity_steps.rs` extended in place | All three present and registered in `tests/steps/mod.rs` | Yes |
| Runner | `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` from `givn/commands.yaml` | Unchanged, read literally | Yes |
| Matcher | Move `glob_matches_path_or_parent` to `e2e_boundary::paths`, re-export, migrate consumers | Body identical; re-export present; `ise::partition` imports it; no second definition | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row | `@e2e` scenario | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| `require-durable-retained-paths`: archive a change whose declared retained paths survive the move | 1 row in design.md | Consumer archives a change whose declared retained paths survive the move | CLI subprocess | `cargo`/`CARGO_BIN_EXE_givn archive --change durable-paths` via `tests/common` | Yes |

**Discrimination probes (fabrication audit).** Each passing scenario was
re-broken and the failure observed on the unmodified tree first (7 delta
scenarios / 51 steps green). No scenario stayed green under a mutation aimed at
its own rule:

| Scenario | Mutation applied (restored) | Observed |
|---|---|---|
| Archive is blocked … (both rows) | Removed the guard call | Both rows failed `Then the command exits non-zero` (`expected failure, got exit Some(0)`) |
| Archive is blocked … (both rows) | Moved the guard after `Receipt::write_pending_progress` | Both rows failed `the change directory has no pending archive receipt`; bounded and malformed stayed green because they do not assert that placement |
| Archive is blocked … wildcard row | Replaced the shape-preserving advice with the narrower archived literal path | Wildcard row failed `stderr names "givn/archive/**"`; literal row stayed green (a literal is legitimately answered with its archived literal) |
| The refusal reports at most twenty … | Widened `MAX_REPORTED_RETAINED_PATHS` to 25 | Failed: `stderr lists 21 declared retained paths, not 20` |
| Archive is blocked when the declaration cannot be loaded | Turned the loader `Err` into `Ok(())` | Failed: `Then the command exits non-zero` (`expected failure, got exit Some(0)`) |
| A retained root that also covers the archive location | Forced `let survives = false` | Failed `Then the command exits 0` with the refusal text |
| A project with ise off … | Removed the `is_effective` early return | Failed `Then the command exits 0` with the refusal text |
| The ise instruction … | Deleted the retained-path survival sentence from `assets/instructions/ise.md` | Failed on the served-text assertion; separately, renaming a rebuild-obligation label failed the permanent five-obligation scenario |
| Consumer archives … (`@e2e`) | Added an existence requirement (an entry matching no content path is an offender) | Failed `Then the command exits 0` (`expected exit 0, got Some(1)`) |
| Predicate unit tests | Dropped the archived-counterpart branch from the survival chain | `an_entry_matching_only_the_archived_counterpart_survives` failed; the other 25 passed |

Ignored-file probe: `git ls-files --others --exclude-standard` in a throwaway
repository lists the untracked `.gitignore` but not `secret.ignored`, so
`Backend::content_paths` omits an ignored file; Fossil's `extras` honours
`ignore-glob` the same way. The guard therefore never refuses for a moved file
the trial cannot read, matching design Q1.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `specs/archive-change/usecase.md` is the permanent narrative plus the retained-path precondition, two step-1 extensions, four rules, four examples, one capability, and one interaction row; the permanent main flow and success guarantee are unchanged | Yes |
| Capability ownership and typed relationships preserved | New leaf `require-durable-retained-paths` under `archive-change`; `Includes: fragment: support`; `Extends: none`; step-1 extensions add no success outcome | Yes |
| Confirmed Persona-relevant outcomes preserved | Developer reads the refusal and the archived path; agent sees a pre-mutation failure instead of an unseedable archive; maintainer owns the archive success guarantee | Yes |
| Actors remain distinct from Persona biographies | Developer is the actor; the three Personas are review lenses only and appear in no Gherkin actor position | Yes |
| Migration ledger and coverage evidence complete, if applicable | No migration; the archive receipt will carry coverage | Yes |

Use-case scope rejection criteria were re-checked against the delta:
one verb phrase in `## Goal`; Developer named by `## Trigger`; no component- or
channel-shaped name; all extensions name step 1; preconditions are established
by state, not by the flow; the new capability protects the archive success
guarantee; `## Main flow` remains one actor goal.

## Deferred mandatory checks

None. No `@e2e` tag was removed, no check was deferred, and no `@wip` remains.

## Visual Review

Contract: `Interface: terminal` (design.md `## Visual Design Contract`),
no viewports, states = refusal and success. `parse_visual_contract` reads the
interface value as `Terminal`.

| `@e2e` Scenario | Evidence file | Present and non-empty? |
|---|---|---|
| Consumer archives a change whose declared retained paths survive the move | `evidence/visual/consumer-archives-a-change-whose-declared-retained-paths-survive-the-move/transcript.txt` | Yes (real executed run, exit 0, archived `design.md`, 116 bytes) |

The second transcript
(`evidence/visual/archive-is-blocked-when-a-declared-retained-path-moves-with-the-change/transcript.txt`)
is not `@e2e` evidence; its own header records it as the pre-fix reproduction
(binary from `23ba7139`, archive exit 0, `ise replace` refusal). It records the
defect honestly and is not claimed to show the guard's fixed refusal.

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| `developer--primary-user` | See why the archive refused and which path to declare | The refusal names the role, entry, moved path, and archived advice | None |
| `agent--technical-role` | Fail before mutation instead of archiving into an unseedable state | Exit status, unchanged change tree, absent receipt, absent candidate | None |
| `givn-maintainer--owner` | Keep the archive success guarantee enforceable | Guard placement and predicate pinned by scenarios and probes | None |

VISUAL-REVIEW: PASS

## Retirements

N/A; this change retires no capability.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `cli::archive` gains one pre-mutation refusal and reuses the declaration loader and shared matcher | `docs/arc42/05-building-block-view.md` | Affected: extend the `cli::archive` row and the `e2e_boundary` matcher ownership | Architecture Impact and insertion point | Batch `retained-path-guard`, Scenario 1 constraints | `src/cli/archive.rs:138`, helpers at `:1403-1600`; `e2e_boundary::paths:146` | Yes |
| Fresh-archive stage 3 and the sequence add the guard between the receipt check and the artifact gates | `docs/arc42/06-runtime-view.md` §6.7 | Affected: stage 3 and the fresh-archive line | Overview flowchart, insertion point | Scenario 1 constraints | Code order confirmed; placement probe failed the scenario when the guard moved after the receipt write | Yes |
| Archive reads the declaration only under `ise`, refuses an isolated retained path, and shares the parent matcher | `docs/arc42/08-crosscutting-concepts.md` | Affected: declaration reader condition and refusal | Guard Algorithm, Decisions D1-D3/Q1-Q2 | Domain Constraints and scenarios | `is_effective` gate, content-path predicate, refusal text | Yes |
| R-149 content-path basis; R-150 accepted symlink hole | `docs/arc42/11-risks-and-technical-debt.md` | Affected: two risks added | Risks and Trade-offs, Q4 | Domain Constraints (content paths, no symlink following) | `content_paths` tracked+untracked non-ignored; no canonicalization | Yes |
| Two glossary terms | `docs/glossary.md`, chapter 12 pointer | Affected: "Retained path", "Content path" | Ubiquitous language section in design-review | Domain Constraints | `docs/glossary.md:148-149` | Yes |

- [x] Every chapter marked affected by either the review or `arc42.md` was
      opened and checked for stale, contradictory, or placeholder content.
- [x] Every relevant Arc42 fact is represented in `design.md` and mapped to
      the batches in `tasks.md`.
- [x] The completed implementation follows those facts; the evidence names the
      built files, commands, and probes.
- [x] No architecture change was discovered that the durable chapters miss.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language and structural model

"Retained path" and "Content path" are defined once in `docs/glossary.md` and
used with the same genus and contrast pair in the proposal, design, delta use
case, served instructions, and the two chapters. "Specification", "retained
role", "archive candidate", and "proven archive" keep their recorded meanings;
the refusal uses "declared retained path" and introduces no synonym. The
structural model (`docs/domain-model.md`) still describes the change/permanent
corpus and change/version-control relationships; this change adds no aggregate,
boundary, or replaceable unit, so the model is correctly unchanged.

## Coverage

No change-scoped coverage receipt exists for this change yet; the archive will
measure the committed candidate and publish the merged report. Classification
below is from inspection plus the executed scenarios and unit tests. Measured
population is `src/`; no branch instrumentation was removed.

| Location | Lines | Classification | Action |
|---|---|---|---|
| `ensure_declared_paths_survive_archive`, offender collection, partition, archived counterpart, reported moved path, refusal bound | covered by 7 unit tests and the delta scenarios | none | — |
| `archive_advice` `None` branch (entry reaching the change tree through another prefix) and the matching `the archived paths must still match '<entry>'` refusal form | uncovered by any test or scenario | bucket 2 — missing test coverage | **Finding 2**: add a unit test with an entry such as `**/moved-limits/**` and assert the `None` advice form; no scenario is required |

No dead code, no redundant unit test, and no fourth-bucket classification.

## Scenario execution evidence

Change-scoped receipt (`verification/tasks-verification.json`), recorded at
`da79603a`: scope `change`, 7 total, 7 passed, 0 failed, 0 skipped, runner exit
0. At review time the receipt is stale on the tested-content digest because the
unrelated untracked `givn/changes/redact-secret-shaped-values-in-history/`
changes the tested content. `givn check tasks --change
guard-retained-paths-across-archive` refuses to refresh it: the same unrelated
untracked directory makes the tree non-clean, and this review was instructed
not to touch it. The change's own production and spec content is unchanged
since the recorded run; the stale digest is not evidence about this change.
The archive receipt does not exist yet (change not archived).

**Receipt status:** change-scoped receipt passed; archive receipt not created.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A retained root that also covers the archive location is unaffected | A project with ise off archives without reading the declaration | boundary |
| Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured | A retained root that also covers the archive location is unaffected | boundary |
| Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured | A project with ise off archives without reading the declaration | boundary |
| Uppercase checkboxes satisfy the archive gate | A retained root that also covers the archive location is unaffected | boundary |
| Uppercase checkboxes satisfy the archive gate | A project with ise off archives without reading the declaration | boundary |
| Archive allowed when @wip only on a @givn.removed placeholder | A retained root that also covers the archive location is unaffected | boundary |
| Archive allowed when @wip only on a @givn.removed placeholder | A project with ise off archives without reading the declaration | boundary |

Each pair protects a different rule: broad-root survival versus addon gating,
versus the permanent e2e-runner, task-checkbox, and `@wip` rules. The net delta
is 7 added, 0 modified, 0 removed, so no removed+added pair needs a
supersession decision.

## Split-or-keep

The gate reported no long-scenario finding for this delta; no decision is
required.

## Findings

**BLOCKER 1 — the permanent `@e2e` step recreates a stale change directory
after archive, and the debris blocks every later archive.**

- `tests/steps/require_durable_retained_paths_e2e_steps.rs:36-38` writes the
  transcript to
  `CARGO_MANIFEST_DIR/givn/changes/guard-retained-paths-across-archive/evidence/visual/<slug>/transcript.txt`
  unconditionally, whatever `GIVN_FEATURES` names.
- Archive verification runs `verify.e2e_command` with
  `GIVN_FEATURES=givn/specs` (`src/cli/archive.rs:1766-1772`). The merged
  permanent scenario therefore runs the step after this change has moved to
  `givn/archive/`, and `fs::create_dir_all` recreates
  `givn/changes/guard-retained-paths-across-archive/evidence/...` as an
  untracked path.
- Probe: with the change directory moved aside and the feature run from
  `target/post-archive-probe`, the scenario passed and the step recreated
  `givn/changes/guard-retained-paths-across-archive/evidence/visual/consumer-archives-a-change-whose-declared-retained-paths-survive-the-move/transcript.txt`.
- Consequence: `is_evidence_path` exempts `givn/changes/<id>/evidence/` only
  for the change being archived (`src/proof.rs:1279-1285`). A later archive
  with a different id sees the untracked file, and `ensure_clean` refuses with
  `archive blocked: the project has uncommitted work` before mutation. Every
  e2e run recreates the file, so the repository cannot archive another change
  until the step is fixed or the file is removed after each run. The file is
  not gitignored (`.gitignore` has no `givn/changes/**/evidence` rule).
- The existing terminal-evidence steps already handle this: derive the change
  id from `GIVN_FEATURES` and fall back to `target/terminal-evidence/<slug>`
  outside a change run (`tests/steps/workflow_enforcement_e2e_steps.rs:20-40`,
  `tests/steps/trial_input_verification_e2e_steps.rs:57-75`). This step
  deviates from that convention.
- Required fix: make the evidence directory change-scoped only while the
  feature path is a delta, and write to a gitignored `target/` path otherwise;
  keep the committed delta transcript while the change is active.

**FINDING 2 (bucket 2 — missing test coverage).** `archive_advice` returns
`None` for an entry that reaches the change tree through a prefix other than
`givn/changes`, and `retained_path_refusal` then prints
`the archived paths must still match '<entry>'`. No unit test or scenario
exercises that branch. Add one unit test with an entry such as
`**/moved-limits/**` over a listing that contains only the moved content, and
assert the `None` advice text. The design's rule is implemented correctly; the
gap is the test.

## Sign-off

- [x] Fabrication audit: 0 empty step bodies; all seven delta scenarios fail
      when their own rule is broken; every batch commit contains production
      code or its regression test; no promised component is missing; no
      downgraded `@e2e` scenario; no HTTP/fetch substitute; no over-produced
      e2e scope.
- [ ] Scenario execution evidence recorded per scope — the change-scoped
      receipt passed at `da79603a` and the tasks record the e2e scope, but the
      receipt is currently stale from unrelated untracked content and cannot be
      refreshed while that directory exists.
- [ ] Every coverage gap is classified and resolved — Finding 2 is bucket 2 and
      is open.
- [x] No `@wip` tags remain; the delta holds seven scenario declarations.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the fixture stack cleanly; no digital twins.
- [x] The exact file(s) `verify.e2e_command` invokes were identified; the tree
      including untracked files has no parallel implementation.
- [x] The e2e execution selects its scope explicitly through
      `GIVN_RESULT_SCOPE`; the change receipt reports zero failures and skips.
- [x] Implementation matches design.md's commands, file layout, and driver; the
      one deviation (the post-archive evidence path) is Finding 1, not a silent
      design change.
- [x] Interaction coverage verified: one inventory entry, one matrix row, one
      `@e2e` scenario, CLI subprocess driving.
- [x] Design.md's Internal Primitives lists the guard and every helper the step
      definitions drive; private carrier structs are implementation details of
      the listed guard.
- [x] arc42 slug mapping verified against the real chapter files.
- [ ] No finding is excused outside the three coverage buckets; Finding 1 is a
      real defect and remains open.

README-IMPACT: none

The README's Description, Quick start, Prerequisites, Usage, Configuration,
Contributing, and License sections do not document the archive gate list; the
behavior is documented in `docs/reference.md` and the served `archive`/`ise`
instructions.

PERSONAS: developer--primary-user

| Persona | Relevance | Evidence | Disposition |
|---|---|---|---|
| `developer--primary-user` | Reads the refusal and the archived path to declare | Refusal scenarios assert role, entry, moved path, archived advice | Dispositioned, no finding |
| `agent--technical-role` | Runs archive in loops and must see the pre-mutation refusal | Exit status, unchanged change tree, absent receipt and candidate | Dispositioned, no finding |
| `givn-maintainer--owner` | Owns the archive success guarantee and the guard's effectiveness | Use-case rules, archive gate list, delta feature, probes | Dispositioned, no finding |

REVIEW: FAIL
