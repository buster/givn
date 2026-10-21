# Tasks: gate-delta-step-definitions-at-archive

RED→GREEN→REFACTOR→COMMIT per delta scenario. Non-`@e2e` scenarios first,
`@e2e` last. Legend: unchecked boxes are pending; check a box only when its
evidence and commit hash are filled in. Check each box immediately after its
step — never batch-check.

Release note (commit subject):

> givn archive refuses a change whose delta step definitions are missing from the permanent test tree, naming the project-relative delta file and the missing step phrases (bounded with a remainder count).

## Interaction coverage pre-flight

Owning use case: `archive-change`
(`givn/specs/archive-change/usecase.md`). Inventory cross-check against
design.md's Interaction Coverage Matrix:

| Inventory entry | Matrix row | Existing `@e2e` scenario | Gap |
|---|---|---|---|
| `require-merged-step-definitions` — archive a change whose delta step definitions are already in the permanent test tree | Present | `Consumer archives a change whose delta step definitions are already in the permanent test tree` (this delta, `@e2e`) | none |
| `require-merged-step-definitions` — the blocked variant | Present (blocking variant of the same action) | Same action; refusal is the failure path of the `@e2e` action | none — variants do not need separate E2E entries |
| `require-merged-step-definitions` — the bounded-list variant | Present (bounded-list variant of the same action) | Same action; bounded refusal is a failure variant | none |
| `require-merged-step-definitions` — the unproven-candidate variant | Present (resume variant of the same action) | Same action; resume refusal is a failure variant | none |

Result: **1 normalized inventory entry, 1 matching `@e2e` scenario, no
coverage gap.** The three refusal scenarios are variants of the same archive
action and are regular scenarios.

Overlap dispositions to carry into `review.md`:

- The bounded-list and pending-candidate scenarios are variants of the fresh
  refusal; they differ by the refusal content and the archive phase.
- The success scenario is the boundary against the two refusal scenarios.

## Domain constraints (from the use case and design)

- Goal: archive a completed change with proven publication.
- Minimal guarantee: a failed archive is never reported as proven, and its
  candidate remains repairable in place.
- Success guarantee: the permanent corpus contains the accepted change, bound
  to a verification receipt for the revision that passed every verification
  scope.
- Rule: delta step definitions are durable project source; archive merges
  feature files and use-case documents only and never copies, moves, or
  deletes a step definition.
- Rule: archive compares the step phrases bound by the change's delta
  step-definition files with the phrases bound by the permanent test tree
  before mutation, and again before an unproven candidate's scopes run.
- Rule: a delta step copy may stay under the change's spec tree as history once
  every phrase it binds is also bound in the permanent test tree.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`.
- Project-relative delta path: `givn/changes/<id>/specs/steps/<file>` (fresh)
  or `givn/archive/<id>/specs/steps/<file>` (pending).
- Permanent tree: the project root minus the documented component names
  (`.git`, `.hg`, `.svn`, `target`, `node_modules`, `.venv`, `venv`,
  `coverage`, `dist`, `build`, `vendor`, `__pycache__`) and the
  project-relative prefixes `givn/changes`, `givn/archive`, `givn/history`.
- The delta walk prunes only the component-name set, so the delta's own step
  files are read.

Behavior rule (from design.md):

| Phase | Before the gate | Gate result | After the gate |
|---|---|---|---|
| Fresh | clean-state witness, artifact gates | any missing delta phrase | refuse before hooks, receipt, and rename; change stays active; no candidate |
| Fresh | clean-state witness, artifact gates | all phrases present | proceed unchanged: hooks, receipt, move, merge, publish, verify, finalize |
| Pending | candidate provenance, clean witness | any missing delta phrase | refuse before `ensure_e2e_configured`, merge, and any scope; candidate stays unproven |
| Pending | candidate provenance, clean witness | all phrases present | proceed unchanged: merge/publish if needed, verify, finalize |

Exact refusal shape:

```
archive blocked: delta step definitions are not present in the permanent test tree:
  - "<phrase>" (<project-relative delta file>)
  ...
  +<N> more
The archive merges feature files and use-case documents only. Merge each
definition into the permanent per-capability step file, keep the delta copy
for the change suite, and rerun `givn archive --change <id>`.
```

- At most `MAX_REPORTED_PHRASES = 20` phrases are listed; the remainder count
  is exact.
- Phrase order is deterministic (file path then phrase, both sorted).
- The extractor is expression-based: binding tokens anywhere in the file,
  matching-paren depth counter that skips escaped/raw strings, `/regex/`
  literals, and comments; first resolvable literal; adjacent Python literals
  joined; internal whitespace normalized. Unreadable/non-UTF-8 files are
  skipped.
- No new dependency; helpers live in `src/cli/archive.rs`.

## Sequencing note

Scenario 1 introduces the gate. To keep RED honest for scenarios 2–4, write
their step-definition functions with `todo!()` during setup and run each
targeted command **before scenario 1's GREEN**. The `@wip` tag does not filter
these scenarios out of the runner, so the targeted commands execute and fail.
Paste that output in each scenario's RED field. After scenario 1's commit,
fill those functions with real assertions; their targeted runs become GREEN
because scenario 1's gate already implements the behavior. Each scenario still
gets its own atomic commit containing its regression test (and, for
scenario 1, the production gate and guidance).

## Setup task

- [ ] **Setup: runner, strict mode, and step skeleton.**

  Runner already configured in `givn/commands.yaml`:
  `verify.command = ./run-tests.sh --non-e2e`,
  `verify.e2e_command = ./run-tests.sh --e2e`. Strict mode is
  `.fail_on_skipped()` in `tests/features_runner.rs`; the permanent
  proof-of-strictness and undefined-step evidence are in the archive-proof
  scenarios. Record the targeted probe below.

  Files:
  - create `tests/steps/require_merged_step_definitions_steps.rs` with the
    shared Givens/Then functions as `todo!("TODO: implement ...")` stubs and
    register it in `tests/steps/mod.rs`;
  - create `tests/steps/require_merged_step_definitions_e2e_steps.rs` with
    only the e2e-specific step (the archived-file assertion) as a stub and
    register it in `tests/steps/mod.rs`;
  - reuse `tests/steps/archive_proof_support.rs`; add helpers there only when
    a scenario needs them.

  Strictness probe: run the first non-e2e scenario while its new Given is a
  `todo!()` stub.

  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    cargo test --locked --test features_runner -- \
    "archive-change/require-merged-step-definitions.feature::Archive is blocked when a delta step definition is missing from the permanent test tree" \
    --exact --nocapture
  ```
  ```
  <!-- paste non-zero output showing the todo panic / undefined step -->
  ```

  Then confirm the baseline is otherwise unchanged: `./run-tests.sh --non-e2e`
  exits 0 with the four delta scenarios filtered out only by `@wip` not being
  a filter — they are included and fail until implemented. Record the
  baseline command after the delta scope is excluded, or record the permanent
  suite result before adding the delta step files.
  ```
  <!-- paste baseline result (0 skipped/failed outside the new delta) -->
  ```

## Scenario 1 — Archive is blocked when a delta step definition is missing from the permanent test tree

`@givn.added` (non-`@e2e`).

- Files:
  - `src/cli/archive.rs`: `is_step_definition_path`,
    `step_binding_expressions`, `binding_phrase`, `collect_step_phrases`,
    `collect_step_phrases_in_tree`, `PruneRules`, `StepDefinitionGap`,
    `ensure_step_definitions_merged`, `MAX_REPORTED_PHRASES`, the fresh call
    site in `run()`, and unit tests;
  - `tests/steps/require_merged_step_definitions_steps.rs`: this scenario's
    Givens/Then;
  - `tests/steps/mod.rs`: module registration;
  - `tests/steps/archive_proof_support.rs`: fixture helpers;
  - durable guidance/docs: `assets/instructions/steps.md`,
    `assets/instructions/archive.md`, `assets/instructions/design.md`,
    `assets/instructions/tasks.md`, `assets/templates/design.md`;
    `docs/arc42/README.md`, `05-building-block-view.md`,
    `06-runtime-view.md`, `08-crosscutting-concepts.md`,
    `11-risks-and-technical-debt.md`, `12-glossary.md`.
- Contracts: no new contract; the preflight is an internal helper in
  `cli::archive`. The external CLI surface is unchanged.
- Guards: the walk prunes by component name and project-relative prefix; the
  delta walk does not prune the change/archive prefixes; unreadable files are
  skipped; a phrase present only under `givn/changes/**` or `givn/archive/**`
  does not satisfy the gate.
- RED: remove `@wip` from this scenario only; run targeting it. Must exit
  non-zero with the missing phrase/stub.
  ```
  <!-- paste RED output -->
  ```
- GREEN: implement the gate and guidance. Files created/modified: as listed
  above. Run targeting this scenario; must exit 0.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    cargo test --locked --test features_runner -- \
    "archive-change/require-merged-step-definitions.feature::Archive is blocked when a delta step definition is missing from the permanent test tree" \
    --exact --nocapture
  ```
  ```
  <!-- paste GREEN output, exit 0 -->
  ```
- REFACTOR: `cargo fmt --all`, `cargo clippy --locked --all-targets -- -D
  warnings`, re-run the targeted scenario and the delta non-e2e scope.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    ./run-tests.sh --non-e2e
  ```
  ```
  <!-- paste output, exit 0 -->
  ```
- COMMIT: one commit containing the gate, fixtures, guidance, and arc42
  docs. Body: the scenario title. Record exactly one checked COMMIT line.
- [ ] COMMIT: `<!-- hash -->` — Archive is blocked when a delta step definition is missing from the permanent test tree

## Scenario 2 — The refusal lists a bounded number of missing step phrases and a remainder count

`@givn.added` (non-`@e2e`).

- Files: `tests/steps/require_merged_step_definitions_steps.rs` (the
  bounded-list Givens/Then and the 21-phrase fixture).
- Contracts: none.
- Guards: 21 distinct phrases produce exactly 20 listed phrases, sorted
  alphabetically, then `+1 more`; the delta file is still named.
- RED (captured before scenario 1's GREEN, while the function is a `todo!()`
  stub):
  ```
  <!-- paste the targeted RED output captured during setup -->
  ```
- GREEN: fill the bounded-list step functions with real assertions. Run
  targeting this scenario; must exit 0.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    cargo test --locked --test features_runner -- \
    "archive-change/require-merged-step-definitions.feature::The refusal lists a bounded number of missing step phrases and a remainder count" \
    --exact --nocapture
  ```
  ```
  <!-- paste GREEN output, exit 0 -->
  ```
- REFACTOR: format/clippy; re-run targeted scenario.
  ```
  <!-- paste refactor output, exit 0 -->
  ```
- COMMIT: one commit containing this scenario's regression test. Body: the
  scenario title.
- [ ] COMMIT: `<!-- hash -->` — The refusal lists a bounded number of missing step phrases and a remainder count

## Scenario 3 — An unproven archive candidate with a missing step definition is not verified again

`@givn.added` (non-`@e2e`).

- Files: `src/cli/archive.rs` (pending/resume call site and the
  `Phase::Resume` message); `tests/steps/require_merged_step_definitions_steps.rs`
  (candidate-step and no-result assertions); fixture helpers as needed.
- Contracts: none.
- Guards: the resume gate runs after the clean-state witness and before
  `ensure_e2e_configured`/`apply_merge_plan`; a `record` runner must not write
  `givn/archive/<id>/results/regular.json`. The `fail` runner used to create
  the initial candidate writes no result file, so the assertion discriminates.
- RED (captured before scenario 1's GREEN, while the functions are `todo!()`
  stubs):
  ```
  <!-- paste the targeted RED output captured during setup -->
  ```
- GREEN: add the pending call site and fill the step functions. Run targeting
  this scenario; must exit 0.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    cargo test --locked --test features_runner -- \
    "archive-change/require-merged-step-definitions.feature::An unproven archive candidate with a missing step definition is not verified again" \
    --exact --nocapture
  ```
  ```
  <!-- paste GREEN output, exit 0 -->
  ```
- REFACTOR: format/clippy; re-run targeted scenario and the delta scope.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    ./run-tests.sh --non-e2e
  ```
  ```
  <!-- paste output, exit 0 -->
  ```
- COMMIT: one commit containing the resume call site and this scenario's
  regression test. Body: the scenario title.
- [ ] COMMIT: `<!-- hash -->` — An unproven archive candidate with a missing step definition is not verified again

## Scenario 4 — Consumer archives a change whose delta step definitions are already in the permanent test tree

`@givn.added @e2e`.

- Files: `tests/steps/require_merged_step_definitions_steps.rs` (success
  Givens/Then), `tests/steps/require_merged_step_definitions_e2e_steps.rs`
  (the e2e-specific archived-file assertion), fixture helpers.
- Contracts: none.
- Guards: the permanent fixture is pinned at
  `tests/steps/catalog_steps.py`, outside `givn/specs`, proving the scan is
  project-root based; the delta copy remains in the archived change as
  history; the change move and merge are unchanged.
- RED (captured before scenario 1's GREEN, while the e2e step is a `todo!()`
  stub):
  ```
  <!-- paste the targeted RED output captured during setup -->
  ```
- GREEN: fill the success step functions. Run targeting this scenario with
  the real CLI; must exit 0.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    cargo test --locked --test features_runner -- \
    "archive-change/require-merged-step-definitions.feature::Consumer archives a change whose delta step definitions are already in the permanent test tree" \
    --exact --nocapture
  ```
  ```
  <!-- paste GREEN output, exit 0 -->
  ```
- REFACTOR: format/clippy; re-run targeted scenario and the delta e2e scope.
  ```
  GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
    ./run-tests.sh --e2e
  ```
  ```
  <!-- paste output with the @e2e count, exit 0 -->
  ```
- COMMIT: one commit containing the e2e fixture and regression coverage.
  Body: the scenario title.
- [ ] COMMIT: `<!-- hash -->` — Consumer archives a change whose delta step definitions are already in the permanent test tree

## Final verification (no scenario commit)

- [ ] Delta scope exits 0 and all four scenarios report passed, none skipped:
      ```
      GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
        ./run-tests.sh --non-e2e
      GIVN_FEATURES=givn/changes/gate-delta-step-definitions-at-archive/specs \
        ./run-tests.sh --e2e
      ```
      ```
      <!-- paste both summaries, all four scenarios passed, none skipped -->
      ```
- [ ] Permanent scopes exit 0:
      ```
      ./run-tests.sh --non-e2e
      ./run-tests.sh --e2e
      ```
      ```
      <!-- paste both summaries -->
      ```
- [ ] `givn lint --change gate-delta-step-definitions-at-archive` exits 0
      with no `@wip` finding.
      ```
      <!-- paste result -->
      ```
- [ ] `givn status --change gate-delta-step-definitions-at-archive` shows
      `tasks` done and `review` next.
      ```
      <!-- paste result -->
      ```

**Done when:** all four scenario boxes are checked with pasted RED/GREEN/
REFACTOR evidence and exactly one real reachable checked COMMIT line each,
the delta scopes and both permanent verification commands exit 0, and no
`@wip` remains.
