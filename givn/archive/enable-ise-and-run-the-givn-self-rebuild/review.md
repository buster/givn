# Review: enable-ise-and-run-the-givn-self-rebuild

Review kind: independent
Author: enable-ise-and-run-the-givn-self-rebuild trial executor (omp, deepseek-flash)
Reviewer context: paseo design-review context, 2026-11-17 (deepseek-flash, message 8d6e24ce); this one context also owns the final review

README-IMPACT: none

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Result

The trial ran and is red by design: the final frozen result is 120 total, 51
passed, 69 failed, 0 skipped. The design's exit criterion is the runnable binary
plus every difference carrying exactly one `Spec gap` outcome, not a green
suite. All 69 residual differences have one outcome. The review did not rerun
the full suite; the archive verification receipt is the full-suite authority.

No blocking finding remains. The deviations recorded below are operator-directed
and recorded in the trial artifacts, not silent shortcuts: the S12 prerequisite
is outstanding under the operator override, the bounded implementation-defect
repair was completed by the trial executor after the clean-room builder was
canceled, and the self-dogfood sequence ran before the archive so the archive
task-progress gate sees the checked task. The trial is not S12 evidence.

## Trial evidence

| Item | Recorded value | Verification |
|---|---|---|
| Trial revision | `9822ffbb29effb0de320789c2882f9a6915714b0`, `docs(enable-ise-and-run-the-givn-self-rebuild): plan the change with the trial inputs` | Reachable from `HEAD`; the report, manifest, and `trial-revision.txt` name it. |
| Spec copy | `/tmp/givn-ise-self-rebuild-20261117T133311Z` | Exists outside the checkout; `givn ise compare` before and after the build prints `Specification files: match` and `End-to-end tests: match`. |
| Manifest | `/tmp/givn-ise-self-rebuild-20261117T133311Z.yaml` | `ise-manifest/v1`; revision, counts, copy directory, checksums, and `givn_version: 0.9.20` agree with the report and the running binary. |
| Rebuild report | `evidence/ise/rebuild-report.txt` | Revision, 90 Specification files, 0 kept source, 187 removed, 100 End-to-end package files, the two checksums, and both paths match the manifest. |
| Initial clean-room binary | `givn 0.9.10`, SHA-256 `ccf0332307a0d67acd96d5dfc68a68c0bbf3ce24ea2c00401b9063c820caccb3`, 3,266,384 bytes | Builder notes: 12 release builds in the sandbox; frozen run 1 is 120/35/85/0. |
| Repair build 1 | SHA-256 `8ba8f36dc13ecb5fcc0850a3ffe032525f3396f1a93778ecb2134cd1fa0148f8`, 3,291,976 bytes | Frozen run 2 is 120/49/71/0. |
| Final binary | `givn 0.9.10`, SHA-256 `f7e808d4b65f1ae8ccb43c19b66719d3e127470089fe204665644e40f9e6738f`, 3,299,216 bytes | `rebuilt-version.txt` and `final-binary.sha256` agree; `e2e-result.json` is 120/51/69/0 and `e2e-run-final.log` is 120 scenarios (51 passed, 69 failed). |
| Trial inputs from the checkout | `GIVN_BIN` is the rebuilt binary; `GIVN_READY_FIXTURE` and `GIVN_ROOT` are test infrastructure from the checkout | `spec-gaps.md` `## Trial inputs read from the checkout` and the builder-notes executor note record the exact command and the D3 disclosure. |
| Promotion | None | The copy, manifest, and rebuilt binary stay outside the checkout; no install, tag, or release; `ise` remains an opt-in addon. |

Manifest invariants: 187 `removed_paths`, 90 `specification_files`, an empty
`source_kept_files`, 156 `end_to_end_files` (100 package files plus 56 feature
paths), 90 Specification checksums, 156 End-to-end checksums, and 56 scenario
checksums. The report's `End-to-end tests: not copied (100 files)` is the
package count; the manifest records the 156-entry union. The 90 Specification
files are the 85 `givn/specs/**` files plus the three declared documents, the
arc42 limits file, and the declaration.

## Spec gap triage

| Measure | Final value | Evidence |
|---|---|---|
| Frozen runner total | 120 scenarios from 115 selected definitions plus 5 Scenario Outline examples | `e2e-inventory.md`, `e2e-result.json`, `e2e-run-final.log` |
| Passed / failed / skipped | 51 / 69 / 0 | `e2e-result.json` and the log summary agree |
| `implementation defect` | 64 | `spec-gaps.md` final residual table, rows 1 to 69 |
| `specification defect` | 5 | Four `attribution_asset` rows (15 to 18) and the product-version row (61), all against `givn/specification.yaml` |
| `E2E test defect` | 0 | `No E2E test defect was observed`; no restart |
| `acceptable undocumented change` | 0 | No row |
| Disclosed build-input row | 1 `specification defect` against `givn/specification.yaml` for `Cargo.toml`, `Cargo.lock`, `assets/**`, the toolchain, and the registry cache | `spec-gaps.md` `## Spec gaps` |
| Trial 1 triage | 85 rows: 80 `implementation defect`, 5 `specification defect`, 0 `E2E test defect`, 0 `acceptable undocumented change` | `spec-gaps.md` `## Trial 1 triage`; the observed column escapes `\|`, so the 3 rows whose text carries a pipe need the escape-aware count; the summary counts are authoritative |
| Bounded repair | 16 of the 85 trial-1 implementation defects were repaired; 64 remain | `repair-notes.md`; the final table has 69 rows, one outcome each |

The final table and the log agree: 69 `✘` scenario lines, 69 rows, 64 plus 5
outcomes, no row with two outcomes, and every row names the wrong artifact. The
four exact integration-asset rows and the product-version row are
`specification defect`; the correction belongs to a later change. No difference
is closed as drift.

## Findings and deviations

No blocking finding. The review found the following deviations and records them
with the operator decision that authorizes each.

1. **S12 prerequisite outstanding.** The plan's phase table gates `S13` on
   `S3`, `S4`, `S5`, and `S12`; section 0 records `S10` to `S12` outstanding.
   The operator's S13 execution instruction of 2026-11-17 starts the trial with
   the `S12` row absent. `proposal.md` `## Operator Override: S12 Prerequisite`
   records the override; `spec-gaps.md` `## Setup failures` records the absence;
   the trial is not S12 evidence and `S14` remains gated on `S12`. The trial did
   not claim a satisfied prerequisite.
2. **Repair actor deviation.** Design D2 routes an `implementation defect` back
   to the builder inside the same trial. The operator directed a bounded repair
   after the clean-room builder was canceled; the trial executor completed it.
   `repair-notes.md` records the actor and the bounded results. The initial
   binary's clean-room claim is unchanged; the final binary is the
   executor-repaired build and is not presented as a clean-room build. The
   classification-before-repair rule held: the 85 trial-1 differences were
   classified before the repair, and the 69 residual differences carry one
   outcome.
3. **Self-dogfood before archive.** Design Verification item 10 orders the
   self-dogfood after archive. The operator ran it before archive so the archive
   task-progress gate sees the checked task. `evidence/ise/self-dogfood.txt`
   records `cargo install`, `givn skills sync`, `givn instructions`,
   `./run-tests.sh`, and `./run-tests.sh --e2e --package e2e`; every command
   exits 0, the installed binary is `givn 0.9.20`, and both suite summaries are
   120 passed. The `givn instructions` transcript shows the in-progress
   `38/46` state from before the C4 tick; the sequence still ran on the final
   tree and the installed binary and skills are current.
4. **Review and self-dogfood task ticks.** Commit
   `6fffac83b024b9ea9db8e92aa26efadde9c05235` records the self-dogfood evidence
   and the review/archive/self-dogfood task ticks. `review.md` and
   `boundary-review-check.txt` are produced by this review and the archive
   commit carries them; the archive itself is the next step. The task boxes are
   checked, as the archive gate requires.
5. **Evidence disclosure edits.** The working tree adds the `## Trial inputs read
   from the checkout` section to `spec-gaps.md` and the executor note to
   `builder-notes.md`. Both are current and correct; the archive commit must
   carry them. The earlier snapshot lacked them; this review evaluates the
   current state and finds the gap closed.

DEFERRAL: S12 prerequisite evidence (the phase table gates S13 on S12)
OPERATOR-DECISION: The S13 execution instruction of 2026-11-17 starts the trial with the S12 row outstanding; the override is recorded in proposal.md and spec-gaps.md; the trial is not S12 evidence and S14 remains gated on S12.

## Implementation Evidence Check

**Step definition scan.** The change adds no step definition, no delta feature,
and no step body: `.givn-skip` lists `specs`, `proposal.md` declares
`Behavior change: none`, and `design.md` `## Step Definitions` states `none`.
The frozen `e2e` package is unchanged; the trial runs its existing step files
through `e2e/tests/e2e_runner.rs`. The scan finds 0 changed step bodies; there
are no changed step files to scan.

**Commit cross-check.** The change adds no production code by design, so a
commit that touches production source is not expected. Every checked task has a
reachable batch commit carrying its recorded evidence or planning artifact.

| Batch | Tasks | Commit | Evidence in the commit |
|---|---|---|---|
| `pre-trial-inputs` | 5 | `9822ffbb29effb0de320789c2882f9a6915714b0` | Proposal, design, design-review, arc42, tasks, declaration lock, boundary transcript, dependency tree, coverage export, inventory, and the S12 absence note |
| `spec-copy` | 3 | `1b1c108f06ed0c9aa8b0013b79e8d21622f82722` | Rebuild report, manifest, and before-build comparison |
| `clean-room-build` | 4 | `1b1c108f06ed0c9aa8b0013b79e8d21622f82722` | Builder notes, registry scan, authored-file and crate list, after-build comparison, rebuilt version |
| `frozen-acceptance` | 1 | `1b1c108f06ed0c9aa8b0013b79e8d21622f82722` | First frozen result and log |
| `gap-triage` | 4 | `1b1c108f06ed0c9aa8b0013b79e8d21622f82722` | Trial-1 triage, repair notes, final gap table, disclosed build-input row |
| `evidence-commit` | 2 | `1b1c108f06ed0c9aa8b0013b79e8d21622f82722` | `trial-revision.txt` and the trial evidence |
| `review-and-archive` | 2 | `6fffac83b024b9ea9db8e92aa26efadde9c05235` (the review/archive work commits with this review and the archive) | Task ticks; the boundary-review transcript and review.md are in the archive commit |
| `self-dogfood` | 2 | `3a46c41cdd5a2091562b187a3fa43429ac2f0f63` and `6fffac83b024b9ea9db8e92aa26efadde9c05235` | AGENTS.md correction and the self-dogfood transcript |

All four commits are reachable from `HEAD`; the tasks file has 23 checked tasks
and 0 unchecked, with a real revision identifier on every `COMMIT:` line.

**Promised-architecture cross-check.** The change creates no component and no
contract. The existing components exercised by the trial are `crate::ise`,
`crate::e2e_boundary`, `crate::config::declaration`, and
`crate::cli::instructions`; the design's Contracts and Components table names
them and the trial does not modify their source. The spec copy and rebuilt
binary are outside the checkout, so no promised component is missing from the
tree.

**Strict-mode proof.** `tests/features_runner.rs` calls `.fail_on_skipped()`;
`e2e/tests/e2e_runner.rs` calls `.fail_on_skipped()` and exits 1 when
`failed > 0`, `skipped > 0`, or `total == 0`. The change adds no step body, so
no delta strictness probe is possible or needed. The change's own suite runs are
green in `evidence/ise/self-dogfood.txt` (120 passed for both summaries); the
trial's frozen run is red by design at 69 differences, all classified.

**Downgraded e2e scenario scan.** No delta `@e2e` scenario exists, and the
permanent scenarios are unchanged. The trial runs the permanent frozen suite
through the real CLI binary and the real subprocess steps; no scenario was
downgraded to a repository-only assertion by this change.

**E2e scope check.** The change adds no `@e2e` scenario; the permanent inventory
has 115 definitions over 56 feature paths, expanding to 120 runner scenarios
from three Scenario Outlines. The inventory row count is 115 and the coverage
export's selected count is 115; the runner's `total` is 120. No capability is
over-produced by this change.

**Duplicate-implementation check.** `verify.e2e_command` is
`./run-tests.sh --e2e --package e2e`; it invokes `e2e/tests/e2e_runner.rs` over
`givn/specs`. The tree, including untracked files, was searched for a second
`@e2e` implementation; the only working-tree changes are this change's
`tasks.md`, `spec-gaps.md`, and `builder-notes.md`, and the self-dogfood
transcript is the change's evidence. No parallel e2e implementation exists.

**Local environment check.** The product is a CLI; it starts no server,
database, queue, or browser. The trial executed
`givn ise rebuild-all`, the `bwrap` sandbox build, `givn ise compare`, and the
frozen suite. The copy and manifest are outside the checkout; the sandbox mount
set is the design's probed set; the rebuild used `cargo build --release
--offline`. The change's own `./run-tests.sh` and `./run-tests.sh --e2e
--package e2e` ran green in the self-dogfood transcript. The review did not
rerun the suite.

**E2E scope selection.** `verify.command` is `./run-tests.sh --non-e2e`;
`verify.e2e_command` is `./run-tests.sh --e2e --package e2e`. `run-tests.sh`
refuses `--e2e` alone and records `GIVN_RESULT_SCOPE` (the trial's result file
records `scope: e2e`). The regular and E2E scopes are explicitly separated. The
change's own scopes are green in the self-dogfood transcript; the trial's frozen
result is a separate 120/51/69/0 record, not the change's verify scope. The
archive receipt is the full-suite authority and does not exist yet.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `prepare-an-ise-test` clean-room and comparison rules are exercised by the report, manifest, and before/after comparisons; `declare-the-specification` is unchanged; `create-change` change-strategy and spec-gap are recorded in D5 and the gap table. | yes |
| Capability ownership and typed relationships preserved | The five capabilities remain owned by the three use cases; no capability is added or moved; the Interaction Coverage Matrix has one row per inventory entry. | yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user` reads the report, comparisons, and gap list; `agent--technical-role` built the copy and records inputs; `givn-maintainer--owner` owns S14 and reads the boundary and gap evidence. | yes |
| Actors remain distinct from Persona biographies | The trial actors are the developer, the builder agent, the verifier, and the maintainer; no biography appears in a Gherkin actor. | yes |
| Migration ledger and coverage evidence complete, if applicable | No migration; the trial keeps no persisted product state. Coverage has no measured receipt yet; the archive carries it. | yes |

Persona disposition: `developer--primary-user` (report, comparisons, gaps),
`agent--technical-role` (builder notes, rebuilt version, gaps), and
`givn-maintainer--owner` (boundary transcripts, result, gaps, S14). The design's
`Interface: terminal` requires at least one confirmed Persona; all three are
confirmed.

## Design.md conformance

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| D1 trial revision | The pre-trial commit, passed with `--at` | `9822ffbb`; report, manifest, and `trial-revision.txt` agree | yes |
| D2 repair route | Classify before repair; an implementation defect returns to the builder inside the trial | Classified before repair; the executor repaired after the builder was canceled; recorded in `repair-notes.md` | route holds; actor deviation recorded |
| D3 checkout inputs | `GIVN_BIN` rebuilt; `GIVN_READY_FIXTURE` and `GIVN_ROOT` from the checkout | Exact command and disclosure in `spec-gaps.md` and `builder-notes.md` | yes |
| D5 change strategy | `whole-product rebuild`, `Dry-Run Ise`, boundary `src/**` and `tests/**` | Report: 187 removed, 0 kept, 100 package files not copied; compare match; no promotion | yes |
| D6 boundary evidence | `givn check tasks` and `givn check review` accept through `preflight_e2e_boundary`; no receipt | `boundary-check.txt` records the not-applicable path and the warning; no `verification/tasks-verification.json` | yes |
| D7 build inputs | Five non-Specification inputs, builder-notes and the disclosed `specification defect` row | Builder notes list the five; `spec-gaps.md` has the row | yes |
| Verification item 9 | Both suite scopes green on the final revision | Self-dogfood transcript: `./run-tests.sh` and `./run-tests.sh --e2e --package e2e` green, 120 passed each | yes |
| Verification item 10 | Self-dogfood after archive | Ran before archive to satisfy the task-progress gate; recorded deviation | order deviation recorded |
| Rebuild command and report | `givn ise rebuild-all --at --into --manifest`; eleven fixed lines | Exact flags; report holds the eleven lines plus `Manifest` | yes |
| Sandbox | The probed `bwrap` mount set; `cargo build --release --offline` | Builder notes quote the exact command; 12 builds | yes |
| Frozen acceptance | `GIVN_BIN` rebuilt; `GIVN_RESULT_FILE` in the change; `--e2e --package e2e` | Exact command and result file | yes |

No design.md edit changed the reviewed plan. The two order/actor deviations are
recorded above and in the trial evidence; the commands, file layout, framework,
and driver match the reviewed design.

## Interaction coverage

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| `clean-room-preparation`: write the spec copy | yes | `Developer rebuilds the whole product from its Specification` | Real `givn ise rebuild-all` subprocess | The trial ran the real command; the report holds the fixed lines | yes |
| `trial-input-verification`: compare the copy and the manifest | yes | `Verifier compares the spec copy with the manifest` | Real `givn ise compare` subprocess | Before and after comparisons both match | yes |
| `specification-declaration`: declare the five roles and source roots | yes | `The declared roles are read by the project-health pass` | Real `givn lint` subprocess | The declaration is unchanged; the trial reads it | yes |
| `change-strategy`: read the design guidance | yes | `The design guidance requires one recorded change strategy` | Real `givn instructions design --change` | D5 records the strategy; the frozen scenario is unchanged | yes |
| `spec-gap`: read the review guidance | yes | `The review guidance requires one spec gap outcome before any fix` | Real `givn instructions review --change` | The gap table follows the four served outcomes; the frozen scenario is unchanged | yes |

The three owning use-case documents supply the five entries; the design's
matrix has five rows, and the permanent scenarios carry `@e2e`. No entry is
unmapped.

## Visual Review

The design declares `Interface: terminal`, viewports not applicable. The trial's
user-visible surface is terminal text.

| Surface | Evidence file | Content | Present and non-empty? |
|---|---|---|---|
| Rebuild report | `evidence/ise/rebuild-report.txt` | Eleven fixed lines plus the manifest path | yes |
| Comparison before | `evidence/ise/compare-before.txt` | Both subjects `match` | yes |
| Comparison after | `evidence/ise/compare-after.txt` | Both subjects `match` | yes |
| Rebuilt binary | `evidence/ise/rebuilt-version.txt`, `final-binary.sha256` | Version and checksum | yes |
| Frozen result | `evidence/ise/e2e-result.json`, `e2e-run-final.log` | 120/51/69/0 and the 69 failure lines | yes |
| Gap list | `evidence/ise/spec-gaps.md` | 69 rows, one outcome each, and the disclosed row | yes |
| Self-dogfood | `evidence/ise/self-dogfood.txt` | Commands, installed binary, both suite summaries | yes |

Baseline: the report lines are fixed; refusal and difference subjects are named;
the gap table names the wrong artifact; no internal Rust module, struct, or
field name appears in the user-facing report or gap subject. The three Personas
can reach their goals from the recorded terminal evidence. No visual finding.

VISUAL-REVIEW: PASS

## Retirements

No capability is retired by this change.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `ise` is effective on Givn and the rebuild boundary removes `src/**` and `tests/**` | `docs/arc42/02-architecture-constraints.md` 2.3 | Yes: chapter 02 records the declaration and the frozen `e2e` package | Scope and D5 name the boundary | Pre-trial and spec-copy tasks verify 187 removed, 0 kept, 100 not copied | Report, manifest, compare | yes |
| Self-rebuild strategy, `Dry-Run Ise`, triage, no promotion | `docs/arc42/04-solution-strategy.md` | Yes: `Self-rebuild` row | D5 and the Scope table | Trial, gap-triage, and evidence-commit tasks | `spec-gaps.md`, `repair-notes.md`, result | yes |
| Undeclared surfaces and the triage that contains them | `docs/arc42/11-risks-and-technical-debt.md` `R-163` | Yes: `R-163` | D7 and Anticipated Coverage Risks | Gap-triage task records the disclosed row; final table records the residual outcomes | 64 implementation defects and 5 specification defects; no promotion | yes |

The three affected chapter files exist, hold decision-specific content, and
contain no ASCII art. Chapter 09 needs no new MADR: the qualification records no
architectural commitment, and ADR-010, ADR-076, and ADR-078 already own the
mechanisms. Chapter 12 remains a link to the glossary. The trial's 69 residual
differences confirm `R-163` rather than contradict it. No material mismatch.

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt exists yet; the archive verification and its Cobertura report
are the measured authority. The change adds no product code: the trial's rebuilt
binary is evidence, not the measured build, and the archive coverage gate runs
the regular suite over the committed candidate. The change's own suite is green
in the self-dogfood transcript. No uncovered region belongs to this change's
implementation.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| This change's implementation | none | No changed production code; the deliverable is planning and trial evidence | None; the archive measurement covers the existing product |

There is no fourth bucket and no excused gap.

## Scenario execution evidence

The change has not been archived, so no archive verification receipt exists yet:
`unproven candidate, no receipt yet`. The review did not rerun the full suite.
The change's own `./run-tests.sh` and `./run-tests.sh --e2e --package e2e` are
recorded green in `evidence/ise/self-dogfood.txt` (120 passed in each summary).
The trial's frozen 120/51/69/0 result is the trial evidence, not the change's
verify scope and not the archive receipt. The archive receipt is the full-suite
authority.

## Overlap dispositions

No shape-match findings: the change has no delta scenarios, `specs` is skipped,
and the review gate reports a net delta of 0 added, 0 modified, 0 removed.

## Split-or-keep

No delta scenarios exist, so there is no long-scenario split-or-keep decision.

## Sign-off

- [x] Fabrication audit: CLEAN. No changed step bodies; no new component; no
      delta `@e2e`; the trial evidence is real and the commits are reachable.
- [x] Every checked task has a reachable batch commit carrying its recorded
      artifact. The change adds no production code by design, so no task claims
      a production change it does not have.
- [x] Every promised component exists. The change creates none; the existing
      components it exercises are unchanged.
- [x] Every consumer uses only its declared contract. No consumer is added.
- [x] Strict-mode proof is present: `.fail_on_skipped()` in both runners plus
      the E2E exit-1 conditions. The trial's frozen run is red by design, and
      the change's own suites are green in the self-dogfood transcript.
- [x] Scenario execution evidence is recorded for the change's own scopes in
      the self-dogfood transcript; the archive receipt is the full-suite
      authority and is pending.
- [x] Coverage classification uses the current evidence. No product code changed;
      the archive receipt will carry the measured result.
- [x] Every coverage gap is classified; no fourth bucket.
- [x] Dead code deleted: none added. Missing tests: none, because the change
      adds no product behavior. Hard-to-test gaps: none.
- [x] Redundant unit tests: none added.
- [x] No `@wip` tags remain: the change has no delta feature.
- [x] The delta contains no implementation-layer detail: there is no delta.
- [x] The local run command matches the product: a CLI with no server or digital
      twin; the trial executed the rebuild, sandbox build, comparison, and
      frozen suite, and the copy and manifest are outside the checkout.
- [x] The exact file `verify.e2e_command` invokes is
      `e2e/tests/e2e_runner.rs`; the tree, including untracked files, has no
      parallel e2e implementation.
- [x] The E2E execution selects its scope explicitly (`GIVN_RESULT_SCOPE`, and
      the runner arguments); the trial result records `scope: e2e`; the change's
      own scopes are green.
- [x] Implementation commands, file layout, framework, and driver match
      `design.md` as reviewed. The D2 repair actor and the pre-archive
      self-dogfood order are recorded deviations with the operator's direction;
      no command, layout, or driver changed silently.
- [x] Interaction coverage maps every inventory entry to a matrix row, an
      existing `@e2e` scenario, and the promised driving mechanism.
- [x] No internal primitive is introduced; `design.md` `## Step Definitions`
      states `none`.
- [x] Every arc42 chapter slug in `arc42.md` maps to a real
      `docs/arc42/<slug>.md` file; chapters 02, 04, and 11 are current.
- [x] No finding is excused with a classification outside the three buckets.
      The 69 trial differences are the rebuilt product's gaps, each with one
      outcome; the S12 and process deviations are operator-recorded, not
      invented coverage exceptions.

REVIEW: PASS
