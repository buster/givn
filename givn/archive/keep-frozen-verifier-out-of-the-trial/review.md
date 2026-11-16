# Review: keep-frozen-verifier-out-of-the-trial

Review kind: independent
Author: keep-frozen-verifier-out-of-the-trial author (omp)
Reviewer context: fd3e0fd7-9ae1-4767-b2b2-dc6ddd46984f (the review context that produced the grill brief)

## Reviewed inputs

Read in full: `proposal.md`; `design.md`; `design-review.md`; `arc42.md`;
`tasks.md`; the three delta features and the delta
`specs/prepare-an-ise-test/usecase.md` with its base digest; the permanent
`givn/specs/prepare-an-ise-test/clean-room-preparation.feature`,
`trial-input-verification.feature`, and
`givn/specs/fragments/adr-qualification-gate.feature`; the permanent use case;
`config-lock.yaml`; `docs/reference.md`; `docs/glossary.md`;
`docs/arc42/adr/ADR-078-frozen-package-outside-the-trial.md`; the affected
arc42 chapters 02, 03, 05, 06, 08, 09, 11; `src/ise/{partition,mod,record,verify}.rs`;
`src/cli/ise.rs`; `src/e2e_boundary/mod.rs`; the four step files; the tasks
receipt; and the committed transcript.

Probes ran at HEAD `a2f3bfb3` with the built `givn` and the delta and permanent
feature scopes. Every production mutation was restored with `git checkout --`
or the inverse edit. Post-probe `git status` shows only the untracked
`verification/` receipt, and the committed transcript is byte-identical to its
commit.

## Implementation evidence check (Step 0)

### 0. `@e2e` tag integrity

No `@e2e` tag was removed. The only tag diff in the change is
`-  @givn.modified @e2e @wip` / `+  @givn.modified @e2e` on the delta
preparation scenario. The permanent preparation scenario keeps
`@e2e @clean-room-preparation`; the permanent verification `@e2e` scenario is
unchanged.

### 1. Empty step bodies

0 empty step bodies found across the four changed step files. The grep for
`{}`, bare `pass`, bare `return`, and whitespace-only bodies found only fixture
source strings such as `"pub fn parse() {}\n"`. Every new binding body drives a
fixture and asserts a report line, a receipt value, a trial path, stderr, or an
exit code.

### 2. Checked scenarios and their commits

| Batch | Scenarios | Commit | Evidence in the commit |
|---|---|---|---|
| `frozen-package-exclusion` | 4 | `6df78e6d` | `partition`, `mod`, `record`, `cli::ise` production changes; preparation bindings; fixtures |
| `frozen-trial-refusal` | 5 | `7759bf5f` | `verify`, `e2e_boundary` production changes; verification bindings; fixtures |
| `pending-register-reconciliation` | 1 | `84651394` | the 30-binding and register-summary assertion; re-issued after `500d78df` lacked the batch label |
| `prepare-trial-frozen-package-e2e` | 1 (`@e2e`) | `1bd97ce0` | e2e steps, fixture counts, reconciled feature body, transcript |

`givn check review` reports `✓ scenario commits passed (11 scenario(s))`. Every
commit contains the scenario's implementation or regression test. The shared
batch revisions name `Batch: <name>` in `tasks.md`. No commit is comment-only,
tag-only, task-only, feature-only, or transcript-only.

### 3. Promised components

Every component `design.md` names exists: `Partition::frozen` and
`frozen_paths()` in `src/ise/partition.rs`; `Report::excluded_roles` and
`frozen_files`, the record field construction, and `copy_set` in
`src/ise/mod.rs`; `TrialRecord::excluded_roles` in `src/ise/record.rs`;
`require_frozen_absent` and `trial_paths` in `src/ise/verify.rs`; the
conditional line in `src/cli/ise.rs`; `E2eBoundary::contains` in
`src/e2e_boundary/mod.rs`. No consumer imports an implementation: `crate::ise`
calls `e2e_file_set` and `E2eBoundary::contains` through the module surface,
and the private `is_under` helper is not re-exported.

### 4. Strict-mode proof

`tests/features_runner.rs` calls `.fail_on_skipped()` on both builders
(lines 56 and 107). `tasks.md` §Setup records the pre-binding probe: the delta
scenario failed with `Step doesn't match any function` and exit 101. The task
record also shows `givn check tasks` refusing the change while any selected
delta scenario carried `@wip`.

### 5. `@e2e` Then assertions

The one delta `@e2e` scenario asserts stdout lines, the trial tree, and the
receipt's keys and counts. The verification `@e2e` scenario is unchanged and
asserts the clean report and exit code. No Then asserts only against a
repository, and no scenario calls `crate::ise` in process. The step defs spawn
the compiled binary through `tests/common::run_givn`.

### 6. Browser-UI check

Not applicable. The product is a CLI; the changed surfaces are subprocess
stdout/stderr, a trial directory, and a YAML record. No HTML, CSS, or
JavaScript is rendered, and no browser driver is claimed. The opposite
classification (Web UI) does not hold.

### 7. E2E command file and parallel implementations

`givn/commands.yaml` sets `verify.e2e_command: "./run-tests.sh --e2e"`, which
runs `tests/features_runner.rs` with the `@e2e` tag filter. Searching the tree,
including untracked files, finds one implementation per capability:
`tests/steps/clean_room_preparation_e2e_steps.rs` and
`tests/steps/trial_input_verification_e2e_steps.rs`. No weaker or duplicate
implementation exists elsewhere.

### 8. E2E scope

`prepare-an-ise-test` has one normalized prepare action and one verify action.
The delta modifies the existing prepare `@e2e` scenario and adds no second
`@e2e` scenario. Every new prepare and verify case is a regular variant of the
same action. The `adr-qualification-gate` delta modifies one permanent scenario
and adds no action.

### 9. Scope selection and receipt

The regular and E2E scopes use distinct commands (`./run-tests.sh --non-e2e`
and `./run-tests.sh --e2e`); `run-tests.sh` supplies `GIVN_RESULT_SCOPE`. The
change receipt records `scope: "change"`, `total: 10`, `failed: 0`,
`skipped: 0`, `runner_exit: 0`, command `./run-tests.sh --non-e2e`, at HEAD
`a2f3bfb3`. The `@e2e` delta scenario is outside the non-e2e scope; the tasks
closeout records the full e2e run (114 passed) and the delta e2e run (1 passed),
and this review ran all 11 delta scenarios in one process (73 steps passed).
No scope is silently shared or counted twice.

### 10. Design match

The as-built code matches the design: one frozen set from
`e2e_file_set(declaration, revision_paths)` minus `DECLARATION_PATH`; four
`!frozen` filters; `copy_paths = retained ∪ kept_source`; the record field with
`#[serde(default)]`; the `Frozen package` line between `E2E digest` and
`Written path`; `require_frozen_absent` after `require_record_belongs` and
before `check_retained`; the declaration load moved into `verify`; the
twenty-path limit. The design names `src/ise/mod.rs` as passing the frozen
paths to `require_copy_set`; the implementation unions the copy set with the
frozen set in `copy_set`, which is the same rule. No dependency, config key,
record scope, or command changed.

### 11. Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `clean-room-preparation`: prepare a trial from a selected revision | present | Developer prepares a clean-room trial from a recorded revision (modified) | Real `givn ise replace` subprocess; stdout, trial tree, receipt |
| `trial-input-verification`: re-check a prepared trial's frozen inputs | present | Verifier re-checks a prepared trial and receives the frozen-input report (unchanged) | Real `givn ise verify` subprocess; stdout/stderr and exit code |

Both inventory entries have a matrix row. No entry is added, removed, or
re-scoped; the new regular scenarios are variants of the same actions.

## Attack points and probes

| Probe | Mutation | Observed result |
|---|---|---|
| P1 | `copy_paths()` chained `&self.frozen` | The `@e2e` preparation scenario failed: `the trial at /tmp/.../trial holds tests/e2e/Cargo.toml` |
| P2 | Removed `retained_paths.retain(|path| !frozen_set.contains(path))` | The same scenario printed `Retained: 4` and failed `does not print the line "Retained: 2"` |
| P3 | Removed both refusal exemptions (`DECLARATION_PATH`, `retained_files`) | `A frozen role that covers the declaration re-checks clean` failed: stdout empty, no `Retained: clean` |
| P4 | Removed the `excluded_roles` early return from `require_frozen_absent` | The compatibility scenario stayed green: its planted package paths are keys of `retained_files`, so the exemption filters them before the frozen test. The refusal cannot fire for that record |
| P5 | Removed `#[serde(default)]` from `excluded_roles` | The compatibility scenario failed: the record without the field no longer deserializes, so no clean line is printed |
| P6 | Made the `Frozen package` line unconditional | `A declaration with no frozen package excludes no role` failed with `Frozen package: , not copied (0 files)` |
| P7 | Set `frozen_files` to `e2e_paths.len()` | `A frozen package root that covers the declaration keeps it in the trial` printed `(2 files)` and failed the `(1 file)` Then |
| P8 | Made `excluded_roles` unconditionally `["e2e"]` | The same empty case failed the no-line Then with `(0 files)` |
| P9 | Removed the `require_frozen_absent` call from `verify` | `Verification names an e2e package file changed in the trial` exited 0 and failed the failure/name Then |
| P10 | Widened `FROZEN_REPORT_LIMIT` to 25 | `Verification bounds the frozen paths it refuses` named `f19.rs` and failed the first-twenty Then; the refusal printed `25 path(s)` |
| P11 | Flipped the ADR-078 register row to `proposed` | Both the delta and the permanent ADR scenario failed with `unexpected accepted ADR count`; restoring `accepted` made both pass |
| P12 | Ran all delta scenarios with `--tags "@givn.delta"` | 3 features, 11 scenarios, 73 steps passed |
| P13 | Ran the permanent ADR scenario, then compared the three other modified bodies | Permanent ADR 11 steps passed; the preparation, dry-run, and two verification bodies match the delta bodies exactly |

P9 and P10 are the refusal-names-it and bounded-at-twenty checks. The refusal
message from the bound probe names the manifest and the first nineteen step
files, then `(and 5 more)`, with `25 path(s)` in the prefix. The path list is
sorted because `trial_paths` sorts before the filter.

The trial holds no path outside `retained ∪ kept_source`: `replace()` builds
`trial_files` from `copy_paths` and `write_trial` iterates only those files.
The frozen set is in neither vector, so no write path and no write-time
exception can add it. P1 confirms the trial-side assertion discriminates.

## Compatibility

An older record has no `excluded_roles`, so `#[serde(default)]` reads an empty
list and `require_frozen_absent` returns early. Its trial's package files are
retained entries and the retained comparison still verifies them. P5 shows the
scenario discriminates the default. P4 shows the refusal cannot fire for such
a record even when the early return is removed, because the record's
`retained_files` lists the planted paths and that exemption runs first. The
residual risk 2 (a deliberately deleted `excluded_roles` field) is recorded in
ADR-078 and remains a record property, not a trial property.

## The report line

The line prints only when `excluded_roles` is non-empty and counts
`partition.frozen.len()`, not `e2e_files.len()`. P6, P7, and P8 break each half
of that statement and the matching scenario fails. The `e2e: []` case prints
no line (P6, P8), and the declaration-covered package prints `(1 file)` while
`e2e_files` holds two identities (P7).

## The ADR register

`docs/arc42/adr/README.md` has 30 `| accepted |` rows, no proposed row, and the
ADR-078 row is accepted. The permanent scenario and
`tests/steps/adr_qualification_gate_steps.rs` both assert 30 and the ADR-078
accepted row, plus the register summary sentence. The delta scenario is green
(P12), the permanent scenario is green, and P11 makes both fail in both scopes.
ADR-078 records the decision, the alternatives, the compatibility consequence,
and the "Bad, because" residual. ADR-076 links ADR-078. Chapter 09 points to
the register, and chapter 02 carries TC-46, TC-47, and TC-51.

## Contract surface

`E2eBoundary::contains(&self, path: &str) -> bool` is the declared addition on
the `E2eBoundary` contract, implemented as `roots().any(|root| is_under(path,
root))`. `is_under` stays private in `src/e2e_boundary/mod.rs`; the copy in
`imports.rs` is a separate private helper. `crate::ise::verify` calls
`E2eBoundary::resolve` and `boundary.contains`, never `is_under`. No ad hoc
re-export exists.

## Evidence

The one `@e2e` preparation scenario commits
`evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`.
`write_transcript` in `clean_room_preparation_e2e_steps.rs` writes the real
`output(world)`: argv, fixture root, binary, revision, exit code, raw stdout,
and raw stderr. The committed transcript shows exit 0, `Retained: 2`, the
frozen line, the written path, and the receipt path. Running the scenario
regenerates the transcript with a new fixture revision; that run output was
restored, so the committed file is unchanged. No regular scenario commits a
transcript.

## The lost feedback

The delta use case states the builder cannot run the acceptance suite and
names the replacement: the behavior corpus's component scenarios are local
feedback. It states the contradiction path: the builder does not adjust the
suite; the difference is a `Spec gap` and the trial restarts after the
Specification or the suite is corrected. R-154 in
`docs/arc42/11-risks-and-technical-debt.md` states the same loss, the same
replacement, and the same contradiction path, and records the accepted
residual. The design's risk 1 matches.

## Coverage classification

| Changed behavior | Classification | Evidence |
|---|---|---|
| Frozen set resolution, declaration exemption, four filters | covered | P1, P2, P3, P7; partition unit test |
| Trial writes only `retained ∪ kept_source` | covered | P1; code inspection of `trial_files`/`write_trial` |
| Record `excluded_roles`, `serde(default)`, `ise-trial/v1` | covered | P5, P8; receipt parsing bindings |
| Report line conditional, count basis, singular/plural | covered | P6, P7, P8; transcript |
| Trial-side refusal, exemptions, bound, record gate | covered | P3, P4, P9, P10 |
| Older-record compatibility | covered | P5; P4 explains the retained exemption |
| ADR register count and binding | covered | P11; both scopes |
| Per-path drift on both retained and checkout sides | covered | Permanent modified scenario; green in P12; no production change |
| Coverage instrumentation for the new branches | not run | O2 |

## Fabrication audit

| Delta scenario | Check broken and observed failing? |
|---|---|
| Developer prepares a clean-room trial from a recorded revision (`@e2e`) | Yes, P1 (trial holds the package) and P2 (`Retained: 4`) |
| The trial record names the frozen role its trial omits | Yes, P8 (`left: []`, `right: ["e2e"]`) |
| A declaration with no frozen package excludes no role | Yes, P6 and P8 (line printed, `(0 files)`) |
| A frozen package root that covers the declaration keeps it in the trial | Yes, P7 (`(2 files)`) and P3 (no clean line without the exemption) |
| A dry run performs every read and writes no trial or record | The shared `copy_paths` mutation P1 is the code path its `Would write:` assertion reads; the scenario was green in P12. I did not rerun the dry-run scenario under P1 |
| Verification names an e2e package file changed in the trial | Yes, P9 (exit 0 instead of the refusal) |
| Verification names every drifting path in one run | Protected requirement, no historical RED. It stayed green under P9 and P10 because it does not depend on the refusal; no mutation of the drift comparison was attempted |
| Verification bounds the frozen paths it refuses | Yes, P10 (named `f19.rs`, no bound) |
| Verification keeps a trial prepared before the package was excluded re-checkable | Yes, P5 (missing `serde(default)`) |
| A frozen role that covers the declaration re-checks clean | Yes, P3 (no clean line) and P7 (count basis) |
| The migration preserves independent architecture history across three proposed decisions | Yes, P11 (both scopes failed) |

No delta scenario stayed green while its asserted behavior was broken. The
dry-run scenario and the per-path drift scenario are named above because I did
not mutate their own code paths; the first shares P1's production path, and the
second protects pre-existing behavior that the change does not modify.

## Arc42 conformance

| Chapter | Claim | Check |
|---|---|---|
| 02 | TC-46 partition, TC-47 refusal, TC-51 frozen set | text matches the code and spec |
| 03 | the two `givn ise` interface rows | text names the frozen line and the refusal |
| 05 | `ise`, `cli::ise`, `e2e_boundary` | text names the frozen placement and `contains` |
| 06 | replace, dry run, report, verify | text matches the verify order and the report line |
| 08 | §8.29 record facts | text names `excluded_roles`, the count basis, and the refusal |
| 09 | ADR-078 and the register | register row present and accepted; chapter points to the register |
| 11 | R-154 | risk, mitigation, and contradiction path present |
| 12 | glossary pointer | `Frozen package`, `Trial record`, `Trial directory` updated |
| 01, 04, 07, 10 | not affected | no new goal, strategy, deployment, or quality scenario; re-derived and unchanged |

All twelve chapter files exist and carry content. The box-drawing and
`+---`/`|---` scans returned no match. No placeholder remains in an affected
chapter. The glossary has `Frozen package`, `Trial record`, and `Trial
directory` entries that state the exclusion, the field, and the count basis.
`docs/domain-model.md` is correctly unchanged: the change moves a file-format
rule, not an aggregate or relationship.

ARC42 CONFORMANCE: CLEAN

## Use-case and Persona conformance

The delta use case keeps the stable ID, goal, actors, trigger, main flow,
extensions, rules, guarantees, capabilities, and interactions of
`prepare-an-ise-test`. It adds no capability and no typed relationship. The
scope rejection criteria pass: one outcome per goal and guarantee; the
Developer is the primary actor; both capabilities are placements, not
components; every extension names its main-flow step; Includes/Extends remain
none. No Persona is invented and no biography is used as a Gherkin actor.

PERSONAS: developer--primary-user

The confirmed Personas are `developer--primary-user`,
`agent--technical-role`, and `givn-maintainer--owner`, all from
`prepare-an-ise-test`'s `## Personas`; the design gives each a concrete impact.
The review records the first as the primary disposition; the other two are
confirmed in the same source.

VISUAL-REVIEW: PASS

The design declares `Interface: terminal` with no viewports. The terminal
evidence is the committed transcript, which was read and regenerated from the
real binary. No screenshot or viewport applies.

README-IMPACT: none

## Findings

### B1 (closed 2026-11-16) — the proposal routing table omitted `adr-qualification-gate`

Original failure: `givn check review` refused with

```text
capability routing decisions do not match the built delta:
delta capability 'adr-qualification-gate' is not declared in the Capability
Routing table — the proposal records no 'NEW in <usecase-id>' or
'EXTEND <capability>' decision for it
```

The delta has three capability tags: `clean-room-preparation`,
`trial-input-verification`, and `adr-qualification-gate`.
`proposal.md`'s `## Capability Routing` table declared the first two only. The
third delta is `specs/fragments/adr-qualification-gate.feature`, which modifies
the permanent ADR register scenario and its binding; `arc42.md` records it, the
design records it, and the tasks commit `84651394` implements it, but the
proposal never routed it.

Closure: commit `a7a0088d` (2026-11-16) adds the third row and refreshes the
section's recorded route score from 17.00 to 27.00. `givn spec route --change
keep-frozen-verifier-out-of-the-trial` now reports
`EXTEND givn-workflow-enforcement` (`verify-change`, score 27.00, signal). The
table lists all three capabilities; each row records that recommendation and a
reasoned deviation: `clean-room-preparation` and `trial-input-verification`
belong to `prepare-an-ise-test`, and `adr-qualification-gate` owns the register
that ADR-078 moves from 29 to 30 accepted decisions. All three rows therefore
deviate for the stated reason, and the prose says so.

Closure checks. `git show --name-only a7a0088d` lists only
`givn/changes/keep-frozen-verifier-out-of-the-trial/proposal.md`, and the diff
touches only the `## Capability Routing` section (the score, the "All three
rows" sentence, the first row's parenthetical, and the new third row). Nothing
else in the proposal changed. `givn check review` on the tree before this
verdict flip reported only `review.md actual verdict is FAIL` and no routing
error; the receipt was reused across the documentation-only revision. The
blocker is closed and no production code changed to close it.

### O1 (observation) — this change's durable docs landed under the sibling's commit

`docs/arc42/02`, `03`, `05`, `06`, `08`, `09`, `11`, `docs/reference.md`,
`docs/glossary.md`, and the ADR-076 link were committed in `a2f3bfb3`, whose
subject is the sibling change `add-delivery-identity-to-trial-record`. The
tasks record why: the shared lines were authored by one session and the commit
tool's per-actor selection could not split them. The content is present and
committed; this is an attribution observation, not a missing-artifact finding.
The archive must not read the sibling commit as evidence that this change made
those edits.

### O2 (observation) — coverage evidence predates this change

`coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml`, and
`coverage/cobertura-coverage.xml` are dated 2026-11-16 01:16, before this
change's implementation commits, and contain none of `frozen_paths`,
`require_frozen_absent`, or `excluded_roles`. This review classifies from
inspection and execution, and the archive receipt must carry the measured
coverage result if the archive scope requires it.

### O3 (observation) — the compatibility scenario discriminates the record default, not the early return

P4 removed the `excluded_roles` early return and the compatibility scenario
stayed green, because the record's `retained_files` lists the planted package
paths and that exemption filters them first. The scenario discriminates
`#[serde(default)]` (P5) and the retained exemption, which is the property the
use case states. The early return remains inspection-only and the residual
risk (a deleted field) is recorded in ADR-078. Not a defect.

## Verdict

The implementation behavior is correct: writer exclusion, record field, report
line, trial-side refusal, bounded refusal, compatibility, ADR register, contract
surface, evidence, and the lost-feedback statement all pass their probes. The
one blocker, the missing `adr-qualification-gate` routing row, was closed by
`a7a0088d` on 2026-11-16; the closed finding and its closure checks are recorded
under B1 above. The review gate now exits 0.

REVIEW: PASS
