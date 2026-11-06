# Design Review: enforce-spec-interaction-integrity

Review kind: independent
Author: authoring-context
Reviewer context: independent-review-1

## Review basis

Read in full: `proposal.md`, `arc42.md`, `design.md`, every delta `.feature` and the
`specs/fragments/fragment.md` narrative; the docs edits under `docs/arc42/` (04, 05,
06, 08, 10, 11, `adr/README.md`, `adr/ADR-075-*`), `docs/reference.md`,
`docs/glossary.md`.

Read in the tree to test the design's claims: `src/gherkin/interaction.rs`,
`src/gherkin/merge.rs`, `src/overlap/analysis.rs`, `src/cli/lint.rs`,
`src/cli/check.rs`, `src/cli/archive.rs`, `src/spec/mod.rs`,
`tests/features_runner.rs`, `run-tests.sh`, `tests/steps/spec_overlap_gates_steps.rs`,
`tests/steps/archive_proof_steps.rs`, `tests/steps/workflow_enforcement_steps.rs`,
`givn/specs/{verify-change,archive-change,initialize-project}/usecase.md`,
`givn/specs/fragments/fragment.md`, `docs/domain-model.md`, `docs/arc42/adr/adr-template.md`.

Facts established by execution, not by reading alone (re-established in the hardening pass):
`sha256sum givn/specs/fragments/fragment.md` = `a219ed71…`; the delta narrative's base
hash was `f8524c9c…` at the first pass and now declares `a219ed71…` (F1, resolved).
`cargo run -- lint` on the permanent corpus reports exactly 11 blocking interaction
mismatches (10 fragment capabilities + the non-`@e2e` init scenario), matching the proposal.
`cargo run -- lint --change enforce-spec-interaction-integrity` is structurally valid
(12 `@wip`, 12 findings, exit 2; the challenge added two lint scenarios). A harness binary
invoked directly rejects a bare unit name with `error: unexpected argument`; the nextest
invocation accepts it — see F7, corrected.

No question required an operator answer; every branch below was resolved by inspection.

---

## Grilling record

### 1. Scope — what the proposal asked for, and "done"

Q: Does the delta contain exactly the proposal's four capabilities and nothing extra?
A (recommended): Yes. Routing table and delta agree: `givn-workflow-enforcement` (2 review
scenarios), `check-specification-health` (3 lint scenarios), `archive-proof` (3 archive
scenarios), `initialize-a-project` (one tagged modification). The narrative repair of
`fragments` carries no scenario because no consumer action changes.
Resolution: **no issue found.** Out-of-scope clauses are respected: no merge-algorithm change,
no receipt-schema change, no new `missing-e2e-runner` command, no shape-match repair
(`design.md:225-246` vs `proposal.md:40-49`).

Q: Is the repaired corpus complete — do the 19 new fragment rows cover every `@e2e`
scenario of the 10 drifting capabilities, and does the init tag repair close the 11th?
A: Yes, verified by extraction: 32 declared rows against 32 actual `@e2e` titles, zero
capability mismatches, zero removed rows, capability list unchanged (21). The init delta
body is step-for-step identical to `givn/specs/initialize-project/initialize-a-project.feature:113-122`
and adds `@e2e`; `DeltaOp::Modified` replaces the scenario block, so the tag lands
(`src/gherkin/merge.rs:130-137`).
Resolution: **no issue found for completeness.** Base-hash validity is a separate finding
(F1).

### 2. Tech choices

Q: Is a temporary directory copy plus the existing merge primitives the right mechanism,
or does a simpler design satisfy the same observable behaviour?
A: Yes. The alternative — re-deriving the merged scenario set in memory — would duplicate
merge semantics, exactly the divergence risk `design.md:219` names. Copying the corpus and
reusing `gherkin::merge`/`MergeEntry` keeps one implementation. `spec::merge_plan` is the
right layer: the plan build carries change/routing knowledge (`routing_usecase_for_new_capability`,
`resolve_permanent_dir`, `spec::owner`) that does not belong in `gherkin::merge`.
Resolution: **no issue found.** Verified `tempfile` is a runtime dependency
(`Cargo.toml:18`), not only a dev-dependency.

### 3. Projection reachability: delta narrative, delta feature, and merge semantics

Q: Can the projection see both the delta narrative and the delta feature?
A: Yes. `MergeEntry` already carries the parsed `gherkin::Feature` (`src/cli/archive.rs:362-368`),
and narrative documents are collected by path and written verbatim (same functions, moved).
The design's order — narrative first, then entries (`design.md:95-98`) — matches
`apply_merge_plan`'s `merge_usecase_documents`-then-entries order (`src/cli/archive.rs:1002`,
`1010-1030`), so a `NEW in <usecase>` capability still gets its directory from the narrative
or from `merge_file`'s own `create_dir_all` (`src/gherkin/merge.rs:278-281`).
Resolution: **no issue found.**

Q: Does the design duplicate merge semantics anywhere?
A: No for the plan/apply path; `spec::merge_plan` owns plan build, narrative copy,
`merge_file`, and retirement cleanup, and `retire_capability`'s receipt/print separation is a
primitive split, not a second algorithm.
Resolution: **no issue found**, except the resume replay defect (F2).

### 4. Specifications root for `gherkin::interaction`

Q: Can the gate take a specifications root without breaking `overlap::analysis::scan_tree`
or the change-scoped filter?
A: Yes. `interaction_gate_findings` is called from exactly one place,
`src/overlap/analysis.rs:118`, which must pass `root.join("givn/specs")`. `usecase_coverages`
has no other consumer in the repository. Finding file paths already derive from the same
root (`src/gherkin/interaction.rs:153-161`), so the stream stays project-relative for the
default scan and becomes temp-relative only inside the projection, which maps it back
(`design.md:99-100`).
Resolution: **no issue found for the refactor.** The in-module tests that pass a project root
are a dependent to update (F4).

### 5. Archive call placement

Q: Are the three archive calls before the right mutations and after the right merge state?
A: Fresh: after `build_merge_plan` (`src/cli/archive.rs:173`) and before
`ensure_e2e_configured` (`:179`), i.e. before the pending receipt (`:186`) and the rename — correct.
Merged candidate: after the `if fresh/else` block and before the scopes (`:275`) — correct,
and it fires on both entry paths.
Resumed-before-merge: placed before `apply_merge_plan` (`:254-258`) — right place, wrong
state handling.
Resolution: **finding F2** (replay of already-merged entries) and **finding F6** (assertions
that do not pin the placement).

### 6. Interaction Coverage Matrix

Q: Are all touched capabilities' inventory entries present as rows?
A: Yes for every touched capability: `check-specification-health` 1/1,
`givn-workflow-enforcement` 5/5, `archive-proof` 4/4, `initialize-a-project` 3/3
(`v-verify-change/usecase.md:91-104`, `archive-change/usecase.md:123-135`,
`initialize-project/usecase.md`). Tested the strict reading ("every entry of every touched
use-case document" would need 26 rows): project convention is touched actions only, as in
`givn/archive/add-coverage-readme-example/design.md`, `add-javascript-outline/design.md`.
The `fragments` use case is narrative-repaired with no action change and its 32 rows are
correctly not duplicated.
Resolution: **no issue found.**

### 7. Testability

Q: Can every scenario fail RED for the claimed reason?
A: Yes for the five refusal/acceptance pairs: before the feature, the two lint refusals exit 0,
`Review refuses` passes review, `Archive refuses … before moving` moves and proves, and
`Archive refuses a merged candidate` proves. The three positive scenarios (`Change-scoped
lint accepts …`, `Review accepts …`, `Archive publishes …`) pass today and cannot go RED;
they are anti-over-blocking guards paired with the refusal scenarios, which is the accepted
guard pattern. Fixture phrases are implementable and collision-free: the archive-ready and
unproven-candidate Givens already exist (`archive_proof_steps.rs:20`, `:1466`) and the review
baseline exists (`workflow_enforcement_steps.rs:372-388`); the new phrases are anchored
regex/strings, and that fixture already documents that a delta-writing step must drop its
`Behavior change: none` declaration and `.givn-skip specs` (`workflow_enforcement_steps.rs:390-410`).
Resolution: **no issue found except F7.** Strict mode (`.fail_on_skipped()`,
`tests/features_runner.rs:55`) and a concrete not-implemented pattern (undefined steps fail)
are present; the single-scenario command is wrong.

Q: Does the design name a single-scenario run command that works?
A: No. `design.md:10` omits `--exact`; `design.md:184` has the correct form. Running the
harness with a bare unit name fails with `error: unexpected argument`, because the
name-predicate branch requires `--exact` (`tests/features_runner.rs:47-52`).
Resolution: **finding F7.**
Correction (hardening pass, executed): the failing invocation was the direct binary, not
nextest. Nextest passes `--exact` to the harness itself, so the documented nextest form runs
the scenario. See F7 and Hardening verification; the first pass's claim was inverted.

### 8. E2E fidelity

Q: Is the normalized action scope, interface type, driver, and primary-assertion rule applied?
A: Yes. The change adds no user action; each matrix row maps to the one existing `@e2e`
scenario per action, driven through a real subprocess or PTY (`design.md:157-179`). The
repaired init scenario's primary assertions read CLI output ("the command exits 0 and reports
installed skills and disabled optional addons"), not repository state. `verify.e2e_command`
is configured (`givn/commands.yaml`), and no `@e2e` tag is removed anywhere; one is restored.
Resolution: **no issue found.**

### 9. Risk

Q: What is the most likely implementation failure, and is it mitigated?
A: Not divergence of the happy path — that is mitigated by one plan/apply implementation.
The likely failure is the projection's state model on the recovery paths (F2) plus the
narrative mechanics of the repair itself (F1, which the design got wrong in its own delta).
Both are concrete, cheap fixes; a behavior-preserving `spec::merge_plan` extraction in
batch 1 is the right first step (`design.md:257-261`).
Resolution: **recorded; F1 and F2 are the required mitigations.** Also
`design.md:220` claims "a scenario asserts the absence of `/tmp` in the output"; no delta
scenario asserts it (F9).

### 10. Use-case and Persona context

Q: Do the use-case identities, contracts, ownership, and Persona coverage hold, and does any
rejection criterion fire?
A: No criterion fires. `verify-change` already carries the interaction-coverage rule
(`usecase.md:47`), `archive-change` already declares that gates run "all before any mutation"
(`usecase.md:29`), so the new refusal is inside the existing contract rather than a new goal.
Capability ownership is unchanged, `fragments` stays the durable home, and no Persona is
invented or promoted: the proposal confirms `developer--primary-user` and
`agent--technical-role`, both exist under `givn/personas/` and both are listed by the two use
cases; the design records their relevance ("read CLI refusal text") and evidence.
`givn-maintainer--owner` is confirmed by the use cases but not by this change; that is a
narrowing, not a promotion, so no edit is required.
Resolution: **no issue found.**

### 11. Decision ledger

Q: Does every proposal question resolve in the design ledger?
A: Yes. D1, D2, D3 each have a `Decides D<n>` entry with the rejected alternative and the
realization point (`design.md:69-71`); the proposal records no seed, so no `(seed Q<n>)`
reference is owed. `No decision is carried` (`design.md:73`) is consistent with the design:
the projection component and the merge-plan split are current requirements, not carried.
Resolution: **no issue found.**

### 12. Contracts and components

Q: Does each touched consumer use case have one contract with exactly one implementing
component, and are the contract rows consumer-accurate?
A: The split is sound: `spec::projection` has a named responsibility and two consumers,
`spec::merge_plan` removes the archive→library inversion, and no consumer imports an
implementation. One row is wrong: "Interaction inventory findings" lists `cli::lint` and
`cli::archive` as consumers, but after this design both reach the gate only through
`spec::projection`; the direct consumers are `spec::projection` and `overlap::analysis`
(`src/overlap/analysis.rs:118`).
Resolution: **finding F8.**

### 13. Decision timing

Q: Is any foundational question carried, or is any abstraction speculative?
A: No carried question, no scenario depending on an unanswered one, no contract treated as
open, no speculative parameterization (the projection has two real consumers today).
Resolution: **no issue found.**

### 14. Visual design contract and interface classification

Q: Is omitting `## Visual Design Contract` correct, and does the opposite classification hold?
A: Yes. The gate exempts a design without the section
(`src/integrity_hook.rs:279-284`), and the change introduces or visibly changes no screen;
the changed surface is CLI refusal text and documentation. The opposite classification fails
for a stated reason: nothing renders HTML/CSS/JS, and the acceptance evidence is a real
subprocess/PTY, so a browser driver is required by neither the system type nor the scenarios
(`design.md:87`, `173-179`).
Resolution: **no issue found.** Deferrals: none.

### 15. ADR qualification (ADR-075)

Q: Does the candidate qualify, and is the cheaper home named?
A: It qualifies. The decision fixes a component/ownership boundary
(`spec::projection`, merge planning out of `cli::archive`) and a dependency direction, and
constrains future gates and consumers — so falsification examples 1 and 2 do not apply, and
example 3 does not apply because a component boundary is moved and stated. Three materially
different options with an operator decision (D1) satisfy "real alternatives"; reversal changes
the gate contract on three commands plus downstream corpora. The cheaper home is named
concretely: the algorithm, temporary-copy mechanics and finding text are assigned to
`design.md`/`src/spec/projection.rs`/the delta features, with the durable
acceptance/publication boundary explicitly not owned there (`arc42.md:53-54`). The ADR's
`## Qualification` block duplicates the assessment, but the canonical ADR template mandates
that block (`docs/arc42/adr/adr-template.md:31-52`), so it is not a violation and this review
does not demand its removal. A refinement (`AMEND_ADR` on ADR-028) would be wrong: ADR-028
owns hook/mandatory-check separation, not a projected corpus predicate.
Resolution: **qualified — one record gap (F10).**

### 16. Architecture documentation (arc42)

Q: Does the 12-row assessment match an independent walk, including omissions?
A: Independent walk: (1) No — no new stakeholder or top-level goal; the quality attribute is
row 10. (2) No — no legal/technical/organizational constraint is added; the gate is workflow
policy, recorded in 8 and 10. (3) No — no new external system or interface *set*; the change
adds a refusal inside existing commands, and §3.2's `givn check review` row stays true because
the projection lives inside change-scoped lint. (4) Yes. (5) Yes. (6) Yes. (7) No — one
binary. (8) Yes — new validation concept and corrected exit-code contract. (9) Yes —
ADR-075. (10) Yes — Q-8 correction and Q-125. (11) Yes — R-135/R-136. (12) Yes — two new
glossary terms. This matches `arc42.md:5-18` row for row, so no omission was found.
Chapter-content check: 04/05/06/08/10/11 diffs match `design.md` (pre-mutation refusal before
receipt/rename, post-merge recheck before scopes, `1` for use-case/interaction, `spec::merge_plan`
and `spec::projection`). Chapter 09 is a register index that delegates to `adr/README.md`, and
the ADR-075 row and relationship row are present there; the "27 accepted records" count matches
the register. All twelve chapter files exist with real content, no placeholder text, no
box-drawing characters and no pseudo-diagrams (all `+`/`|` blocks found are Markdown tables).
Chapter 12 remains the intended link to the standalone glossary. The two "Bad, because…"
consequences are reflected: drift → R-135; copy cost/new component → recorded as a deliberate
tradeoff in `design.md:222` rather than debt, which the instructions permit when nothing
actionable remains.
Resolution: **no omission found; F10 on the qualification evidence wording.**

### 17. Ubiquitous language and structural model

Q: Are the new terms in `docs/glossary.md`, and was the structural model assessed?
A: Terms are present and used consistently: "Interaction inventory" and "Projected interaction
inventory" (`docs/glossary.md:88-89`), including the anti-term that separates the design matrix
from the inventory, which this very design respects. Structural model: `docs/domain-model.md:5-7`
explicitly excludes internal components, so no domain-model edit is correct — but the change
records no assessment at all.
Resolution: **finding F11** (record the assessment; do not edit the model).

---

## Findings — required edits

**F1 (blocking). Stale narrative base hash — the corpus repair cannot land.**
`givn/changes/enforce-spec-interaction-integrity/specs/fragments/fragment.md:1` declares
`<!-- givn:base-sha256:f8524c9ccf8cf30aef23e946ff6fb20022944d466831e9551f72fde143e7a002 -->`.
`src/cli/archive.rs:1749-1763` compares that value with `content_hash(current permanent)`, which is
`a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65fe14774878c22b` (`f8524c9c…` is the hash of
`git show 94943412:…`, i.e. the content before `6dc3b904`). The archive therefore refuses with
"stale narrative replacement" before the projection runs, and the 19-row repair never merges — the
change's own corpus repair is unachievable as written.
Required change: set line 1 of the delta narrative to
`<!-- givn:base-sha256:a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65fe14774878c22b -->`,
and add a line to `design.md:62` (`## Data Model`) naming the base-hash refresh as part of the repair.

**F2 (blocking). Resumed-before-merge projection replays already-merged entries.**
`design.md:124` projects `plan_from_receipt`'s full plan with strict merge
(`design.md:97` → `merge_file`). A candidate with `merge_completed: false` may already hold
applied entries: `apply_merge_plan` skips `receipt.merged_deltas` and uses `merge_file_retry`
for `merge_in_progress` (`src/cli/archive.rs:1010-1027`). The projection has neither, so it fails
with "cannot add scenario … title already exists" instead of a finding. That contradicts
`design.md:255` ("the existing archive proof suite remains green, including the interrupted-merge
… scenarios") and the permanent scenarios `An interrupted merge resumes from its pending candidate`
and `An in-flight delta file is retried without duplicating its scenarios`
(`givn/specs/archive-change/archive-proof.feature:266-283`).
Required change: state that the projection applies only entries not recorded in
`merged_deltas`, or applies entries in retry mode; either way, pass the applied/in-flight state
into the projection (e.g. `interaction_findings_for_plan(project_root, change_specs, plan, receipt)`),
and say so at `design.md:91-103` and `:124`.

**F3 (blocking). The projection is skipped when the delta has no feature files.**
`design.md:113` runs the projection inside the tree-scan block, but
`src/cli/lint.rs:78-92` returns `Ok(0)` before that block when the change's specs tree has no
`.feature` files. A narrative-only delta (for example a delta that only adds or edits an
interaction row, which is exactly the fragment repair's shape) is never projected, so
`givn lint --change` and `givn check review` pass while the projected corpus declares an
interaction with no `@e2e` scenario. That breaks the D1 option-1 promise that review blocks.
Required change: place the change-scope projection before the empty-feature early return (or
exclude change scope from it), and state the behaviour at `design.md:111-118` plus one row in
`## External Boundary Cases` (`design.md:203-214`) for "delta contains no feature files".

**F4 (blocking). The finding-message change breaks or hollows existing evidence, and the design
understates it.**
`design.md:63-65`, `:151` and `:254` name one affected assertion. The new title drops the
declared/actual counts, so:
- `tests/steps/spec_overlap_gates_steps.rs:743` (`wrong_capability_scenario`) fails on
  `line.contains("0 @e2e scenario(s)")` — that phrase no longer exists. It must be re-anchored
  (for example `missing @e2e scenario 'Consumer submits an order'`) while keeping the permanent
  rule that a same-titled scenario in another capability is not evidence
  (`givn/specs/verify-change/check-specification-health.feature:403-410`).
- `src/gherkin/interaction.rs:327` asserts `missing declared scenarios`; the three in-module
  tests (`:303-339`) pass a project-root fixture to `interaction_gate_findings`/`usecase_coverages`
  and must be re-rooted to the new specifications-root contract.
- `tests/steps/spec_overlap_gates_steps.rs:750-751` assert the absence of the old plural labels
  and become stale (the surviving `!text.contains("interaction")` at `:752` still detects a
  finding, so this is hygiene, not a hole).
Required change: list every dependent explicitly in `design.md:249-255` and name the replacement
assertion in `## Step Definitions` (`design.md:151`).

**F5 (blocking). Missing scenario for the deliberate "no baseline subtraction" rule.**
`design.md:107` and R-135 make it a deliberate contract that pre-existing drift blocks review and
archive. No scenario pins it: all three lint scenarios and both review scenarios have the mismatch
introduced by the delta, so an implementation that subtracts permanent findings passes every
scenario while silently re-creating the false green this change exists to remove.
Required change: add one `@givn.added @wip` scenario to
`givn/changes/enforce-spec-interaction-integrity/specs/verify-change/check-specification-health.feature`
— Given a permanent fragment whose inventory declares a scenario its feature does not tag `@e2e`,
And a delta that is itself consistent, When `givn lint --change <id>`, Then exit `1` and stderr
contains the mismatch title. This is a distinct rule and distinct Given from the existing three.

**F6 (blocking). Two new scenarios do not pin what the design claims.**
- `specs/archive-change/archive-proof.feature:6-13` asserts only "the change directory still
  exists". That is also true if the pending receipt was already written into the active change, so
  the scenario cannot distinguish the promised placement (`design.md:123`, proposal D1 gain "no red
  archive candidate"). Add `And no archive candidate exists for "archive-projection"`
  (step exists: `tests/steps/archive_proof_steps.rs:842`).
- `specs/archive-change/archive-proof.feature:24-30` asserts only the receipt status. An
  implementation that runs the verification scopes and only then refuses also leaves the candidate
  pending. Add an assertion that no scope ran (mirror
  `tests/steps/archive_proof_steps.rs:1915-1920`: `scope_evidence` and `scopes` have no `regular`
  entry) — the fixture primitives already exist.

**F7 (blocking under Testability). Wrong single-scenario command.**
`design.md:10` gave `cargo nextest run --test features_runner '<feature>::<title>'`; the first
pass claimed this fails because the harness only takes the name-predicate branch when `--exact`
is present (`tests/features_runner.rs:47-52`).
Correction (hardening pass, executed): the claim was inverted. Nextest selects the unit and
invokes the harness with `--exact`, so the bare nextest form runs the scenario. Passing
`--exact` to nextest itself is rejected, and the direct binary invocation (the first pass's
observed `error: unexpected argument`) is not the documented command. `design.md:9-15` now
states all three behaviours correctly and `design.md:197` repeats the working form.
Status: resolved; no edit remains.

**F8 (minor). Contract consumers are wrong.**
`design.md:38` lists `cli::lint` and `cli::archive` as consumers of "Interaction inventory
findings". After this design they consume `spec::projection`; the direct consumers are
`spec::projection` and `overlap::analysis` (`src/overlap/analysis.rs:118`).
Required change: correct the Consumers column (and keep the split trigger honest).

**F9 (minor). A claimed mitigation names a scenario that does not exist.**
`design.md:220` says "a scenario asserts the absence of `/tmp` in the output"; no delta scenario
asserts that. The Test Plan already has the right vehicle (`design.md:235`, projection unit test).
Required change: point the mitigation at that unit test, or add the assertion to a scenario.

**F10 (minor). The ADR qualification record omits the archived-register search.**
`arc42.md:55-56` says "searched the active register"; the qualification requires the active and
archived registers. `docs/arc42/adr/archive/` holds the superseded semantic-engine records.
Required change: one line in `arc42.md` naming both registers and stating that the archived
records are disjoint from interaction/mandatory-check/archive-publication decisions.

**F11 (minor). No structural-model assessment is recorded.**
No artifact says whether `docs/domain-model.md` is affected. `docs/domain-model.md:5-7` excludes
internal components, so the correct outcome is "unchanged", but it must be recorded.
Required change: one line in `design.md` (`## Data Model` or `## Architecture Impact`) stating
that `docs/domain-model.md` is unchanged because the change adds no domain concept or
relationship and the model excludes internal components. Do not edit the model itself.

---

## Hardening verification

Re-read after the author's hardening: `proposal.md`, `arc42.md`, `design.md`, all four delta
`.feature` files, the `specs/fragments/fragment.md` narrative, the docs edits under
`docs/arc42/` (04, 05, 06, 08, 10, 11, `adr/README.md`, `ADR-075-*`), `docs/reference.md`,
`docs/glossary.md`, and the code the design touches. Commands were re-run; the evidence below
is from this pass, not from the first pass.

| Finding | Status | Evidence |
|---|---|---|
| F1 | Resolved | Delta `specs/fragments/fragment.md:1` declares `a219ed71…78c22b`; `sha256sum givn/specs/fragments/fragment.md` returns the same value; `content_hash` (`src/cli/archive.rs:1674-1679`) hashes the whole text read at `:1755`, header included; `design.md:69` records the refresh as part of the repair. |
| F2 | Resolved | `design.md:96-104` defines `MergeProgress<'_> { merged, in_progress }` and applies skip-merged, retry-in-progress, strict-otherwise; `:130` passes `MergeProgress::empty()`, `:131` passes the receipt's `merged_deltas` and `merge_in_progress`; mirrors `src/cli/archive.rs:1010-1029`. |
| F3 | Resolved | `design.md:120` places the projection before the empty-feature return and states the narrative-only case; `:121-122` set the counters and exit `1`; `:249` adds the unit test. Early return is `src/cli/lint.rs:78-92`. |
| F4 | Resolved | `design.md:66-68` keeps `declares <N> interaction(s) and has <M> @e2e scenario(s)`; `:158` names `spec_overlap_gates_steps.rs:743`, `:750-751`, the in-module tests and their re-rooting, and the new fixtures; `:270-272` repeat the dependents. |
| F5 | Resolved | `specs/verify-change/check-specification-health.feature:31-38` is `@givn.added @wip` with the pre-existing-mismatch Given pair; `design.md:258` lists it. |
| F6 | Resolved | `specs/archive-change/archive-proof.feature:14` asserts `no archive candidate exists` (step `tests/steps/archive_proof_steps.rs:843-851`); `:32` asserts `no verification scope ran for the candidate` (new phrase `design.md:162`, receipt `scopes`/`scope_evidence`). |
| F7 | Resolved (corrected) | Executed: `cargo nextest run --test features_runner '<unit>'` → `PASS`, 1 test run, 1 passed, 1015 skipped, exit 0; nextest with `--exact` → `error: unexpected argument '--exact' found`, exit 2; direct binary → `error: unexpected argument '<unit>' found`, exit 2. `design.md:9-15` states exactly this. |
| F8 | Resolved | `design.md:41` consumers are `spec::projection`, `overlap::analysis`; the only other call site is `src/overlap/analysis.rs:118`. |
| F9 | Resolved | `design.md:226` and `:233` point the `/tmp` mitigation at the projection unit test; `:248` names that test. |
| F10 | Resolved | `arc42.md:61` names the active register and `docs/arc42/adr/archive/` and states the archived records are disjoint; `docs/arc42/adr/ADR-075-projected-interaction-integrity.md:33-36` repeats both; `docs/arc42/adr/README.md:62,78` register the ADR and its relationship. |
| F11 | Resolved | `design.md:70` and `arc42.md:32-34` record the assessment; `docs/domain-model.md` is absent from `git status` and still excludes internal components. |

Detail behind the compressed evidence:

- F1. The permanent file's own first line still carries `f8524c9c…`; that line is part of the
  bytes the archive hashes, which is why the correct delta value is the hash of the file as it
  stands, `a219ed71…78c22b`. The delta narrative mentions no other hash, and its 32 interaction
  rows still equal the 32 permanent fragment `@e2e` titles exactly (set comparison: zero
  missing, zero extra), so the refresh did not disturb the repair content.
- F2. `apply_merge_plan` retries an entry when it is `merge_in_progress` and skips
  `merged_deltas` (`src/cli/archive.rs:1010-1027`); the projection's three-way rule is the same.
  The fresh call site is after `build_merge_plan` and before `ensure_e2e_configured`
  (`:173`/`:179`); the resumed call site is before `apply_merge_plan` (`:254-258`); the
  merged-candidate path does not project (`design.md:98`) and runs the permanent gate after the
  merge block (`:273`) and before the scopes (`:275`).
- F3. `design.md:120` is the operative sentence: the change-scope projection runs for
  `paths.is_empty() && change.is_some()` before the early return, and "the empty-feature
  shortcut applies only to the parse/scan phase". The boundary-case row requested by the
  original finding's second half is not present in `## External Boundary Cases`; the boundary
  case itself is recorded at `:120` and pinned by the unit test at `:249`, so the behaviour is
  recorded, in the enforcement section rather than the boundary list.
- F4. Feasibility re-checked against the tree: `render_prose` prints `finding.titles[0]`
  verbatim to stderr for this kind (`src/cli/lint.rs:616-626`), so the new leading
  `interaction inventory mismatch:` phrase is reachable by the new `stderr contains` steps.
  `wrong_capability_scenario` (`tests/steps/spec_overlap_gates_steps.rs:736-745`) selects the
  `capability 'place-order'` line and asserts `0 @e2e scenario(s)` at `:743`, which the new
  format keeps; the stale phrase assertion is the next line, `:744` (the design cites `:743`
  for the re-anchor — one line off, same function, no ambiguity). `missing_e2e_evidence`
  (`:725-735`), the absence assertions at `:750-751`, and `src/gherkin/interaction.rs:327` are
  the only other textual consumers, and `:158`/`:270-272` name them all.
- F6. The merged-candidate Given (`tests/steps/archive_proof_steps.rs:1466`) is the existing
  unproven-candidate fixture, so `no verification scope ran for the candidate` can read the
  pending receipt's empty `scopes` and `scope_evidence` maps, the same primitives used at
  `:1915-1927`.
- F7. The first pass's statement was inverted, not merely imprecise. The executed evidence:
  `cargo nextest run --test features_runner
  'verify-change/check-specification-health.feature::All clean .feature files report a clean
  summary on stdout'` printed `PASS [1.272s] (1/1)` and `Summary [1.273s] 1 test run: 1
  passed, 1015 skipped`, exit 0. `cargo nextest run --test features_runner --exact '<unit>'`
  printed `error: unexpected argument '--exact' found`, exit 2. The direct binary
  `target/debug/deps/features_runner-… '<unit>'` printed `error: unexpected argument '<unit>'
  found`, exit 2, which is the failure the first pass misattributed to nextest. The author's
  correction is accepted.

Plan-integrity checks after the hardening:

- The delta parses. `cargo run -- lint --change enforce-spec-interaction-integrity` reports
  `4 file(s) checked, 11 finding(s)`, exit 2, no parse error.
- The `@wip` count is expected: 11 (6 `check-specification-health`, 2
  `givn-workflow-enforcement`, 3 `archive-proof`); the modified `initialize-project` scenario
  carries `@e2e`, not `@wip`, and 11 findings equal 11 `@wip` scenarios.
- The interaction matrix still covers the touched capabilities: 13 rows match the
  `## Interactions` entries exactly — `check-specification-health` 1,
  `givn-workflow-enforcement` 5, `archive-proof` 4, `initialize-a-project` 3.
- The 12-row arc42 assessment is unchanged and still matches the independent walk in §16.
- `cargo run -- lint` on the permanent corpus still reports exactly 11 blocking interaction
  mismatches (10 fragment capabilities + `initialize-project`), matching the proposal.
- No `@e2e` tag was removed: the init delta adds `@e2e` to a body step-for-step identical to
  `givn/specs/initialize-project/initialize-a-project.feature:112-122`, and `givn/specs` is
  unmodified in the working tree.

Observations (non-blocking; they do not reopen F1-F11):

- O1. `apply_merge_plan` also retries every entry when the receipt records no merge plan, no
  merged deltas, and no in-progress file (`src/cli/archive.rs:1003-1008`, `:1018`).
  `MergeProgress` models only the recorded branches, so a legacy pending receipt would project
  with strict `merge_file` and can fail with a duplicate-title error instead of resuming. No
  scenario or unit test covers a legacy pending receipt, so no executable contract is at stake;
  if that recovery must keep working, carry the flag into `MergeProgress` during
  implementation.
- O2. `src/cli/lint.rs:91` returns `Ok(0)` on the empty-feature path. The implementation must
  return the blocking code when projection findings exist, per `design.md:120-122`. The design
  states the outcome; the branch rewrite is an implementation obligation.
- O3. The `## External Boundary Cases` row requested by F3 is absent; the case is recorded at
  `design.md:120` and pinned at `:249`. Adding the row is a consistency improvement, not a gap.

## Challenge-driven hardening

This is the second hardening pass: H1-H3, M1-M7, and the minor items from the challenge.
The first-pass F1-F11 record above is unchanged. The earlier O1-O3 observations are closed
by this pass: O1 by `retry_unrecorded` (`design.md:72`, `:102`), O2 by the projection before
the empty-feature return (`design.md:126`), and O3 by the boundary row at `design.md:239`.

| Item | Status | Evidence |
|---|---|---|
| H1 | **Resolved** | `design.md:154` names `archive_proof_support.rs`, `enforce_post_merge_e2e_gate_steps.rs`, and `review_disposition_gates_steps.rs`; `:306` repeats them in the regression checks; `:260` and `:312` make the shared-fixture repair part of risks and batch 1; `:156` states the repaired `ready_change` corpus. The fixture shape is fixable: `write_change_artifacts` (`tests/steps/archive_proof_support.rs:179-243`) and `write_permanent_usecase` (`tests/steps/enforce_post_merge_e2e_gate_steps.rs:354-367`) build the narrative, and the parser accepts `- none` (`src/gherkin/usecase_md.rs:777-779`). The new archive scenarios use the repaired `ready_change` (`tests/steps/archive_proof_support.rs:430-451`). See H2 for `write_fixture_usecase` and the H4 final verification for the completed writer set. |
| H2 | **Resolved** | `design.md:154` (Step Definitions), `:158` (fixture-repair paragraph), `:260` (Risks), `:306`/`:308` (Regression checks), and `:312` (Batch plan) all name `tests/common/mod.rs::write_fixture_usecase`; those locations state rows from the actual `@e2e` titles with `- none` when none. `:127` and `:308` condition the existing-scenario greenness on that repair, not on the `NEW in` routing alone. Implementability and sufficiency verified: see "H2 verification" below. |
| H3 | Resolved | Reverse check in `design.md:27`, `:73`, `:80`, `:116`; new scenario `check-specification-health.feature:27-37`; docs `08` 8.3c and 8.24 (`docs/arc42/08-crosscutting-concepts.md:164-183`, `:1001-1010`), `docs/reference.md:16`, `docs/workflow.md:131-132`, `docs/glossary.md:88-90`, and R-137 (`docs/arc42/11-risks-and-technical-debt.md:124`). An independent scan of `givn/specs` found zero feature files absent from their document's `## Capabilities` list. |
| H4 | **Resolved** | `tests/steps/readme_enforcement_steps.rs` and `tests/steps/grouped_merge_steps.rs` are named in all five required locations: `design.md:154`, `:160`, `:260`, `:306`, and `:312`. The repair rule is stated at each (`:154`, `:160`, `:260`, `:306`, `:312`), and the retirement narrative rule at `:162` and `:248`. The audit obligation covers every current and future fixture writer at `:160` and `:312`. The complete re-scan finds no unnamed writer that feeds a projected corpus. See "H4 final verification" below. |
| M1 / O2 | Resolved | The scenario now says `a change "projected-change" with no delta features` (`check-specification-health.feature:65`); `design.md:126` places the projection before the empty-feature return and `:246` records the boundary case. `src/cli/lint.rs:77-92` is the branch to move. |
| M2 / O1 | Resolved | `MergeProgress` carries `retry_unrecorded` (`design.md:72`, `:102`); the resumed path passes `legacy_unrecorded_merge` (`:138`); every entry uses `merge_file_retry` when true (`:108`, `:247`); the unit test is at `:279`; the risk mitigation names legacy-retry at `:257`. This mirrors `apply_merge_plan` (`src/cli/archive.rs:1011-1027`). |
| M3 | Resolved | `docs/glossary.md:103` (Lint exit precedence and projected inventory), `docs/workflow.md:131-132`, `docs/reference.md:16`, and `docs/arc42/08-crosscutting-concepts.md:1001-1010` include the use-case/interaction code and undeclared capabilities. No stale `3` authority for invalid use-case documents or interaction mismatches remains in those files. |
| M4 | Resolved | The release note names "drift that predates the change" (`proposal.md:7`); R-135 (`docs/arc42/11-risks-and-technical-debt.md:122`) and the design risk (`design.md:257`) say the release note names the drift and the proposal/design record the repair. Neither claims the release note records the repair. |
| M5 | Resolved | `design.md:154-156` says the repaired `ready_change` corpus lets the unproven-candidate fixture create its candidate cleanly and the merged-candidate scenario inject the mismatch afterwards; `:306-308` repeats the fixture repair and green checks. The positive guard is feasible: `ready_change(..., false, ...)` now yields `- none`, and the new "with its interaction row" step adds the row; the candidate fixture uses `with_e2e=true` and the repaired row for `Demo e2e scenario` (`tests/steps/archive_proof_steps.rs:1433-1464`). The injection step must commit before the resume witness; `design.md:253` records the witness ordering. |
| M6 / O3 | Resolved | `design.md:246` has the "Delta contains no feature files" boundary row. |
| M7 | Resolved | `arc42.md:67` and `docs/arc42/adr/ADR-075-projected-interaction-integrity.md:33-37` name ADR-019 in the active-plus-archived search and explain that it delegates the lint contract to `givn/specs/verify-change/` and does not own the projection boundary. ADR-019 is in `docs/arc42/adr/archive/ADR-019-lint-contract.md` and the register (`docs/arc42/adr/README.md:68`, `:102`). |

Minor items:

- Contracts consumers no longer list `cli::check` as a direct projection consumer; the
  Projected-interaction-inventory row lists `cli::lint` and `cli::archive`, and the
  finding row lists `spec::projection` and `overlap::analysis` (`design.md:39-41`).
- The archive bail is `archive blocked: <titles>` with titles that already begin
  `interaction inventory mismatch`; no duplicated prefix (`design.md:137`).
- The refusal scenarios assert use case and capability: lint unlisted/missing/undeclared
  (`check-specification-health.feature:11-35`), review (`givn-workflow-enforcement.feature:11-14`),
  and archive (`archive-proof.feature:11-14`, `:32-34`). The raw unplaceable case has no
  use case to name and asserts the capability.
- The glossary wording is "a merged candidate rechecks the actual permanent inventory
  before its verification scopes", not a proven archive (`docs/glossary.md:90`).
- Unit tests and feature scenarios are separate sections (`design.md:268-315`); the
  `MergeProgress`/`retry_unrecorded` coverage is explicitly "unit tests" (`:307`).
- The clean-state witness row now says the witness runs before the projection and
  distinguishes the merged-candidate gate from the finalization guard (`design.md:253`).
- The new fixture file is registered: `tests/steps/interaction_projection_steps.rs`
  "(registered in `tests/steps/mod.rs`)" (`design.md:153`).
- Chapter 6 distinguishes the `missing-testrunner`/`missing-run-command` sentinel
  commands from the `missing-e2e-runner` comparison value
  (`docs/arc42/06-runtime-view.md:25-26`).
- The delta/permanent distinction is explicit: the delta narrative supplies the repaired
  fragment and archive writes the permanent path (`design.md:69-70`); the permanent corpus
  result is a separate regression check from the delta feature scenarios (`:301`).

### H2 detail — routed fixture scaffold projects a missing interaction

Status: resolved by the author's hardening pass; see "H2 verification" below. The
generated row renders the capability name in both cells (not `exercise the capability`),
which is the one factual correction to the text below.

`tests/steps/mod.rs:357-381` `feature_file_at` writes the change feature and calls
`common::declare_fixture_routing`. For a capability with no permanent owner,
`declare_fixture_routing` (`tests/common/mod.rs:540-576`) calls `write_fixture_usecase`
(`tests/common/mod.rs:578-594`), which writes `usecase.md` from `usecase_md_template()` with
the interaction row `| <capability> | exercise the capability | No E2E interaction |`.

The projection copies that narrative over the temporary corpus (`design.md:106-110`) and
runs the interaction gate. The routed fixture therefore projects one declared interaction
and zero `@e2e` scenarios. Executed check: a temporary
`givn/specs/cap/{usecase.md,cap.feature}` built from those exact replacements and a plain
feature yields

`use case 'cap' capability 'cap' declares 1 interaction(s) and has 0 @e2e scenario(s) — missing declared scenarios [No E2E interaction]`

exit `1` on the current gate.

Consequences: `givn/specs/verify-change/check-specification-health.feature:79-96`
(`--change scopes`, expects exit `0`) and
`givn/specs/verify-change/review-disposition-gates.feature:3-30` (expects `split-or-keep`)
fail under the new projection. The design's causal claim at `design.md:127` and `:298` that
the `NEW in` routing alone keeps them green is false. The named shared-fixture repair
(`design.md:154`, `:296`) covers the three builders but omits this scaffolder in
`tests/common/mod.rs`.

Required edit: add `tests/common/mod.rs::write_fixture_usecase` (the helper called by
`declare_fixture_routing`) to the named shared-fixture repair in `design.md` Step Definitions
(`:154`), Test Plan (`:298`), Risks (`:253`), and Batch plan (`:304`), and state that its
`## Interactions` rows are generated from the delta's actual `@e2e` titles, with `- none`
when the delta has no `@e2e` scenario. Then the routing-only green claim becomes true.
Without that edit, the design cannot claim the existing `--change scopes` and
review-disposition scenarios stay green.

### H2 verification — `write_fixture_usecase`

The required edit landed in every named location:

- Step Definitions table: `design.md:154` names `tests/common/mod.rs::write_fixture_usecase`
  (called by `declare_fixture_routing`) and states the inventory comes from the actual
  `@e2e` titles, with `- none` when none.
- Fixture-repair paragraph: `design.md:158` says the placeholder `No E2E interaction` row
  must become rows generated from the delta feature's actual `@e2e` titles, and `- none`
  when there are none.
- Risks: `design.md:260` includes the helper in the mitigation.
- Regression checks: `design.md:306` includes the helper in the generated-inventory check;
  `design.md:308` states the two existing scenarios stay green because routing plus
  `write_fixture_usecase` writing `- none` (not routing alone).
- Batch plan: `design.md:312` (batch 1) includes the helper.
- Causal claim: `design.md:127` now conditions the greenness of the `--change` and
  review-disposition fixtures on `write_fixture_usecase` generating `- none`, so the
  routing-only claim is gone from the operative sentence. `design.md:245` still attributes
  greenness to routing in the unplaceable-capability boundary row; the sentence is scoped to
  that finding and the four listing locations carry the repair, so this is a wording nit, not
  a hole.

Implementability, checked against the code:

- `declare_fixture_routing` (`tests/common/mod.rs:540-576`) already parses the change specs
  (`collect_capability_names`, `:596-634`), decides `NEW in <usecase>` only when no permanent
  owner exists, and calls `write_fixture_usecase(&specs, usecase, &capability)` at `:557`.
  The helper (`:578-594`) receives the specs root and the use-case directory, so it can parse
  the delta features itself; it needs no new parameter.
- The same logic already exists: `tests/steps/mod.rs::ensure_fixture_usecase` (`:383-450`)
  collects the `@e2e` titles from the feature text, writes one row per title, and replaces the
  whole table with `- none` when the list is empty. The repair can share or copy that shape.
- The one factual correction to the original finding: the rendered row is
  `| <capability> | <capability> | No E2E interaction |`, because `write_fixture_usecase`
  replaces `<consumer action>` with the capability name (the permanent-fixture helper uses
  `exercise the capability`). The substance is unchanged.
- `- none` is already the supported sentinel: `src/gherkin/usecase_md.rs:777-779` returns an
  empty interaction list for it, and `interaction_gate_findings`
  (`src/gherkin/interaction.rs:130-142`) compares declared and actual title sets, so zero
  declared and zero `@e2e` is clean.

Sufficiency for the two existing scenarios, checked by execution:

- Both scenarios are green on the current binary: `--change scopes` and `Review blocks when a
  long scenario in the delta has no split-or-keep entry`.
- Their fixtures: the `--change scopes` delta is `Feature: Cap` + `Scenario: S`, the
  review-disposition delta is `Feature: Cap` + `Scenario: The undisposed marathon`; neither
  carries `@e2e`. The permanent `foo` side is written by `ensure_fixture_usecase`, which
  already emits `- none`.
- Simulated merged corpora with `givn lint` (the same `interaction_gate_findings` call the
  projection uses): with the repaired `- none` narrative the corpus reports no interaction
  finding; with the current placeholder row it reports `use case 'cap' capability 'cap'
  declares 1 interaction(s) and has 0 @e2e scenario(s) — missing declared scenarios [No E2E
  interaction]`, exit `1`. The repair is therefore both necessary and sufficient for those
  two scenarios.

### H4 finding — the fixture-repair set is still incomplete

H2 named one omitted writer. The same defect exists in further fixture writers that the
design still does not name, and each feeds an existing scenario that runs change-scoped lint,
review, or archive. `usecase_coverages` reads every use-case document in the projected corpus
(`src/gherkin/interaction.rs:42-110`), and the projection deliberately does not subtract
pre-existing findings (`design.md:120`), so these permanent-corpus fixtures are in scope.

Verified by simulating each fixture's merged/permanent corpus and running the current gate:

- `tests/steps/deduplicate_change_dir_detection_steps.rs:221-237` (`write_permanent_example`)
  writes `| example | exercise the capability | No E2E interaction |` and a permanent
  `example.feature` with no `@e2e` scenario. It feeds
  `givn/specs/verify-change/resolve-change-directories.feature:3-11` and `:13-24`
  (`givn check review --change ...`, exit `0`) and the `@e2e` `detect-int` scenario. The
  simulated corpus reports `use case 'example' capability 'example' declares 1 interaction(s)
  and has 0 @e2e scenario(s) — missing declared scenarios [No E2E interaction]`, exit `1`.
- `tests/steps/fix_uppercase_checkbox_parsing_steps.rs:106-115` writes the same row for
  `givn/specs/example/usecase.md` and only a delta `Feature: example` with no scenarios.
  `givn/specs/inspect-change-state/read-task-progress.feature:24-32`
  (`givn archive --change checkbox-archive`, exit `0`) fails the same way.
- `tests/steps/workflow_enforcement_steps.rs:1268-1287` (`clean_project_with_passing_change`)
  writes `| demo-cap | … | Existing scenario |` with a permanent `demo-cap.feature` whose only
  scenario is not `@e2e`. `givn/specs/verify-change/givn-workflow-enforcement.feature:292-298`
  (`Review accepts a current verification receipt`, exit `0`) fails the same way.
- `tests/steps/workflow_enforcement_steps.rs:1158-1193` (`change_new_capability`) writes
  `| new-cap | … | New scenario |` with a `@givn.added` (not `@e2e`) scenario;
  `givn-workflow-enforcement.feature:238-247` expects the unproven-candidate message, which
  the earlier projection refusal replaces.
- `tests/steps/extract_coverage_gate_steps.rs:76-132` writes `| example | … | stub |`; the
  `@e2e` `givn/specs/archive-change/extract-coverage-gate.feature:11-19` archive-success
  scenario fails the same way.

A second, distinct sub-case: a narrative-only delta whose use case declares a capability with
no feature in the projected corpus. `tests/steps/mod.rs:1429-1446`
(`change_usecase_document_records_persona`) writes such a delta, and
`givn/specs/verify-change/check-specification-health.feature:295-302`
(`givn lint --change persona-change`, exit `0`) hits it. The gate returns a blocking
`usecase_interaction_invalid` finding (`failed to parse …/persona-change.feature`), which the
design does not describe: `## External Boundary Cases` covers an unreadable use-case document
(`design.md:232`) but not a declared capability with no feature.

Required edit (exact): extend the shared-fixture repair in `design.md:154`, `:158`, `:253`,
`:298`, and `:304` to every `usecase_md_template()` writer that emits a literal interaction
row — at minimum `tests/steps/deduplicate_change_dir_detection_steps.rs::write_permanent_example`,
`tests/steps/fix_uppercase_checkbox_parsing_steps.rs:106-115`,
`tests/steps/workflow_enforcement_steps.rs::clean_project_with_passing_change` and
`::change_new_capability`, and `tests/steps/extract_coverage_gate_steps.rs:76-132` — and add the narrative-only
declared-capability-without-feature case to `## Projection Semantics` /
`## External Boundary Cases`, with the persona fixture (`tests/steps/mod.rs:1429-1446`)
repaired accordingly. Add the affected scenarios to the regression checks
(`design.md:293-300`): `resolve-change-directories.feature:3-24`,
`read-task-progress.feature:24-32`, `givn-workflow-enforcement.feature:238-247` and `:292-298`, and
`extract-coverage-gate.feature:11-19`.

### H4 verification — named writers, audit obligation, residual fixtures

The author's H4 edits landed. The five named writers, the persona fixture, and
`write_fixture_usecase` appear in every required location: the Step Definitions table
(`design.md:154`), the fixture-repair paragraph (`:160`), the persona repair (`:162`), the
risks (`:260`), the regression checks (`:306`), and the batch plan (`:312`). The audit
obligation is explicit at `:160`: "audits every test fixture that writes a `## Interactions`
table, not only the builders above ... Each must emit only rows for actual `@e2e` titles, or
`- none`." The persona repair at `:162` says the feature is added with no `@e2e` and the table
emptied, or the capabilities become `- none`, so the advisory scenario keeps exit `0`.
`design.md:116`, `:242`, and `:279` state the `usecase_interaction_invalid` rule and its unit
test.

Implementability of each named repair, checked against the tree:

- `deduplicate_change_dir_detection_steps.rs:221-237`: row `No E2E interaction` (`:235`) over a
  permanent `Scenario: Existing behavior` with no `@e2e` (`:224-227`). `- none` clears it.
- `fix_uppercase_checkbox_parsing_steps.rs:106-115`: the same row (`:112`); the delta is
  `@givn.delta Feature: example` with no scenarios. `- none` clears it.
- `workflow_enforcement_steps.rs:1158-1193` (`change_new_capability`): row `New scenario`
  (`:1177`) over `@givn.added Scenario: New scenario`, not `@e2e` (`:1192`). `- none` clears
  it, so the unproven-candidate review message survives.
- `workflow_enforcement_steps.rs:1268-1287` (`clean_project_with_passing_change`): row
  `Existing scenario` (`:1278`) over a permanent scenario that is not `@e2e` (`:1282`).
  `- none` clears it.
- `extract_coverage_gate_steps.rs:76-132`: row `stub` (`:121`) over `Scenario: stub`, not
  `@e2e` (`:127`). `- none` clears it.
- `tests/steps/mod.rs:1429-1446` (persona): the template declares `persona-change` with row `S`
  and the unreplaced `<consumer action>`, and writes no feature. Writing the feature next to
  the document with a scenario that carries no `@e2e`, plus `- none` for the row, or a
  `- none` capability list, is both implementable and sufficient.

The sentinel and severity are as the design states: `- none` parses to an empty interaction
list (`src/gherkin/usecase_md.rs:777-779`); a missing feature produces the blanket
`usecase_interaction_invalid` finding (`src/gherkin/interaction.rs:114-126`) at blocking
severity (`src/cli/lint.rs:547`). Executed probe: a valid document declaring `persona-change`
with no feature file reports `failed to parse .../persona-change.feature: Could not read
path ...`, exit `1`. The design's description and unit test match the gate.

The scan for unnamed writers is not clean. Two fixture writers outside the design's list feed
archive scenarios that expect exit `0`:

- `tests/steps/readme_enforcement_steps.rs:323-330` (`completed_change_short`, `:289-337`)
  writes the permanent `givn/specs/example/usecase.md` row `stub` and merges a
  `Feature: example` with `Scenario: stub` and no `@e2e`. Probe on that corpus: `use case
  'example' capability 'example' declares 1 interaction(s) and has 0 @e2e scenario(s) —
  missing declared scenarios [stub]`, blocking, exit `1`. It feeds
  `keep-documentation-current/readme-enforcement.feature:130-141` (`givn archive` expects exit
  `0`; green today, executed) and `:144-156` (`@e2e`, archive succeeds). The README gate
  (`src/cli/archive.rs:161-164`) runs before the projection, so only these success paths lose
  their expected result.
- `tests/steps/grouped_merge_steps.rs:39-49` (`write_change_usecase`) and `:132-166`
  (`permanent_usecase_with_only_capability`) write a permanent `retire-home/usecase.md` with no
  `## Level`/`## Capabilities` (`:160`) and a delta narrative `only-cap/usecase.md` whose row
  is `only-cap merges`. The retirement removes the owner directory `retire-home`, but
  `narrative_target` (`src/cli/archive.rs:1690-1705`) targets the change-relative path, so
  `givn/specs/only-cap/usecase.md` survives with no feature. Probe on that post-archive
  corpus: `failed to parse .../only-cap/only-cap.feature: Could not read path ...`, blocking,
  exit `1`. `archive-change/grouped-merge-for-new-capabilities.feature:28-36` expects
  `givn archive` exit `0` (green today, executed). This also contradicts the retired-capability
  boundary row at `design.md:246`, which says the projection removes "the narrative delta": the
  removal only covers a narrative that already sits in the permanent owner directory.

Other literal-row writers were checked and need no repair: `ensure_fixture_usecase`
(`tests/steps/mod.rs:416-450`) already derives its rows from the `@e2e` titles;
`preserve_docstrings_on_archive_steps.rs:276` writes a row for `permanent: None` failure
fixtures, and the conflicting-Background/Rule/dialect merge aborts before the gate, so its
scenarios keep their messages; `spec_discovery_deterministic_steps.rs:42` drives `givn spec`
commands, and the `spec_overlap_gates_steps.rs` fixtures run default-scope lint, neither of
which projects. `extract_coverage_gate_steps.rs` and the other named files are already in the
list.

The audit obligation was real, but at that pass it was not a substitute for naming these
writers: the batch plan and the regression checks are the executable checklist, and neither
carried the affected scenarios. H4 remained unresolved at that point. The follow-up edit and
the final verification of the completed list are recorded below.

### Plan-integrity checks in this pass

- `givn lint --change enforce-spec-interaction-integrity` (this checkout's `target/debug/givn`):
  exit `2`, `4 file(s) checked, 11 finding(s)`; the 11 blocking findings are exactly the 11
  `@wip` scenarios (6 `check-specification-health`, 2 `givn-workflow-enforcement`, 3
  `archive-proof`), plus advisory shape/subset lines. No parse error: the output's only
  "parse" strings are advisory scenario titles.
- `givn readme check`: `README check: OK`, exit `0`.
- `givn lint` on the permanent corpus: `62 file(s) checked, 22 finding(s)`, exit `1`; the
  blocking set is exactly 11 interaction mismatches (10 fragment capabilities plus the
  non-`@e2e` init scenario), matching the proposal.
- Permanent-corpus capability declaration scan: zero feature files absent from their
  use-case or fragment `## Capabilities` list.
- Fragment delta: `sha256sum givn/specs/fragments/fragment.md` equals the delta's
  `a219ed71…78c22b`; the delta has 32 interaction rows versus the permanent 13 (19 added),
  and the capability lists are equal.
- Init delta: `givn/changes/enforce-spec-interaction-integrity/specs/initialize-project/initialize-a-project.feature:5`
  carries `@givn.modified @initialize-a-project @e2e`; the scenario body is identical to
  `givn/specs/initialize-project/initialize-a-project.feature:113-122`. `givn/specs` is
  unmodified in the working tree, so no `@e2e` tag was removed.
- Baseline scenarios are green on the current binary: `--change scopes`, the
  review-disposition long-scenario scenario, `Integrity hook resolves the change directory…`,
  `Uppercase checkboxes satisfy the archive gate`, and `Review accepts a current verification
  receipt`. Those last three are the H4 casualties: their fixtures project a blocking
  mismatch once the projection lands.
- Merged/permanent corpus probes (`givn lint` in scratch directories, exercising the same
  `interaction_gate_findings` call the projection makes): repaired `- none` narrative → no
  interaction finding; placeholder row → `declares 1 interaction(s) and has 0 @e2e
  scenario(s)` and exit `1`; dedup, uppercase, demo-cap shapes → the same blocking finding;
  persona-only narrative → `usecase_interaction_invalid` plus `capability 'persona-change' is
  not present at …/persona-change.feature`.

### H4 final verification — both residual writers named, re-scan clean

The follow-up edit names both residual writers and the retirement narrative shape. Checked in
this pass against `design.md` (315 lines) and the test tree.

Five required locations; each names `tests/steps/readme_enforcement_steps.rs` and
`tests/steps/grouped_merge_steps.rs` and states the repair:

| Location | Line | `readme_enforcement_steps.rs` | `grouped_merge_steps.rs` | Repair stated |
|---|---|---|---|---|
| Step Definitions table | `design.md:154` | named in the shared-fixture-repair row | named in the same row | "every literal interaction row is generated from the actual `@e2e` titles, with `- none` when none" |
| Fixture-repair paragraph | `design.md:160` | "`readme_enforcement_steps.rs` (README enforcement)" | "`grouped_merge_steps.rs` (retirement and grouped merge)" | "Each must emit only rows for actual `@e2e` titles, or `- none`" |
| Risks | `design.md:260` | named in the fixture-writer mitigation | named in the same mitigation | "generates the inventory from the actual `@e2e` titles, with `- none` when none; the retirement fixture updates its narrative" |
| Regression checks | `design.md:306` | named | named | the same rule; `:307` keeps the suite-green check |
| Batch plan | `design.md:312` | named in batch 1 | named in batch 1 | "so their inventories match their actual `@e2e` titles, with `- none` when none, and the retirement fixture removes the retired capability from its narrative" |

The retirement shape is stated twice: `design.md:162` ("its delta narrative must remove the
retired capability from the use case's `## Capabilities` list ... The fixture repair updates the
retirement narrative") and the boundary row at `design.md:248` ("A retirement without that
narrative update is a blocking `usecase_interaction_invalid` finding. The
`grouped_merge_steps.rs` fixture is repaired to update the narrative"). The fixture matches the
described shape: `write_change_usecase` (`tests/steps/grouped_merge_steps.rs:39-49`) writes the
delta narrative with `- <cap>` and the row `{cap} merges`; the retirement delta
(`:190-199`) carries zero scenarios; `retiring_change` (`:172-201`) is the fixture behind
`archive-change/grouped-merge-for-new-capabilities.feature:28-36`. Removing the capability and
writing `- none` leaves the projected corpus clean. A row over a still-declared capability would
produce the named blocking finding.

The audit obligation covers every current and future writer. `design.md:160`: "The
fixture-repair batch audits every test fixture that writes a `## Interactions` table, not only
the builders above ... Each must emit only rows for actual `@e2e` titles, or `- none`." The
batch plan opens with "Audit and repair every fixture writer" (`design.md:312`), so a writer
added after this design still falls under batch 1.

Re-scan of the test tree (completed). Every `usecase_md_template()` caller, every literal
`## Interactions` table, and every `usecase.md` write site:

| Writer site | Corpus use | Disposition |
|---|---|---|
| `tests/common/mod.rs:578-594` `write_fixture_usecase` (called at `:557` by `declare_fixture_routing`) | New-capability `--change` fixtures; writes the placeholder `No E2E interaction` row today | Named at `design.md:154`, `:158`, `:260`, `:306`, `:312`; repair derives rows |
| `tests/steps/mod.rs:383-451` `ensure_fixture_usecase` | Permanent `.feature file at` fixtures | Cleared: already derives rows from the actual `@e2e` titles (`:404-411`, `:429-446`) |
| `tests/steps/mod.rs:1430-1447` persona narrative | `givn lint --change persona-change` | Named at `design.md:164`; repair adds the feature with no `@e2e` and `- none`, or `- none` capabilities |
| `tests/steps/fix_uppercase_checkbox_parsing_steps.rs:106-115` | `givn archive --change checkbox-archive` | Named (`design.md:160`) |
| `tests/steps/deduplicate_change_dir_detection_steps.rs:221-237` | `resolve-change-directories` review fixtures | Named (`design.md:160`) |
| `tests/steps/readme_enforcement_steps.rs:323-330` (`completed_change_short`, `:289-339`) | `readme-enforcement.feature:130-141`, `:144-156` archive success | Named in all five locations; row `stub` over a non-`@e2e` `Scenario: stub` (`:333-336`) becomes `- none` |
| `tests/steps/grouped_merge_steps.rs:39-49` `write_change_usecase`, `:132-166` `permanent_usecase_with_only_capability`, `:239-248` `write_usecase_document` | Grouped-merge archive fixtures; retirement at `:28-36` | Named in all five locations; retirement covered at `:162`/`:248`; non-`@e2e` rows become `- none` |
| `tests/steps/archive_proof_support.rs:185-243` `write_change_artifacts_at` | `ready_change` corpora for the archive suite | Named (`design.md:154`, `:260`, `:306`, `:312`) |
| `tests/steps/enforce_post_merge_e2e_gate_steps.rs:354-367` `write_permanent_usecase` | Post-merge E2E gate archive fixtures | Named; its row `e2e test` already matches the delta's `@e2e Scenario: e2e test` (`:347`) |
| `tests/steps/extract_coverage_gate_steps.rs:76-132` | `givn archive` coverage scenarios | Named (`design.md:160`) |
| `tests/steps/workflow_enforcement_steps.rs:1171`, `:1275`, literal tables `:1594`/`:1675`/`:2080`/`:2982-2998` | Review and archive workflow fixtures | Named (file-level) |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:276-283` | Only `permanent: None` fixtures (`:743-751`, `:810-817`) | Cleared: both fail in `build_merge_plan`'s `validate_feature_merge` (`src/cli/archive.rs:936`) before the projection call |
| `tests/steps/spec_discovery_deterministic_steps.rs:34-49`, `:628` | `givn spec` commands only (`:267-308`) | Cleared: never projects |
| `tests/steps/spec_overlap_gates_steps.rs:220-234`, `:495-518`, literal tables `:126`, `:181`, `:458`, `:580` | Default-scope `givn lint` (`check-specification-health.feature:402-425`) | Cleared: default scope does not project (`design.md:130`) |
| `tests/steps/archive_proof_e2e_steps.rs:106-114` | Edits a delta usecase created by `write_change_artifacts` | Not a new writer; covered by the named builder |

No other `usecase_md_template()` caller, `## Capabilities` literal, `## Interactions` literal,
or `usecase.md` write site exists under `tests/`. The only `fragment.md` writers are the
cleared `spec_overlap_gates_steps.rs` fixtures. No writer that feeds a projected corpus is
unnamed, so the known list plus the audit obligation is complete.

Plan-integrity checks in this final pass:

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit `2`;
  `givn lint: 4 file(s) checked, 11 finding(s)`; exactly the 11 `@wip` scenarios (6
  `check-specification-health`, 2 `givn-workflow-enforcement`, 3 `archive-proof`); the only
  "parse" strings are advisory scenario titles, so there is no parse error.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- `git status --short givn/specs` is empty: no `@e2e` tag was removed. No `tasks.md` was
  written and no commit was made in this pass.

## Re-challenge verification — N1-N9

Second re-challenge pass. Read in this pass: the current `design.md` (325 lines), all five
delta specification files, `archive-proof.feature`, `check-specification-health.feature`,
`givn-workflow-enforcement.feature`, the named fixture files under `tests/steps/`,
`tests/common/mod.rs`, `src/cli/archive.rs`, `src/cli/lint.rs`,
`src/gherkin/{interaction,usecase_md}.rs`, `src/spec/owner.rs`,
`docs/arc42/08-crosscutting-concepts.md`, `docs/reference.md`, and `docs/glossary.md`.
The N1-N9 claims were checked against the current text and the implementation primitives.

| Item | Status | Evidence |
|---|---|---|
| N1 | **Resolved** | `specs/archive-change/archive-proof.feature:28` uses `Given an unproven archive candidate "archive-projection" with a merged corpus and no verification scopes`. `design.md:160` names `archive_proof_support.rs::write_pending_receipt` and says the fixture does not use `e2e_toggle_candidate`, which has already run a scope. The writer (`tests/steps/archive_proof_support.rs:247-280`) emits `"status": "pending"`, `"scopes": {}`, and no `scope_evidence` (deserialised as empty), so the no-scope assertion is true by construction. The merged corpus is the existing `ready_change` plus candidate-copy shape (`archive_proof_steps.rs:633-655`); the new phrase is listed at `design.md:183`. |
| N2 | **Resolved** | `design.md:166` and `:256` state the two-case rule. `retire_capability` (`src/cli/archive.rs:1934-1973`) removes the capability feature, then removes `usecase.md` and the directory when no feature remains; `parse_usecase_md` rejects a leaf whose `## Capabilities` is empty (`src/gherkin/usecase_md.rs:261-268`, `:343-348`), so a leaf `- none` document is not a usable shape. The last-capability fixture (`grouped_merge_steps.rs:171-201`) can omit `write_change_usecase`; `build_merge_plan` resolves the retirement target with `resolve_capability_owner` (`src/spec/owner.rs:3-29`) and does not need the narrative. Minor: the current fixture set has only the last-capability retirement scenario, so the sibling case is a design rule rather than an executed case. |
| N3 | **Resolved with a required clarification** | `design.md:168` states both shapes and correctly rejects a leaf `- none` capability list. The permanent-feature shape is implementable, but under N4 it also needs a permanent `givn/specs/persona-change/usecase.md` declaring `persona-change` with `- none` interactions: a bare permanent feature is a no-document directory, and the reverse check reports `undeclared_capability`, which would fail the default-scope scenario `A dangling persona reference is reported as an advisory` (`check-specification-health.feature:281-291`, exit `0`). The shared row at `design.md:154` ("a no-document feature directory gains its document") covers the persona fixture if applied; `:168` should say so or use only the routing-proposal + change-feature shape, which avoids the permanent feature. |
| N4 | **Resolved** | `design.md:251` reports a no-document directory as `undeclared_capability`; `:154` gives the repair ("a no-document feature directory gains its document") and names `repository_ownership_fixtures_support.rs`; `:162` repeats it. `docs/arc42/08-crosscutting-concepts.md:171-173` (8.3c) states the same. The fixture (`repository_ownership_fixtures_support.rs:20-27`) has two permanent features and no documents; adding a `usecase.md` with `- none` interactions clears both. Residual: the boundary row names only this fixture; the other no-document fixtures are covered only by the file-level rule at `:154` (see the scan below). |
| N5 | **Resolved** | `design.md:116` says both `undeclared_capability` and `usecase_interaction_invalid` join the `severity_for` blocking set and the `has_use_case_error` exit-code match. The default-scope scenario is `check-specification-health.feature:38-44` (exit `1`); `docs/reference.md:16` lists undeclared capability in the exit-`1` clause. The runtime lists at `src/cli/lint.rs:228-242` and `:534-551` do not yet contain the kind; adding it in both is the implementation obligation the design states. |
| N6 | **Resolved** | `design.md:111` rewrites a temporary path embedded in a finding title and requires the unit test to assert that neither `files` nor `titles` contains `/tmp`; `:284` repeats the `files`/`titles` assertion. This covers the `usecase_interaction_invalid` title, which embeds the failed feature path. Residual: `:260` and `:269` still describe only `files`, and `:289` does not restate the title assertion for the `usecase_interaction_invalid` unit test; tighten for consistency, not behaviour. |
| N7 | **Not resolved** | `design.md:253` still says "the existing `.feature file at` fixtures declare routing through `declare_fixture_routing`, so their scope scenarios stay green." Routing alone is insufficient: `write_fixture_usecase` writes the placeholder `No E2E interaction` row (`tests/common/mod.rs:578-594`), and without the `- none` repair those fixtures project a blocking mismatch. `:127`, `:248`, and `:318` carry the complete rule; the boundary row does not. |
| N8 | **Resolved** | `design.md:261` says the clean-state witness runs before the projection and catches an uncommitted edit; a committed edit between projection and publication is caught by the merged-candidate gate before verification, and finalization repeats the clean-state check. Anchors: `src/cli/archive.rs:125` (fresh witness), `:222`/`:240` (resume witness), `:174-179` (projection), `:273-275` (merged-candidate gate before scopes), `:339` (tested witness). |
| N9 | **Resolved** | `design.md:164` selects `fragment.md` for the `fragments` use case and keys fixture capabilities by use case and capability; the current helper writes `usecase.md` unconditionally (`tests/common/mod.rs:578-594`) and `collect_capability_names` keys by use case only (`:596-634`), so both are real changes. The nested-feature limitation is recorded at `:164` and `:252`. |

### Recheck — H1-H4, M1-M7, minor items

- H1 stands: the shared archive/review builders are named at `design.md:154`, and the
  `ready_change`/`write_change_artifacts` shape is the one the merged-candidate scenario needs
  (`tests/steps/archive_proof_support.rs:179-243`). H2 stands: `write_fixture_usecase` is named
  at `:127`, `:154`, `:158`, `:164`, `:248`, `:268`, `:316`, `:318`, and `:322`; the two
  existing scenarios are green only with the `- none` repair, which is the N7 wording gap.
  H3 stands: the reverse check is at `:116`, the scenarios exist, and 8.3c/8.24 state it; an
  independent scan of `givn/specs` found 18 directories, zero feature files absent from their
  document's capability list, and zero no-document feature directories (`author-change` and
  `maintain-project` are `- none` summary documents with no features). H4 stands for the README
  and retirement writers: both are named in all five locations and the retirement rule is at
  `:166`/`:256`.
- M1-M7 remain resolved: the narrative-only scenario and boundary row (`:126`, `:254`);
  `MergeProgress`/`retry_unrecorded` (`:72`, `:102`, `:108`, `:138`, unit test `:287`); the
  lint/undeclared docs (`docs/reference.md:16`, `docs/glossary.md:88-90`, `docs/workflow.md:131-132`,
  8.3c/8.24); the release-note drift wording (`proposal.md:7`, R-135); the merged-candidate
  fixture and `ready_change` repair (`:156`, `:160`); the no-delta-feature boundary row
  (`:254`); and ADR-019 in `arc42.md:70` plus ADR-075.
- Minor items stand: contract consumers (`:39-41`), the archive bail text, the refusal
  assertions, the glossary wording, the unit/feature separation, the clean-state witness row,
  and the new fixture registration. Two documentation residues are non-blocking: the exit-`1`
  bullets in `docs/arc42/06-runtime-view.md:182` and `docs/arc42/08-crosscutting-concepts.md:425`
  still name parse/use-case/interaction but not undeclared capability (reference.md:16 does),
  and the `/tmp` wording at `design.md:260`/`:269` is files-only.

### New fixture residuals

Scan of every `usecase_md_template()` caller, every literal `## Interactions` table, and every
`givn/specs` feature write under `tests/`:

| Writer | Projected use | Disposition |
|---|---|---|
| `tests/steps/readme_enforcement_steps.rs::write_reviewable_change` (`:244-279`) | Permanent `givn/specs/example/example.feature`, no document; `readme-enforcement.feature:68-75`, `:77-85`, `:159-165` review exit `0` | File named at `design.md:154`; the no-document rule at `:154` adds the missing `usecase.md`. Not individually named; name it in batch 1 or the green review scenarios fail under the reverse check. |
| `tests/steps/workflow_enforcement_steps.rs::archive_ready_change_wip_only_on_removed` (`:1025-1063`) | Permanent `givn/specs/test-cap/test-cap.feature`, no document; `givn-workflow-enforcement.feature:173-178` archive exit `0` | File named; the no-document rule applies. The `@wip` boundary row at `design.md:257` only covers the WIP-refusal case, so this success case is not cleared by that row. |
| `tests/steps/enforce_post_merge_e2e_gate_steps.rs:47-59`, `:209-221` | Permanent `givn/specs/existing/existing.feature`, no document; `enforce-post-merge-e2e-gate.feature:15-26` archive exit `0` and `:34-45` | File named; the no-document rule applies. |
| `tests/steps/repository_ownership_fixtures_support.rs:20-27` | Two permanent features, no documents; review/archive | Named at `design.md:251`; gains documents. |
| `tests/steps/workflow_enforcement_steps.rs::review_fixture_all_schema_problems` (`:1716-1723`) | Deliberate `register-readiness` interaction error; `givn-workflow-enforcement.feature:334-342` asserts the schema diagnostic | **Blocked by the audit wording**: `design.md:162`/`:322` says every `## Interactions` writer must emit only actual `@e2e` titles or `- none`; applying that here removes the deliberate error. Add an exemption for fixtures that intentionally fail the use-case schema gate. |
| `tests/steps/run_upgrade_steps.rs::seed_user_files` (`:517-525`), `tests/steps/spec_discovery_deterministic_steps.rs:65-209` | No-document permanent features | Cleared: `givn upgrade` and `givn spec` do not run lint, review, archive, or the projection. |
| `tests/steps/grouped_merge_steps.rs::permanent_usecase_with_only_capability` (`:132-169`) | Permanent feature plus a document without `## Capabilities`; the last-capability retirement removes both | Cleared by N2. |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:276` | `permanent: None` failure fixtures | Cleared: `build_merge_plan`/`validate_feature_merge` aborts before the projection (`src/cli/archive.rs:934-938`). |

The previously named writers remain named, and no unnamed writer feeds a projected corpus
outside this table.

### Plan-integrity checks in this pass

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit `2`,
  `4 file(s) checked, 12 finding(s)`; exactly 12 `@wip` scenarios (7
  `check-specification-health`, 2 `givn-workflow-enforcement`, 3 `archive-proof`); the only
  "parse" strings are advisory scenario titles, so there is no parse error.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- `git status --short givn/specs` is empty: no `@e2e` tag was removed. No `tasks.md` was written
  and no commit was made in this pass.
- Fragment delta: base hash `a219ed71…78c22b`; 32 interaction rows against 32 permanent
  fragment `@e2e` titles, set-equal.

## Verdict

The N1-N9 hardening is largely sound. N1, N2, N4, N5, N6, N8, and N9 are resolved; the
archive, lint, and gate primitives support the named fixtures, and the prior H4 record stands
for the README and retirement writers. Two items are unresolved and one clarification is
required, so the verdict is FAIL.

1. **N7 — boundary-row green claim (`design.md:253`).** Replace "the existing `.feature file
   at` fixtures declare routing through `declare_fixture_routing`, so their scope scenarios
   stay green" with "the existing `.feature file at` fixtures declare routing through
   `declare_fixture_routing`, and `write_fixture_usecase` emits `- none` when the delta has
   no `@e2e` titles, so their scope scenarios stay green". Routing alone is false: the
   placeholder row projects a blocking mismatch.

2. **H4 audit wording (`design.md:162`, `:322`).** The blanket "each must emit only rows for
   actual `@e2e` titles, or `- none`" would rewrite the deliberate `register-readiness`
   interaction in `tests/steps/workflow_enforcement_steps.rs:1716-1723`, which
   `givn/specs/verify-change/givn-workflow-enforcement.feature:334-342` asserts as a schema
   problem. Add: "Fixtures that intentionally carry a use-case schema error keep their
   deliberate rows; the repair applies to fixtures whose projected corpus is expected to
   pass." Name that fixture as the example.

3. **N3 clarification (`design.md:168`).** Either add the permanent
   `givn/specs/persona-change/usecase.md` (capability `persona-change`, `- none` interactions)
   alongside the permanent feature, or use only the routing-proposal + change-feature shape.
   Without the permanent document the default-scope persona scenario
   (`check-specification-health.feature:281-291`, exit `0`) fails under the N4 reverse check.

## Final verification after the author update

Read in this pass: the current `design.md` (329 lines), the five delta specs, and the named
fixtures. Re-ran the fixture scan over every `## Interactions` writer, every `usecase.md` /
`fragment.md` write site, and every `declare_fixture_routing` caller under `tests/`.

| Item | Status | Evidence |
|---|---|---|
| N7 — routing-only green claim | **Not resolved** | The `- none` rule now appears at `design.md:252` (boundary row), `:158` (fixture paragraph), `:272` (risks), `:320`/`:322` (regression checks), and `:326` (batch plan). But `design.md:257` is unchanged: "the existing `.feature file at` fixtures declare routing through `declare_fixture_routing`, so their scope scenarios stay green." That is the sentence the previous pass required replaced, and it remains false without the `write_fixture_usecase` `- none` clause. |
| Intentional-schema-error exception | **Names the fixture; the join contradicts the named scenario** | `design.md:164` names the `register-readiness` interaction at `workflow_enforcement_steps.rs:1716-1723` (row at `:1721`) and `givn-workflow-enforcement.feature:334-342`; `:326` repeats the exception. The deliberate row stays. The problem is "the projection finding is expected to join the schema finding": the scenario at `givn-workflow-enforcement.feature:340` runs `stderr reports each distinct document problem once`, whose step at `workflow_enforcement_steps.rs:1941-1955` asserts `stderr.matches(...).count() == 1` for `got '# Use Case: review-diagnostics-demo'`, `unknown heading '## Guarantees'`, and `interaction references undeclared capability 'register-readiness'`. The schema gate already reports all three once; the projection's parse-error finding repeats them. Probe on a permanent corpus with the same case-mismatched kebab title and undeclared interaction: the UCDOC diagnostic includes the interaction phrase. A second projection copy fails the `once` step. |
| Persona repair | **Shape resolved; one consequence unstated** | `design.md:172` names the permanent `givn/specs/persona-change/usecase.md` (`persona-change`, `- none`) and permanent `persona-change.feature` with no `@e2e`, or the routing-proposal + change-feature shape; it excludes a bare feature and a leaf `- none` capability list. The fixture's change document (`tests/steps/mod.rs:1435-1442`) still writes the template row `| persona-change | <consumer action> | S |`. The merged narrative replaces the permanent document, so the projection compares `S` with the no-`@e2e` feature. Probe: the merged corpus exits `1` with `declares 1 interaction(s) and has 0 @e2e scenario(s)`; the same corpus with `- none` exits `0`. The `:162` audit rule covers the change-document writer, but `:172` should state that its row becomes `- none` (or the alternative change feature carries `@e2e Scenario: S`). |
| No-document fixtures | **Resolved** | `design.md:166` and `:326` name `readme_enforcement_steps.rs:244-279`, `workflow_enforcement_steps.rs:1025-1063`, and `enforce_post_merge_e2e_gate_steps.rs:47-59,209-221`; both say they gain `usecase.md`/`fragment.md` in batch 1 under the N4 rule. |
| Re-scan | **Complete** | Every projected-corpus writer is named or uses a named builder. `visual_review_gate_steps.rs:103` writes a delta feature but routes it through `declare_fixture_routing`/`write_fixture_usecase`; `run_command_steps.rs`, `fix_domain_modeling_skill_trigger_steps.rs`, and `development_timeline_support.rs` do not run lint/review/archive; `archive_proof_e2e_steps.rs:106` edits a `write_change_artifacts` narrative; `preserve_docstrings_on_archive_steps.rs:276` writes `permanent: None`; `spec_discovery_deterministic_steps.rs` runs `givn spec`; `run_upgrade_steps.rs` runs `givn upgrade`; `spec_overlap_gates_steps.rs` is default-scope and deliberately mismatched. No unnamed projected-corpus writer remains. The audit obligation at `:162`/`:326` stands. |

### Plan-integrity checks

- `givn lint --change enforce-spec-interaction-integrity` (installed 0.9.17 and
  `./target/debug/givn`): exit `2`; `4 file(s) checked, 12 finding(s)`; exactly the 12 `@wip`
  scenarios (7 `check-specification-health`, 2 `givn-workflow-enforcement`, 3 `archive-proof`);
  the only "parse" strings are advisory scenario titles, so there is no parse error.
- `givn readme check`: `README check: OK`, exit `0`.
- `git status --short givn/specs` is empty: no `@e2e` tag was removed. No `tasks.md` exists and
  no commit was made in this pass.
- Persona probes on `target/debug/givn lint` (temporary corpora): the merged change narrative
  with row `S` and no `@e2e` exits `1`; the permanent `- none` document with no `@e2e` exits `0`.
- `register-readiness` probe: the case-mismatched kebab title keeps the interaction diagnostic,
  so the schema gate reports the phrase once; a projection parse-error finding would make it
  twice.

### Outcome

The exception's fixture naming, the persona permanent-document shape, and the no-document
fixtures are resolved; the re-scan is complete and the audit obligation stands. Two blocking
items and one clarification remain, so the verdict is FAIL.

1. **N7 — routing-only green claim (`design.md:257`).** Replace "the existing `.feature file
   at` fixtures declare routing through `declare_fixture_routing`, so their scope scenarios
   stay green" with "the existing `.feature file at` fixtures declare routing through
   `declare_fixture_routing`, and `write_fixture_usecase` emits `- none` when the delta has no
   `@e2e` titles, so their scope scenarios stay green".

2. **Schema-error exception — projection duplication (`design.md:126`, `:164`).** The
   `register-readiness` scenario asserts each diagnostic once
   (`givn-workflow-enforcement.feature:340`, `workflow_enforcement_steps.rs:1944-1954`), and
   the schema gate already reports the title, the unknown heading, and the interaction. Change
   the projection condition at `:126` to `!has_parse_error && !has_use_case_error` and replace
   "the projection finding is expected to join the schema finding, not to be removed by the
   repair" with "the schema gate reports the deliberate interaction once; the projection is
   skipped when the delta scan already reports a use-case schema error, so the diagnostic is
   not duplicated." The fixture keeps its deliberate row; only the projection's second copy is
   removed.

3. **Persona consequence (`design.md:172`).** Append: "The fixture's change document also writes
   its interaction row as `- none` (or the alternative change feature carries `@e2e Scenario:
   S`), because the merged narrative replaces the permanent document."

## Final verification after the third author update

Scope of this pass: the three edits the last pass required, plus a recheck that N1-N9 and
H1-H4/M1-M7 were not disturbed. Read in this pass: `design.md` (329 lines), all five delta
specification files, `specs/fragments/fragment.md`, `src/cli/lint.rs`,
`src/gherkin/interaction.rs`, `src/gherkin/usecase_md.rs`, `src/overlap/analysis.rs`,
`tests/common/mod.rs`, `tests/steps/mod.rs`,
`tests/steps/workflow_enforcement_steps.rs`, and the permanent
`verify-change/givn-workflow-enforcement.feature` / `check-specification-health.feature`
scenarios that the repairs must keep green.

| Item | Status | Evidence |
|---|---|---|
| Projection condition and schema-error exception | **Resolved** | `design.md:126` gates the projection on `paths.is_empty() && change.is_some() && !has_parse_error && !has_use_case_error`; `:128` still increments `total_errors` and sets `has_use_case_error` for each projection finding. `design.md:164` now reads "the schema gate reports the deliberate interaction once, and the projection is skipped when the delta scan already reports a use-case schema error". The mechanism is real and correctly scoped: `has_use_case_error` (`src/cli/lint.rs:98`) is set for `usecase_invalid`, `interaction_mismatch`, and `usecase_interaction_invalid` (`:228-242`), but only for findings that survive the delta-path filter (`:213-219`). The `register-readiness` error lives in a change document (`tests/steps/workflow_enforcement_steps.rs:1716-1723`), so it sets the flag; a permanent-only pre-existing mismatch is filtered out of change scope, so the F5 scenario's projection stays reachable. `stderr_reports_each_problem_once` (`:1944-1954`) asserts `text.matches(needle).count() == 1` for `interaction references undeclared capability 'register-readiness'`; executed baseline: `cargo nextest run --test features_runner 'verify-change/givn-workflow-enforcement.feature::Consumer runs review and receives every use-case schema problem in one pass'` → `1 test run: 1 passed`. The deliberate row stays; only the projection's second copy is removed. |
| Boundary row (N7) | **Resolved** | `design.md:257` now reads "the existing `.feature file at` fixtures declare routing through `declare_fixture_routing`, and `write_fixture_usecase` emits `- none` when the delta has no `@e2e` titles, so their scope scenarios stay green"; `:252` adds that the `- none` arrives through `write_fixture_usecase`, "not from the routing row alone". Routing-only is no longer claimed anywhere. The chain exists: `tests/steps/mod.rs:357-381` (`feature_file_at` → `declare_fixture_routing`) → `tests/common/mod.rs:540-576` (the `NEW in <usecase>` branch calls the writer) → `:578-594` (`write_fixture_usecase`, today the placeholder `No E2E interaction` row at `:589` that `design.md:158` requires replaced). |
| Persona repair | **Resolved** | `design.md:172` keeps the permanent `givn/specs/persona-change/usecase.md` (`persona-change`, `- none` interactions) plus the permanent `persona-change.feature` with no `@e2e`, or the routing-proposal + change-feature shape, and now states "The change document's interaction row becomes `- none` (or the alternative change feature carries `@e2e Scenario: S`)". The primitives support it: `interaction_section` accepts a one-line `- none` body as an empty list (`src/gherkin/usecase_md.rs:775-778`); a declared capability with no feature is `usecase_interaction_invalid` (`src/gherkin/interaction.rs:114-126`), which is why the permanent feature is required; `- none` against a feature with zero `@e2e` titles is an equal (empty) set comparison (`:129-142`). The merged narrative replaces the permanent document, so the row change is the operative consequence. |

### N1-N9 and H1-H4/M1-M7 recheck

- N1 stands: `specs/archive-change/archive-proof.feature:28` is "an unproven archive candidate
  \"archive-projection\" with a merged corpus and no verification scopes"; `design.md:160` still
  names `write_pending_receipt` and excludes `e2e_toggle_candidate`.
- N2 stands: the two-case retirement rule is at `design.md:170` and `:260`, and
  `parse_usecase_md` still rejects an empty `## Capabilities` leaf (`src/gherkin/usecase_md.rs:264-267`, `:369-371`).
- N3/N4 stand: the permanent-document shape at `:172`; the no-document rule and fixture names at
  `:154`, `:166`, `:255`, `:326`.
- N5 stands: `design.md:116` keeps both kinds in the `severity_for` blocking set and the
  `has_use_case_error` exit match.
- N6 stands with its recorded residue: `:111` requires the `files`/`titles` `/tmp` assertion;
  `:264` is still files-only.
- N7 is now resolved (`:257`, `:252`).
- N8 stands: `:265` keeps the witness ordering and the merged-candidate gate.
- N9 stands: `:168` keeps `fragment.md` selection, use-case+capability keying, and the
  nested-feature limitation.
- H1-H4 and M1-M7 are untouched: the shared builders (`:154`), `write_fixture_usecase` naming
  (`:127`, `:154`, `:158`, `:168`, `:252`, `:257`, `:272`, `:320`, `:322`, `:326`), the reverse
  check (`:116`), the fixture-repair audit and its exception (`:162`, `:164`), the narrative-only
  scenario (`check-specification-health.feature:73-76` with the boundary row at `:258`),
  `MergeProgress` (`:102`, `:108`, `:138`), contract consumers (`:39-41`), and the merged-candidate
  fixture (`:156`, `:160`).

### Plan-integrity checks in this pass

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit `2`;
  `4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP  ] finding` lines (7
  `check-specification-health`, 2 `givn-workflow-enforcement`, 3 `archive-proof`); every other
  stderr line is an advisory (`82 [SHAPE]`, `815 [SUBST]`); zero findings whose kind or title is a
  parse error.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- Fragment delta: base hash `a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65fe14774878c22b`
  equals `sha256sum givn/specs/fragments/fragment.md`; 32 interaction rows against 32 permanent
  fragment `@e2e` titles, set-equal (zero missing, zero extra, no duplicate titles).
- `git status --short givn/specs` is empty: no `@e2e` tag was removed. No `tasks.md` exists and no
  commit was made in this pass.

### Residual implementation obligations (non-blocking)

1. **Placement of the lint projection.** `has_use_case_error` is set only inside the tree-wide
   scan (`src/cli/lint.rs:228-242`), which today sits after the empty-feature early return
   (`:78-92`). The projection call must therefore run after that scan, and the early return must
   not preempt projection findings, otherwise the pre-existing-mismatch scenario exits `0`.
   `design.md:126`, `:128` ("after the delta-path filter"), and `:164` ("the delta scan already
   reports") imply this order but never name `scan_tree`. Both behaviours are pinned by scenarios,
   so a misplaced call is RED in the first TDD cycle; no design change is required.
2. **Persona feature content.** The permanent `persona-change.feature` must carry at least one
   non-`@e2e` scenario. A zero-scenario feature sets `has_empty_feature`
   (`src/cli/lint.rs:156-169`) and exits `3`, contradicting the persona scenarios' exit `0`
   (`check-specification-health.feature:281-301`). "No `@e2e`" means "scenarios without the tag".
3. **Pre-existing N6 residue.** `design.md:264` still describes only `files` for the `/tmp`
   assertion while `:111` and the unit test require `titles` too. Carried over; not blocking.

### Outcome

All three required edits are applied and correct against the implementation. The projection is
skipped when a delta-local use-case schema error already exists, the deliberate
`register-readiness` row is preserved and reported once, the boundary row no longer claims that
routing alone keeps the scope scenarios green, and the persona change document's `- none` row is
stated with the permanent document/feature shape that makes it valid. N1-N9, H1-H4, and M1-M7
stand, the fragment repair is intact and hash-matched, and the plan-integrity checks reproduce
(exit `2`, 12 `@wip`, no parse error; README check OK; no `@e2e` removed). The verdict is PASS;
the three residuals above are implementation obligations already pinned by executable scenarios.

DESIGN-REVIEW: PASS

## Final verification after the challenge-driven update

Scope: the six item groups the final challenge requested, a full re-scan of every test-side
writer that feeds a projected corpus or leaves a permanent `givn/specs` directory without a
document, and a recheck of N1-N9, H1-H4, and M1-M7. Read in this pass: `design.md` (329 lines),
the five delta specifications, `specs/fragments/fragment.md`, `docs/arc42/06-runtime-view.md`,
`docs/arc42/08-crosscutting-concepts.md`, `docs/reference.md`, `docs/glossary.md`,
`docs/arc42/11-risks-and-technical-debt.md`, the named fixture files under `tests/`,
`tests/common/mod.rs`, `tests/steps/mod.rs`, `src/cli/lint.rs`, `src/cli/archive.rs`,
`src/spec/owner.rs`, `src/gherkin/interaction.rs`, and the permanent scenarios the fixtures feed.

| Requested check | Status | Evidence |
|---|---|---|
| Three no-document writers named in all five locations | **Resolved** | Step Definitions table `design.md:154` names `workflow_enforcement_steps.rs`, `preserve_docstrings_on_archive_steps.rs`, and `grouped_merge_steps.rs`; the fixture paragraph `:162` names `workflow_enforcement_steps.rs` and `grouped_merge_steps.rs` and `:166` names `preserve_docstrings_on_archive_steps.rs:99-110,267-284`, `workflow_enforcement_steps.rs:910-916` and `:1025-1063`, and `grouped_merge_steps.rs:521-555`; risks `:272`; regression checks `:320`; batch plan `:326`. The line ranges are real: `workflow_enforcement_steps.rs:910-916` is `change_complete_for_archive`, `:1025-1063` is `archive_ready_change_wip_only_on_removed`, `preserve_docstrings_on_archive_steps.rs:267-284` is the `permanent: Some` branch of `write_archive_artifacts`, and `grouped_merge_steps.rs:521-555` is `shared_capability_owners`. |
| Both repository-ownership documents | **Resolved** | `design.md:166` says `repository_ownership_fixtures_support.rs` writes `fixture-existing` and `fixture-retained` and needs two documents, not one; `:326` says "add both repository-ownership documents". The fixture writes exactly those two feature directories with no document (`tests/steps/repository_ownership_fixtures_support.rs:19-27`). |
| Default-scope sentence names `scan_tree` and the reverse check | **Resolved** | `design.md:130`: "The default scope (`change.is_none()`) does not project, but `scan_tree` now also reports undeclared capabilities through the reverse check; the permanent inventory mismatch remains direct." |
| Duplicate reporting for a document-less directory; root-level and nested feature limits | **Resolved** | `design.md:168`: "A document-less directory is reported by both the existing use-case-document check and the new reverse check; both are blocking, and the duplicate is intentional because the projection does not run the existing check." The existing check is `overlap::analysis`'s `usecase_invalid` "usecase.md cannot be read" (`src/overlap/analysis.rs:355-369`), which default-scope lint runs through `scan_tree`; the projection runs only `interaction_gate_findings`. Root-level and nested features are recorded at `:168` and the nested boundary row at `:256`. |
| Chapter 6 exit-1 list | **Resolved** | `docs/arc42/06-runtime-view.md:182`: "parse errors, invalid use-case documents, interaction mismatches, undeclared capabilities, and unplaceable delta capabilities produce exit code `1`". `:188-194` also states the `--change` projection and both new blocking kinds. |
| Chapter 8 exit-1 list | **Resolved** | `docs/arc42/08-crosscutting-concepts.md:425`: "`1` for a parse error, an invalid use-case document, an interaction mismatch, an undeclared capability, or an unplaceable delta capability". |
| Projection described after the tree-wide `scan_tree`/flag check | **Resolved** | `design.md:127`: "The implementation runs the projection after the tree-wide `scan_tree` has set the parse and use-case flags; the empty-feature early return is moved or bypassed for change scope." `:126` keeps the call "before the empty-feature early return". The current order is the early return at `src/cli/lint.rs:78-92`, the scan and `has_use_case_error` mapping at `:223-245`, so the move is stated. |
| Persona feature has at least one non-`@e2e` scenario | **Resolved** | `design.md:172`: the permanent `persona-change.feature` "containing at least one non-`@e2e` scenario"; the sentence also rejects a bare feature and a leaf `- none` capability list, and repairs the change document's row to `- none`. |
| `/tmp` assertions cover `files` and `titles` | **Resolved** | `design.md:111` requires the unit test to assert "neither `files` nor `titles` contains `/tmp`"; `:264` says "in both fields"; `:273` says "the finding files and titles"; `:288` says "in `files` and `titles`". The earlier files-only residue at `:264` is gone. |
| `write_fixture_usecase` counts `Scenario Outline:` `@e2e` titles | **Resolved (stated)** | `design.md:168`: "Its `@e2e` extraction counts `Scenario Outline:` titles as well as `Scenario:` titles. The audit covers that." No scenario or unit test pins the outline count, so it remains an implementation obligation under the batch-1 audit rather than an executable check. |
| Fixture steps commit/refresh; refusal step records long-scenario disposition; undeclared delta carries a placement route | **Resolved (stated)** | `design.md:272`: "The fixture steps commit their edits and refresh the change receipt before the archive clean-state witness; the archive refusal step records any long-scenario disposition; the undeclared-capability delta carries a placement route." The generic `the user runs` step already does the commit and receipt refresh for `archive` (`tests/steps/mod.rs:544-556`); the new fixture steps must do the same because they run after the receipt. |

### N1-N9 and H1-H4/M1-M7 recheck

- N1 stands: `design.md:160` names `write_pending_receipt` and excludes `e2e_toggle_candidate`;
  the writer emits `"status": "pending"`, `"scopes": {}`, and no `scope_evidence`
  (`tests/steps/archive_proof_support.rs:247-277`).
- N2 stands: the two-case retirement rule at `design.md:170` and `:260`;
  `parse_usecase_md` still rejects an empty `## Capabilities` leaf
  (`src/gherkin/usecase_md.rs:363-372`).
- N3/N4 stand: the permanent-document shape at `:172`; the no-document rule and the named
  writers at `:154`, `:162`, `:166`, `:255`.
- N5 stands: `design.md:116` keeps `undeclared_capability` and `usecase_interaction_invalid` in
  the `severity_for` blocking set and the `has_use_case_error` exit-code match; the current
  match lists are `src/cli/lint.rs:228-242` and `:539-551`.
- N6 stands resolved: `:111`, `:264`, `:273`, and `:288` all require both `files` and `titles`.
- N7 stands: `:257` conditions the scope fixtures' greenness on `write_fixture_usecase` writing
  `- none`, and `:252` repeats that it is "not from the routing row alone".
- N8 stands: `:265` keeps the witness, the merged-candidate gate, and finalization.
- N9 stands: `:168` keeps `fragment.md` selection, use-case+capability keying, and the
  nested-feature limitation; `:256` records the boundary.
- H1-H4 stand: the shared builders at `:154`, `write_fixture_usecase` at `:127`, `:154`,
  `:158`, `:272`, `:320`, `:326`, the reverse check at `:116`, the README/retirement writers in
  all five locations, and the retirement narrative rule at `:170`/`:260`.
- M1-M7 stand: the narrative-only boundary row at `:258`; `MergeProgress` at `:72`, `:102`,
  `:108`, `:138` and the legacy unit test at `:291`; the docs at `docs/reference.md:16`,
  `docs/glossary.md:88-90`, `:103`, `docs/workflow.md:131-132`, and 8.3c/8.24; R-135/R-137 at
  `docs/arc42/11-risks-and-technical-debt.md:122,124`; the merged-candidate fixture at `:156`,
  `:160`; ADR-019 at `arc42.md:70` and ADR-075.

### Re-scan of the test tree

Every test-side writer of `givn/specs` features, every `usecase_md_template()` caller, every
literal `## Interactions` table, and every `declare_fixture_routing` caller was checked. The
named set is complete for the challenged writers:

| Writer | Projected use | Disposition |
|---|---|---|
| `tests/steps/workflow_enforcement_steps.rs:910-916` `change_complete_for_archive` | Archive success at `givn-workflow-enforcement.feature:135-144` | Named `:166`/`:326`; gains a document |
| `tests/steps/workflow_enforcement_steps.rs:1011` `archive_ready_change_with_wip` | Archive WIP refusal at `:166-171` | Cleared: `build_merge_plan(..., validate=true)` bails on `@wip` before the projection (`src/cli/archive.rs:856-876`, call at `:173`) |
| `tests/steps/workflow_enforcement_steps.rs:1058` `archive_ready_change_wip_only_on_removed` | Archive success at `:173-179` | Named `:166`/`:326`; gains a document |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:267-284` `write_archive_artifacts` (`permanent: Some`) | Three `@preserve-scenario-docstrings` archive successes | Named `:166`/`:326`; gains a document |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:99-110` `permanent_spec_with_scenario` | Archived/test-corpus feature only | Named; harmless |
| `tests/steps/grouped_merge_steps.rs:521-555` `shared_capability_owners` | Review/archive Scenario Outline owner fixtures | Named `:166`/`:326`; each written directory gains its document |
| `tests/steps/repository_ownership_fixtures_support.rs:19-27` | Review/archive ownership fixtures | Two documents named at `:166`/`:326` |
| `tests/steps/readme_enforcement_steps.rs:244-279`, `enforce_post_merge_e2e_gate_steps.rs:47-59,209-221` | Archive/review successes | Named `:166`/`:326` |

Cleared by their command surface or an earlier gate: `run_upgrade_steps.rs:517-525` (`upgrade`
only), `spec_discovery_deterministic_steps.rs` (`givn spec` only), `release_0_3_0_e2e_steps.rs`
(migration change carries `.givn-skip specs`, so the permanent corpus is empty),
`preserve_docstrings_on_archive_steps.rs` `permanent: None` shapes (merge validation aborts
before the projection), `grouped_merge_steps.rs:132-169` (last-capability retirement removes the
document and feature together), `archive_proof_support.rs` (writes `demo-cap/usecase.md`),
`review_disposition_gates_steps.rs` and `workflow_enforcement_steps.rs` `review_ready` (the
document comes from `ready_change`), `spec_overlap_gates_steps.rs` (writes each use-case
document with its feature), and `tests/steps/mod.rs` `feature_file_at`/`ensure_fixture_usecase`
(derives `- none` or the actual `@e2e` rows).

**One projected-corpus no-document writer is still unnamed.**
`tests/steps/preserve_docstrings_on_archive_steps.rs:180-206`
(`e2e_project_with_completed_change`) writes the permanent
`givn/specs/real-cap/real-cap.feature` at `:192-198` with no document. It feeds the `@e2e`
scenario `Consumer archives a modified scenario and the permanent spec keeps the docstring`
(`givn/specs/archive-change/preserve-scenario-docstrings.feature:26-35`), which expects archive
exit `0`; `complete_change_with_delta` routes the delta to the same owner (`EXTEND real-cap`),
so the projected corpus has a feature directory with no document. The reverse check therefore
reports a blocking `undeclared_capability` and the scenario turns red. `design.md:166` and `:326`
enumerate only `:99-110,267-284` for this file, and `:320` does not name this scenario. The
scenario is green on the current binary (executed: `cargo nextest run --test features_runner
'archive-change/preserve-scenario-docstrings.feature::Consumer archives a modified scenario and
the permanent spec keeps the docstring'` → PASS, 1 passed, 1015 skipped), so only the new
projection turns it red.

### Plan-integrity checks in this pass

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit `2`;
  `givn lint: 4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP ] finding` lines (7
  `check-specification-health`, 2 `givn-workflow-enforcement`, 3 `archive-proof`); the only
  `parse` strings are advisory `[SHAPE]`/`[SUBST]` scenario titles, so there is no parse error.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- `git status --short givn/specs` is empty: no `@e2e` tag was removed. The init delta carries
  one `@e2e`. No `tasks.md` exists and no commit was made in this pass.
- The docstring scenario above passes today, which confirms the fixture is live and the break
  is caused by the new gate, not by a stale fixture.

### Remaining edit

1. **Name the docstring E2E no-document writer.** Add `:180-206`
   (`e2e_project_with_completed_change`) to the `preserve_docstrings_on_archive_steps.rs`
   no-document list at `design.md:166` and to the batch-plan list at `design.md:326`; the repair
   writes `givn/specs/real-cap/usecase.md` with `- none` interactions (the merged scenario is
   not `@e2e`). Add `Consumer archives a modified scenario and the permanent spec keeps the
   docstring` to the regression checks at `design.md:320`.

### Outcome

The six challenged item groups are resolved: the three writers and the two repository-ownership
documents are named in all five locations, the default-scope sentence names `scan_tree` and the
reverse check, the duplicate reporting and the root-level/nested limits are recorded, both
exit-1 documentation lists carry undeclared and unplaceable capabilities, the projection is
ordered after the tree-wide flag scan, the persona feature is required to carry a non-`@e2e`
scenario, the `/tmp` assertion covers `files` and `titles`, `write_fixture_usecase` counts
`Scenario Outline:` titles, and the three fixture-step obligations are stated. N1-N9, H1-H4,
and M1-M7 stand. One projected-corpus no-document writer remains unnamed in the executable
batch plan, so the verdict is FAIL pending the single edit above.

Earlier verdict (superseded): FAIL

## Final verification after the docstring-fixture update

Scope of this pass: the single residual from the previous pass
(`tests/steps/preserve_docstrings_on_archive_steps.rs:180-206`), a fresh re-scan of every
projected-corpus writer and every no-document fixture under `tests/`, and a recheck of N1-N9
and H1-H4/M1-M7. Read in this pass: `design.md` (329 lines), the five delta specifications,
`specs/fragments/fragment.md`, the named fixtures, `tests/steps/mod.rs` (both fixture
helpers), `tests/common/mod.rs`, `src/cli/archive.rs::build_merge_plan`,
`src/gherkin/merge.rs`, and `src/gherkin/parse.rs`.

| Check | Status | Evidence |
|---|---|---|
| Docstring writer named in the fixture paragraph | **Resolved** | `design.md:166` lists `preserve_docstrings_on_archive_steps.rs:99-110,180-206,267-284` among the no-document fixtures. The range is real: the `#[given]` attribute starts at `tests/steps/preserve_docstrings_on_archive_steps.rs:180`, `e2e_project_with_completed_change` at `:183`, and the permanent `givn/specs/real-cap/real-cap.feature` write at `:190-198` has no document. `complete_change_with_delta` routes the delta through `declare_fixture_routing` (`:75`), which resolves the existing feature to `EXTEND real-cap`, so the delta merges into that directory. |
| `- none` repair stated | **Resolved** | `design.md:166`: "The `preserve_docstrings_on_archive_steps.rs` success fixtures gain a `- none` `usecase.md` beside their permanent feature." The repair is correct: the merged `real-cap.feature` carries no `@e2e` scenario, so `- none` is the empty-set equality the gate compares, while the document's `## Capabilities` entry keeps the capability declared for the reverse check. Adding the document does not change the routing, which resolves from the feature file. |
| Docstring writer named in risks, regression checks, batch plan | **Resolved** | Risks `:272` names `preserve_docstrings_on_archive_steps.rs` in the writer list and states the repair "adds documents to the named no-document fixtures", whose list is `:166`; regression checks `:320` names the file; the batch plan `:326` repeats `preserve_docstrings_on_archive_steps.rs:99-110,180-206,267-284` in the batch-1 action. |
| Docstring regression scenario named | **Resolved** | `design.md:321` adds `Consumer archives a modified scenario and the permanent spec keeps the docstring` to the archive-proof regression list (`givn/specs/archive-change/preserve-scenario-docstrings.feature:26-35`, `@e2e`). Executed baseline: `cargo nextest run --test features_runner 'archive-change/preserve-scenario-docstrings.feature::Consumer archives a modified scenario and the permanent spec keeps the docstring'` → PASS, 1 test run, 1015 skipped. The scenario is green today; only the new reverse check turns it red without the repair. |

### Re-scan of the test tree

Every writer of a feature under `givn/specs`, every `usecase_md_template()` caller, every
literal `## Interactions` table, and every `declare_fixture_routing` caller was checked again,
including the writers the earlier tables did not name and the ones added by the docstring
fixture.

| Writer | Projected use | Disposition |
|---|---|---|
| `tests/steps/preserve_docstrings_on_archive_steps.rs:180-206` `e2e_project_with_completed_change` | `@e2e` archive success `preserve-scenario-docstrings.feature:26-35` | **Named** at `:166`/`:326`; gains a `- none` `usecase.md`. This was the last unnamed projected-corpus no-document writer. |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:99-110`, `:267-284` | Archive successes and archived-corpus features | Named at `:166`/`:326`; gain documents. |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:793-799` `permanent_conflict_background` | Archive refusal at `preserve-scenario-docstrings.feature:71` (permanent `conflict/conflict-cap.feature`, no document) | Cleared: `build_merge_plan(validate = true)` calls `validate_feature_merge` (`src/cli/archive.rs:936`) and bails with `conflicting Background` (`src/gherkin/merge.rs:202`) before the projection runs. |
| `tests/steps/preserve_docstrings_on_archive_steps.rs:739-744` `change_contains_rule` | Archive refusal at the unsupported-Rule scenario | Cleared: the delta parse fails at `src/gherkin/parse.rs:71` inside `build_merge_plan`, before the projection. |
| `tests/steps/mod.rs:383-464` `ensure_fixture_usecase` | Permanent `.feature file at` fixtures in `check-specification-health`, `spec-overlap-gates`, `review-disposition-gates`, `givn-workflow-enforcement` | Not named and needs no repair: it already emits the actual `@e2e` `Scenario:` titles or `- none`. No permanent `.feature file at` body in the corpus carries an `@e2e` outline (checked across `givn/specs/**/*.feature`), so the `Scenario:`-only extraction matches the gate. |
| `tests/steps/grouped_merge_steps.rs:27-54` `write_delta`/`write_change_usecase` | Grouped-merge archive/review fixtures | Named by file at `:154`, `:272`, `:320`, `:326`; the literal change-document row is converted by the batch-1 audit (`@e2e` titles, `- none` when none). |
| `tests/steps/enforce_post_merge_e2e_gate_steps.rs:354-364` `write_permanent_usecase` | `enforce-post-merge-e2e-gate.feature:15-25`, `:34-45` | Named by file; the `no-e2e` scenario needs its row to become `- none` because the merged feature has no `@e2e` scenario. The `existing` no-document writers at `:47-59`, `:209-221` stay named at `:166`. |
| `tests/steps/workflow_enforcement_steps.rs:1272-1286` `clean_project_with_passing_change` | Review/receipt fixtures | Named by file; the literal row is converted by the audit. |
| `tests/steps/run_command_steps.rs:150-167`, `:400-406` | `givn status` only | Cleared: no lint/review/archive call in the file. |
| `tests/steps/development_timeline_support.rs:561-565` | `givn report` fixtures | Cleared: reached only from `development_report_steps.rs`. |
| `tests/e2e/` | Browser-driver assets (`browser-driver.mjs`, npm manifests) | Cleared: no spec writer. |

The previously recorded clearances stand: `archive_proof_support.rs` (writes
`demo-cap/usecase.md`), `archive_proof_steps.rs` (delta-only writes and read/snapshot
helpers), `deduplicate_change_dir_detection_steps.rs`, `extract_coverage_gate_steps.rs`,
`fix_uppercase_checkbox_parsing_steps.rs`, `readme_enforcement_steps.rs`,
`repository_ownership_fixtures_support.rs`, `review_disposition_gates_steps.rs`,
`grouped_merge_steps.rs:132-169`, `spec_discovery_deterministic_steps.rs` (`givn spec`),
`run_upgrade_steps.rs` (`givn upgrade`), `release_0_3_0_e2e_steps.rs` (`.givn-skip specs`),
`spec_overlap_gates_steps.rs` (documents written with each feature, default scope), and
`tests/steps/mod.rs` `feature_file_at`/`declare_fixture_routing`.

The docstring fixture was the last unnamed projected-corpus no-document writer; no other
writer or fixture remains unnamed or unexplained.

### N1-N9 and H1-H4/M1-M7 recheck

- N1 stands: `:160` names `write_pending_receipt` and excludes `e2e_toggle_candidate`.
- N2 stands: the two-case retirement rule at `:170` and `:260`; `parse_usecase_md` still
  rejects an empty `## Capabilities` leaf (`src/gherkin/usecase_md.rs:363-372`).
- N3/N4 stand: the permanent-document shape at `:172`; the no-document rule and the named
  writers at `:154`, `:166`, `:255`.
- N5 stands: `:116` keeps `undeclared_capability` and `usecase_interaction_invalid` in the
  `severity_for` blocking set and the `has_use_case_error` exit-code match.
- N6 stands: `:111`, `:264`, `:273`, and `:288` all require both `files` and `titles`.
- N7 stands: `:257` conditions the scope fixtures' greenness on `write_fixture_usecase`
  writing `- none`, and `:252` repeats that it is "not from the routing row alone".
- N8 stands: `:265` keeps the clean-state witness, the merged-candidate gate, and
  finalization.
- N9 stands: `:168` keeps `fragment.md` selection, use-case+capability keying, the
  `Scenario Outline:` count, and the nested/root-level limitations; `:256` records the
  nested boundary.
- H1-H4 stand: the shared builders at `:154`; `write_fixture_usecase` at `:127`, `:154`,
  `:158`, `:168`, `:252`, `:257`, `:272`, `:320`, `:326`; the reverse check at `:116`; the
  fixture-repair audit and the `register-readiness` exception at `:162`/`:164`.
- M1-M7 stand: the narrative-only unit test at `:289` and boundary row at `:258`;
  `MergeProgress` at `:72`, `:102`, `:108`, `:138`; the docs at `docs/reference.md:16`,
  `docs/glossary.md:88-90`, `docs/workflow.md:131-132`, and 8.3c/8.24; R-135/R-137 at
  `docs/arc42/11-risks-and-technical-debt.md:122,124`; the merged-candidate fixture at
  `:156`, `:160`; contracts at `:39-41`; ADR-019 at `arc42.md:70` and ADR-075.

### Plan-integrity checks in this pass

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit `2`;
  `givn lint: 4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP ] finding` lines (7
  `check-specification-health`, 2 `givn-workflow-enforcement`, 3 `archive-proof`). The only
  `parse` strings are advisory `[SHAPE]`/`[SUBST]` scenario titles, so there is no parse
  error.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- Fragment delta: base hash `a219ed71…78c22b` equals
  `sha256sum givn/specs/fragments/fragment.md`; 32 interaction rows against 32 permanent
  fragment `@e2e` titles, set-equal (zero missing, zero extra, no duplicate titles).
- `git status --short givn/specs` is empty: no `@e2e` tag was removed. No `tasks.md` exists
  and no commit was made in this pass.

### Minor items (non-blocking)

1. The Step Definitions table row at `design.md:154` now includes
   `preserve_docstrings_on_archive_steps.rs` in the "Shared fixture repair" file list; the
   paragraph at `:166`, the risks at `:272`, the regression checks at `:320`, and the batch
   plan at `:326` carry the exact ranges and the `- none` repair.
2. The implementation obligations recorded earlier stand: the projection call placement after
   `scan_tree`, the persona feature's non-`@e2e` scenario, and the `register-readiness`
   one-report rule. All are pinned by executable scenarios.
3. The persona fixture reference `tests/steps/mod.rs:1429-1446` covers
   `change_usecase_document_records_persona` (`:1429-1449`); the row change it requires is
   stated at `:172`.

### Outcome

The docstring writer is named with exact ranges in the fixture paragraph and the batch plan,
listed in the risks and the regression checks, repaired with a `- none` `usecase.md`, and its
`@e2e` archive scenario is added to the regression list. The re-scan found no remaining
unnamed projected-corpus writer or no-document fixture: the conflict and Rule fixtures abort
before the projection, `ensure_fixture_usecase` already emits matching rows, and every other
writer is named or cleared. N1-N9, H1-H4, and M1-M7 stand, and the plan-integrity checks
reproduce (exit `2`, 12 `@wip`, no parse error; README check OK; fragment inventory set-equal;
no `@e2e` removed). The plan is closed.

DESIGN-REVIEW: PASS

## In-memory projection revision

This pass reviews the user-directed revision that replaces the temporary
specifications copy with an in-memory `SpecCorpus`, reuses the projection once
per review/archive path, and keeps one merge implementation. The prior sign-off sections above are historical; they validate the
temporary-copy revision and are superseded here. Reviewed: `design.md` (339
lines, including the revised `## Architecture Impact`, `## Data Model`,
`## Projection Semantics`, `## Archive Integration`, `## External Boundary
Cases`, `## Risks`, and `## Test Plan`), `docs/arc42/{04,05,06,08,10,11}`,
`docs/arc42/adr/README.md`, ADR-075, `arc42.md`, `docs/reference.md`,
`docs/glossary.md`, `docs/workflow.md`, and the current primitives in
`src/gherkin/{merge,interaction}.rs`, `src/overlap/analysis.rs`,
`src/cli/{lint,archive}.rs`, `src/spec/owner.rs`, and `src/state.rs`.

### What the revision gets right

- The projection storage decision is sound and is recorded as a decision at
  `design.md:87`: compute a `SpecCorpus` in memory and reuse
  `merge_feature_text`; the rejected temporary copy remains rationale. The
  component row at `:27`, the semantics at `:109-120`, and the boundary row at
  `:272` all say the projection writes no file and creates no tempdir.
- `SpecCorpus` is a text map keyed by project-relative paths (`design.md:25`,
  `:69`), and `gherkin::interaction` is explicitly re-based on a corpus
  (`:28`, `:124`, `:126`). The default `scan_tree` path, lint, review, and
  archive all call the same gate (`:29`, `:32`, `:126`, `:138`); there is no
  second projection path.
- The pre-mutation placement is preserved: fresh archive after
  `build_merge_plan` and before `ensure_e2e_configured` (`:145`), resumed
  before `apply_merge_plan` (`:146`), and the merged candidate on the actual
  permanent corpus before the verification scopes (`:147-148`). Review still
  reaches the projection through `lint::run_scoped` (`:141`).
- `MergeProgress` retains `merged`, `in_progress`, and `retry_unrecorded`
  (`:78`, `:109`), matching `apply_merge_plan`'s receipt fields
  (`src/cli/archive.rs:995-1031`). New capabilities keep the `NEW in
  <usecase>` routing path (`:265`, `:305`); narrative overlay (`:112`), the
  reverse `undeclared_capability` check (`:79`, `:124`), the `- none`
  sentinel (`:79`, `:124`), and the unplaceable-delta scenario (`:122`,
  `:265`) are retained.
- The no-document rule, the persona fixture, the merged-candidate fixture, and
  the shared fixture audit are unchanged (`:162-180`, `:336`). The retirement
  fixture needs the two fixes in M3/M8; the other prior H1-H4/M1-M7 clearances
  stand.
- The plan-integrity checks reproduce: `givn lint --change
  enforce-spec-interaction-integrity` exits `2` with `4 file(s) checked, 12
  finding(s)` and exactly 12 `@wip` scenarios; `givn readme check` prints
  `README check: OK` and exits `0`; the delta fragment base hash is
  `a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65ce14774878c22b`, equal
  to `sha256sum givn/specs/fragments/fragment.md`; the delta adds
  `@e2e` to the init scenario and removes none; `git status --short
  givn/specs` is empty.

### Blocking findings (required edits)

**M1 — The named merge entry point is the scenario-slice API, not the
feature-level merge the archive actually uses.**

`design.md:34` and `:77` say the single merge implementation is
`gherkin::merge::apply_deltas` / `apply_deltas_retry` and that
`merge_feature_text` wraps that core. That is not the archive's merge today.
`apply_deltas_retry` is `#[cfg(test)]` (`src/gherkin/merge.rs:41-42`), so it
cannot be called from `spec::merge_plan` in a production build. The archive's
writer calls `merge_file`/`merge_file_retry`
(`src/gherkin/merge.rs:165-172`), which calls the feature-level
`apply_feature_mode` (`:185-253`, `:277`). `apply_feature_mode` does three
things `apply_deltas` does not: it rejects a conflicting `Background`, it
propagates the delta feature's non-`@givn` tags onto every delta scenario
(including a feature-level `@e2e`), and it builds a new-capability baseline
from the delta feature's tags, description, and background. `apply_deltas`
goes straight to `apply_deltas_mode` with the caller's `DeltaScenario` list
(`:25-36`, `:56-158`). A `merge_feature_text` built on `apply_deltas` would
therefore either omit those rules (the projected corpus can differ from the
published one) or re-implement them in `spec::merge_plan` (a second merge
implementation). The design's central one-merge claim is false as written.

Required edit: state that `merge_feature_text` calls the feature-level merge
core, not the scenario-slice wrappers. Add a production
`apply_feature_deltas_retry` (or make `apply_feature_mode` reachable through a
public strict/retry pair) that funnels into the existing `apply_deltas_mode`,
and say that `apply_deltas_retry` is test-only today and is either promoted or
left internal. Add parity tests for a feature-level `@e2e` tag, a new
capability's feature-level tags/description/background, and a background
conflict; the current unit-test list at `:290-306` does not cover them.

**M2 — The archive writer's location is unspecified and would create a
`gherkin::merge` / `spec::merge_plan` cycle.**

`design.md:34` and `:120` say the archive's `merge_file` is a thin read/write
wrapper around `merge_feature_text`, but `merge_file`/`merge_file_retry` live
in `gherkin::merge` (`src/gherkin/merge.rs:165-172`). `spec::merge_plan`
imports `gherkin::merge` for the merge core, so leaving the writer in
`gherkin::merge` while it calls `spec::merge_plan` creates a module cycle. The
component table puts the thin writer on `cli::archive` (`design.md:30`), which
is the clean direction, but the merge text at `:34`/`:120` does not say so and
the `gherkin::merge` functions would then be orphaned or must delegate.

Required edit: name the writer's home. Either add
`spec::merge_plan::merge_feature_file` (or an archive-local
`write_merged_feature`) that reads the permanent text, calls
`merge_feature_text`, and writes atomically, and delete/redirect
`gherkin::merge::merge_file`/`merge_file_retry`; or keep the writer in
`gherkin::merge` and have it call the same feature-level core directly, with
`merge_feature_text` also calling that core. Preserve `is_delta_feature`
validation (`src/gherkin/merge.rs:261-269`), parent-directory creation, the
`.givn-tmp` rename, and the target-path error context (`:277-292`). State which
functions move in the batch plan at `design.md:336`.

**M3 — The corpus retirement rule omits `retire_capability`'s `fragments`
exemption and its idempotence.**

`design.md:116` says a retired entry "removes the capability feature from the
corpus and, when it is the last capability, removes the narrative". The archive
does not do that unconditionally: `retire_capability`
(`src/cli/archive.rs:1934-1962`) keeps `fragment.md` when the directory is
`fragments` (`:1949-1952`), and treats an already-removed feature or narrative
as a retry, not an error. A projection that removes the fragment narrative
would pass the pre-mutation gate while the archive keeps it, so the
merged-candidate gate would then report the stale declaration after the merge
— a false green followed by a red candidate. For a non-fragment use case the
corpus op must remove the narrative only when no feature remains in that
directory, mirroring `holds_feature_files` (`:1787-1806`); for `fragments` it
must keep the document and let the gate report the resulting
missing-feature/empty-fragment finding before mutation.

Required edit: state the `fragments` exemption, the no-feature-remains rule,
and the idempotent missing-file behavior in `## Projection Semantics`; add a
unit test for a sibling retirement, a last-capability retirement, a
fragments retirement, and an already-applied retirement. The existing
`grouped_merge_steps.rs` fixture only covers the non-fragment `only-cap` case
(`tests/steps/grouped_merge_steps.rs:171-202`), so it does not pin the
fragments rule.

**M4 — The unplaceable plan error has no typed boundary, so the required
finding cannot be produced without string matching.**

`design.md:122` says the convenience entry returns a blocking
`interaction_mismatch` with `capability '<cap>' cannot be placed in the
projected corpus: <error>` when `build_merge_plan` cannot place a delta
capability, and that other plan errors propagate. `build_merge_plan` returns
`anyhow::Result<Vec<MergeEntry>>`; the unplaceable case is an `anyhow!` string
in `resolve_permanent_dir` (`src/cli/archive.rs:1831-1849`) and the routing
lookup (`:1911-1920`). There is no variant the projection can match, so the
implementation would have to compare the `archive blocked:` text, which is
fragile and conflates the archive's message with the projection's finding.

Required edit: add a typed plan error in `spec::merge_plan` (for example
`MergePlanError::UnplaceableCapability { cap, detail }`) and have
`interaction_findings` match only that variant; name the finding `files` (the
delta feature path and the change specs root) and keep the `capability '<cap>'`
title. The raw-delta scenario at
`givn/changes/enforce-spec-interaction-integrity/specs/verify-change/check-specification-health.feature:52-63`
depends on this distinction.

**M5 — The corpus read/parse boundary and the `gherkin::interaction` /
`scan_tree` signatures are not defined for a fallible corpus.**

`design.md:25` says `SpecCorpus` exposes "capability/feature maps", `:69` says
it stores feature and narrative text, `:111` says `SpecCorpus::read` fails
with a named path on an unreadable or non-UTF-8 entry, `:124` says the gate
parses from the corpus, and `:271` says a corpus read or parse failure fails
the command closed. The current signatures are infallible:
`scan_tree(root) -> Vec<Finding>` (`src/overlap/analysis.rs:107-126`) and
`interaction_gate_findings(root) -> Vec<Finding>`
(`src/gherkin/interaction.rs:112-176`), with `usecase_coverages(root) ->
Result<...>` (`:42-110`). The design does not say which function parses, which
one returns `Result`, or how a missing `givn/specs` differs from an unreadable
one (today `usecase_coverages` returns an empty list for a missing directory,
`src/gherkin/interaction.rs:44-48`).

Required edit: define the boundary explicitly. Choose one entry point and
state it: `SpecCorpus::read(project_root: &Path) -> Result<SpecCorpus>` with
keys `givn/specs/<usecase>/<file>`, or `read(specs_root: &Path)` with
`<usecase>/<file>` keys and a fixed `givn/specs/` render in the gate. Use the
chosen form at `:69`, `:111`, and `:114`. The reader reads only `usecase.md`,
`fragments/fragment.md`, and `*.feature` files, skips `.retired.yaml` and
temporary files, returns an empty corpus for a missing specs root, and returns a
named `Result` error for I/O or non-UTF-8. `gherkin::interaction::interaction_gate_findings(&SpecCorpus) ->
Vec<Finding>` parses narratives and feature text and converts a parse error into
a blocking `usecase_interaction_invalid` finding (as today, `:114-126`), not an
`Err`. `scan_tree` keeps its infallible signature and maps a `SpecCorpus::read`
error to the same blocking finding (or the design must say it becomes
`Result` and update both call sites, `src/cli/lint.rs:212` and
`src/overlap/analysis.rs:604`). `permanent_interaction_findings` and
`interaction_findings_for_plan` are fallible for the read and return
`Result<Vec<Finding>>`; lint and archive fail with the named path. Remove or
re-base `usecase_coverages` on the corpus; a path-based helper left in place
would be the second gate path the revision is meant to eliminate. Update the
test-plan line at `:329`, which still says the in-module tests are "re-rooted to
the specifications-root contract"; they must build a `SpecCorpus` and call the
corpus gate.

**M6 — The logical path key and `MergeEntry.target` mapping are
under-specified, and the receipt path fallback can escape the corpus.**

`design.md:25`, `:69`, and `:111` mix "project-relative paths under
`givn/specs`" with reading `project_root/givn/specs`; `:114` says "compute the
target's project-relative path under `givn/specs`" but `MergeEntry.target` is a
feature file for a merge (`src/cli/archive.rs:943`) and the use-case directory
for a retirement (`:926`), so the retirement feature key must be
`target/<cap>.feature`. `receipt_plan` currently strips `project_root` with
`unwrap_or(&entry.target)` (`:370-387`), so a receipt target outside the
project is stored as an absolute path; the projection would then either key
outside `givn/specs` or fail without a stated error.

Required edit: define keys as `givn/specs/<usecase>/<file>` (project-relative,
matching `Finding.files`) and one
`logical_target(project_root, target) -> Result<PathBuf>` helper that strips the
project root and requires the result to start with `givn/specs`; a target
outside the specs root is a named command error, not the
`unwrap_or(&entry.target)` fallback. State the merge-vs-retire target
derivation at `:114` (merge: the target file; retirement:
`target/<cap>.feature`), and use the helper for `receipt_plan`, the projection
keys, and the finding `files` (`:118`, `:272`). Normalize separators to `/`
as `receipt_plan` does (`src/cli/archive.rs:378-384`), so a `PathBuf` key on
Windows and the `Finding.files` string agree. A unit test should assert the
key and the outside-target refusal; `:296` currently asserts only that the
projection creates no tempdir.

**M7 — ADR-075 still describes a temporary copy as the chosen option.**

`docs/arc42/adr/ADR-075-projected-interaction-integrity.md:123`, under
"Pros and Cons of the Options → Project before mutation", says the chosen
option "copies the specification corpus per review and archive". That is the
rejected temporary-copy design, not the in-memory revision. The other durable
docs are consistent: `docs/arc42/04-solution-strategy.md:55-66`,
`05-building-block-view.md:105`, `06-runtime-view.md:180-190,394-410`,
`08-crosscutting-concepts.md:161-188`, `11-risks-and-technical-debt.md:122-124`,
and `arc42.md` all say in-memory `SpecCorpus` and no temporary path.

Required edit: replace that clause with "reads the specification corpus into
an in-memory `SpecCorpus` per review and archive" (or move the copy cost into
the rejected temporary-copy option). The rejection itself may remain as
rationale.

**M8 — The last-capability retirement fixture writes a stray change narrative
that the new merged-candidate gate rejects.**

`design.md:178` says a last-capability retirement omits the change-local
narrative, but `tests/steps/grouped_merge_steps.rs:172-200` calls
`write_change_usecase(&change_dir, &cap, &cap)` for `retire-last` / `only-cap`;
the permanent owner is `retire-home` (`:154-169`). `merge_usecase_documents`
(`src/cli/archive.rs:1769-1781`) writes the change narrative to
`givn/specs/only-cap/usecase.md` via `narrative_target` (`:1690-1710`), while
`retire_capability` removes `retire-home/only-cap.feature` and
`retire-home/usecase.md`. The merged candidate therefore contains
`only-cap/usecase.md` declaring a capability with no feature; the new
`permanent_interaction_findings` gate reports `usecase_interaction_invalid` and
the scenario `Retiring a use case's last capability removes the use-case
directory and its narrative`
(`givn/specs/archive-change/grouped-merge-for-new-capabilities.feature:28-36`,
which expects exit `0`) fails after the merge. The design's fixture-repair list
names `grouped_merge_steps.rs` (`:176`, `:336`) but not this edit, and
`design.md:331` claims the existing archive proof suite remains green.

Required edit: state the exact fixture change at `design.md:178` and `:336` —
omit `write_change_usecase` in `retiring_change` for the last-capability case,
or write the narrative under the permanent owner with the base hash and let
`retire_capability` remove it. If the narrative is kept at `specs/only-cap/`,
the design must state why `givn/specs/only-cap/usecase.md` is not part of the
merged corpus (it is) and why the merged-candidate gate does not report it.

### Minor required edits

1. `design.md:78` and `:267` still say `retry_unrecorded` applies entries with
   `merge_file_retry`. The projection never calls the file writer; say
   `merge_feature_text(..., retry = true)` / retry mode. The archive's legacy
   branch can keep the `merge_file_retry` reference in its own description.
2. `design.md:5` justifies the design with "`tempfile` is already a
   dependency". The in-memory revision needs no production tempdir; say no new
   dependency is needed and `tempfile` remains a test-fixture dependency, or
   remove the clause.
3. `design.md:281` says "the current 58-file corpus is small"; the tree has 58
   `.feature` files plus 18 narratives and `.retired.yaml` (77 files, about
   740 KiB). The risk is still low, but the count should not understate the
   corpus the projection reads and parses.
4. `design.md:25` says `SpecCorpus` exposes "capability/feature maps" while
   `:69` says feature text. Either say the gate builds the maps from text (as
   M5) or say the corpus carries parsed features; do not leave both.

### Requested checks

| Check | Result | Evidence |
|---|---|---|
| `SpecCorpus` reads `givn/specs` into narrative/feature text keyed by project-relative paths; the projection overlays in memory and never writes a tempdir or file | **Pass in intent, under-specified keys** | `design.md:25,27,69,87,109-120,272`; no tempdir in the design. The key format and target mapping are M6. |
| `gherkin::interaction` accepts a `SpecCorpus`; `scan_tree` builds the same corpus and calls the same gate; lint/review/archive reuse it; no second projection path | **Pass in intent, API undefined** | `design.md:28-32,124-126,138`; current path-based gate at `src/gherkin/interaction.rs:112-176` and `scan_tree` at `src/overlap/analysis.rs:107-126`. M5. |
| One merge implementation; archive writer is a thin wrapper; projection calls the same text function | **Fail** | Design names the scenario-slice `apply_deltas`/`apply_deltas_retry` (`design.md:34,77`); `apply_deltas_retry` is test-only (`src/gherkin/merge.rs:41-42`); the archive uses feature-level `apply_feature_mode` (`:185-253,277`). M1/M2. |
| `MergeProgress` (including `retry_unrecorded`), new capabilities, retirement, narrative overlay, and reverse `undeclared_capability` are preserved | **Partial** | `MergeProgress` and routing are preserved (`design.md:78,109,265`); retirement omits the fragments exemption/idempotence (M3); `retry_unrecorded` wording is stale (minor 1). |
| Projection runs before mutation in review/archive; merged-candidate check reads the actual corpus with the same gate | **Pass** | `design.md:141,145-148`; current call sites `src/cli/archive.rs:173-201,251-258` and `src/cli/check.rs:154-165`. |
| Durable docs no longer describe a temporary copy as the current design | **Fail** | ADR-075 line 123 (M7); the other docs pass. |
| Fixture repair, no-document rule, persona/retirement/merged-candidate fixtures, shared fixture audit unaffected | **Fail for the retirement fixture** | The no-document rule, persona fixture, merged-candidate fixture, and shared audit are unchanged (`design.md:162-180,336`); the retirement corpus rule is M3 and the retirement fixture's stray narrative is M8. |
| `givn lint --change enforce-spec-interaction-integrity` parses the delta (exit `2`, 12 `@wip`); `givn readme check` passes | **Pass** | Executed below. |

### Executed checks

- `cargo run --quiet -- lint --change enforce-spec-interaction-integrity`:
  exit `2`; `givn lint: 4 file(s) checked, 12 finding(s)`; 12 `[WIP ]`
  findings (7 `check-specification-health`, 2
  `givn-workflow-enforcement`, 3 `archive-proof`); no parse-error finding.
- `cargo run --quiet -- readme check`: `README check: OK`, exit `0`.
- `sha256sum givn/specs/fragments/fragment.md` =
  `a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65ce14774878c22b`, equal
  to the delta narrative's `<!-- givn:base-sha256:... -->` header.
- Delta `@e2e` tags: 6 occurrences, all on scenarios; the init delta adds
  `@e2e` to `Consumer runs init non-interactively and gets opencode skills`
  (`givn/changes/enforce-spec-interaction-integrity/specs/initialize-project/initialize-a-project.feature:5`).
  `git status --short givn/specs` is empty: no `@e2e` tag was removed. No
  `tasks.md` was written and no commit was made in this pass.

### Outcome

The in-memory model is the right direction: it removes the tempdir, keeps one
projection, and preserves the pre-mutation and merged-candidate gates. The
revision cannot be signed off as written because the design names the
scenario-slice merge instead of the feature-level merge the archive uses
(M1/M2), the retirement rule does not mirror `retire_capability`'s fragments
exemption (M3), the unplaceable error has no typed boundary (M4), the corpus
read/parse and gate signatures are undefined (M5), the logical path key and
target mapping are ambiguous (M6), ADR-075 still describes the rejected
temporary copy as the chosen option (M7), and the last-capability retirement
fixture writes a stray narrative the new merged-candidate gate rejects (M8).
Apply the exact edits above and
re-run the two plan-integrity checks; the remaining fixture and scenario
obligations are unchanged.

Earlier verdict (superseded): FAIL

## In-memory revision — verification after the M1-M8 update

This pass verifies the author's response to the preceding FAIL. Reviewed:
`design.md` (343 lines), ADR-075, the change delta specs, and the current
primitives at `src/gherkin/{merge,interaction}.rs`, `src/cli/{archive,lint}.rs`,
`src/overlap/analysis.rs`, `src/spec/`, and `tests/steps/grouped_merge_steps.rs`.
The prior markers above are historical. The prior in-memory FAIL is
superseded by this section.

### M1-M8 verification

| Finding | Status | Evidence |
|---|---|---|
| M1 feature-level merge core | **Fixed** | `design.md:34` names `gherkin::merge::apply_feature_deltas` plus a new public `apply_feature_deltas_retry`, and scopes out the scenario-slice `apply_deltas`/`apply_deltas_retry`. `apply_feature_deltas` is public at `src/gherkin/merge.rs:178-183` and enters `apply_feature_mode`, which holds the Background conflict (`:193-202`), the non-`@givn` tag inheritance (`:203-211`), and the new-capability header (`:218-247`). The test-only `apply_deltas_retry` (`:41-42`) is no longer the claimed core. The existing tests pin inherited tags, a new feature's tags/background, and the Background conflict (`src/gherkin/merge.rs` tests at `:461-505`). Non-blocking: the test plan (`design.md:291-310`) still adds no parity row for those three behaviours through `merge_feature_text`. |
| M2 writer home / no cycle | **Fixed in structure** | `design.md:34` and `:122` move the thin writer to `spec::merge_plan::write_feature`/`write_feature_retry`, keep `gherkin::merge` text-only, and make `cli::archive` call `write_feature`; the dependency is `spec::merge_plan -> gherkin::merge`, so the `gherkin::merge` -> `spec::merge_plan` cycle is gone. Residual required detail: the design does not restate the writer's preserved guards — `is_delta_feature` (`src/gherkin/merge.rs:261-269`), parent-directory creation, the `.givn-tmp` rename, and the target-path error context (`:279-291`). "Thin read/write wrapper" alone does not tell the implementer to keep the delta-tag refusal. |
| M3 retirement semantics | **Partial** | `design.md:79` and `:118` now mirror `retire_capability` (`src/cli/archive.rs:1934-1962`): the `fragments` exemption, the no-feature-remains rule, and the already-absent feature no-op. The four unit tests the prior finding required are absent: the test plan at `:298` covers only "a retired capability removes the feature", and `:300` says "retired" generically. No test pins the `fragments` retirement — the false-green branch named in the finding. |
| M4 typed plan error | **Fixed** | `design.md:124` defines `MergePlanError::UnplaceableCapability { capability, detail }`, matches that variant in the convenience entry, keeps the `capability '<cap>' cannot be placed...` title, and propagates other plan errors. The raw-delta scenario at `specs/verify-change/check-specification-health.feature:46-59` depends on exactly that boundary. |
| M5 corpus read/parse boundary | **Partial** | The contract is now explicit: `SpecCorpus::read(specs_root)` with `givn/specs/...` keys, a missing root as an empty corpus, other read failures named (`design.md:69,113,273-274`); `interaction_gate_findings(&SpecCorpus) -> Vec<Finding>` is infallible with parse errors as blocking `usecase_invalid` findings (`:69,113,119`); `scan_tree` maps a read error to a blocking finding (`:136`); the projection returns the read error (`:113,274`); the unit tests are re-rooted (`:333`). `usecase_invalid` is the existing parse-error kind (`src/cli/lint.rs:230`, `src/overlap/analysis.rs:136`). Residual: the path-based `usecase_coverages` (`src/gherkin/interaction.rs:42`, consumed at `:115`) is not named as re-based on `&SpecCorpus` or removed, so the one-read-path claim stays unpinned; and `design.md:259` ("the gate returns a named parse error") contradicts the blocking-finding contract at `:69`/`:274`. |
| M6 logical target mapping | **Partial** | `design.md:78` adds `target_rel: PathBuf` under `givn/specs` (feature file for a merge, use-case directory for a retirement) and says `receipt_plan` and the projection use it with no absolute fallback; `:116-117` keys the merge by it. Residual: `plan_from_receipt` (`design.md:149`; `src/cli/archive.rs:950-980`) builds entries from `receipt.merge_targets`, and the design says only `build_merge_plan` resolves `target_rel` — the receipt branch is unstated, so an implementation can push an empty or absolute key. No behaviour is defined for a receipt target outside `givn/specs` (the old `unwrap_or(&entry.target)` at `src/cli/archive.rs:378-384` writes it absolute), no `/` separator normalization is stated, no unit test pins the key or the refusal (`:299` asserts only logical paths plus no tempdir), and `:118` does not state the retirement feature key `target_rel/<cap>.feature`. |
| M7 ADR-075 | **Fixed** | `docs/arc42/adr/ADR-075-projected-interaction-integrity.md:123` now says the chosen option "reads the specification corpus into memory per review and archive"; the temporary-copy clause is gone and the rejection remains rationale. |
| M8 last-capability fixture | **Fixed** | `design.md:181` names the exact edit — remove the `specs/only-cap/usecase.md` write at `grouped_merge_steps.rs:172-201` for the last-capability case. The call is `write_change_usecase(&change_dir, &cap, &cap)` at `tests/steps/grouped_merge_steps.rs:199`, inside `retiring_change` (`:172-201`). `:271`, `:335`, and `:340` carry the sibling versus last-capability rule. |

### Minor items

1. **Fixed.** No `merge_file*` reference remains in `design.md`; `:80` and
   `:270` say `merge_feature_text(..., retry = true)`.
2. **Fixed.** `design.md:5` says no new dependency and no tempdir, and keeps
   `tempfile` for test fixtures only.
3. **Fixed.** `design.md:284` and `:286` say 58 feature files and 77 files;
   verified below.
4. **Partial.** `design.md:69` says the corpus holds text maps, `:284` says
   "the specification text plus parsed features", and `:25` says the corpus
   exposes "the logical narrative and feature maps". The text-versus-parsed
   ambiguity the prior pass flagged is still present in `:284`.

### Preserved revision content

- `spec::corpus` and `spec::projection` are reused by lint, review, and archive;
  default lint builds the same corpus and calls the same gate
  (`design.md:29-32,128,136,141,144-150`).
- The projection writes no file and creates no tempdir
  (`design.md:27,89,120,122,299`); the rejected temporary copy remains
  rationale at `:89`.
- `MergeProgress` with `merged`, `in_progress`, and `retry_unrecorded`, the
  fresh/resumed/merged-candidate placements, and the retry mapping are intact
  (`design.md:80,111,116,148-150,270,305`).
- The reverse `undeclared_capability` check, the `- none` sentinel, the
  no-document rule, the shared fixture audit, the persona fixture, and the
  merged-candidate pending-receipt fixture are intact
  (`design.md:81,126,167-179,264,306,334,340`).
- The retirement fixture now follows the sibling and last-capability cases
  (`design.md:181,271,335`).

### Executed checks

- `cargo run --quiet -- lint --change enforce-spec-interaction-integrity`:
  exit `2`; `4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP ]` findings
  (7 `check-specification-health`, 2 `givn-workflow-enforcement`, 3
  `archive-proof`); no parse-error finding.
- `cargo run --quiet -- readme check`: `README check: OK`, exit `0`.
- `find givn/specs -name '*.feature' | wc -l` = 58;
  `find givn/specs -type f | wc -l` = 77; `du -sh givn/specs` = 740K,
  matching `design.md:284,286`.
- `sha256sum givn/specs/fragments/fragment.md` =
  `a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65fe14774878c22b`, equal to
  the delta narrative's base hash at `design.md:74`.
- Delta `@e2e` tags: 6 occurrences, all on scenarios; the init delta adds
  `@e2e` to `Consumer runs init non-interactively and gets opencode skills`
  (`specs/initialize-project/initialize-a-project.feature:5`).
  `git status --short givn/specs` is empty: no `@e2e` tag was removed and no
  permanent spec was edited. No `tasks.md` was written and no commit was made.

### Remaining edits

1. `design.md:298` — add the four retirement unit tests: sibling retirement,
   last-capability retirement, `fragments` retirement, and already-applied
   retirement.
2. `design.md:78` and `:149` — state that `plan_from_receipt` populates
   `target_rel` from `receipt.merge_targets[rel]`, and define the named command
   error for a target outside `givn/specs` (no `unwrap_or` fallback, no empty
   key). Add the `/`-normalization clause and the `:299` unit test for the key
   and the outside-target refusal.
3. `design.md:118` — state the retirement feature key `target_rel/<cap>.feature`
   explicitly.
4. `design.md:69` or `:113` — name `usecase_coverages` as re-based on
   `&SpecCorpus` (or removed), and correct `:259` so a parse failure is the
   blocking finding the rest of the design describes.
5. `design.md:34` or `:122` — state that `write_feature` preserves
   `is_delta_feature` validation and the `.givn-tmp` atomic rename.
6. `design.md:284` — drop "plus parsed features" or make the corpus carry
   parsed features, aligning with the text maps at `:69`.

### Outcome

The revision fixes M1 (feature-level core), M2's cycle, M4 (typed plan error),
M7 (ADR-075), and M8 (last-capability fixture), and it preserves the in-memory
projection, the single corpus/gate reuse, the pre-mutation and merged-candidate
placements, the retry mapping, the reverse check, and the fixture audit. It is
not signable yet: M3's required retirement tests are missing, M6 leaves the
resumed-receipt `target_rel` and the outside-specs refusal undefined, M5 leaves
the path-based parser unnamed, and `design.md:259`/`:284` still contradict the
rest of the design. Apply the six edits above and re-run the two
plan-integrity checks.

Earlier verdict (superseded): FAIL

## In-memory revision 2 — verification after the M2/M3/M5/M6 update

Second verification pass on the author's response to the preceding FAIL.
Reviewed `design.md` (345 lines), `proposal.md`, `arc42.md`, ADR-075, every
delta `.feature`, the delta `specs/fragments/fragment.md`, and the current
primitives at `src/gherkin/{merge,interaction,delta,usecase_md}.rs`,
`src/cli/{archive,lint,check}.rs`, `src/overlap/analysis.rs`, `src/spec/`, and
the fixture writers under `tests/steps/` and `tests/common/mod.rs`. The markers above are historical; this section supersedes the
prior in-memory FAIL.

### Claimed fixes

| Claim | Status | Evidence |
|---|---|---|
| M3 four retirement unit tests | **Fixed** | `design.md:301` names exactly the four required cases: sibling retirement, last-capability retirement, the `fragments` narrative exemption, and the already-applied no-op. Each maps to a real branch of `retire_capability` (`src/cli/archive.rs:1934-1981`): the absent-feature no-op (`:1940-1944`), the `fragments` exemption (`:1949-1952`), `holds_feature_files` (`:1952`, defined `:1787-1802`), and the narrative removal (`:1953-1961`). |
| M5 corpus signatures | **Fixed except one line** | `design.md:69` re-bases `usecase_coverages(&SpecCorpus)` and `interaction_gate_findings(&SpecCorpus) -> Vec<Finding>`, states the gate is infallible after the read, and gives `SpecCorpus::read` the named-error contract; `:113` and `:119` drive both from the corpus; `:273-274` separate a missing root (empty corpus) from an unreadable root (named error); `:304` and `:312` carry the read/parse tests. The replaced root signatures exist at `src/gherkin/interaction.rs:42` and `:114`. Residual: `design.md:259` still says the gate returns a named parse error. |
| M6 `target_rel` | **Fixed** | `design.md:78` defines the forward-slash-normalized key under `givn/specs`, the merge-file versus retirement-directory shape, `build_merge_plan` and `plan_from_receipt` resolution with the outside-specs refusal, and the "no absolute-path fallback" rule for `receipt_plan` and the projection; `:149` repeats the receipt branch; `:79` fixes the retirement feature key `target_rel/<cap>.feature`; `:302` adds the normalization/refusal/resume unit test. The code to change is `MergeEntry` (`src/cli/archive.rs:362-368`), `receipt_plan`'s `strip_prefix(project_root).unwrap_or(&entry.target)` (`:370-388`), and `plan_from_receipt`'s `receipt.merge_targets` lookup and `project_root.join` (`:950-991`). |
| M2 writer guards | **Fixed** | `design.md:34` states that `write_feature`/`write_feature_retry` preserve the `is_delta_feature` guard and the atomic `.givn-tmp` write/rename. The moved body is `merge_file_mode` (`src/gherkin/merge.rs:256-295`): guard `:261-269`, `.givn-tmp` `:289-291`; parent-directory creation and the target-path error context move with that extracted body. |
| Minor: corpus memory wording | **Fixed** | `design.md:284` now says "The corpus is the specification text"; `:69` declares text maps only, and no "parsed features" claim remains anywhere in the design. |

### M1-M8 status

| Finding | Status | Evidence |
|---|---|---|
| M1 feature-level merge core | **Fixed** | `design.md:34` names `gherkin::merge::apply_feature_deltas` plus a new public `apply_feature_deltas_retry` and scopes out the scenario-slice wrappers. `apply_feature_deltas` is public at `src/gherkin/merge.rs:178-183` and enters `apply_feature_mode` (`:185`), which owns the Background conflict (`:197-205`), the non-`@givn` tag inheritance (`:209-213`), and the new-capability header (`:222-252`). `apply_deltas_retry` remains `#[cfg(test)]` (`:41-42`) and is not claimed as the core. Non-blocking carry-over: the test plan adds no parity row for those three behaviours through `merge_feature_text`; the existing merge unit tests (`src/gherkin/merge.rs:460` onward) pin them at the shared core. |
| M2 writer home / no cycle | **Fixed** | `design.md:34`, `:77`, `:122`, and `:303` put the writer in `spec::merge_plan`, keep `gherkin::merge` text-only, and make `cli::archive` call `write_feature`; the dependency is `spec::merge_plan -> gherkin::merge`, so the former `gherkin::merge -> spec::merge_plan` cycle is gone. The preserved guard list at `:34` covers the load-bearing parts (delta tag, atomic rename). |
| M3 retirement semantics | **Fixed** | `design.md:79` and `:118` mirror `retire_capability` (`src/cli/archive.rs:1934-1981`): `fragments` exemption, no-feature-remains narrative removal, already-absent no-op, and no `.retired.yaml` in the projection (`:118`). The four unit tests are at `:301`. |
| M4 typed plan error | **Fixed** | `design.md:124` defines `MergePlanError::UnplaceableCapability { capability, detail }`, matches only that variant in the convenience entry, keeps the `capability '<cap>' cannot be placed in the projected corpus` title, and propagates other plan errors. The raw-delta scenario at `specs/verify-change/check-specification-health.feature:45-59` exercises exactly that boundary. |
| M5 corpus read/parse boundary | **Partial** | The boundary is explicit at `:69`, `:113`, `:273-274`, `:304`, `:312`; `usecase_coverages` is re-based rather than left as a second path; `scan_tree` maps a read error to a blocking finding (`:136`) while the projection returns the named read error (`:69`, `:274`). `usecase_invalid` is the existing use-case read/parse kind (`src/overlap/analysis.rs:351` read, `:397-401` parse; blocking set `src/cli/lint.rs:236-241`). The one remaining contradiction is `design.md:259`. |
| M6 logical target mapping | **Fixed** | `design.md:78`, `:79`, `:116-117`, `:149`, and `:302` cover the key shape, normalization, the outside-specs refusal in both `build_merge_plan` and `plan_from_receipt`, the removal of the `unwrap_or` absolute fallback, the retirement feature key, and the unit test. `receipt_plan` and the projection consume `target_rel` directly. |
| M7 ADR-075 | **Fixed** | `docs/arc42/adr/ADR-075-projected-interaction-integrity.md:100-103` says the chosen option "reads the specification corpus into memory per review and archive"; the temporary-copy clause survives only in the rejected option. The index lists ADR-075 as accepted (`docs/arc42/adr/README.md:62`). |
| M8 last-capability fixture | **Fixed** | `design.md:181` names the exact edit (omit the change-local narrative for the last-capability case); the call is `write_change_usecase(&change_dir, &cap, &cap)` at `tests/steps/grouped_merge_steps.rs:199` inside `retiring_change` (`:171-201`). The stray document is real: `merge_usecase_documents` maps it through `narrative_target` (`src/cli/archive.rs:1690-1710`) to `givn/specs/<cap>/usecase.md`. Minor wording: `:271`, `:335`, and `:342` say the fixture follows "the two cases", but the only retirement scenario (`givn/specs/archive-change/grouped-merge-for-new-capabilities.feature:28-36`) is the last-capability case; the sibling deletion is pinned by the `:301` unit test. |

### In-memory constraints recheck

- No temporary directory and no file write in the projection: `design.md:27`,
  `:89` (temporary copy kept as rejected rationale), `:118` (no
  `.retired.yaml`), `:120`, `:122`, `:275`, `:299`. `tempfile` is declared
  test-fixture-only at `:5`.
- One `merge_feature_text`: twelve references (`:34,41,77,80,89,117,122,249,270,282,303,342`);
  no `merge_file` reference remains in the design; `:34`, `:122`, and `:303`
  pin the archive writer and the projection to the same function.
- Default lint, review, and archive reuse the same corpus and gate:
  `design.md:29-32`, `:69`, `:128`, `:136-141`, `:144-150`. Review reaches it
  through `cli::check::run_mandatory_checks` -> `lint::run_scoped`
  (`src/cli/check.rs:155`); default lint reaches it through `scan_tree`
  (`src/overlap/analysis.rs:118`).
- `MergeProgress`: `design.md:80` (fields and the `legacy_unrecorded_merge`
  mirror), `:111` (fresh versus resumed), `:116` (skip merged, retry
  in-flight/legacy), `:148-150` (all three placements), `:270`, `:305-307`.
  The code mirror is `src/cli/archive.rs:1006-1021`: skip `merged_deltas`,
  `retry = legacy_unrecorded_merge || merge_in_progress == relative`.
- Reverse check: `design.md:81`, `:126` (title, blocking severity, exit
  `1`), `:264`, `:266`, `:308`. The lint blocking set and `severity_for`
  exist at `src/cli/lint.rs:236-241` and `:534-547`.
- No-document rule: `design.md:177`, `:179`, `:266`, `:283`. The cited
  fixture ranges write permanent features without a document
  (`readme_enforcement_steps.rs:244-279`,
  `workflow_enforcement_steps.rs:910-916,1025-1063`,
  `enforce_post_merge_e2e_gate_steps.rs:47-59,209-221`,
  `preserve_docstrings_on_archive_steps.rs:99-110,180-206,267-284`,
  `grouped_merge_steps.rs:521-555`); `repository_ownership_fixtures_support.rs`
  writes two directories (`:21`, `:25`).
- Fixture audit: `design.md:165`, `:173`, `:336`, `:342`. The writer to repair
  is real: `tests/common/mod.rs:578-593` always writes the
  `No E2E interaction` row and never `fragment.md`; it is called from
  `declare_fixture_routing` (`:540-577`), which `feature_file_at` invokes at
  `tests/steps/mod.rs:379`.

### Executed checks

- `cargo run --quiet -- lint --change enforce-spec-interaction-integrity`:
  exit `2`; `4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP ]` findings
  (7 `check-specification-health`, 2 `givn-workflow-enforcement`,
  3 `archive-proof`); no parse-error finding.
- `givn lint --change enforce-spec-interaction-integrity` (installed 0.9.17):
  identical output, exit `2`, 12 `[WIP ]`.
- `givn readme check`: `README check: OK`, exit `0`.
- `cargo run --quiet -- lint` on the permanent corpus: exit `1`, exactly 11
  blocking interaction mismatches (10 fragment capabilities plus
  `initialize-a-project`), matching `proposal.md`; 23 findings total.
- The delta `specs/fragments/fragment.md` declares 32 interaction rows across
  15 fragment capabilities, and every declared title equals the capability
  feature's actual `@e2e` titles (script check, zero mismatches); it carries
  19 rows beyond the permanent 13. Its base hash equals
  `sha256sum givn/specs/fragments/fragment.md` =
  `a219ed7197dd6d8a424ca7f418d7ef956c89ec7fa116b2ce65ce14774878c22b`.
- The init delta is the permanent scenario body plus `@e2e`; it is the only
  `@e2e` tag in the delta specs. The other five `@e2e` occurrences are the new
  assertion strings.
- Corpus: 58 `.feature` files, 77 files, 740K, matching `design.md:284,286`.
  No undeclared feature and no feature directory without a document exists
  under `givn/specs` (script over 18 directories), so the reverse check and
  the arc42 claim hold after the repair.
- `git status --short givn/specs` is empty; no `tasks.md` exists; no commit
  was made.

### Remaining edits

1. `design.md:259` (blocking) — replace "the gate returns a named parse error"
   with the boundary the rest of the design states: "Corrupt use-case
   document: the infallible gate returns a blocking `usecase_invalid`
   finding; an unreadable document is a named `SpecCorpus::read` error. The
   command fails closed." This is the half of the prior edit 4 that was not
   applied; `:69` and `:274` are already correct.
2. `design.md:271`, `:335`, `:342` (minor) — say the fixture keeps the
   last-capability case and the sibling retirement is pinned by the `:301`
   unit test, or add the sibling scenario the sentences claim.

### Outcome

The revision fixes M3 (four retirement unit tests), M5's corpus signatures
and read/parse boundary, M6 (normalized `target_rel`, receipt recomputation,
outside-specs refusal, retirement key, unit test), M2's preserved writer
guards, and the corpus-memory wording. It preserves the in-memory projection
with no file write, the single `merge_feature_text`, the shared
corpus/gate on default lint, review, and archive, the `MergeProgress` retry
mapping, the reverse check, the no-document rule, and the full fixture audit.
One blocking contradiction remains: `design.md:259` still describes a fallible
gate, against the infallible `interaction_gate_findings(&SpecCorpus) ->
Vec<Finding>` contract at `:69`, `:113`, and `:274`. Apply the `:259` rewrite
(and the minor fixture wording) and the revision is signable; the two
plan-integrity checks already pass.

Earlier verdict (superseded): FAIL

## In-memory revision 3 — verification after the `:259` and fixture-wording update

Third verification pass on the author's response to the preceding FAIL. Reviewed
`design.md` (345 lines), the retirement fixture ground truth
(`givn/specs/archive-change/grouped-merge-for-new-capabilities.feature:28-36`,
`tests/steps/grouped_merge_steps.rs:171-201`), and the current primitives at
`src/gherkin/{merge,interaction}.rs`, `src/cli/{lint,archive}.rs`,
`src/overlap/analysis.rs`, and `docs/arc42/adr/ADR-075-*.md`. The
prior markers above are historical; this section supersedes the prior
in-memory FAIL.

### Claimed fixes

| Claim | Status | Evidence |
|---|---|---|
| `design.md:259` names a blocking `usecase_invalid` finding instead of a named parse error | **Partial** | The parse half is fixed: a corrupt document is the gate's blocking finding, matching `:69`, `:113`, `:274`, and `:304`. The unreadable half is not: `:259` still routes an unreadable document through the gate, while `:69` ("any other read failure is a named error"), `:274` ("Corpus read failure: the command fails with a named path"), `:312` ("An unreadable file returns a named error"), and `:260` (an unreadable feature fails the projection with its path) all put that case on the error path. |
| Three fixture-wording locations say the fixture keeps the last-capability case and the sibling case is unit-tested | **Partial** | `:283`, `:336`, and `:342` carry the new wording and match the ground truth. `:271` still ends "The `grouped_merge_steps.rs` fixture follows the two cases." That is the same claim the previous FAIL named, and it is false: the only retirement scenario is the last-capability one (`grouped-merge-for-new-capabilities.feature:28-36`), and the sibling retirement is the `:301` unit test. |

Fixture ground truth: `retiring_change` (`tests/steps/grouped_merge_steps.rs:171-201`)
writes `specs/<cap>/usecase.md` through `write_change_usecase(&change_dir, &cap, &cap)`
at `:199`; the scenario at `grouped-merge-for-new-capabilities.feature:28-36` retires
`only-cap` from `retire-home`; no sibling-retirement scenario exists in any permanent
feature file (`grep` over `givn/specs/*/*.feature` returns only that one retirement
scenario). The `:181` edit that removes the stray narrative write is therefore correct,
and the `:283`/`:336`/`:342` wording now matches it.

### M1-M8 recheck

| Finding | Status | Evidence |
|---|---|---|
| M1 feature-level merge core | **Fixed** | `design.md:34` names `gherkin::merge::apply_feature_deltas` plus a new public `apply_feature_deltas_retry` and scopes out the scenario-slice wrappers. `apply_feature_deltas` is public at `src/gherkin/merge.rs:178-183` and enters `apply_feature_mode`, which owns the Background conflict, the non-`@givn` tag inheritance, and the new-capability header; `apply_deltas_retry` stays `#[cfg(test)]` (`src/gherkin/merge.rs:41-42`). Non-blocking carry-over: the test plan still has no parity row for those three behaviours through `merge_feature_text`. |
| M2 writer home / no cycle | **Fixed** | `design.md:34`, `:77`, `:122`, and `:303` put the writer in `spec::merge_plan`, keep `gherkin::merge` text-only, and make `cli::archive` call `write_feature`. The preserved guard list at `:34` matches `merge_file_mode` (`src/gherkin/merge.rs:256-293`): `is_delta_feature` refusal (`:261-269`), parent creation (`:280-283`), target-path context (`:277-278`), and the `.givn-tmp` write/rename (`:289-291`). Dependency direction is `spec::merge_plan -> gherkin::merge`. |
| M3 retirement semantics | **Fixed** | `design.md:79` and `:118` mirror `retire_capability` (`src/cli/archive.rs:1934-1981`): already-absent feature no-op, `fragments` exemption, `holds_feature_files` narrative removal, and no `.retired.yaml` in the projection. The four unit tests are at `:301`. |
| M4 typed plan error | **Fixed** | `design.md:124` defines `MergePlanError::UnplaceableCapability { capability, detail }`, matches only that variant in the convenience entry, keeps the `capability '<cap>' cannot be placed in the projected corpus` title, and propagates other plan errors. The raw-delta scenario at `specs/verify-change/check-specification-health.feature:45-59` exercises exactly that boundary. |
| M5 corpus read/parse boundary | **Partial** | The boundary is explicit at `:69`, `:113`, `:136`, `:273-274`, `:304`, `:312`; `usecase_coverages` is re-based on `&SpecCorpus` rather than left as a second path; `usecase_invalid` is the existing read/parse kind (`src/overlap/analysis.rs:351`, `:396-401`; blocking set `src/cli/lint.rs:226-241`, `:534-547`). The one remaining contradiction is `design.md:259`'s "or unreadable". |
| M6 logical target mapping | **Fixed** | `design.md:78`, `:79`, `:116-117`, `:149`, and `:302` cover the `target_rel` shape, `/` normalization, the outside-specs refusal in both `build_merge_plan` and `plan_from_receipt`, removal of the `unwrap_or` absolute fallback, the retirement feature key, and the unit test. |
| M7 ADR-075 | **Fixed** | The chosen option says review and archive "read and parse one in-memory copy of the specification corpus" (`docs/arc42/adr/ADR-075-projected-interaction-integrity.md:95-102`); the temporary copy appears only as the rejected option; the index lists ADR-075 as accepted (`docs/arc42/adr/README.md:62`). |
| M8 last-capability fixture | **Fixed** | `design.md:181` names the exact edit (omit the change-local narrative write); the write is `tests/steps/grouped_merge_steps.rs:199` inside `retiring_change` (`:171-201`); the only retirement scenario is the last-capability case (`grouped-merge-for-new-capabilities.feature:28-36`). Residual wording only, at `:271`. |

### In-memory constraints recheck

- No temporary directory and no file write in the projection: `design.md:27`,
  `:89` (temporary copy kept as rejected rationale), `:118` (no
  `.retired.yaml`), `:120`, `:122`, `:275`, `:299`; `tempfile` is
  test-fixture-only at `:5`.
- One `merge_feature_text`: twelve references
  (`:34,41,77,80,89,117,122,249,270,282,303,342`); no `merge_file` reference
  remains; `:34`, `:122`, and `:303` pin the archive writer and the projection
  to the same function.
- Default lint, review, and archive reuse the same corpus and gate:
  `design.md:29-32`, `:69`, `:128`, `:136-141`, `:144-150`. Review reaches it
  through `cli::check::run_mandatory_checks` -> `lint::run_scoped`; default
  lint reaches it through `scan_tree` (`src/overlap/analysis.rs:107`).
- `MergeProgress`: `design.md:80` (fields and the `legacy_unrecorded_merge`
  mirror), `:111` (fresh versus resumed), `:116` (skip merged, retry
  in-flight/legacy), `:148-150` (all three placements), `:270`, `:305-307`.
  The code mirror is `src/cli/archive.rs:1006-1021`: skip `merged_deltas`,
  `retry = legacy_unrecorded_merge || merge_in_progress == relative`.
- Reverse check: `design.md:81`, `:126` (title, blocking severity, exit
  `1`), `:264`, `:266`, `:308`; the lint blocking set and `severity_for` exist
  at `src/cli/lint.rs:226-241` and `:534-547`.
- No-document rule: `design.md:177`, `:179`, `:266`, `:283`; the cited fixture
  ranges write permanent features without a document, and
  `repository_ownership_fixtures_support.rs` needs two documents (`:177`).
- Fixture audit: `design.md:165`, `:173`, `:336`, `:342`; `write_fixture_usecase`
  (`tests/common/mod.rs`) remains the repaired builder, and the intentional
  schema-error exception is recorded at `:175`.

### Executed checks

- `givn lint --change enforce-spec-interaction-integrity`: exit `2`;
  `4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP ]` findings (7
  `check-specification-health`, 2 `givn-workflow-enforcement`, 3
  `archive-proof`); no `[ERROR]` finding and no parse finding.
- `givn readme check`: `README check: OK`, exit `0`.
- `git status --short givn/specs` is empty: no permanent spec changed, so no
  `@e2e` tag was removed. The delta still carries its single real `@e2e` tag
  on `initialize-a-project.feature:5` plus five assertion strings (6
  occurrences). No `tasks.md` exists; no commit was made in this pass.

### Remaining edits

1. `design.md:259` (blocking) — replace "Corrupt or unreadable use-case
   document: the gate returns a blocking `usecase_invalid` finding; the command
   fails closed." with "Corrupt use-case document: the infallible gate returns
   a blocking `usecase_invalid` finding. An unreadable document is a named
   `SpecCorpus::read` error. The command fails closed." The read failure has
   its own error path at `:69`, `:274`, and `:312`; only the parse failure is
   the gate's finding.
2. `design.md:271` (minor) — replace the final sentence "The
   `grouped_merge_steps.rs` fixture follows the two cases." with "The
   `grouped_merge_steps.rs` fixture follows the last-capability case; the
   sibling retirement is unit-tested." That matches `:283`, `:336`, `:342`,
   the `:301` unit test, and the single retirement scenario at
   `grouped-merge-for-new-capabilities.feature:28-36`.

### Outcome

Both requested fixes are partial. `:259` no longer claims a named parse error,
but it still routes a corrupt-or-unreadable document through the gate, so the
read-error branch stays undefined against `:69`, `:274`, and `:312`. The three
fixture-wording locations the author changed are correct, but `:271` keeps the
"two cases" claim that the same document refutes elsewhere. M1-M4 and M6-M8
remain fixed; M5 is one line short. The in-memory projection with no file
write, the single `merge_feature_text`, the shared corpus/gate on default lint,
review, and archive, the `MergeProgress` retry mapping, the reverse check, the
no-document rule, and the fixture audit all still hold, and the two
plan-integrity checks pass. Two exact lines remain; no structural change is
needed.

Earlier verdict (superseded): FAIL

## Final verification after the `:259` and `:271` update

Scope of this pass: the two exact edits the preceding section required, a fresh
recheck of M1-M8 against the current tree, the in-memory constraints, and the
two plan-integrity checks. Read in this pass: `design.md` (345 lines),
`docs/arc42/adr/ADR-075-projected-interaction-integrity.md`,
`src/gherkin/merge.rs`, `src/gherkin/interaction.rs`,
`src/overlap/analysis.rs`, `src/cli/lint.rs`, `src/cli/archive.rs`,
`tests/steps/grouped_merge_steps.rs`, and the permanent retirement scenario.

| Required edit | Status | Evidence |
|---|---|---|
| `design.md:259` separates a corrupt document from an unreadable one | **Applied** | The line now reads: "Corrupt use-case document: the infallible gate returns a blocking `usecase_invalid` finding. An unreadable document is a named `SpecCorpus::read` error. The command fails closed." It agrees with the read contract at `:69` ("`SpecCorpus::read` returns `Result`: a missing `givn/specs` yields an empty corpus; any other read failure is a named error"), the projection step at `:113`, the boundary rows at `:273-274`, the unit-test list at `:304`/`:312`, and the malformed-input rows at `:260`. No location routes a read failure through the gate any more. |
| `design.md:271` states the fixture follows the last-capability case and the sibling retirement is unit-tested | **Applied** | The line now ends: "The `grouped_merge_steps.rs` fixture follows the last-capability case; the sibling retirement is unit-tested." Ground truth matches: `retiring_change` (`tests/steps/grouped_merge_steps.rs:171-201`) writes only the last-capability retirement delta and the change narrative at `:199`; the permanent corpus holds exactly one retirement scenario, `Retiring a use case's last capability removes the use-case directory and its narrative` (`givn/specs/archive-change/grouped-merge-for-new-capabilities.feature:28-36`); the sibling retirement is pinned by the unit test at `design.md:301`. The "two cases" claim survives nowhere: `grep -n "two cases" design.md` is empty, and `:181`, `:283`, `:336`, `:342` carry the same last-capability/unit-tested split. |

### M1-M8 recheck

| Finding | Status | Evidence |
|---|---|---|
| M1 feature-level merge core | **Fixed** | `design.md:34` names `gherkin::merge::apply_feature_deltas` plus a new public `apply_feature_deltas_retry` and scopes out the scenario-slice wrappers. `apply_feature_deltas` is public at `src/gherkin/merge.rs:178-183`; `apply_feature_mode` (`:185-253`) owns the Background conflict (`:196-205`), the non-`@givn` tag inheritance (`:206-213`), and the new-capability header (`:216-247`). `apply_deltas_retry` remains `#[cfg(test)]` (`:41-42`), which is why the design adds the retry pair. |
| M2 writer home / no cycle | **Fixed** | `design.md:34`, `:77`, `:122`, `:303` put the thin writer in `spec::merge_plan` (`write_feature`/`write_feature_retry`), keep `gherkin::merge` text-only, and route `cli::archive` through it. The preserved guards match `merge_file_mode` (`src/gherkin/merge.rs:256-293`): `is_delta_feature` refusal (`:261-269`), target-path context (`:277-278`), parent creation, and the `.givn-tmp` write/rename (`:289-291`). Dependency direction stays `spec::merge_plan -> gherkin::merge`. |
| M3 retirement semantics | **Fixed** | `design.md:79` and `:118` mirror `retire_capability` (`src/cli/archive.rs:1934-1985`): the `fragments` exemption (`:1949-1952`), the no-feature-remains narrative removal through `holds_feature_files` (`:1952`, defined `:1787-1802`), the already-absent feature no-op (`:1940-1944`), and no `.retired.yaml` in the projection. The four unit tests are at `design.md:301`. |
| M4 typed plan error | **Fixed** | `design.md:124` defines `MergePlanError::UnplaceableCapability { capability, detail }`, matches only that variant in the convenience entry, keeps the `cannot be placed in the projected corpus` title, and propagates other plan errors. The current unplaceable paths are `anyhow!` strings in `resolve_permanent_dir` (`src/cli/archive.rs:1831-1849`) and the routing lookup (`:1908-1920`), so the typed variant is the implementation obligation the design names. The raw-delta scenario at `specs/verify-change/check-specification-health.feature:45-59` pins it. |
| M5 corpus read/parse boundary | **Fixed** | The boundary is now consistent at `:69`, `:113`, `:136`, `:259`, `:273-274`, `:304`, `:312`: a missing root is an empty corpus, an unreadable file or root is a named `SpecCorpus::read` error, and a corrupt document or feature is the infallible gate's blocking `usecase_invalid` finding. Current root-based signatures to replace are `src/gherkin/interaction.rs:42` and `:114`, with the single call site at `src/overlap/analysis.rs:118` inside the infallible `scan_tree` (`:107`); the re-based `usecase_coverages(&SpecCorpus)` is named at `:69`. |
| M6 logical target mapping | **Fixed** | `design.md:78` defines `target_rel` as a `/`-normalized path under `givn/specs`, resolved by `build_merge_plan` and recomputed by `plan_from_receipt`, with an outside-specs refusal and no absolute fallback; `:79` gives the retirement feature key `target_rel/<cap>.feature`; `:116-117` consume the key; `:149` repeats the receipt branch; `:302` adds the normalization/refusal/resume unit test. The code to change is `receipt_plan`'s `.strip_prefix(project_root).unwrap_or(&entry.target)` and `replace('\\', "/")` (`src/cli/archive.rs:370-386`). |
| M7 ADR-075 | **Fixed** | The chosen option says review and archive "reads the specification corpus into memory per review and archive" (`docs/arc42/adr/ADR-075-projected-interaction-integrity.md:121-124`); the temporary copy appears nowhere in the record (`grep -n temporary` is empty), and the in-memory consequence is stated at `:102-104`. |
| M8 last-capability fixture | **Fixed** | `design.md:181` names the exact edit (omit the change-local narrative write for the last-capability case); the write is `tests/steps/grouped_merge_steps.rs:199` inside `retiring_change` (`:171-201`), and the only permanent retirement scenario is the last-capability case (`grouped-merge-for-new-capabilities.feature:28-36`). `:271` now matches instead of claiming both cases. |

### In-memory constraints recheck

- No temporary directory and no file write in the projection: `design.md:5`
  (no tempdir; `tempfile` is test-fixture-only), `:27` (component "No temporary
  directory"), `:89` (temporary specifications copy kept as rejected rationale),
  `:118` (no `.retired.yaml`), `:120` (no temporary path, no mapping step), `:122`
  ("The projection never writes a file"), `:275`, `:299`.
- One merge entry point: twelve `merge_feature_text` references
  (`:34,41,77,80,89,117,122,249,270,282,303,342`); no `merge_file` reference
  remains in the design; `:34`, `:122`, `:303` pin the archive writer and the
  projection to the same function.
- Default lint, review, and archive reuse the same corpus and gate: `:29-32`,
  `:69`, `:128`, `:136-141`, `:144-146`. Review reaches it through
  `cli::check::run_mandatory_checks` -> `lint::run_scoped`; default lint through
  `scan_tree` (`src/overlap/analysis.rs:118`).
- `MergeProgress`: `:80` (fields and the `legacy_unrecorded_merge` mirror), `:111`
  (fresh versus resumed versus merged candidate), `:116` (skip merged, retry
  in-flight/legacy), `:148-150` (all three placements), `:270`, `:305-307`. The
  receipt fields are `merged_deltas`/`merge_in_progress` in `src/cli/archive.rs`.
- Reverse `undeclared_capability` check: `:81`, `:126` (title, blocking
  severity, exit `1`), `:264`, `:266`, `:308`; the blocking set and
  `severity_for` registries that must gain the kind are `src/cli/lint.rs:226-241`
  and `:534-551`.
- No-document rule: `:177`, `:179`, `:266`, `:283`; the named fixtures write
  permanent features without a document, and `repository_ownership_fixtures_support.rs`
  needs two documents.
- Fixture audit and its schema-error exception: `:165`, `:173`, `:175`, `:336`,
  `:342`. `write_fixture_usecase` (`tests/common/mod.rs`) remains the repaired
  builder behind `declare_fixture_routing`.
- Projection ordering: `:137` runs the projection under
  `paths.is_empty() && change.is_some() && !has_parse_error && !has_use_case_error`
  before the empty-feature return; the branch to move is
  `src/cli/lint.rs:77-92`, and `has_use_case_error` is set at `:226-241`.

### Executed checks

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit
  `2`; `givn lint: 4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP ]`
  findings (7 `check-specification-health`, 2 `givn-workflow-enforcement`, 3
  `archive-proof`), zero `[ERROR]` findings. The only `parse` strings in the
  output are advisory `[SHAPE]`/`[SUBST]` scenario titles, so there is no parse
  error. The 12 `@wip` scenarios equal the 12 feature scenarios listed at
  `design.md:316-327`.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- `git status --short givn/specs` is empty: no permanent spec changed, so no
  `@e2e` tag was removed. The delta carries exactly one real `@e2e` tag
  (`specs/initialize-project/initialize-a-project.feature:5`); the other five
  `@e2e` strings are assertion text.
- No `tasks.md` exists and no commit was made in this pass.

### Outcome

Both required edits are applied exactly as specified and are correct against
the tree: `:259` now gives the read failure its own named `SpecCorpus::read`
error path and leaves only corruption to the infallible gate's blocking
`usecase_invalid` finding, and `:271` now says the `grouped_merge_steps.rs`
fixture follows the last-capability case with the sibling retirement unit-tested,
which the fixture, the single permanent retirement scenario, and the `:301` unit
test all confirm. M1-M8 stand, the in-memory projection keeps its no-tempdir /
no-file-write contract, `merge_feature_text` remains the only merge entry point,
and the two plan-integrity checks reproduce (exit `2`, 12 `@wip`, no parse error;
`README check: OK`). The design is signable; the remaining items are the
implementation obligations already pinned by scenarios and unit tests.

DESIGN-REVIEW: PASS

## Challenge round N1-N8 — verification after the author's update

Scope of this pass: the eight findings raised by the challenge after the in-memory
sign-off, a fresh M1-M8 recheck against the current tree, the in-memory constraints, and
the two plan-integrity checks. Read in this pass: `design.md` (346 lines), the five delta
feature files, the permanent retirement scenario
(`givn/specs/archive-change/grouped-merge-for-new-capabilities.feature:28-36`),
`tests/common/mod.rs` (`declare_fixture_routing`, `write_fixture_usecase`,
`collect_capability_names`), `tests/steps/mod.rs` (`ensure_fixture_usecase`, the persona
fixture), `tests/steps/preserve_docstrings_on_archive_steps.rs`,
`tests/steps/grouped_merge_steps.rs`, `tests/steps/spec_overlap_gates_steps.rs`,
`tests/steps/archive_proof_steps.rs`, `src/gherkin/{merge,interaction,usecase_md}.rs`,
`src/cli/{archive,lint}.rs`, `src/overlap/analysis.rs`, and ADR-075.

### N1-N8

| Item | Status | Evidence |
|---|---|---|
| N1 corpus keys and `target_rel` carry the `givn/specs/` prefix | **Resolved** | `design.md:69` keys `narratives` and `features` by project-relative paths that include the prefix and gives `givn/specs/fragments/fragment.md` as the example; `:78` gives `target_rel` as `givn/specs/<usecase>/<cap>.feature` (merge) or `givn/specs/<usecase>` (retirement), with the retirement feature key `target_rel/<cap>.feature` and the narrative key `target_rel/usecase.md` or `givn/specs/fragments/fragment.md`; `:302` adds the lookup test ("looked up with the same project-relative key as an existing permanent feature"). The shape matches the receipt fixture, whose targets are already `givn/specs/demo-cap/demo-cap.feature` (`tests/steps/archive_proof_steps.rs:647-650`), and `receipt_plan`'s `strip_prefix(project_root)` (`src/cli/archive.rs:370-386`) produces exactly that key. `:116-117` consume `target_rel` directly. |
| N2 `interaction_findings_for_plan` returns `Result`; a merge-text failure is a named command error; boundary and test rows | **Resolved** | `design.md:113` makes both failure classes explicit: "another read failure is a named command error. ... A `merge_feature_text` failure (added already present, modified/removed absent, background conflict) is a named command error; lint, review, and archive fail closed." That is the fallible return: the function yields findings or the command's named error. `:69` says "the projection returns the read error to the command", `:124` says the convenience entry propagates other plan errors as command errors, `:274` adds the boundary row ("Corpus read failure: the command fails with a named path ... A merge-text failure is a named command error"), `:303` adds the merge-text unit-test row, and `:305` the read-error row. The `:109` line is an abbreviated function reference (it heads the numbered algorithm), and `:128` likewise; the contract is the prose, consistent with `SpecCorpus::read` being described as "returns `Result`" at `:69` rather than as a Rust signature. No location remains on the infallible path. |
| N3 `SpecCorpus` owns the narrative operations | **Resolved** | `design.md:26` says the merge planner delegates narrative overlay and retirement to `SpecCorpus`; `:34` and `:77` name `SpecCorpus::overlay_narrative` and `SpecCorpus::remove_capability` as the owners and state that `spec::merge_plan` delegates; `:118` puts the retirement removal in the corpus step. No location assigns the overlay or removal to `spec::merge_plan`. |
| N4 fixture-helper responsibilities and `Scenario Outline` counting | **Resolved** | `design.md:179` assigns the three responsibilities explicitly: `collect_capability_names` keys by use case and capability so multiple capabilities in one use case do not collapse (`tests/common/mod.rs:596-599` currently keys by `Option<usecase>` only); `ensure_fixture_usecase` extracts the actual `@e2e` titles and must count `Scenario Outline:` as well as `Scenario:` (`tests/steps/mod.rs:403-417` currently recognises only `Scenario:`); `write_fixture_usecase` writes the document and selects `fragment.md` for `fragments` (`tests/common/mod.rs:578-594` currently always writes `usecase.md`). The split is consistent with `:169` (the writer generates rows from the actual titles) and with the title hand-off from the extracting helper. |
| N5 `SpecCorpus::read` selects direct use-case/fragments documents and features; nesting is a limitation | **Resolved** | `design.md:113` restricts the read to the direct child use-case and `fragments` documents and their direct feature files, and records nested features and documents as a limitation; `:179` repeats the direct-only rule for the reverse check and also excludes a `.feature` directly under `givn/specs`; `:267` records the nested case as a limitation rather than a finding. The current traversal (`src/gherkin/interaction.rs:42-60`) is already direct-child only, so the rule is `SpecCorpus::read`'s selection, not a new gate behaviour. |
| N6 corpus size | **Resolved** | `design.md:284` says 58 feature files, 77 files under `givn/specs`, about 566 KiB of text, 740 KiB on disk. Measured: `find givn/specs -name '*.feature'` = 58; `find givn/specs -type f` = 77; `.md` + `.feature` text = 579,653 bytes = 566.07 KiB; `du -sk givn/specs` = 740 KiB. The figures match. |
| N7 persona and `preserve_docstrings` repairs | **Resolved** | `design.md:177` gives the `preserve_docstrings_on_archive_steps.rs:99-110,180-206,267-284` fixtures a `usecase.md` that declares the capability and sets `## Interactions` to `- none`; `:183` does the same for the persona fixture and adds the permanent feature with at least one non-`@e2e` scenario. A document with a declared capability and `- none` interactions is valid (`src/gherkin/usecase_md.rs:765-779` returns an empty interaction list for the sentinel; the capability list is non-empty). A leaf `- none` capability list remains invalid: `parse_usecase_md` rejects it (`src/gherkin/usecase_md.rs:261-269`), and `parse_fragment_md` rejects an empty fragment capability list (`:369-372`); `design.md:181` and `:183` keep that shape out. |
| N8 `wrong_capability_scenario` citation | **Resolved** | `design.md:200` cites `spec_overlap_gates_steps.rs:743-744`. `wrong_capability_scenario` starts at `tests/steps/spec_overlap_gates_steps.rs:736`; `:743` is the `0 @e2e scenario(s)` count assertion and `:744` is the old `missing declared scenarios` label assertion, which is exactly the pair to re-anchor to `missing @e2e scenario '<title>'`. The scenario that uses the step is `givn/specs/verify-change/check-specification-health.feature:410`. |

### M1-M8 recheck

| Finding | Status | Evidence |
|---|---|---|
| M1 feature-level merge core | **Fixed** | `design.md:34` names `gherkin::merge::apply_feature_deltas` plus a new public `apply_feature_deltas_retry`; `apply_feature_deltas` is at `src/gherkin/merge.rs:178-183`, `apply_feature_mode` at `:185-253` owns the background conflict, tag inheritance, and new-capability header, and `apply_deltas_retry` remains `#[cfg(test)]` at `:41-42`, which is why the retry pair is required. |
| M2 writer home / no cycle | **Fixed** | `design.md:34`, `:77`, `:122`, and `:304` put the thin writer in `spec::merge_plan` (`write_feature`/`write_feature_retry`), keep `gherkin::merge` text-only, and route the archive through it. The preserved guards match `merge_file_mode` (`src/gherkin/merge.rs:256-293`): `is_delta_feature` refusal (`:261-269`), target-path context (`:277-278`), parent creation, and the `.givn-tmp` write/rename. Dependency direction stays `spec::merge_plan -> gherkin::merge`. |
| M3 retirement semantics | **Fixed** | `design.md:79` and `:118` mirror `retire_capability` (`src/cli/archive.rs:1934-1983`): the `fragments` exemption, the `holds_feature_files` narrative removal (`:1787-1802`), the already-absent feature no-op, and no `.retired.yaml` in the projection. The four unit tests are at `design.md:301`. |
| M4 typed plan error | **Fixed** | `design.md:124` defines `MergePlanError::UnplaceableCapability { capability, detail }`, matches only that variant in the convenience entry, keeps the `cannot be placed in the projected corpus` title, and propagates other plan errors. The current unplaceable paths are the `anyhow!` strings in `resolve_permanent_dir` (`src/cli/archive.rs:1831-1849`) and the routing lookup (`:1908-1920`), so the typed variant remains the implementation obligation. The raw-delta scenario at `specs/verify-change/check-specification-health.feature:45-59` pins it. |
| M5 corpus read/parse boundary | **Fixed** | The boundary is consistent at `design.md:69`, `:113`, `:136`, `:259`, `:273-274`, `:304`, `:312`: a missing root is an empty corpus; an unreadable file or root is a named `SpecCorpus::read` error; a corrupt document or feature is the infallible gate's blocking `usecase_invalid` finding. The current root-based signatures to replace are `src/gherkin/interaction.rs:42` and `:114`, with the single call site at `src/overlap/analysis.rs:118` inside `scan_tree` (`:107`); the re-based `usecase_coverages(&SpecCorpus)` is named at `design.md:69`. |
| M6 logical target mapping | **Fixed** | `design.md:78` defines the normalized `target_rel` with the prefix, the outside-specs refusal, and the receipt recomputation; `:79` gives the retirement keys; `:116-117` consume the key; `:149` repeats the receipt branch; `:302` adds the normalization/refusal/resume/lookup test. The code to change is `receipt_plan`'s `strip_prefix(project_root).unwrap_or(&entry.target)` and `replace('\\', "/")` (`src/cli/archive.rs:370-386`) and the `project_root.join(value)` reconstruction in `plan_from_receipt` (`:950-990`). |
| M7 ADR-075 | **Fixed** | The chosen option says review and archive read and parse one in-memory copy (`docs/arc42/adr/ADR-075-projected-interaction-integrity.md:103`, `:123`); the temporary-copy wording appears only as the rejected option; the index lists ADR-075 as accepted (`docs/arc42/adr/README.md:62`). |
| M8 last-capability fixture | **Fixed** | `design.md:181` names the exact edit (omit the change-local narrative write for the last-capability case); the write is `tests/steps/grouped_merge_steps.rs:199` inside `retiring_change` (`:171-201`); the only permanent retirement scenario is the last-capability case (`grouped-merge-for-new-capabilities.feature:28-36`); `:271`, `:283`, `:337`, and `:342` all state the last-capability/unit-tested split. |

### In-memory constraints recheck

- No temporary directory and no file write in the projection: `design.md:5` (no tempdir;
  `tempfile` is test-fixture-only), `:27` ("No temporary directory"), `:89` (the temporary
  specifications copy is the rejected option), `:118` (no `.retired.yaml`), `:120` (no
  temporary path, no mapping step), `:122` ("The projection never writes a file"), `:275`,
  and `:299`.
- One `merge_feature_text`: twelve references (`design.md:34,41,77,80,89,117,122,249,270,282,304,342`);
  no `merge_file` reference remains in the design; `:34`, `:122`, and `:304` pin the archive
  writer and the projection to the same function.
- Shared corpus and gate: `design.md:29-32`, `:69`, `:128`, `:136-141`, `:144-146`; default
  lint reaches the gate through `scan_tree`, review through `lint::run_scoped`, and archive
  through the projection and `permanent_interaction_findings`.
- `MergeProgress`: `design.md:80` (fields and the `legacy_unrecorded_merge` mirror), `:111`
  (fresh, resumed, and merged-candidate paths), `:116` (skip merged, retry in-flight/legacy),
  `:148-150` (all three placements), `:270`, and `:305-308`. The code mirror is
  `src/cli/archive.rs:1006-1021`: skip `merged_deltas`, `retry = legacy_unrecorded_merge ||
  merge_in_progress == relative`.
- Reverse check: `design.md:81`, `:126` (title, blocking severity, exit `1`), `:264`, `:266`,
  and `:308`; the blocking set and `severity_for` live at `src/cli/lint.rs:226-241` and
  `:534-551`.
- No-document rule: `design.md:177`, `:179`, `:266`, and `:283`; the cited fixtures write
  permanent features without a document, and `repository_ownership_fixtures_support.rs`
  needs two documents.
- Fixture audit: `design.md:165`, `:173`, `:283`, `:337`, and `:342`; the intentional
  schema-error exception is `:175`, and `write_fixture_usecase` remains the repaired
  builder behind `declare_fixture_routing`.

### Executed checks

- `./target/debug/givn lint --change enforce-spec-interaction-integrity`: exit `2`;
  stderr carries `givn lint: 4 file(s) checked, 12 finding(s)`; exactly 12 `[WIP  ]`
  findings (3 `archive-proof`, 7 `check-specification-health`, 2
  `givn-workflow-enforcement`), zero `[ERROR]` findings, and no parse finding. The delta
  carries exactly 12 `@wip` tags, matching the 12 feature scenarios at `design.md:317-328`.
- `./target/debug/givn readme check`: `README check: OK`, exit `0`.
- `givn status --change enforce-spec-interaction-integrity --json`: `design-review` is
  `done`, `next_required` is `tasks`, and `stage` is `design-review-done`.
- `git status --short givn/specs` is empty, so no permanent spec changed and no `@e2e` tag
  was removed. The delta still carries its single real `@e2e` tag on
  `specs/initialize-project/initialize-a-project.feature:5`; the other five `@e2e`
  occurrences are assertion text. No `tasks.md` exists and no commit was made in this pass.

### Non-blocking observation

The `:109` and `:128` lines are abbreviated function references rather than Rust signatures,
so neither shows `-> Result<Vec<Finding>>` literally. The behavior is unambiguous at `:113`
and `:274`, and the test rows at `:303`/`:305` pin it; the implementation must return a
`Result` in both places. This is a presentational abbreviation, not an unresolved contract,
and needs no edit before tasks. Adding the return type when the prose is next touched would
remove the last inference.

No blocking edit remains. The N1-N8 fixes are in the text and match the tree; M1-M8 stand;
the in-memory projection keeps its no-tempdir and no-file-write contract, the single
`merge_feature_text`, the shared corpus/gate, the `MergeProgress` retry mapping, the reverse
check, the no-document rule, and the fixture audit.

### Outcome

The author's update resolves all eight challenge findings. N1's corpus keys and `target_rel`
carry the `givn/specs/` prefix and the retirement narrative key, with the lookup test in the
Test Plan. N2's projection is fallible for both the corpus read and `merge_feature_text`,
with the boundary and unit-test rows added; the abbreviated signature lines do not contradict
that contract. N3 moves the narrative overlay and removal to `SpecCorpus`. N4 assigns the
three fixture-helper responsibilities and adds the `Scenario Outline` rule. N5 restricts the
read to direct children and records the nesting limitation. N6's measurements reproduce. N7
repairs the persona and `preserve_docstrings` documents while keeping leaf `- none` invalid.
N8's citation points at the two assertions it names. The plan remains free of a temporary
directory and file write, keeps one `merge_feature_text`, reuses the shared corpus/gate on
default lint, review, and archive, keeps the `MergeProgress` retry mapping, the reverse check,
the no-document rule, and the full fixture audit. The design is signable; the next artifact
is `tasks.md`.

DESIGN-REVIEW: PASS
