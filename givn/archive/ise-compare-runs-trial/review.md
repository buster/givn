# Review: ise-compare-runs-trial

Review kind: independent
Author: givn-maintainer--owner
Reviewer context: independent-reviewer

README-IMPACT: updated - Configuration
PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Review context and pre-gate record correction

This is one fresh independent final-review context. The reviewer did not author
the change and received no author conversation. It reviewed the committed tree
at `10b8074a`: implementation batch `eaefc0bf20`, task record `08c9ef4e`,
receipt/transcript `239edcc3`, and the docs-only table alignment `10b8074a`.
The context covers the design-review obligation by re-reading `design.md`,
`design-review.md`, and the implementation; the design findings and the
implementation agree.

Two pre-gate record corrections were required because both artifacts still
carried the earlier self-review record. They were applied before the gate run:

| Artifact | Current record | Required record |
|---|---|---|
| `proposal.md` | `Review kind: self-review`, reason "focused correction…" | `Review kind: independent`; reason: the operator requested an independent final review on 2026-11-19 |
| `design-review.md` | `Review kind`, `Author`, and `Reviewer context` sit inside a `text` code fence, so `review_context::parse` strips them | put `Review kind: independent`, `Author: givn-maintainer--owner`, `Reviewer context: independent-reviewer` outside any fence, and note that this independent context confirmed the grilling and the `10b8074a` correction |

`src/review_context.rs` would reject the independent record until both updates
land. They are now applied. The substance below is clean.

## Implementation evidence check

### `@e2e` tag integrity

No `@wip` tag remains. The delta keeps exactly one active `@e2e` scenario:

- `@givn.removed @e2e` placeholder for `Verifier compares the spec copy with the manifest`.
- `@givn.added @e2e` for `Verifier compares the spec copy with the manifest and runs the trial`.

The read-only action survives as the `@givn.modified` regular scenario
`Comparison labels frozen-input equality as unverified`, driven with
`--inputs-only`. No surviving scenario lost `@e2e`; the tag moved with the
replacement of the action, and the new scenario drives the real compiled
binary.

### Step definition bodies

0 empty or trivial step bodies were found across the five step-definition
files this change edits:

- `tests/steps/trial_input_verification_steps.rs`
- `tests/steps/prose_consolidation_steps.rs`
- `tests/steps/ise_clean_room_support.rs`
- `e2e/tests/steps/trial_input_verification_e2e_steps.rs`
- `e2e/tests/steps/ise_clean_room_support.rs`

A repository-wide scan of 156 step files found 9 pre-existing trivial helper
or binding bodies in files this change does not touch
(`llm_attribution_e2e_steps.rs`, `preserve_docstrings_on_archive_steps.rs`,
`run_command_steps.rs`, `spec_discovery_deterministic_steps.rs`,
`workflow_enforcement_steps.rs`). None is introduced or modified here. Every
new `Then` asserts stdout, stderr, the exit code, or a JSON record field; the
new component fixtures execute a bound `Given nothing` step and fail on an
unbound step.

### Checked-task commit verification

All checked tasks name the implementation batch revision; `git show --stat
eaefc0bf20` carries the production code, tests, fixtures, delta specs, and
docs, not a comment-only or tag-only change. `git merge-base --is-ancestor
eaefc0bf20 HEAD` holds.

| Scenario / check | Commit | Touches production code or a regression test? |
|---|---|---|
| Verifier compares the spec copy with the manifest and runs the trial | `eaefc0bf20` | yes — `src/cli/ise.rs`, `src/ise/verify.rs`, `src/main.rs`, e2e fixture |
| Comparison labels frozen-input equality as unverified | `eaefc0bf20` | yes — compare path and regular steps |
| Comparison refuses a missing component runner | `eaefc0bf20` | yes — `run_component` refusal and fixture |
| Comparison refuses a trial result inside the spec copy | `eaefc0bf20` | yes — `prepare_trial_result` |
| Served ise guidance distinguishes the trial gate from input equality | `eaefc0bf20` | yes — `assets/instructions/ise.md` and prose steps |
| Served ise guidance requires the ordinary component workflow | `eaefc0bf20` | yes — served guidance and prose steps |
| Four result-safety checks | `eaefc0bf20` | yes — `src/ise/verify.rs`, `tests/ise_component_tier_gate.rs` |
| Two Passuff-shaped checks | `eaefc0bf20` | yes — `tests/ise_passuff_trial.rs`, fixtures |
| Setup / strict mode | `eaefc0bf20` | yes — runner evidence and tasks proof |

`givn status --change ise-compare-runs-trial` reports `14/14 tasks` and stage
`tasks-done`.

### Promised components and contracts

| Component / surface from design.md | Exists in the tree? | Path |
|---|---|---|
| `CompareCommand` and the compare flow | yes | `src/cli/ise.rs` |
| `prepare_trial_result` and the shared `verify_trial` gate | yes | `src/ise/verify.rs` |
| `compare`/`verify-trial` flags and dispatch | yes | `src/main.rs` |
| Served ise guidance | yes | `assets/instructions/ise.md` |
| Embedded commands guidance | yes | `assets/commands.yaml.tmpl` |
| Reference, glossary, README, arc42, ADR-076 | yes | `docs/reference.md`, `docs/glossary.md`, `README.md`, `docs/arc42/**` |
| Five step-definition/fixture files | yes | as listed in `design.md` |
| `src/ise/compare.rs` unchanged as the internal input-only helper | yes | no diff in the batch |

Consumers use the declared boundary: `main.rs` builds
`cli::ise::CompareCommand`; `cli::ise` calls `ise::prepare_trial_result` and
`ise::verify_trial`; no consumer imports `src/ise/verify.rs` internals. The
public `prepare_trial_result` is the reviewed CLI preflight surface, not an
implementation leak into another consumer.

### Strict-mode proof

`tasks.md` records the unbound-step probe with `Step doesn't match any
function` and exit 1. Both runners remain strict: `tests/features_runner.rs`
calls `.fail_on_skipped()` and exits nonzero for failed or skipped stats;
`e2e/tests/e2e_runner.rs` does the same and also exits nonzero when no scenario
ran. The change adds no runner or framework.

### Design match

| design.md value | As built | Match? |
|---|---|---|
| `ise compare` gains `--result` and `--inputs-only`, conflicting | `src/main.rs` | yes |
| Default result `<manifest>.trial-result.json` beside the manifest | `default_result_path` | yes |
| `prepare_trial_result` refuses the manifest, checkout, copy, symlink; removes only regular Givn results | `src/ise/verify.rs` | yes |
| `--inputs-only` executes nothing, writes nothing, prints the unverified line | `cli::ise::compare` | yes |
| Compare executes the shared gate and exits 0 only for a clean trial | `cli::ise::compare` → `verify_trial` | yes |
| Step-definition file paths and e2e support files | all five exist and are bound | yes |
| Runner commands `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e --package e2e` | `givn/commands.yaml` | yes |
| `src/ise/compare.rs` remains the internal input-only helper | no diff | yes |
| `10b8074a` table correction puts the file in column 2 for the integrity hook | `design-review.md` records the correction | yes |

No deviation from the reviewed design was left as a silent shortcut.

### E2E real-interface check

The delta `@e2e` scenario's Then steps assert:

- the two subject lines,
- exit zero,
- no `- ` difference line,
- `Trial result: passed`.

The `@e2e` step invokes the compiled `givn` binary through
`common::run_givn` (`GIVN_BIN`) and reads its stdout and exit status. It is not
repository-only and does not use an HTTP or in-page shortcut. The committed
transcript at
`evidence/visual/verifier-compares-the-spec-copy-with-the-manifest-and-runs-the-trial/transcript.txt`
records the real stdout, the argv without `--inputs-only`, and exit 0. The
modified read-only title is regular, and its step invokes `--inputs-only`; the
e2e transcript selector appends `--inputs-only` for that title when it is run
in the e2e package. The pre-existing permanent @e2e title remains the selector's
inputs-only case until the archive merge retires it.

### E2e scope and receipt

| `verify.command` | `verify.e2e_command` | Explicit scope selection | Regular scope evidence | E2e scope evidence | Zero failed or skipped? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e --package e2e` | `GIVN_RESULT_SCOPE=change` and `GIVN_RESULT_SCOPE=e2e` in `run-tests.sh` | receipt `result`: 5 passed, 0 failed, 0 skipped | receipt `e2e_scope`: 1 passed, 0 failed, 0 skipped | yes |

`givn check review` reused the change receipt across the docs-only revision
`10b8074a` (`tested at 08c9ef4e`, 5 regular scenarios). The full merged suites
are the archive gate's run; the change-scoped e2e passed and the full
non-e2e suite passed at implementation preflight, per the main-agent direction.
This review did not re-run the suite.

### Duplicate-implementation check

`verify.e2e_command` runs the standalone `e2e` package,
`e2e/tests/e2e_runner.rs`, over the shared feature corpus. The trial capability
has exactly one e2e step file,
`e2e/tests/steps/trial_input_verification_e2e_steps.rs`; the regular
`tests/steps/trial_input_verification_steps.rs` is the regular tier's binding,
not a parallel e2e implementation. The e2e package links no product crate.
A tree-wide search including untracked files found no second e2e implementation
for this capability.

### Interaction coverage

| Inventory entry (owning use case) | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `prepare-an-ise-test` / `trial-input-verification`: compare the frozen inputs and execute the trial by default, or stop at input equality with `--inputs-only` | `design.md` Use-Case Traceability row "trial-input-verification interaction inventory" (D1) | `Verifier compares the spec copy with the manifest and runs the trial` | e2e package invokes the compiled `givn` binary via `common::run_givn`; asserts stdout and exit code |
| `obtain-effective-guidance` / `serve-extended-rules`: request effective instructions by id; the ise guidance forms are variants of that action | `design.md` Use-Case Traceability row "Guidance names the normal path and the safe path" (D2, D5, D7) | Existing `An agent receives the extended rules through the Givn CLI`; the two delta guidance scenarios are regular variants | `givn instructions ise` through the compiled binary; the prose steps assert the served text |

Every inventory entry for the affected use cases maps to a design row, an
existing `@e2e` action, and the promised real CLI mechanism. No inventory entry
was added, removed, or left unmapped. The new trial action has the one new
`@e2e` scenario; the inputs-only, missing-runner, result-path, count, and
guidance cases are regular variant scenarios.

### Independent probes

Each probe used `target/debug/givn` against a temporary Git fixture in `/tmp`;
the repository was not modified and `git status --porcelain` stayed empty.

- Clean default `compare` printed the two subject lines, `Component scenarios:
  2/2`, `End-to-end tests: 1/1`, and `Trial result: passed`; exit 0; it wrote
  `<manifest>.trial-result.json` and the `.component`/`.e2e` siblings beside
  the manifest.
- A drifted copy rerun exited 1 with `Trial result: not verified`; it removed
  all three Givn-owned result files and left the manifest untouched.
- `--result <manifest>` exited 1 naming `is the manifest`; the manifest bytes
  survived.
- An unrelated file at the default result path exited 1 naming `not a Givn
  trial result`; the file content survived.
- `--inputs-only` on drifted inputs exited 1, printed
  `Trial result: unverified (inputs only; no trial command was executed)`, and
  left a pre-existing result file untouched.
- A symlink result path exited 1 naming `is a symlink`; no file was written
  into the copy.
- A component script that wrote no result exited 1 naming
  `component_executed=0/2`, recorded `clean: false`, and printed no pass line.
- A zero-unit runner result exited 1 naming `component_executed=0/2`, recorded
  `clean: false`, and printed no pass line.
- A real component execution reporting one of two declared units exited 1
  naming `component_executed=1/2`, recorded `clean: false`, and printed no
  pass line.

## Use-Case and Persona conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case goal, actors, trigger, and preconditions preserved | delta `specs/prepare-an-ise-test/usecase.md`; Developer primary, Independent verifier supporting | yes |
| Frozen-input equality precedes every trial result | main-flow steps 6-7, the extensions, the success guarantee, and the delta scenarios | yes |
| The declared component corpus executes exactly | rules and `component_executed` refusals; missing/zero/count-mismatch tests and probes | yes |
| The trial result is outside the copy and checkout and describes the current attempt | `prepare_trial_result`, revocation tests, and probes | yes |
| The End-to-end suite drives the rebuilt product | unchanged product binding; Passuff-shaped clean test and probes | yes |
| Guidance names the normal path and the safe path | two delta guidance scenarios against the served text | yes |
| Capability ownership and typed relationships preserved | `prepare-an-ise-test` and `obtain-effective-guidance`; lint reports both capabilities already owned; no new relationships | yes |
| Actors remain distinct from Persona biographies | Developer and Independent verifier actors; Personas separate | yes |
| Migration ledger / structural model | no persistent schema or migration; `docs/domain-model.md` unchanged | N/A |

The use-case scope rejection criteria do not hold: one goal verb phrase; a
named primary actor; both capabilities placed under the goal; step-keyed
extensions; concrete preconditions; no observing goal carrying a mutation;
component- and channel-free capability names. The Goal and Trigger name no
implementation mechanism; the guarantees keep the project's established
user-facing command and record vocabulary, which the pre-change guarantee also
used.

Persona dispositions:

| Persona | Outcome preserved | Evidence |
|---|---|---|
| `developer--primary-user` | The consumer-project owner sees the executed compare result and the safe `--inputs-only` inspection | command output and the compare scenarios |
| `agent--technical-role` | The builder reads the component workflow and writes the runner, steps, and fakes the frozen scenarios need | served ise guidance scenarios |
| `givn-maintainer--owner` | The owner sees the gate run only an executed trial and the result-path revocation protect the evidence | gate refusals, probes, and this review |

No invented or silently promoted Persona exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 2 TC-53: compare executes by default, `--inputs-only` stays read-only, result path safety | `docs/arc42/02-architecture-constraints.md` TC-53 | Yes | Scope, Architecture Impact, D1-D4 | compare-runs-trial and result-safety rows | `src/cli/ise.rs:59-130`, `src/ise/verify.rs:121-268` | yes |
| 3 verifier entity and CLI rows | `docs/arc42/03-context-and-scope.md` | Yes | Architecture Impact | compare batch | `src/main.rs:295-320`, `src/cli/ise.rs` | yes |
| 4 clean-room strategy row | `docs/arc42/04-solution-strategy.md` | Yes | D1, D3, D5 | compare batch | compare flow and verify-trial CLI | yes |
| 5 `cli::ise` and `ise` blocks | `docs/arc42/05-building-block-view.md` | Yes | Architecture Impact | compare and result-safety batches | `src/cli/ise.rs`, `src/ise/verify.rs` | yes |
| 6 6.21 compare and Verify trial flows | `docs/arc42/06-runtime-view.md` | Yes | Data Model, Architecture Impact | result-safety rows | `prepare_trial_result` before compare; gate after equality | yes |
| 8 trial-gate crosscutting paragraph | `docs/arc42/08-crosscutting-concepts.md` | Yes | Scope, Decisions | compare batch | `src/ise/verify.rs` and CLI rendering | yes |
| 9 ADR-076 amendment | `docs/arc42/09-architecture-decisions.md`; `docs/arc42/adr/ADR-076-clean-room-trial-record.md` | Yes (`AMEND_ADR`) | D5 | compare batch | ADR-076 amendment 2026-11-19 and docs | yes |
| 11 R-167 to R-173 | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Scope, Decisions | result-safety rows | R-172 execution risk, R-173 result-path collision | yes |
| 12 glossary: Trial result names both executed paths and the inputs-only form | `docs/arc42/12-glossary.md` links `docs/glossary.md:159` | Yes | Architecture Impact | compare batch | `docs/glossary.md` updated | yes |

- [x] Every affected chapter was opened and checked for current, specific
      content; no placeholder or stale contradiction remains.
- [x] Every relevant architecture fact and decision is represented in
      `design.md` and mapped to the compare or result-safety batch.
- [x] The completed implementation follows those facts and decisions; the
      evidence names the built files, tests, and probes.
- [x] Architecture changes discovered in the implementation are recorded in
      the durable chapters, ADR-076, R-172/R-173, and `arc42.md`.
- [x] No mismatch remains; the only documentation correction, the step-table
      column order, was recorded in `design-review.md` at `10b8074a`.

ARC42 CONFORMANCE: CLEAN

The Ise rebuild ledger records exactly one decision: `No rebuild` (design D8),
with no `Rebuilt code` line. No Ise rebuild trial was executed, so no `Spec
gap` outcome is due. `docs/domain-model.md` needs no structural change: the
change alters command behavior and result-file rules, not a domain entity,
boundary, or relationship.

## Ubiquitous language conformance

The change reuses the durable terms **Trial result**, **Verification
contract**, **Execution unit**, **Spec copy**, and **Rebuilt product**. The
`Trial result` glossary entry now names both executed paths and the
`--inputs-only` form; no new domain term is introduced. The specs, design,
reference, served guidance, and code use the same terms; no deleted constant or
old rule remains in the glossary, README, or reference. No structural-model
change is required.

## Coverage

**Coverage result:** No archive coverage receipt exists yet; this is a
pre-archive candidate. The change verification receipt records scenario
execution only (5 regular + 1 e2e, zero failed/skipped), not line or branch
percentages. The archive's `./measure-coverage.sh` Cobertura run will carry the
measured result. This review classifies from the changed code, the scenario
tests, and the probes.

### Coverage gap classification

1. **Dead code — none.** Every new symbol is consumed: `CompareCommand` by
   `main.rs`; `prepare_trial_result` by the CLI preflight and `verify_trial`;
   `revoke_previous_result` and `PreviousResult::{Trial,Component,EndToEnd}` by
   prepare; `default_result_path` by compare. The post-resolution symlink
   guard is defense-in-depth on the same tested refusal path, not an unused
   symbol.
2. **Missing test coverage — none open.** The new decision points have named
   evidence: clean default compare and the beside-manifest record
   (`compare_runs_the_trial_and_records_a_clean_result`); read-only
   `--inputs-only` (`compare_inputs_only_is_read_only_and_prints_unverified`,
   `inputs_only_leaves_a_previous_givn_result_untouched`); manifest, checkout,
   copy, and symlink result paths; non-Givn and non-Givn raw-sibling refusal;
   revocation of a previous Givn result on an input difference; missing and
   zero-unit runner refusal; count mismatch; product binding; the two guidance
   scenarios. The positive removal of a prior raw Givn sibling shares
   `revoke_previous_result` with the asserted trial-result removal and was
   exercised by the review probe; its false branch is asserted by
   `a_non_givn_raw_sibling_is_never_deleted`. The e2e selector's
   `--inputs-only` branch is exercised by the permanent read-only e2e title
   before the archive merge; the delta's read-only title is a regular scenario
   with the same `--inputs-only` step.
3. **Legitimately hard to test — the OS read/remove error propagation in
   `revoke_previous_result`.** Concrete justification: it requires a hostile
   filesystem state (permission denial or a vanishing path) to reach, makes no
   product decision of its own, and fails closed by returning the I/O error.
   No scenario or fault-injection test is added; the branch is context
   propagation, not a hidden acceptance rule.

No classification outside these three buckets is used. No redundant unit test
duplicates a `.feature` behavior; the new integration tests protect safety
conditions the real-binary scenarios cannot easily inject. No `@wip` tag and no
implementation-layer class or function name appears in the delta spec.

## Scenario execution evidence

The change verification receipt
`givn/changes/ise-compare-runs-trial/verification/tasks-verification.json`
records `head: 08c9ef4e`, command `./run-tests.sh --non-e2e`, change scope
5 passed / 0 failed / 0 skipped, and e2e scope 1 passed / 0 failed / 0
skipped. `givn check review` reused it across the documentation-only revision
`10b8074a`. The change-scoped e2e passed. The full merged e2e suite is the
archive gate's run, not a review run; this review did not re-run the suite.

## Overlap dispositions

`givn lint --change ise-compare-runs-trial` reports 4 `SHAPE` advisories, 11
`SUBST` advisories, and 0 `LONG` advisories; net delta 4 added, 2 modified, 1
removed. Every shape match involving the delta is dispositioned:

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Comparison refuses a deleted kept Source code file` | `Comparison refuses a trial result inside the spec copy` | boundary |
| `Comparison refuses while the ise addon is off` | `Comparison refuses a trial result inside the spec copy` | boundary |
| `Comparison refuses while architecture principles are off` | `Comparison refuses a trial result inside the spec copy` | boundary |
| `Verification refuses a missing component runner` | `Comparison refuses a missing component runner` | variant |

The first three pairs share the same Given/When/Then shape but assert distinct
refusal rules: copy integrity, addon effectiveness, architecture-principles
effectiveness, and result-path safety. The fourth pair is the same missing
runner rule exercised through the explicit `verify-trial` surface and the
default `compare` surface; both protect distinct command wiring.

The removed `Verifier compares the spec copy with the manifest` is superseded
by the added `Verifier compares the spec copy with the manifest and runs the
trial` for the default action; the read-only check survives as the modified
`Comparison labels frozen-input equality as unverified` with `--inputs-only`.
The remaining `SUBST` advisories are shared-step subset heuristics over
unrelated rules and are not disposition rows.

## Split-or-keep

No delta scenario exceeds the 14-step `long_scenario` threshold; the gate
reports 0 `LONG` advisories, so this section has no rows.

## Retirements

No capability is retired. The delta removes one scenario placeholder and adds
its replacement; it does not retire a capability.

## Visual review

Not applicable. `design.md` declares no `## Visual Design Contract`; the
capability's interface is a CLI, not a Web UI. The opposite classification — a
browser-rendered HTML/CSS/JS interface — does not hold: the product has no
page, HTTP server, or browser asset, and the `@e2e` step drives the compiled
binary and reads stdout and the exit code. The committed terminal transcript is
evidence for the executed trial title; the inputs-only title is regular and its
`--inputs-only` invocation is asserted by the step definitions rather than a
separate terminal artifact.

VISUAL-REVIEW: PASS

## Sign-off checklist

- [x] Fabrication audit clean: 0 empty step bodies in the changed step files,
      all checked tasks backed by `eaefc0bf20`, every promised component
      exists, no downgraded e2e scenario, and one normalized e2e action.
- [x] Every checked task has a verified commit touching production code or a
      regression test.
- [x] Every promised component exists and every consumer uses only its
      declared contract.
- [x] Strict-mode proof present and passing in both runners.
- [x] Scenario execution evidence recorded per scope in the change
      verification receipt (5 regular + 1 e2e, zero failed/skipped); the full
      merged suites are the archive gate's run.
- [x] Coverage classification uses the latest available measured evidence;
      no archive receipt exists yet and the archive will carry the merged
      Cobertura result.
- [x] Every coverage gap is classified under the three buckets; no fourth
      bucket is used.
- [x] No dead code; no missing test coverage left open; the one hard-to-test
      area is justified.
- [x] No redundant unit test duplicates a scenario behavior.
- [x] No `@wip` remains and the delta spec carries no implementation-layer
      detail.
- [x] Canonical E2E policy was read; every normalized inventory action has one
      `@e2e` scenario and the variants are regular scenarios.
- [x] Every e2e scenario has a real-interface primary assertion and uses the
      reviewed driver; no browser UI was replaced by HTTP or `fetch()`.
- [x] The local run declaration is the CLI entry point (`cargo run --
      instructions`); `target/debug/givn instructions review` and `givn status`
      started cleanly. No server or digital twin exists.
- [x] The exact files `verify.e2e_command` invokes were identified and read,
      and the tree including untracked files was searched for a parallel e2e
      implementation.
- [x] The e2e scope is explicit (`GIVN_RESULT_SCOPE=change` / `=e2e`); both
      recorded executions report zero failed and zero skipped.
- [x] Implementation commands, file layout, and framework/driver match
      `design.md`; the one table-shape correction is recorded in
      `design-review.md` at `10b8074a`.
- [x] Interaction coverage maps every affected inventory entry to a design
      row, an existing `@e2e` scenario, and the promised driving mechanism.
- [x] The design introduces no unlisted production primitive that a step
      definition relies on; `CompareCommand` and `prepare_trial_result` are the
      named new surfaces.
- [x] Arc42 slug mapping is verified for the affected chapters; the glossary
      row names the durable `docs/glossary.md` linked by chapter 12, and that
      entry was edited.
- [x] No finding anywhere in this report is excused with a classification
      outside the three coverage buckets.

REVIEW: PASS
