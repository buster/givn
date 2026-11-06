# Review: enforce-spec-interaction-integrity

Review kind: independent
Author: authoring-context
Reviewer context: independent-review-1

## Evidence base

- Implementation commit: `3a33fb39` (`feat(spec): enforce projected interaction integrity`).
  `4d429d84` and `6b82844e` touch only `tasks.md`; verified with
  `git show --stat`.
- Review-fix commit: `424682fa` (`fix(spec): close the independent review findings`).
  It carries the F1 record (`tasks.md:8`), the F2 and F5 design edits, the F4
  record, the F3 unit tests, the design re-review closure, and the change
  verification receipt files. It changes no production behavior.
- Closure commands run for this revision: `cargo test --lib projection::tests`
  (6 passed), `cargo test --lib receipt_target_outside_specs_is_refused`
  (1 passed), `cargo nextest list` for the change features (13 units),
  `cargo nextest run --locked --test features_runner` with the change non-E2E
  tags (12/12 passed), `cargo fmt --all -- --check` (exit 1), `cargo clippy
  --locked --all-targets -- -D warnings` (exit 0), and `cargo run -- check
  review --change enforce-spec-interaction-integrity` (exit 1: stale receipt,
  review-context mismatch, FAIL verdict). The full suite was not run.
- Change verification receipt: `verification/tasks-verification.json` records
  `./run-tests.sh --non-e2e`, scope `change`, 12 total, 12 passed, 0 failed,
  0 skipped, at head `4d429d8426051f5fcc226fc65c68d6b1a041825b`. The receipt
  predates `424682fa`; `check review` rejects it as stale (R2).
- No archive receipt and no Cobertura report exist yet. The archive will carry
  the per-scope execution and coverage evidence. This review re-ran only the
  closure commands above; the full suite was not run.
- At the implementation revision, `cargo run -- check review --change
  enforce-spec-interaction-integrity` passed the run declaration, change-scoped
  lint, overlap dispositions, scenario commits, and the then-current change
  verification receipt; the only remaining mechanical error was the missing
  verdict in `review.md`. At `424682fa` the same command exits 1 with the stale
  receipt (R2), the review-context mismatch (R3), and the FAIL verdict.
- `cargo run -- lint --change enforce-spec-interaction-integrity` exits 0 with
  `givn lint: 4 file(s) checked — clean`. Default `givn lint` exits 1 with the
  11 pre-repair findings (10 fragment capabilities plus
  `initialize-a-project`); the delta repairs all 11:
  `specs/fragments/fragment.md:30-63` matches every permanent fragment `@e2e`
  title for all 15 fragment capabilities, and the init delta adds `@e2e` to
  the declared non-interactive scenario.

## Step 0: implementation evidence check

### Scenario commit evidence

Every checked scenario records `COMMIT: 3a33fb39` with a batch name, and
`3a33fb39` contains the scenario text, its step definitions, and the production
code they exercise. `check review` independently reports
`✓ scenario commits passed (12 scenario(s))`.

| Scenario | COMMIT | Batch | Contained in 3a33fb39 |
|---|---|---|---|
| Change-scoped lint reports an E2E scenario the projected inventory does not declare | 3a33fb39 | projected-lint | Feature, `tests/steps/interaction_projection_steps.rs`, `src/spec/projection.rs`, `src/cli/lint.rs` |
| Change-scoped lint reports a declared interaction whose projected corpus has no E2E scenario | 3a33fb39 | projected-lint | Same batch |
| Change-scoped lint reports a feature the projected inventory does not declare | 3a33fb39 | projected-lint | Same batch plus `src/gherkin/interaction.rs` reverse check |
| Default lint reports an undeclared capability | 3a33fb39 | projected-lint | Same batch plus `src/cli/lint.rs:246,569` |
| Change-scoped lint reports a delta capability with no projected owner | 3a33fb39 | projected-lint | Same batch plus `MergePlanError::UnplaceableCapability` |
| Change-scoped lint accepts a delta that keeps the projected inventory consistent | 3a33fb39 | projected-lint | Same batch |
| Change-scoped lint reports a pre-existing inventory mismatch | 3a33fb39 | projected-lint | Same batch plus the empty-feature path in `src/cli/lint.rs` |
| Review refuses a projected interaction-inventory mismatch before archive | 3a33fb39 | review-projection | Feature, `tests/steps/workflow_enforcement_steps.rs:372-490`, `src/cli/check.rs` via `lint::run_scoped` |
| Review accepts a delta whose projected interaction inventory is consistent | 3a33fb39 | review-projection | Same batch |
| Archive refuses a projected interaction-inventory mismatch before moving the change | 3a33fb39 | archive-projection | Feature, `tests/steps/archive_proof_steps.rs:1387-1448`, `src/cli/archive.rs:186-192` |
| Archive publishes a consistent projected interaction inventory | 3a33fb39 | archive-projection | Same batch |
| Archive refuses a merged candidate whose interaction inventory is inconsistent | 3a33fb39 | archive-projection | Same batch plus `src/cli/archive.rs:315` |

The setup batch (`projection-substrate`) also records `COMMIT: 424682fa`
(review fix, `tasks.md:12`) and `COMMIT: 3a33fb39`; the closeout batch
(`docs-closeout`) records `COMMIT: 3a33fb39`. No scenario records only a
comment, tag, or task edit. No scenario uses `CHARACTERIZATION`.

### Empty or trivial step bodies

0 empty or trivial bodies across the 9 new or changed bindings in
`tests/steps/interaction_projection_steps.rs`,
`tests/steps/workflow_enforcement_steps.rs`, and
`tests/steps/archive_proof_steps.rs`: each new binding writes fixture files or
asserts CLI output.

Pre-existing declaration-style no-op steps remain in two changed files:
`tests/steps/workflow_enforcement_steps.rs:2990,3013,3016` and
`tests/steps/preserve_docstrings_on_archive_steps.rs:800,930`. `git blame`
attributes them to `595abe2ac` (2026-10-29), they are not bound by any scenario
in this change, and this commit adds no no-op step. They are not this change's
fabricated evidence.

### @e2e tag integrity

No delta `.feature` removes `@e2e`. The only tag change is the init delta
`specs/initialize-project/initialize-a-project.feature:5`, which adds `@e2e` to
an existing scenario. The permanent corpus `@e2e` tags are unchanged by the
delta. The new lint, review, and archive scenarios are regular scenarios: they
are refusal and acceptance variants of the existing inventory actions, not new
user actions.

### Then-step quality and driver

The delta's only `@e2e` scenario is `Consumer runs init non-interactively and
gets opencode skills`. Its Then steps assert CLI output
(`tests/steps/initialize_a_project_e2e_steps.rs:107-116`: the report contains
`synced skill: givn-propose` and `disabled (--no-addons)`), installed file
content, and file existence. It is not repository-only. The driving step is
`the consumer runs ...` (`tests/steps/init_agent_target_selection_steps.rs:193`),
which spawns `CARGO_BIN_EXE_givn` through `common::run_givn`.

All 13 matrix `@e2e` scenarios were checked: every one asserts a CLI exit code,
terminal output, or a CLI-written artifact. No `@e2e` scenario asserts only a
repository or database state.

Opposite classification test for the non-browser decision: the repaired init
interaction is a Web UI flow. That classification does not hold. The product is
a CLI, the scenario drives `givn init` through the process boundary (the
interactive variant uses a PTY), and the design declares no visual interface.
The visual review block is therefore not applicable.

### Design components and Step Definitions table versus the tree

| Design promise (`design.md:25-32, 158-165`) | In the tree | Consumers use the declared surface only |
|---|---|---|
| `src/spec/corpus.rs` | Present; `SpecCorpus::read`, `overlay_narrative`, `remove_capability` | `cli::lint` reaches it through `spec::projection`; `gherkin::interaction` takes `&SpecCorpus`; `overlap::analysis` reads it directly (`src/overlap/analysis.rs:118`) as declared. |
| `src/spec/merge_plan.rs` | Present; `MergeEntry`, `build_merge_plan`, `merge_feature_text`, `write_feature`, `plan_from_receipt` | `cli::archive` imports the declared items (`src/cli/archive.rs:1-14`); `spec::projection` imports `MergeEntry`, `MergePlanError`, `merge_feature_text`, `narrative_target`, `collect_narrative_documents`. |
| `src/spec/projection.rs` | Present; `MergeProgress`, `interaction_findings_for_plan`, `interaction_findings`, `permanent_interaction_findings` | `cli::lint` calls `interaction_findings`; `cli::archive` calls the plan and permanent forms. |
| `src/gherkin/interaction.rs` | Present; `interaction_gate_findings`, `usecase_coverages`, `undeclared_capability` | `spec::projection` and `overlap::analysis` are the only consumers. |
| `src/cli/lint.rs` changed | Present | Calls `spec::projection`; no import of `cli::archive`. |
| `src/cli/archive.rs` changed | Present; projection at `:186`, `:287`, `:315` | Calls `spec::merge_plan` and `spec::projection`; no `cli::lint` use. |
| `src/cli/check.rs` unchanged | Unchanged; review branch calls `lint::run_scoped` | No new surface. |
| `src/overlap/analysis.rs` changed call site | Present at `:118-124` | Uses `spec::corpus` and `gherkin::interaction`. |
| `tests/steps/spec_overlap_gates_steps.rs` | Present | Existing e2e bindings. |
| `tests/steps/workflow_enforcement_steps.rs` | Present; review fixtures at `:372-490` | Required bindings present. |
| `tests/steps/archive_proof_steps.rs` | Present; merged-candidate fixtures at `:1387-1448` | Required bindings present. |
| `tests/steps/init_agent_target_selection_steps.rs` | Present; the driving `the consumer runs` step at `:193` | The scenario's Then assertions are also bound in `tests/steps/initialize_a_project_e2e_steps.rs:107-145` and its Given in `tests/steps/mod.rs:327`; F5 resolved, `design.md:165,234` names all three files. |
| `tests/steps/interaction_projection_steps.rs` | Present; registered at `tests/steps/mod.rs:47` | All nine projection fixture phrases bound. |

No consumer imports a component's implementation module to reach around its
declared contract. `spec::merge_plan` is the only merge entry point; the
archive's writer is the thin `write_feature` wrapper.

### Interaction coverage cross-reference

The inventory source is the `## Interactions` table in each owning use-case or
fragment document. The design matrix is `design.md:212-226`.

| Owning document and capability | Inventory entries | Matrix rows | `@e2e` scenario exists | Driving mechanism in the step tree | Result |
|---|---|---|---|---|---|
| `verify-change` / `check-specification-health` | 1 | 1 | `All clean .feature files report a clean summary on stdout` (`@e2e`) | CLI subprocess: `the user runs givn lint` -> `tests/steps/mod.rs:554` -> `common::run_givn` | Match |
| `verify-change` / `givn-workflow-enforcement` | 5 | 5 | `Consumer runs migration coverage check and sees passed evidence`; `Consumer runs check tasks and receives a change verification receipt`; `Consumer invokes bare check and receives explicit non-executing guidance`; `The result helper writes four counts without a scope`; `Consumer runs review and receives every use-case schema problem in one pass` (all `@e2e`) | CLI subprocess: `the user runs` / `the consumer runs` -> `common::run_givn` | Match |
| `archive-change` / `archive-proof` | 4 | 4 | `Consumer archives a completed change and receives a proven archive`; `Consumer repairs a blocked archive and the rerun proves it`; `Consumer inspects an unproven archive candidate`; `Consumer archives a change after documentation-only review edits and the proof is reused` (all `@e2e`) | CLI subprocess: `the user invokes givn archive` -> `common::run_givn` | Match |
| `initialize-project` / `initialize-a-project` | 3 | 3 | `Consumer selects agent targets during interactive init` (`@e2e`, PTY); `Consumer runs init non-interactively and gets opencode skills` (`@e2e` added by the delta); `Consumer re-syncs skills after init and the selection is honoured` (`@e2e`) | CLI subprocess / real PTY: `tests/steps/init_agent_target_selection_steps.rs:193`, `tests/steps/initialize_a_project_e2e_steps.rs:107-145` | Match after the delta merges the tag |
| `fragments` (all 15 capabilities) | 32, of which 19 rows are added by this change | 1 summary row | Existing `@e2e` scenarios exist for every row | CLI subprocess on the existing rows | Resolved: `design.md:212` scopes the 19 repaired declarations and `:216` adds the summary row; see F2 Resolution |

The three use-case documents' touched actions are fully mapped. The fragment
document is a different case: this change adds 19 interaction rows to it, and
none of them appears as a matrix row. `design.md:210` states the opposite
("The change adds no new inventory entry"), while `design.md:102` correctly
says the delta adds the 19 missing rows. The design review's §6 recorded a
"touched actions only" convention for the use-case documents; that convention
does not cover rows the change itself introduces, which the review instruction
requires to be mapped.

Closure (`424682fa`): the false "adds no new inventory entry" sentence is gone;
`design.md:212` scopes the 19 repaired declarations and `design.md:216` adds
their summary row. The 19 rows, their 10 capabilities, and all 19 `@e2e` titles
were re-verified against the corpus. See F2 Resolution.

### Strict-mode proof

`tasks.md:7` promises an executed strict-mode probe: one `@wip` scenario under
`GIVN_FEATURES=givn/changes/enforce-spec-interaction-integrity/specs`, confirm
non-zero for the undefined step, and record the command and output. No command
or output is recorded in `tasks.md`, `design-review.md`, the verification
receipt, or anywhere else in the change. The runner still calls
`.fail_on_skipped()` (`tests/features_runner.rs`), but the performed proof is
missing; see F1.

Closure (`424682fa`): `tasks.md:8` records the executed probe, its non-zero
output, and the probe's removal; see F1 Resolution.

## Arc42 implementation conformance

Affected chapters were derived from `arc42.md`, `design.md`, `tasks.md`, and
the implementation, then checked against the durable chapters.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Pre-mutation projection and the shared merge-text boundary | `docs/arc42/04-solution-strategy.md:55-66` | Chapter 4 affected: projected-corpus gate and `spec::merge_plan`/`spec::projection` boundary | `design.md:17-45, 107-130` | `projection-substrate`, `projected-lint`, `archive-projection` | `src/spec/projection.rs:34-107`; `src/spec/merge_plan.rs:20-66`; `src/cli/archive.rs:186-192, 287` | Yes |
| Building blocks: `spec::corpus`, `spec::merge_plan`, `spec::projection`; changed `cli::lint`, `cli::archive`, `gherkin::interaction` | `docs/arc42/05-building-block-view.md:48-100` | Chapter 5 affected | `design.md:25-32` | Same batches | The files and signatures exist; `cargo check`/clippy ran in the receipted `./run-tests.sh --non-e2e` | Yes |
| Lint/review/archive runtime flow and merged-candidate recheck | `docs/arc42/06-runtime-view.md:22-38, 177-190, 394-410, 437-445` | Chapter 6 affected: projection, pre-mutation refusal, post-merge check | `design.md:132-152` | `projected-lint`, `review-projection`, `archive-projection` | `src/cli/lint.rs:239-270`; `src/cli/archive.rs:186-192, 272-300, 315`; `src/cli/check.rs` via `lint::run_scoped` | Yes |
| Cross-cutting interaction gate, reverse capability check, exit-code contract, severity classification | `docs/arc42/08-crosscutting-concepts.md:161-190, 422-425, 1005-1010` | Chapter 8 affected: 8.3c and 8.8 exit codes | `design.md:64-91, 126-141` | `projected-lint` | `src/gherkin/interaction.rs:136-252`; `src/cli/lint.rs:246, 567-569` | Yes |
| ADR-075 and the active register | `docs/arc42/adr/ADR-075-projected-interaction-integrity.md`; `docs/arc42/adr/README.md:27, 59-60, 75-76, 89-90` | Chapter 9 affected: add ADR-075 and the register row | `design.md:83-91` (decisions D1-D4) | `docs-closeout` | ADR file present; README count is 27 and 27 `ADR-*.md` files exist; register row and relationship row present | Yes |
| Quality Q-8 and Q-125 | `docs/arc42/10-quality-requirements.md:47, 118-119` | Chapter 10 affected: correct Q-8; add Q-125 | `design.md:85-86` | `projected-lint` | Q-8 and Q-125 text present; behavior matches the refusal scenarios | Yes |
| Risks R-135, R-136, R-137 | `docs/arc42/11-risks-and-technical-debt.md:119-124` | Chapter 11 affected: add R-135-137 | `design.md:278-287` | `docs-closeout` | Rows present; R-87 retirement note present | Yes |
| Interaction-inventory vocabulary | `docs/glossary.md:85-100`; `docs/arc42/12-glossary.md` (link only) | Chapter 12 affected: add the three terms | `design.md:6, 26-27, 64-81` | `docs-closeout` | Terms present in the glossary; chapter 12 remains a link to the single authority; no duplicate table | Yes |

ARC42 CONFORMANCE: CLEAN

The marker covers the chapter table above. The review-level findings F1-F5
are closed under Review closure and do not change any Arc42 chapter claim.
R1-R3 are process and receipt regressions, not chapter claims, but they block
the overall verdict.

## Ubiquitous language conformance

- `docs/glossary.md:85-89` defines **Interaction inventory**, **Undeclared
  capability**, and **Projected interaction inventory**. The specs, design,
  docs, and code use those terms: `interaction inventory`, `projected
  interaction inventory`, `undeclared capability`, `interaction_mismatch`,
  `usecase_interaction_invalid`. The glossary marks `interaction matrix` as an
  anti-term and distinguishes the design's coverage matrix, which the design
  calls `Interaction Coverage Matrix`; no collision.
- `docs/glossary.md:100` updates the **Lint** exit precedence to parse/use-case/
  interaction `1`, duplicate/empty `3`, WIP `2`, clean `0`; `docs/reference.md`
  and `docs/arc42/06-runtime-view.md` state the same contract.
- `docs/domain-model.md` is unchanged. The change adds internal Rust modules
  and a CLI gate; the model explicitly excludes internal components, and no
  bounded context, aggregate, or relationship changed. The no-op edit is the
  correct maintenance result.

## Use-case and Persona conformance

- `verify-change` goal, actor, and rule "Interaction coverage matches
  capability plus scenario identity, including declarations owned by
  fragments" are preserved; the projection and the reverse check implement that
  rule. The `archive-change` main flow step 1 ("all before any mutation") and
  success guarantee are preserved; the new refusal is an additional gate inside
  that step, and the merged-candidate check protects the success guarantee. No
  capability ownership moved; the change extends the four declared capabilities.
- Use-case scope rejection criteria: no goal conjunction, a named primary
  actor, capability placement unchanged, no mutation under an observing goal,
  no new extension without a step reference, no inconsistent precondition, no
  mechanism vocabulary in Goal/Trigger/guarantees, no component-shaped
  capability, no oversized leaf, no typed-relationship misuse. No criterion
  fires.
- Personas: `developer--primary-user` and `agent--technical-role` are
  confirmed. Both read the CLI refusal text (`design.md:103`): the developer
  gets the named use case, capability, and scenario from `givn lint` and
  `givn check review`; the agent gets the same finding through the served
  instructions and the review gate. No visual design contract is declared
  (`design.md:105`), so the visual review block is not applicable.

## Coverage classification

No measured Cobertura evidence exists in `verification/` for this change; the
archive receipt will carry the measured result. Classification is by static
inspection of the implementation and the receipted change scenarios.

| Region | Classification |
|---|---|
| `src/spec/corpus.rs` | Covered: 7 unit tests plus the projection scenarios. |
| `src/spec/merge_plan.rs` | Covered; the receipt-target refusal at `src/cli/archive.rs:905-908` is tested by `src/cli/archive.rs:1921` (F3 resolved). |
| `src/spec/projection.rs` | Covered; the skip-merged branch at `:59-63` and the no-write property are tested at `src/spec/projection.rs:190,207` (F3 resolved). |
| `src/gherkin/interaction.rs` | Covered: 7 unit tests plus the lint scenarios. |
| `src/cli/lint.rs` | Covered: 7 lint scenarios, including the default-scope and narrative-only paths. |
| `src/cli/archive.rs` | Covered: 3 archive scenarios plus the existing interrupted-merge, in-flight, and merged-candidate scenarios, and the receipt-target refusal test at `src/cli/archive.rs:1921`. |
| `src/overlap/analysis.rs` | Covered by the default lint and overlap scan; the changed call site is exercised by `givn lint`. |
| Shared fixture repairs | Exercised by the change scenarios; the full-suite result is the archive's scope. |
| Dead code | None found: the new public functions are called by the declared consumers; the receipted `./run-tests.sh --non-e2e` includes clippy with `-D warnings`. |
| Hard to test | None claimed. |
| Redundant unit tests | None identified; the unit tests cover paths the scenario boundary does not. |

## Findings

### F1 - The promised strict-mode proof is not recorded

`tasks.md:7` promises an executed probe and a recorded command/output. No
output exists in the change. The runner does call `.fail_on_skipped()`, but the
performed evidence is absent, so every PASS in this change is unverified under
the review rule.

Fix: run one `@wip` probe scenario under
`GIVN_FEATURES=givn/changes/enforce-spec-interaction-integrity/specs`, record
the command and the non-zero undefined-step output in `tasks.md` (setup batch),
then remove the probe. Do not change `verify.command`.

Resolution (`424682fa`): resolved for the recorded proof. `tasks.md:8` records
the temporary `probe/strictness.feature` with an undefined `Given`, the exact
`GIVN_FEATURES`/`GIVN_CUCUMBER_TAGS` command, the failing unit
`probe/strictness.feature::Undefined step probe`, `12/14 tests run: 11 passed,
1 failed, 0 skipped`, and nextest exit `100`. The numbers reconcile with the
tree: the change delta lists 13 scenario units under `not @givn.removed`
(verified with `cargo nextest list`), 13 + 1 probe = 14, and nextest's default
fail-fast leaves 2 units unstarted after the failure. No `probe/` file remains.
The 12 non-E2E scenarios pass 12/12 when the feature runner is invoked
directly. Caveat: the declared `./run-tests.sh --non-e2e` does not reach them
because the format gate fails first (R1).

### F2 - The interaction matrix omits the 19 fragment interactions this change adds

`design.md:210` says "The change adds no new inventory entry. It repairs one
existing entry and leaves the other touched capabilities' entries unchanged."
The delta adds 19 rows to the fragment inventory
(`specs/fragments/fragment.md:30-63`), as `design.md:102` itself records. The
matrix at `design.md:212-226` has 13 rows and zero fragment rows, so 19
interactions the change introduces into the durable inventory are unmapped.
The design review's §6 "touched actions only" convention does not cover rows
the change itself adds.

Fix: add the 19 fragment rows to the Interaction Coverage Matrix, each with its
existing `@e2e` title and the CLI driver, or record the explicit scope decision
and correct `design.md:210`. A `design.md` edit requires re-running
design-review.

Resolution (`424682fa`): resolved by the explicit scope decision.
`design.md:212` replaces the false "adds no new inventory entry" claim with the
19 repaired declarations, states that their scenarios and steps are unchanged,
and points at the delta corpus. `design.md:216` is the summary row the finding
required. Verified: the delta adds exactly 19 interaction rows and removes none
(`givn/changes/enforce-spec-interaction-integrity/specs/fragments/fragment.md:32-63`);
the permanent table has 13 data rows
(`givn/specs/fragments/fragment.md:32-44`), so 13 + 19 = 32; the 19 added rows
span 10 capabilities; and all 19 scenario titles are existing `@e2e` scenarios
in the permanent fragment capability files (0 missing). The summary-row
examples are real `@e2e` scenarios
(`givn/specs/fragments/instruction-integrity.feature:50-51` and
`givn/specs/fragments/measure-coverage-per-project.feature`).
`givn lint --change enforce-spec-interaction-integrity` is clean. The design
re-review closure is signed at `design-review.md:2280`.

### F3 - Promised unit tests for the receipt target and the skip-merged branch are absent

`design.md:302` promises `target_rel` normalization, an outside-`givn/specs`
refusal, and receipt recomputation; `design.md:307` promises the skip-merged
branch; `design.md:299` promises the no-temporary-directory test. `tasks.md:10`
claims these tests exist. They do not:

- `src/cli/archive.rs:905-908` refuses a pending merge target outside
  `givn/specs`, but no unit test or scenario supplies such a receipt. This is
  external-input validation and classifies as missing test coverage.
- `src/spec/projection.rs:59-63` skips entries already in `merged_deltas`. The
  projection-relevant pending fixtures pass an empty `merged_deltas`
  (`tests/steps/archive_proof_steps.rs:674`); the non-empty receipt at
  `tests/steps/archive_proof_steps.rs:1405-1412` has `merge_completed: true`,
  so the archive skips the projection. The skip branch is untested.
- No test asserts that the finding `files` and `titles` use project-relative
  paths or that the projection creates no temporary directory
  (`design.md:299`).

Fix: add unit tests for the outside-target refusal, a non-empty `merged_deltas`
projection, and the no-write/no-temp property; record the batch commit in
`tasks.md`.

Resolution (`424682fa`): resolved. `src/spec/projection.rs:190`
`a_merged_entry_is_skipped_by_the_projection` passes a non-empty `merged_deltas`
and would report the unlisted delta scenario if the skip were absent;
`src/spec/projection.rs:207` `the_projection_writes_no_file` byte-snapshots the
fixture tree before and after; `src/cli/archive.rs:1921`
`receipt_target_outside_specs_is_refused` asserts `plan_from_receipt` rejects a
`../outside` target with `outside givn/specs`. Commands run: `cargo test --lib
projection::tests` -> 6 passed, 488 filtered out; `cargo test --lib
receipt_target_outside_specs_is_refused` -> 1 passed. `design.md:77` and
`tasks.md:11` now describe exactly these tests; the finding `files` relative
path is already pinned by `src/gherkin/interaction.rs:333`. The fix's own test
code is unformatted (R1).

### F4 - The implementation edits two permanent specs directly, contradicting tasks.md:9 and design.md:75

`tasks.md:9` says "No permanent spec is edited directly." `design.md:70,75`
lists the permanent corpus repair as the delta `fragments/fragment.md` and the
init `@e2e` tag only. `3a33fb39` also edits:

- `givn/specs/fragments/adr-qualification-gate.feature:122` changes the
  permanent assertion from "26 accepted decisions" to "27 accepted decisions".
- `givn/specs/verify-change/review-disposition-gates.feature:61,85,89` adds
  fixture `@e2e` tags and a permanent `other.feature` fixture.

Neither edit appears as a delta modification; the change's net delta is 12
added and 1 modified (the init scenario), so neither permanent edit is visible
in the delta review. The first is a permanent scenario assertion change, not
just test infrastructure.

Fix: move both edits into delta feature modifications for the owning
capabilities (with the base hash and `@givn.modified`), or record them as
reviewed permanent repairs in `design.md` and `tasks.md` and re-run
design-review. Make `tasks.md:9` true.

Resolution (`424682fa`): resolved by the second option, and re-reviewed.
`design.md:76` and `tasks.md:10` record both edits as reviewed direct test-spec
repairs and distinguish product specs (delta-only) from test specs. Verified:
the `@e2e` additions at
`givn/specs/verify-change/review-disposition-gates.feature:61,85` are inside
fixture docstrings, not tags on the permanent feature's own scenarios, and
`:89` declares the `givn/specs/other/other.feature` fixture that the scenario
writes under its temporary project root at runtime; no `givn/specs/other/`
exists. `givn/specs/fragments/adr-qualification-gate.feature:122` says 27
accepted decisions; ADR-075 is accepted and 27 `ADR-*.md` files exist. Because
the fixture tags do not tag permanent scenarios, default `givn lint` still
reports the expected 11 findings.

### F5 - The design's init step-file map is incomplete

`design.md:163` and `design.md:231` name only
`tests/steps/init_agent_target_selection_steps.rs` for the repaired init
scenario. The scenario's Then assertions are bound in
`tests/steps/initialize_a_project_e2e_steps.rs:107-145`, and its Given in
`tests/steps/mod.rs:327`; the named file hosts only the driving
`the consumer runs` step at `:193`. The implementation did not introduce a new
step, but the reviewed file map does not match the tree.

Fix: name the actual binding files in `design.md`'s Step Definitions table and
E2E Infrastructure section, then re-run design-review.

Resolution (`424682fa`): resolved. `design.md:165` names
`tests/steps/init_agent_target_selection_steps.rs` (driving step),
`tests/steps/initialize_a_project_e2e_steps.rs` (Then assertions), and
`tests/steps/mod.rs` (Given); `design.md:234` repeats the map in E2E
Infrastructure. Verified: `the consumer runs` at
`tests/steps/init_agent_target_selection_steps.rs:192-193`; the scenario Thens
at `tests/steps/initialize_a_project_e2e_steps.rs:105,119,131,137`; the Givens
at `tests/steps/mod.rs:327` and
`tests/steps/initialize_a_project_e2e_steps.rs:101`; the generic file Thens at
`tests/steps/mod.rs:1010-1011`. All three modules are declared at
`tests/steps/mod.rs:40-42`. No new step was introduced.

## Overlap dispositions

Every shape-match finding in the change's scan is listed below. `variant` marks
the two pairs where both scenarios are this change's scenarios and differ only
by direction; `boundary` marks a pair with a distinct capability, workflow
outcome, or fact set. No pair is merged or removed.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Change-scoped lint reports an E2E scenario the projected inventory does not declare` | `Change-scoped lint reports a declared interaction whose projected corpus has no E2E scenario` | variant |
| `Default lint reports an undeclared capability` | `Change-scoped lint reports a pre-existing inventory mismatch` | variant |
| `A structural subset is reported as an overlap` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `Archived scenarios are excluded from the audit` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `Lexical search reports an empty result for unknown terms` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `The transition is idempotent after the removal is committed` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `The dedicated check passes on a clean project` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `A proposal matching an existing capability surface recommends EXTEND` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `A proposal with no matching surface reports no signal` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `givn addons disable when already disabled is idempotent` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `The check passes on a complete README` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `Updated impact decision passes the review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A disabled addon needs no impact decision` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A recorded impact decision passes the review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `The upgrade reports rules that survive in reachable history` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `A parse error is reported on stderr and exits 1` | `Change-scoped lint reports a feature the projected inventory does not declare` | boundary |
| `A non-existent change id is not surfaced` | `Default lint reports an undeclared capability` | boundary |
| `A non-existent change id is not surfaced` | `Change-scoped lint reports a pre-existing inventory mismatch` | boundary |
| `A parse error outranks a @wip finding` | `Change-scoped lint reports a feature the projected inventory does not declare` | boundary |
| `A blocking finding renders its severity` | `Default lint reports an undeclared capability` | boundary |
| `A blocking finding renders its severity` | `Change-scoped lint reports a pre-existing inventory mismatch` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `Integrity hook passes when arc42 is disabled` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `Explicit retirement documents a changed behavior` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `Consumer runs migration coverage check and sees passed evidence` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `An opaque revision identifier passes the integrity check` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `Review accepts a current verification receipt` | `Change-scoped lint accepts a delta that keeps the projected inventory consistent` | boundary |
| `A recorded self-review is accepted` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `An independent review by a different context is accepted` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A legacy review record without a kind keeps its existing gate result` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `Review passes when a shape match exists only between permanent scenarios` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A modified delta matching its merge target in the same capability is allowed` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A removed delta referencing its target by title is allowed` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `The default scope excludes the archive tree` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `givn check with known artifact proceeds to run hooks` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A visual change with complete evidence passes review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A change without a visual contract is exempt from the visual gate` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A terminal change passes review with a transcript` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A change that declares no visual interface is exempt` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A web contract with complete state evidence passes review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A visual change with a confirmed persona disposition passes review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A deferral with an operator decision passes review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A change without a visual interface passes review with a reasoned none` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |
| `A machine-only CLI records not applicable and passes review` | `Review accepts a delta whose projected interaction inventory is consistent` | boundary |

## Split-or-keep

No scenario in the delta exceeds the 14-step threshold. The scan reports no
long-scenario finding involving the change; no split-or-keep decision is
required.

## README impact

README-IMPACT: none

The change updates `docs/reference.md`, `docs/workflow.md`, the glossary, and
the arc42 chapters. The README documents neither `givn lint` nor its exit
contract, so no README section is affected.

## Review closure

Targeted closure of F1-F5 against `424682fa` and the current tree. No full
re-review was performed; the earlier review stands except where a Resolution
line or this table says otherwise.

| Finding | Status | Closure evidence |
|---|---|---|
| F1 - strict-mode proof | Resolved | `tasks.md:8` records the executed probe; 13 change units + 1 probe = the recorded 14; no probe file remains; 12/12 non-E2E scenarios pass directly. |
| F2 - fragment matrix scope | Resolved | `design.md:212,216`; 19 added rows, 10 capabilities, 0 missing `@e2e` titles. |
| F3 - promised tests | Resolved | `src/spec/projection.rs:190,207`; `src/cli/archive.rs:1921`; 6 + 1 tests pass. |
| F4 - permanent test-spec repairs | Resolved | `design.md:76`, `tasks.md:10`; fixture tags/runtime fixture verified; ADR count 27 matches 27 ADR files. |
| F5 - init step map | Resolved | `design.md:165,234`; three binding files verified. |

The five recorded findings are closed. Three material regressions introduced by
the fix commit block the verdict.

### Remaining findings

#### R1 - `424682fa` fails the declared format gate

`cargo fmt --all -- --check` exits 1 on code added by the fix commit:
`src/spec/projection.rs:226` (the one-line `out.push` tuple) and
`src/cli/archive.rs:1940-1943` (the receipt-target assertion). `./run-tests.sh`
runs this gate first, so `./run-tests.sh --non-e2e`, the declared
`verify.command`, fails before clippy or the scenarios. The 12 scenarios pass
only when `cargo nextest run --test features_runner` is invoked directly. Fix:
run `cargo fmt` and commit the result.

#### R2 - The change verification receipt is stale

`verification/tasks-verification.json:5` records head `4d429d84` and `:10`
records `tested_content_digest 813edf7f...`; the current tree hashes to
`aad1b601...`. `cargo run -- check review --change
enforce-spec-interaction-integrity` exits 1 with `change verification receipt
is stale ... tested content changed`. The fix commit added the receipt but
generated it before its own source changes. Fix: after R1, re-run `givn check
tasks --change enforce-spec-interaction-integrity` (or the receipted
`./run-tests.sh --non-e2e`) and commit the regenerated receipt.

#### R3 - The design re-review closure breaks the single-context gate

`design-review.md:2247` records `Reviewer context: independent-review-1
(continuation of the reviewer context in the header; ...)`; `review.md:5`
records `independent-review-1`. `cargo run -- check review` fails with
`design-review.md and review.md must record one review context for an
independent review: '<long value>' vs 'independent-review-1'`. Fix: normalize
`design-review.md:2247` to `independent-review-1`.

### Non-blocking observations

1. `tasks.md:12` (`COMMIT: 424682fa -- Review fix: ...`) is uncommitted;
   `git status --short` shows `M tasks.md`, and `424682fa` does not contain the
   line. Commit it with the next batch.
2. `design-review.md:2269` cites the F1 evidence as `tasks.md:7`; the executed
   proof is `tasks.md:8` (line 7 is the promise). Citation drift only.
3. The F3 no-write test snapshots the fixture tree; it does not observe a
   transient system-temp directory. Static inspection shows no tempdir call in
   `src/spec/projection.rs`, and `design.md:77` states the narrower no-write
   claim; `design.md:302` still says "creates no temporary directory". Not a
   blocker for the three tests the finding named.

## Sign-off checklist

- Fabrication audit: F1-F5 resolved in Review closure; not clean until R1-R3
  are fixed.
- Every checked task has a verified commit touching production code: yes,
  `3a33fb39`; `check review` reports `✓ scenario commits passed (12 scenarios)`.
- Every promised component exists: yes.
- Every consumer uses only its declared contract: yes; no implementation import
  reaches around a declared boundary.
- Strict-mode proof present and passing: yes; the executed probe and its
  non-zero output are recorded at `tasks.md:8` (F1 resolved). The declared suite
  is separately blocked by R1.
- Scenario execution evidence per scope: the receipted 12/12 predates
  `424682fa` and is stale (R2); the 12 non-E2E scenarios still pass 12/12 under
  the direct runner, but the declared `./run-tests.sh --non-e2e` fails at the
  format gate (R1).
- Coverage classification uses the available evidence; no measured Cobertura
  report exists yet; the archive will carry the merged result.
- Every coverage gap classified and resolved: yes; the receipt-target,
  skip-merged, and no-write tests exist and pass (F3 resolved).
- Dead code deleted; missing tests added; hard-to-test gaps justified: yes;
  the three promised tests were added (F3 resolved); no missing tests remain
  unclassified.
- Redundant unit tests removed: none identified.
- No `@wip` tags remain: 0 in the delta.
- Canonical E2E policy read from the resolved specs instruction: every
  normalized action has one `@e2e` scenario; the new candidates are regular
  variants of existing actions.
- Every E2E scenario has a real-interface primary assertion and the reviewed
  driver: yes; the init repair uses the real CLI/PTY and asserts CLI output.
- Local run command starts the full stack: the run declaration passed; the
  product is a CLI with no server, database, or container.
- `verify.e2e_command` was read from `givn/commands.yaml`:
  `./run-tests.sh --e2e` -> `tests/features_runner.rs` under the `@e2e` filter.
  The tree was searched for a second init E2E implementation; none exists.
- E2E scope is selected explicitly: `run-tests.sh` derives `e2e` from the
  argument and exports `GIVN_RESULT_SCOPE` for the result contract; the change
  receipt records its own `change` scope.
- Implementation matches the design's commands, file layout, and framework:
  yes; `design.md:165,234` name every init binding file (F5 resolved).
- Interaction coverage verified: yes; the 19 repaired fragment declarations
  are explicitly scoped and summarized at `design.md:212,216` (F2 resolved).
- Review-fix regressions: R1 (format gate), R2 (stale receipt), and R3
  (design-review context mismatch) block the verdict.
- No finding was classified outside the three buckets.

REVIEW: FAIL
