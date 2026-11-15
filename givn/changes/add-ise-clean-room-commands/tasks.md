# Tasks: add-ise-clean-room-commands

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`), both already set in `givn/commands.yaml`. Strict mode:
`.fail_on_skipped()` at both registration points in `tests/features_runner.rs`;
an undefined or skipped step fails the run. Not-implemented stub (design.md
"Strict Mode"): `Err(anyhow!("not implemented: <step text>"))`.

Single-scenario run command (design.md, "Step Definitions"):

```sh
GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
  cargo test --locked --test features_runner -- --exact \
  "prepare-an-ise-test/<capability>.feature::<title>"
```

`<capability>.feature` is `clean-room-preparation.feature` or
`trial-input-verification.feature` under this change's `specs/`. `<title>` is
the scenario title without `@wip`; the runner matches the name exactly.

Release note (every batch commit subject, verbatim from `proposal.md`, without
backticks):

```text
givn ise replace materializes a clean-room trial from a recorded revision on Git or Fossil, and givn ise verify re-checks its frozen retained and e2e inputs per path.
```

Batch commit rule: one atomic `givn commit` per batch with explicit paths. The
subject is the release note; the body is `Batch: <batch-name>`, plus the
scenario title for a single-scenario batch. Every scenario of the batch records
the same revision as `COMMIT: <hash> -- Batch: <batch-name>`. A commit contains
the batch's implementation or its regression test. `@wip` is removed from a
scenario when its batch starts, never in bulk; `givn check tasks` refuses the
change while any selected delta scenario still carries `@wip`.

**Evidence fields.** Per the served `tasks` instruction, no separate
TEST-FIRST, RED, or TEST-ORDER field is authored here. Each implementation
checkbox carries the slot the implementer fills with the observed failure and
the GREEN run (`givn instructions steps`, TESTS and IMPLEMENTATION). A scenario
that passes on its first run is recorded as existing behavior protected; no
production edit is manufactured for it.

**Ordering.** Both `@e2e` scenarios sit last inside the batch that drives the
real binary: `trial-record` for prepare (`clean-room-preparation`) and `verify`
for the re-check (`trial-input-verification`). This is the archived
`run-command` shape and the operator's direction for this change; it is the one
place this file deviates from the format bullet "non-@e2e scenarios first,
`@e2e` last" at file level. Every non-`@e2e` scenario inside a batch precedes
its `@e2e` sibling.

**Eleven report lines.** design.md "Report" fixes ten lines plus `Receipt path`
when `--receipt` is given. The eleven-line form is asserted only where a
receipt is written (the prepare `@e2e` scenario and the dry-run scenario).

## Batch map

| Batch | Scenarios | Owning capability |
|---|---|---|
| `admission-refusals` | 9 | `clean-room-preparation` |
| `partition-and-write` | 8 | `clean-room-preparation` |
| `replacement-refusals` | 7 | `clean-room-preparation` |
| `target-and-copy-refusals` | 10 | `clean-room-preparation` |
| `copy-kinds` | 3 | `clean-room-preparation` |
| `trial-record` | 5 (4 regular + 1 `@e2e`) | `clean-room-preparation` |
| `verify` | 20 (19 regular + 1 `@e2e`) | `trial-input-verification` |
| `durable-docs` | 0 | — |
| `review-closure` | 5 | 1 `clean-room-preparation` + 4 `trial-input-verification` |
| `ADR-076` | 1 (`@givn.modified`) | `adr-qualification-gate` |
| **Total** | **64** | 43 `clean-room-preparation` + 20 `trial-input-verification` + 1 `adr-qualification-gate` |

Every scenario of `specs/prepare-an-ise-test/clean-room-preparation.feature`
(43) and `specs/prepare-an-ise-test/trial-input-verification.feature` (20)
appears in exactly one batch by exact title. The delta carries 63 `@givn.added`
scenarios and one `@givn.modified` unit: the `adr-qualification-gate` fragment
scenario in `specs/fragments/adr-qualification-gate.feature`, which moves the
permanent register assertion from 27 to 28 accepted decisions. Nothing is
`@givn.removed`.

---

## Interaction coverage pre-flight

Cross-checked the owning `## Interactions` inventory against design.md's
Interaction Coverage Matrix (`## Interaction Coverage Matrix`).

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/prepare-an-ise-test/usecase.md` | 2 | 2 | yes |
| **Total** | **2** | **2** | **both `@e2e`** |

| Inventory entry | Matrix target | Delta scenario |
|---|---|---|
| clean-room-preparation: prepare a clean-room trial from a selected revision (the `--path` and `--full` forms, the nine refusals, dry run, and both backends are variants) | Developer prepares a clean-room trial from a recorded revision | present, `@e2e` |
| trial-input-verification: re-check a prepared trial's frozen inputs (clean, retained drift, `e2e` drift, current-checkout drift, and a record that does not belong to the trial are variants) | Verifier re-checks a prepared trial and receives the frozen-input report | present, `@e2e` |

No gap: both matrix targets resolve to a scenario in this change's delta
features and both are tagged `@e2e`. No task is written for an `@e2e` scenario
that does not exist.

---

## Domain Constraints

Copied from `specs/prepare-an-ise-test/usecase.md` (guarantees, main flow,
extensions, rules, Personas) and the ideation seed
`givn/ideation/ise-rebuildability/changes/add-ise-clean-room-commands.md`
(`Q3`, `Q4`, `Q5`, `Q13`, `S4`). Each scenario's Design constraints block
repeats the rules its behavior exercises.

| Capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| `clean-room-preparation` | Preparation reads the selected revision (`HEAD` when none is named), never the working tree, so an ignored or untracked file cannot enter the trial; the checkout is never written, including the receipt path. | `developer--primary-user`, `agent--technical-role` |
| `clean-room-preparation` | Exactly one of `--full` and `--path` is required, and `--into` is required. `--path` is repeatable and accepts a tracked file or a directory that expands to every tracked file beneath it at the revision; a glob is never expanded; a named path absent from the revision removes nothing and is not a tenth refusal. | `developer--primary-user` |
| `clean-room-preparation` | The nine refusals are checked before the first write; each names its condition and subject and leaves the checkout and the target path unchanged. Neither form, both forms, or an omitted `--into` is a usage error with exit code 2. | `agent--technical-role`, `givn-maintainer--owner` |
| `clean-room-preparation` | A retained role entry that matches a directory retains every tracked file under it; a path matched by a role is never replaced; retained precedence applies to `--full` as well. | `givn-maintainer--owner` |
| `clean-room-preparation` | `Not replaced` counts the direct tracked files in the replaced files' parent directories that the developer did not name; it is a containment count, never a refusal, and it never changes the recorded sets. | `developer--primary-user` |
| `clean-room-preparation` | `Outside the declaration` counts the tracked files outside every declared role and every source root; they are not copied and are not an error. | `agent--technical-role` |
| `clean-room-preparation` | The trial is a plain directory outside the checkout, never a branch or a worktree; relative paths, the executable bit, and symlink targets are preserved; it carries no version-control metadata and no ignored or untracked file. | `developer--primary-user`, `givn-maintainer--owner` |
| `clean-room-preparation` | Every written file is re-read and compared to the file recorded at the revision; a mismatch names the drifting paths, removes the partial trial, and writes no trial record. | `givn-maintainer--owner` |
| `clean-room-preparation` | The report prints the ten fixed lines and `Receipt path` only when a receipt path was given; `--dry-run` prints them plus the `Would write:` block and writes nothing; an omitted `--receipt` prints the record to stdout and no `Receipt path` line. | `agent--technical-role` |
| `clean-room-preparation` | The record carries the revision, the named paths, the counts, one digest per retained, `e2e`, and replaced path plus one rollup per identity, the backend, the written trial path, and the running Givn version. | `givn-maintainer--owner` |
| `clean-room-preparation` | The two write targets lie outside the checkout; the trial does not exist; the receipt does not exist, does not lie inside the trial, and is not an ancestor of it. Both are created with create-new semantics. | `developer--primary-user` |
| `clean-room-preparation` | With `ise` off, or its `architecture_principles` prerequisite off, preparation refuses by name and reads no declaration. The effective project configuration decides effectiveness. | `agent--technical-role` |
| `trial-input-verification` | The re-check refuses unless the record's written path canonicalizes to `--at`, the record's backend is the selected backend, the record path lies outside the trial, and `replaced_digest` is a 64-hex value. | `givn-maintainer--owner` |
| `trial-input-verification` | The re-check compares the recorded retained files with the trial and the recorded `e2e` files and `@e2e` scenario digests with the current checkout, names every drifting path once in one run, and exits nonzero on drift. | `developer--primary-user`, `agent--technical-role` |
| `trial-input-verification` | Only the `End-to-end (E2E) test` package is frozen; a changed component step definition outside the frozen inputs produces no drift; the verifier never edits the trial. | `developer--primary-user`, `givn-maintainer--owner` |
| `trial-input-verification` | With `ise` off the re-check refuses by name and reads no record. | `agent--technical-role` |

Carried by every batch: a refusal leaves project state unchanged and names the
condition and its subject in domain language; no Rust module, struct, or field
name appears in user-facing output; exit `0` is success or clean, `1` is a
refusal, drift, or read/write failure, and `2` is a clap usage error; the trial
is evidence, never the acceptance authority.

---

## Contradictions

Recorded, not fixed. `design.md`, the delta features, and the durable docs are
unchanged by this section. No design-named scenario is missing: every scenario
described in design.md "Use-Case Traceability", "Replace Flow", "Verify Flow",
"Partition Rules", "External Boundary Cases", and the design-review decision
set resolves to one of the 58 delta scenarios, either by exact title or by the
descriptor below.

1. **`--full` versus three scenarios that assert a file under the declared
   source root survives.** `Preparation reads the revision and leaves an
   ignored file out of the trial` (declaration with one source root `src/**`,
   committed `src/kept.rs`, `--full`, then the trial holds `src/kept.rs`),
   `Preparation preserves the executable bit and the symlink target`
   (`--full`, then the trial's `src/run.sh` is executable and `src/link` is a
   symlink), and `Preparation reads an explicit revision and reports it`
   (`--full`, then the trial holds `src/kept.rs`) sit against design.md D1 and
   "Partition Rules", where `--full` removes **every tracked file under every
   declared `source:` root** that no retained role matches. The only fixture
   that satisfies the Then without editing either artifact is a declaration
   whose retained role names exactly the asserted file(s), which is D1's
   retained precedence. The batches below state that fixture per scenario. If
   the feature author intended kept source, the When must name `--path`
   instead of `--full`; that is a feature fix outside this artifact's
   authority and is reported, not applied.

2. **Two refusal scenarios name a path the Given does not commit.**
   `Preparation refuses a path outside every source root` names
   `docs/reference.md` and `Preparation refuses a path matched by a retained
   role` names `givn/specs/demo.feature`, but neither Given commits that file.
   Under D1 a `--path` value that resolves to no tracked file (and is no
   directory) removes nothing and is **not** a refusal, so the Then is
   unreachable unless the fixture commits the file. The batches require the
   commit; the features are reported, not edited.

3. **`Preparation refuses a path matched by a retained role` names no source
   root.** A declaration without `source:` is refused as malformed
   (`DeclarationError::MissingSource`), so the refusal under test would never
   run. The fixture declares one source root (`src/**`) that does not cover the
   retained path.

4. **`Verification refuses a record it cannot read` asserts more than its title
   implies.** Its Then requires both "names the record" and "names the expected
   record scope". A record that is not YAML at all yields a parse reason and no
   scope reason, so the fixture is readable YAML whose `content_scope` is wrong
   (for example `other/v1`), which produces both messages. Reported, not fixed.

5. **`Verification names an e2e package file changed in the trial` is detected
   by the retained check.** Every resolved `e2e` file is also a retained file,
   so a change inside the trial is the retained reason
   `the trial differs from the record`, not the checkout reason. The scenario
   asserts only that the path is named, so its Then holds; design.md's reason
   table assigns the retained reason.

6. **`RevisionContent` has no consumers outside the seam.** design.md "Contracts
   and Components" names `ise` as the only consumer of
   `revision_files`/`revision_blobs`; the tree has no caller outside
   `src/scm.rs`. The `RevisionEntry` return-type change is therefore internal to
   the seam and migrates no consumer. An earlier draft of this contradiction
   named `cli::archive` and `cli::migration`; that text was stale and is
   corrected here, matching the design-review post-review clarification 2.

7. **The re-read mismatch has no delta scenario.** design.md ("Blockers and
   gaps" 2, "Strict Mode") places the write/re-read comparison in a unit test
   with an injectable writer because no deterministic fixture can corrupt a
   file between the write and the re-read. Batch `partition-and-write` carries
   that unit test in `src/ise/mod.rs`; no scenario records it, and no scenario
   is claimed to cover it.

---

## Durable documentation placement

The arc42 chapters, the glossary rows, the ADR, and the arc42 README already
carry this change from the authoring revision `ea543c79`. They describe planned
behavior, so each statement is verified in the batch whose code makes it true,
and a statement that reality contradicts is corrected in that batch.
`docs/reference.md` documents delivered CLI behavior and receives its command
section in `durable-docs`.

| Statement | Location | Batch | What makes it true |
|---|---|---|---|
| `ise replace` / `ise verify` CLI rows, report lines, record scope and fields, nine refusals, verify drift lines, exit codes, receipt placement, dry-run output, Git/Fossil entry kinds | `docs/reference.md` — `## CLI commands` table rows plus a new `### Clean-room trial` subsection under `## Verification and recovery` | `durable-docs` (written); statements checked by `admission-refusals` … `verify` | the implemented command, report, refusals, and record |
| `Trial directory`, `Trial record`, `Trial E2E digest`, scoped `E2E identity` anti-term | `docs/glossary.md:73-76` | `trial-record` | the `ise-trial/v1` record and the printed report lines |
| `TC-46`, `TC-47` | `docs/arc42/02-architecture-constraints.md:54-55` | `admission-refusals` (revision read, refusals), `copy-kinds` (entry kinds), `verify` (re-check) | the preflight order, the backend readers, and the drift check |
| Two CLI rows, the independent-verifier entity, clean-room preparation in the in-scope list | `docs/arc42/03-context-and-scope.md:32,62-63` | `admission-refusals` (CLI surface), `verify` (verifier) | the two subcommands and the re-check |
| Clean-room trial strategy row | `docs/arc42/04-solution-strategy.md:24` | `partition-and-write` | the revision-read partition and the plain-directory write |
| `cli::ise`, `ise`, and the two read seams in the module map | `docs/arc42/05-building-block-view.md:62-63,90` | `admission-refusals` (command + preflight), `trial-record` (record + `e2e_file_set` + scenario digest) | the module responsibilities |
| `6.21` replace and verify flows | `docs/arc42/06-runtime-view.md:982` | `partition-and-write` (replace), `verify` (re-check) | the implemented flow order |
| `8.29` record shape, per-path identity, receipt placement, dry-run output, backend parity | `docs/arc42/08-crosscutting-concepts.md:1464` | `trial-record` (record, identity, placement, dry run), `copy-kinds` (backend parity) | the serialized record and both readers |
| `ADR-076-clean-room-trial-record` and its register row | `docs/arc42/adr/ADR-076-clean-room-trial-record.md`, `docs/arc42/adr/README.md:63,80` | `durable-docs` (verified, corrected if stale) | the durable record decision |
| `R-142` | `docs/arc42/11-risks-and-technical-debt.md:129` | `replacement-refusals` | an absent `--path` value removes nothing and reports `Replaced: 0` |
| `R-145` | `docs/arc42/11-risks-and-technical-debt.md:132` | `copy-kinds` | the Fossil manifest reader and the `allow-symlinks on` fixture |
| `R-143`, `R-144`, `R-146` | `docs/arc42/11-risks-and-technical-debt.md:130-133` | `trial-record` (`R-143`), `target-and-copy-refusals` (`R-144`), `verify` (`R-146`) | the recorded-set limit, the target containment check, the `Rule:`-nested selection |
| Clean-room trial commands in the current-state list | `docs/arc42/README.md` | `durable-docs` | the two subcommands |
| `docs/domain-model.md` unchanged; `assets/instructions/ise.md` unchanged | — | `durable-docs` (recorded, not edited) | the model scopes domain units, not command parsing or file formats; the five `ise` obligations already name the boundary and the frozen package |

---

## Setup

- [x] Read the owning use-case document and this change's seed before writing
      any step: `specs/prepare-an-ise-test/usecase.md` (guarantees, main flow,
      extensions, rules, Personas, Interactions) and
      `givn/ideation/ise-rebuildability/changes/add-ise-clean-room-commands.md`
      (`Q3`, `Q4`, `Q5`, `Q13`, `S4`). Domain constraints are in
      `## Domain Constraints` above.
      Evidence: every scenario's Design constraints carry its owning rule, and
      the pre-flight maps both inventory entries to matrix rows and existing
      `@e2e` scenarios.
      ```
      Read 2026-11-15: usecase.md sections Goal..Interactions and the seed's
      Q3/Q4/Q5/Q13/S4; ## Domain Constraints and each scenario's Design
      constraints carry the same rules.
      ```

- [x] Confirm the runner and both commands. No new runner is installed:
      `tests/features_runner.rs` uses cucumber-rs 0.23 with `.fail_on_skipped()`
      at both registration points; `verify.command` is
      `./run-tests.sh --non-e2e` and `verify.e2e_command` is
      `./run-tests.sh --e2e` in `givn/commands.yaml`. `run-tests.sh` selects the
      regular scope with `not @e2e and not @givn.removed` and the e2e scope
      with `@e2e and not @givn.removed`, so the two executions select different
      scenarios; `GIVN_RESULT_SCOPE` stays the explicit scope declaration.
      ```text
      verify.command:     "./run-tests.sh --non-e2e"
      verify.e2e_command: "./run-tests.sh --e2e"
      regular scope: not @e2e and not @givn.removed
      e2e scope:     @e2e and not @givn.removed
      GIVN_RESULT_SCOPE: regular | e2e
      ```

- [x] Create the four step-definition skeletons named by design.md "Step
      Definitions", one file per capability, and register them in
      `tests/steps/mod.rs`:
      `tests/steps/clean_room_preparation_steps.rs`,
      `tests/steps/clean_room_preparation_e2e_steps.rs`,
      `tests/steps/trial_input_verification_steps.rs`,
      `tests/steps/trial_input_verification_e2e_steps.rs`, plus the
      fixture-only `tests/steps/ise_clean_room_support.rs` (no
      `#[given]`/`#[when]`/`#[then]`).
      Bind every delta phrase in the per-capability file with the
      not-implemented stub `Err(anyhow!("not implemented: <step text>"))`; the
      prepare feature holds 211 step lines and the verify feature 64, and
      one binding serves every scenario that repeats the phrase. Reuse an
      existing permanent binding whenever the phrase already exists in
      `tests/steps/`; a phrase is bound exactly once across all files, because
      a duplicate registration panics the runner.
      ```
      cargo check --locked --tests → Finished `dev` profile, exit 0
      bindings per file:
        clean_room_preparation_steps.rs 125
        clean_room_preparation_e2e_steps.rs 10
        trial_input_verification_steps.rs 36
        trial_input_verification_e2e_steps.rs 0 (the verify batch binds its @e2e phrases)
      171 distinct phrases from the 211 + 64 step lines; no duplicate phrase binding.
      ```

- [x] Proof-of-strictness. Write one scratch feature outside the delta scope
      (a scratch `GIVN_FEATURES` directory) with one unbound step, and run it
      through both runner paths: the plain path with `GIVN_RESULT_FILE` and
      `GIVN_RESULT_SCOPE=e2e`, and the `--exact` path the batches use for
      RED/GREEN.
      ```
      /tmp/ise-strictness-probe/probe.feature:
        Feature: Strictness probe
          Scenario: An unbound step fails
            Given this step is deliberately unbound

      plain:
        GIVN_FEATURES=/tmp/ise-strictness-probe \
        GIVN_RESULT_FILE=/tmp/ise-probe-result.json GIVN_RESULT_SCOPE=e2e \
        cargo test --locked --test features_runner
        → exit 1; "Step doesn't match any function"
        → {"failed":1,"passed":0,"scope":"e2e","skipped":0,"total":1}

      exact:
        GIVN_FEATURES=/tmp/ise-strictness-probe cargo test --locked \
        --test features_runner -- --exact "probe.feature::An unbound step fails"
        → exit 101; "1 step failed"; no result file written
      ```
      If either path exits 0, stop: `.fail_on_skipped()` is not in effect on
      the builder that run used.

- [x] E2E setup, before the first `@e2e` scenario. The two `@e2e` step files
      exist from the skeleton task. `verify.e2e_command` is
      `./run-tests.sh --e2e` with `GIVN_RESULT_SCOPE=e2e`. No server, database,
      container, browser, or digital twin exists (design.md "Local
      Runnability"): each `@e2e` scenario writes a temp project, runs the built
      binary through `env!("CARGO_BIN_EXE_givn")`, and asserts its stdout,
      stderr, and exit code. Prove the explicit scope selection before writing
      any `@e2e` step:
      ```
      GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
        cargo test --locked --test features_runner -- --list | grep -c ': test'
      → 58
      the same with GIVN_CUCUMBER_TAGS='@e2e and not @givn.removed' → 2
      ./run-tests.sh --e2e (permanent scope) → exit 0;
        Summary [39.143s] 111 tests run: 111 passed, 980 skipped
      ```
      The e2e count is strictly less than the full delta count: 2 of 58 at the
      setup point, 2 of 64 after the review-closure batch added five scenarios.

- [x] Restore a passing baseline: delete the scratch feature and result file,
      run `./run-tests.sh --non-e2e`, and confirm exit 0. The delta scenarios
      stay `@wip` at this point in the record; the permanent tree binds none of
      the new phrases.
      ```
      ./run-tests.sh --non-e2e → exit 0 (2026-11-15)
      Summary [167.740s] 980 tests run: 980 passed, 111 skipped
      ```
      The failure recorded here earlier (`unexpected proposed ADR count: left
      1, right 0`, exit 100) was the permanent ADR assertion that `ea543c79`
      made stale. `d5fa08ec` reconciled it by accepting ADR-076 and moving the
      permanent fragment scenario to 28 accepted decisions, so the permanent
      suite is green from that commit onward. The scratch feature and result
      file were removed before this run.

- [x] COMMIT: `c6524e2591890cd0c5a361e11927f497cb56117f` -- Batch: `setup`
      The skeletons, the support module, and its registration land in this
      commit, so the behavior batches start from a clean tree.

---

## Behavior batches

Each batch names the complete file set for its scenarios; its scenarios share
one test phase, one implementation phase, one refactor phase, and one commit. A
scenario's Design constraints name its fixture, its assertions, the step file
that binds it, and the production symbols it exercises. Every batch removes
`@wip` from the scenarios it starts, and no batch removes `@e2e`.

### Batch 1: `admission-refusals`

The preflight conditions that run before any trial content is read, plus the
Git and Fossil revision-entry readers the later batches consume. No scenario in
this batch writes a trial.

**Files created:** `src/ise/mod.rs` (preflight skeleton), `src/cli/ise.rs`,
`tests/steps/ise_clean_room_support.rs` (Git/bare-marker/dirty fixtures built in
this batch).
**Files modified:** `src/main.rs` (`Commands::Ise` with `replace` and `verify`,
dispatch), `src/cli/mod.rs` (`pub mod ise;`), `src/scm.rs`
(`RevisionEntry { path, kind, executable }` and the `revision_files` return
type), `src/git.rs` (`ls-tree` mode to kind and executable bit),
`src/fossil.rs` (check-in manifest `F` cards to path, kind, and executable bit,
with the `\s` decode and the 40/64-hex hash check; unit-tested in
`src/fossil.rs`),
`src/config/declaration.rs` (`normalize_project_path` becomes `pub(crate)`),
`tests/steps/mod.rs` (register the four delta files),
`tests/steps/clean_room_preparation_steps.rs`.

**Design constraints (batch-wide):**
- `Commands::Ise` holds `Replace { at: Option<String>, full: bool, path:
  Vec<String>, into: PathBuf, receipt: Option<PathBuf>, dry_run: bool }` and
  `Verify { trial: PathBuf, at: PathBuf }`. `--at` defaults to `HEAD`, and
  `--into` is required at the parser, because this batch's fixtures pass it.
  The exactly-one-form `ArgGroup` over `--full` and `--path`, and its exit-2
  messages, land in `replacement-refusals`; `Preparation refuses an omitted
  target` may therefore pass on its first run there and is recorded as existing
  behavior protected.
- The preflight order is design.md "Replace Flow" steps 1 to 5: effective
  manifest (`config::overlay::load_effective_at`) with `ise` and
  `architecture_principles`, then `SpecificationDeclaration::load`, then
  `scm::select`, then `Backend::dirty_entries`, then the revision
  (`Backend::revision` for `HEAD`, `commit_exists` for an explicit value).
  Every check precedes the first write; none of them writes.
- The add-on refusal names `ise` or `architecture_principles` and the enable
  command `givn addons enable <name>`; it runs before the declaration load, so
  an `ise`-off project with a malformed declaration reports the add-on.
- Every refusal prints `ise replace: <reason>` on stderr and exits 1. No Rust
  module, struct, or field name appears in the text.
- `RevisionEntry` is the backend-neutral shape from design.md "Data Model
  Changes": `path: String`, `kind: EntryKind` (`Regular | Symlink |
  Submodule`), `executable: bool`; `Backend::revision_files(revision, root)`
  returns `Vec<RevisionEntry>`, and an empty `root` means every tracked path.
- Git: `git ls-tree -r -z <revision> -- <root>`; `100755` sets `executable`,
  `120000` is `Symlink`, `160000` (a `commit` object) is `Submodule`,
  otherwise `Regular`.
- Fixtures live in `tests/steps/ise_clean_room_support.rs`; steps drive
  `common::run_givn_with_envs_in` against `env!("CARGO_BIN_EXE_givn")` and
  import no `givn` module except through the public API used by `common`.

**Batch verification:** the single-scenario command below for each scenario,
then `./run-tests.sh --non-e2e` to prove the seam change broke no permanent
suite. **Commit:** `givn commit <paths> -m '<release note>\n\nBatch:
admission-refusals'`.

- [x] Scenario: `Preparation refuses when the declaration is missing`
  - Design constraints:
    - Fixture: a temp Git project with `givn/config.yaml` (`addons.ise: true`,
      `addons.architecture_principles: true`), one committed file, and **no**
      `givn/specification.yaml`; `--into` names a fresh directory outside the
      checkout, `--receipt` a fresh file beside it.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `givn/specification.yaml` and the missing
      declaration; the trial directory and the receipt path remain absent.
    - Contract: `SpecificationDeclaration::load` returns `Ok(None)`; the
      refusal is the first check after the add-on pair and writes nothing.
    - Step file: `tests/steps/clean_room_preparation_steps.rs`; fixture
      builder: `ise_clean_room_support.rs`.
  - [x] Implement the scenario and its tests (test-first per `design.md`).
        Files: `src/cli/ise.rs`, `src/ise/mod.rs`, `src/main.rs`,
        `src/cli/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`.
        Run the single-scenario command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses when the declaration is missing"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed: the fixture builders and the
        preflight stay one responsibility each; cargo fmt --check and
        cargo clippy --workspace --all-targets -- -D warnings are clean; the
        re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses a malformed declaration`
  - Design constraints:
    - Fixture: `givn/config.yaml` with both add-ons on, and a
      `givn/specification.yaml` carrying one unknown top-level key (for
      example `component: ["x"]`) beside the five roles and `source:`.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `givn/specification.yaml` **and** the loader's
      reason (`unknown key 'component'`), which is
      `DeclarationError::detail`; the trial and the receipt remain absent.
    - Contract: `SpecificationDeclaration::load` returns
      `Err(DeclarationError::…)`; the refusal renders the path and the reason
      and never the Rust enum.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a malformed declaration"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses while the ise addon is off`
  - Design constraints:
    - Fixture: `givn/config.yaml` with `addons.ise: false` and
      `architecture_principles: true`, **plus a malformed declaration** so that
      an implementation which reads the declaration first would report it.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `ise` and the enable command
      (`givn addons enable ise`); stderr does **not** mention
      `givn/specification.yaml` (the declaration is not read); targets absent.
    - Contract: the manifest add-on pair is checked before
      `SpecificationDeclaration::load`; the add-on is the effective project
      manifest's value (D3), never a change lock.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses while the ise addon is off"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses while architecture principles are off`
  - Design constraints:
    - Fixture: `givn/config.yaml` with `addons.ise: true` and
      `addons.architecture_principles: false`, and a malformed declaration on
      disk (`component: ["x"]` beside the six valid keys).
      `givn addons disable architecture_principles` refuses that combination,
      so the fixture writes the file directly.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `architecture_principles` and never mentions
      `givn/specification.yaml`; targets absent.
    - Contract: D10 in the design ledger — both commands refuse when either
      add-on is off, before the declaration or the record is read.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses while architecture principles are off"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        REVIEW CLOSURE (F6): the fixture now carries a malformed declaration and
        the Then adds `the declaration is not read`, so the ordering rule is
        observable. DISCRIMINATION (not historical RED): with
        `require_addons(&manifest)?` moved after the declaration load in
        `src/ise/admit`, the scenario fails:
          stderr "ise replace: givn/specification.yaml: unknown key 'component' — …"
          does not name "architecture_principles"
        Probe reverted with `git checkout -- src/ise/mod.rs`. COMMIT:
        23ad9e6f -- Batch: `review-closure`.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses a dirty checkout and names every dirty path`
  - Design constraints:
    - Fixture: committed `src/modified.rs` modified in the working tree, and an
      untracked `notes-untracked.md` that no ignore rule covers.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names both `src/modified.rs` and
      `notes-untracked.md`; targets absent.
    - Contract: `Backend::dirty_entries` reports modified tracked files plus
      untracked non-ignored files; every path is printed, never only the first.
      The dirty check precedes the revision read.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a dirty checkout and names every dirty path"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses a project with no version-control backend`
  - Design constraints:
    - Fixture: a plain directory with `givn/config.yaml` (both add-ons on), a
      valid declaration, one file, and no `.git` directory and no
      `.fslckout`/`_FOSSIL_` marker.
    - When: `givn ise replace --full --into <dir> --receipt <file>` with the
      project root as cwd.
    - Then: exit 1; stderr names the missing backend using `scm::select`'s own
      message; targets absent.
    - Contract: the command never guesses a backend and adds no second
      selector.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a project with no version-control backend"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses a project root nested inside a Git worktree`
  - Design constraints:
    - Fixture: `git init` at `top/`, and the Givn project at
      `top/nested/project/` with `givn/config.yaml` and a valid declaration; the
      binary runs with `top/nested/project` as cwd.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names the nested project root (the selector's
      message); targets absent.
    - Contract: `scm::select` refuses a project root that is not the worktree
      top level; the command adds no rule of its own.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a project root nested inside a Git worktree"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses a checkout carrying both Git and Fossil markers`
  - Design constraints:
    - Fixture: a project root with a `.git` directory **and** an `.fslckout`
      file, `givn/config.yaml` with both add-ons on, and a valid declaration.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names the ambiguous version control (the selector's
      message); targets absent.
    - Contract: `scm::select` refuses the mixed marker set; no backend is
      preferred silently.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a checkout carrying both Git and Fossil markers"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

- [x] Scenario: `Preparation refuses an absent revision`
  - Design constraints:
    - Fixture: a clean Git project with both add-ons on, a valid declaration,
      and one commit.
    - When: `givn ise replace --at 0000000000000000000000000000000000000000
      --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names that revision; targets absent.
    - Contract: `HEAD` resolves through `Backend::revision`; an explicit value
      is checked with `commit_exists` before `revision_files` is called, so the
      refusal names the revision rather than a backend command error.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses an absent revision"
        → failed: assertion `left: Some(2)` `right: Some(1)`; stderr=
          "error: unrecognized subcommand 'ise'".
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change was needed; fmt and clippy are clean and
        the re-run stayed green.
        ```
  - [x] COMMIT: `373591cc1437daa21fd895c7c6d78cdf0e1dd538` -- Batch: `admission-refusals`

---

### Batch 2: `partition-and-write`

The partition, the copy set, the plain-directory write, and the re-read
comparison. Every scenario in this batch reads one revision and writes a trial;
none refuses.

**Files created:** none (the modules exist from batch 1).
**Files modified:** `src/ise/mod.rs` (partition, copy set, writer, re-read unit
test), `src/cli/ise.rs` (report model: `Backend`, `Revision`, and the five count
lines), `src/scm.rs` (`Backend::revision_blobs` reports an unreadable path and
the revision for both backends; each adapter's skipped path becomes that named
error), `tests/steps/ise_clean_room_support.rs`,
`tests/steps/clean_room_preparation_steps.rs`. `src/git.rs` and `src/fossil.rs`
are unchanged: the seam owns the contract and neither adapter grows.

**Design constraints (batch-wide):**
- `Partition { retained, kept_source, replaced, outside, not_replaced }`, all
  sorted, project-relative `/` paths. Role test order is
  `behavior, structure, contracts, limits, e2e` (`SpecificationDeclaration::
  roles()`), first match wins, and a role entry matches a tracked file or every
  tracked file under a derived directory.
- `--full` is the union of every declared `source:` root resolved at the
  revision, **after** the retained test, so a retained file under a source root
  stays. A tracked file inside a source root that is neither retained nor
  replaced is kept source; a tracked file outside every role and root is
  outside the declaration and is not copied.
- A `--path` value is normalized with the declaration's normalizer
  (`normalize_project_path`, now `pub(crate)`): a tracked file contributes
  itself, a directory expands to every tracked file beneath it, a value that is
  neither contributes nothing, a glob-shaped value is never expanded, and
  repeats collapse to one entry.
- `not_replaced` counts the direct tracked files in the distinct parent
  directories of `replaced_paths` that are not themselves replaced.
- Copy set is `retained ∪ kept_source`. Read every blob through
  `Backend::revision_blobs` **before** `create_dir`, so a read failure leaves
  the target absent; then create the parent, `create_dir` the trial, create
  each directory, write each regular file, and set the executable bit on unix.
- Re-read: every written path is read again and compared by kind, executable
  bit, and bytes to the revision entry. `WriteOutcome::Mismatched { paths }`
  removes the trial and writes no record. The mismatch is proven by the
  injectable-writer unit test in `src/ise/mod.rs` (Contradictions 7), not by a
  delta scenario: a deterministic fixture cannot corrupt a file between write
  and re-read, so no scenario is claimed to cover it.
- The report model prints `Backend: <git|fossil>`, `Revision: <value>` (`HEAD`'s
  resolved identifier when `--at` is omitted or `HEAD`, the supplied identifier
  otherwise), then `Retained`, `Kept source`, `Replaced`,
  `Outside the declaration`, and `Not replaced`. The digest lines and
  `Written path` land in `trial-record`.
- Every success assertion reads the trial directory on disk. A scenario that
  passes on its first run is recorded as existing behavior protected.

**Batch verification:** the single-scenario command below for each scenario,
then `./run-tests.sh --non-e2e` for the `revision_blobs` behavior change
(observed on `5d3a6ef`: 980 tests run, 980 passed, 111 skipped; `cargo test
--locked --lib ise::tests` → 2 passed).
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch:
partition-and-write'`.

- [x] Scenario: `Preparation counts the files left in a replaced path's directory`
  - Design constraints:
    - Fixture: a declaration with all five roles present (`behavior`,
      `structure`, `contracts`, `limits`, `e2e` empty) and
      `source: ["src/**"]`; a committed revision with
      `src/keys/parser.rs`, `src/keys/mod.rs`, and `src/keys/tests.rs`.
    - When: `givn ise replace --path src/keys/parser.rs --into <dir>`.
    - Then: exit 0; the report prints `Not replaced: 2`; the trial holds
      `src/keys/mod.rs` and `src/keys/tests.rs`.
    - Contract: `not_replaced` counts direct tracked siblings of the replaced
      files in their parent directories.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation counts the files left in a replaced path's directory"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        UNIT (re-read mismatch, Contradictions 7):
          cargo test --locked --lib ise::tests → 2 passed; the corrupting
          writer yields Mismatched { paths: ["src/kept.rs"] } and removes the
          trial.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Removed an avoidable `replaced` clone in the partition; cargo fmt
        --check and clippy -D warnings are clean; the re-run stayed green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `Preparation expands a directory path to the tracked files beneath it`
  - Design constraints:
    - Fixture: `source: ["src/**"]`, empty retained roles; a committed revision
      with `src/keys/parser.rs`, `src/keys/mod.rs`, `src/other.rs`.
    - When: `givn ise replace --path src/keys --into <dir>`.
    - Then: exit 0; `Replaced: 2` and `Not replaced: 0`; the trial omits both
      `src/keys` files and holds `src/other.rs`.
    - Contract: the directory is derived from the tracked file paths (a
      backend lists no directory entries); the expansion is bounded by the
      revision, and the retained-role test runs over the expansion.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation expands a directory path to the tracked files beneath it"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `A full replacement removes every tracked file under the source roots`
  - Design constraints:
    - Fixture: `behavior: ["givn/specs/**"]`, `source: ["src/**"]`; a committed
      revision with `src/one.rs`, `src/nested/two.rs`, `givn/specs/demo.feature`.
    - When: `givn ise replace --full --into <dir>`.
    - Then: exit 0; `Replaced: 2` and `Not replaced: 0`; the trial holds
      `givn/specs/demo.feature` and holds no path under `src`.
    - Contract: `--full` is the union of the source roots at the revision after
      the retained test; the retained role sits outside the source roots here,
      which is what makes `Not replaced: 0` (design-review blocker 1).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::A full replacement removes every tracked file under the source roots"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `Preparation does not expand a glob-shaped path`
  - Design constraints:
    - Fixture: `source: ["src/**"]`, empty retained roles; a committed revision
      with `src/one.rs` and `src/nested/two.rs`.
    - When: `givn ise replace --path 'src/**' --into <dir>`.
    - Then: exit 0; `Replaced: 0`; the trial holds both files.
    - Contract: `src/**` is neither a tracked file nor a directory, so it
      removes nothing and is not a refusal (D1, R-142). The value stays in
      `named_paths` only. No glob matching is used for deletion.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation does not expand a glob-shaped path"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `A named path absent from the revision removes nothing`
  - Design constraints:
    - Fixture: `source: ["src/**"]`, empty retained roles; a committed revision
      with `src/keys/parser.rs`.
    - When: `givn ise replace --path src/keys/parsr.rs --into <dir>`.
    - Then: exit 0; `Replaced: 0`; the trial holds `src/keys/parser.rs`.
    - Contract: a named value that resolves to no tracked file is admitted, not
      a tenth refusal; the `Replaced` count is the developer's signal (R-142).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::A named path absent from the revision removes nothing"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `Preparation reads an explicit revision and reports it`
  - Design constraints:
    - Fixture (Contradictions 1): `structure: ["src/kept.rs"]`,
      `source: ["src/**"]`; commit one revision holding `src/kept.rs`, then a
      second commit adding `src/later.rs`. The retained role is what keeps
      `src/kept.rs` in the trial under `--full`.
    - When: `givn ise replace --at <first revision> --full --into <dir>`.
    - Then: exit 0; the `Revision:` line holds the supplied first identifier;
      the trial holds `src/kept.rs` and not `src/later.rs`.
    - Contract: an explicit `--at` prints the supplied identifier (design.md
      "Trial record": only `HEAD` is resolved); the read is
      `revision_files(<supplied>, "")`, never the working tree.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation reads an explicit revision and reports it"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `Preparation reads the revision and leaves an ignored file out of the trial`
  - Design constraints:
    - Fixture (Contradictions 1): `structure: ["src/kept.rs"]`,
      `source: ["src/**"]`; a committed `src/kept.rs`; a `.gitignore` rule plus
      an ignored file `ignored.txt` in the working tree.
    - When: `givn ise replace --full --into <dir>`.
    - Then: exit 0; the trial holds `src/kept.rs`; the trial does not hold
      `ignored.txt`.
    - Contract: preparation reads `revision_files`, not `Backend::content_paths`,
      so no ignored or untracked file can enter the trial.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation reads the revision and leaves an ignored file out of the trial"
        → exit 101: assertion `left == right` failed: stderr=ise replace:
          the clean-room trial write is not implemented yet
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

- [x] Scenario: `Preparation refuses when a revision blob cannot be read`
  - Design constraints:
    - Fixture (Contradictions 1): `structure: ["src/kept.rs"]`,
      `source: ["src/**"]`; commit `src/kept.rs`, then delete its loose object
      under `.git/objects/<xx>/<rest>` so the tree lists the path while the blob
      is absent. The retained role is what puts the file in the copy set under
      `--full`, so the copy-set blob read is the one under test.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `src/kept.rs` and the revision; the trial and
      the receipt remain absent.
    - Contract: `Backend::revision_blobs` must report the unreadable path with
      the revision, and the copy-set blob read must complete before
      `create_dir`, so the refusal leaves nothing behind. `src/git.rs`
      currently skips a path whose `git show` fails
      (`src/git.rs:463-477`); that skip becomes a named error.
  - [x] Implement the scenario and its tests. Files: `src/scm.rs`,
        `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses when a revision blob cannot be read"
        → exit 101: stderr "ise replace: the clean-room trial write is not
          implemented yet\n" does not name the unreadable path
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: the named refusal is the Backend::revision_blobs
          contract in src/scm.rs, shared by both backends; each adapter's
          skipped path becomes that error, so src/git.rs stays at its recorded
          size and src/fossil.rs does not grow.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No behavior-preserving change beyond the batch-wide clone removal;
        cargo fmt --check and clippy -D warnings are clean; the re-run stayed
        green.
        ```
  - [x] COMMIT: `5d3a6efb73aa1b44ef40ee72a265fdf02632c779` -- Batch: `partition-and-write`

---

### Batch 3: `replacement-refusals`

The partition-time refusals and the usage errors of the replacement form.

**Files modified:** `src/ise/mod.rs` (refusal 5, 6, 7), `src/main.rs` (the
`--full`/`--path` clap argument group and the required `--into`),
`src/cli/ise.rs` (usage-error pass-through), `tests/steps/ise_clean_room_support.rs`,
`tests/steps/clean_room_preparation_steps.rs`.

**Design constraints (batch-wide):**
- Refusal 5: a declared **non-empty** role entry that matches no tracked file
  and no derived directory at the revision refuses, naming the entry and the
  revision. An empty role list is admitted.
- Refusal 6: a `--path` value that resolves to a tracked file (or a directory
  with tracked files) outside every declared `source:` root refuses, naming the
  path.
- Refusal 7: a `--path` value, or a tracked file under a named directory,
  matched by a retained role refuses, naming the path and the role. The
  retained-role test runs over the expansion and **before** the source-root
  test.
- Usage errors: a clap `ArgGroup` named `replacement` holds `--full` and
  `--path`, `required(true)`, `multiple(false)`; `--into` is already required
  from `admission-refusals`. Neither form, both forms, or an omitted `--into`
  exits 2 with clap's message naming the flag(s), and nothing is written. An
  omitted `--into` is refused by the parser before this batch's body runs, so
  that scenario's RED may be the existing refusal; it is recorded as existing
  behavior protected rather than manufactured.
- Every refusal precedes `create_dir`; every fixture asserts the trial and the
  receipt stay absent.

**Batch verification:** the single-scenario command below for each scenario.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch:
replacement-refusals'`.

- [x] Scenario: `Preparation refuses a retained path missing at the revision`
  - Design constraints:
    - Fixture: `contracts: ["docs/contract.md"]`, `source: ["src/**"]`; a
      committed revision **without** `docs/contract.md`.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `docs/contract.md` and the revision; both
      targets absent.
    - Contract: refusal 5 tests every non-empty declared role entry against the
      revision's tracked files and derived directories.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a retained path missing at the revision"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: refusal 5 in `src/ise/mod.rs`
          (`require_declared_retained_entries`) tests every declared retained
          entry against the revision's tracked paths and derived directories
          before the trial is created.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

- [x] Scenario: `Preparation refuses a path outside every source root`
  - Design constraints:
    - Fixture (Contradictions 2): `source: ["src/**"]`, empty retained roles;
      the revision **commits `docs/reference.md`** so the value resolves to a
      tracked file; a tracked `src/one.rs`.
    - When: `givn ise replace --path docs/reference.md --into <dir> --receipt
      <file>`.
    - Then: exit 1; stderr names `docs/reference.md`; both targets absent.
    - Contract: refusal 6 applies to a value that resolves to tracked files; an
      unresolvable value would remove nothing instead (D1).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a path outside every source root"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        IMPLEMENTATION: `require_replaceable_paths` rejects a candidate outside
          every source root; `named_candidates` is fallible, so a value the
          declaration normalizer rejects refuses instead of silently removing
          nothing.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

- [x] Scenario: `Preparation refuses a path matched by a retained role`
  - Design constraints:
    - Fixture (Contradictions 2 and 3): `behavior: ["givn/specs/**"]`,
      `source: ["src/**"]` (a source root that does not cover the retained
      path); the revision **commits `givn/specs/demo.feature`**.
    - When: `givn ise replace --path givn/specs/demo.feature --into <dir>
      --receipt <file>`.
    - Then: exit 1; stderr names `givn/specs/demo.feature` and `behavior`; both
      targets absent.
    - Contract: refusal 7 names the path and the role; the retained test runs
      before the source-root test, so the role is the reported condition.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a path matched by a retained role"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        IMPLEMENTATION: `require_replaceable_paths` runs the retained-role test
          over the whole candidate set before the source-root test and names the
          path and the role.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

- [x] Scenario: `Preparation refuses a directory path that contains a retained path`
  - Design constraints:
    - Fixture: `behavior: ["givn/specs/**"]`, `source: ["givn/**"]`; the
      revision commits `givn/specs/demo.feature`.
    - When: `givn ise replace --path givn/specs --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `givn/specs/demo.feature` and `behavior`; both
      targets absent.
    - Contract: the directory expansion happens first and the retained test
      runs over every expanded file, so the refusal names the retained file,
      not the directory.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a directory path that contains a retained path"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `named_candidates` expands the directory before the
          retained test, so the refusal names the expanded retained file and
          its role, never the directory value.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

- [x] Scenario: `Preparation refuses when neither replacement form is given`
  - Design constraints:
    - Fixture: a clean Git project with both add-ons on and a valid
      declaration; the trial and receipt paths the scenario watches are created
      as absent markers in the Given.
    - When: `givn ise replace --into <dir> --receipt <file>`.
    - Then: exit 2; stderr names `--full` and `--path`; both targets absent.
    - Contract: the clap `replacement` `ArgGroup` is `required(true)`; the
      message is clap's, and the command body never runs.
  - [x] Implement the scenario and its tests. Files: `src/main.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses when neither replacement form is given"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(2)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: the clap `replacement` `ArgGroup` in `src/main.rs` is
          `required(true)` over `--full` and `--path`; the parser refuses before
          the command body runs.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

- [x] Scenario: `Preparation refuses when both replacement forms are given`
  - Design constraints:
    - Fixture: as the previous scenario, plus a committed `src/one.rs`.
    - When: `givn ise replace --full --path src/one.rs --into <dir> --receipt
      <file>`.
    - Then: exit 2; stderr names `--full` and `--path` as mutually exclusive;
      both targets absent.
    - Contract: `multiple(false)` on the same group; no partial execution.
  - [x] Implement the scenario and its tests. Files: `src/main.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses when both replacement forms are given"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(2)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: the same `replacement` group is `multiple(false)`, so
          clap refuses `--full` beside `--path` before any read or write.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

- [x] Scenario: `Preparation refuses an omitted target`
  - Design constraints:
    - Fixture: a clean Git project with both add-ons on and a valid
      declaration; the Given records the candidate trial and receipt paths.
    - When: `givn ise replace --full` (no `--into`).
    - Then: exit 2; stderr names `--into`; the candidate trial and receipt
      paths remain absent.
    - Contract: `--into` is required at the parser; the refusal precedes every
      read and write.
  - [x] Implement the scenario and its tests. Files: `src/main.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses an omitted target"
        → exit 101: assertion `left == right` failed: stderr=error: the
          following required arguments were not provided:
          left: Some(2)
         right: Some(1)
        The failure was the shared `preparation fails and names "..."` step
        hard-coding exit 1 for a clap usage error, not a production gap: the
        parser already required `--into` and exited 2. The step now derives the
        expected code from the failure class (clap `error:` is 2, a refusal is
        1), so neither branch can borrow the other's code.
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: existing behavior protected; no production edit for
          this requirement.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: `retained_roles` now carries each declared pattern text
        beside its compiled glob, so refusal 5 reuses the compiled roles
        instead of compiling every pattern twice. Re-run stayed green;
        cargo fmt --check and clippy -D warnings are clean.
        ```
  - [x] COMMIT: `9cd9c0af04bc2c8dae5305311cfb1b274c29e937` -- Batch: `replacement-refusals`

---

### Batch 4: `target-and-copy-refusals`

The write-target checks (refusal 8) and the copy-set check (refusal 9).

**Files modified:** `src/ise/mod.rs` (target containment, absence, receipt
placement; copy-set marker check), `tests/steps/ise_clean_room_support.rs`
(targets, receipts, symlinked parents, gitlinks), `tests/steps/clean_room_preparation_steps.rs`.

**Design constraints (batch-wide):**
- Both targets resolve to absolute paths: canonicalize the deepest **existing**
  ancestor and append the missing components, so a symlinked parent cannot
  alias a path inside the checkout. Containment uses the canonicalized project
  root.
- Order: trial outside the checkout; trial absent; receipt outside the
  checkout; receipt absent; receipt not inside the trial; receipt not an
  ancestor of the trial. The refusal names the offending target. Existing paths
  are never overwritten (create-new semantics).
- Copy set: a `Submodule` entry, or any path component `.git`, `.fslckout`, or
  `_FOSSIL_`, refuses (refusal 9) naming the file. The check runs before
  `create_dir`.
- Gitlink fixture: write the `160000` index entry with
  `git update-index --add --cacheinfo 160000,<sha>,vendor/lib`, so no second
  repository is cloned.

**Batch verification:** the single-scenario command below for each scenario.
**Coverage limit:** `Preparation refuses a repository link in the copy set`
passes with refusal 9's submodule branch removed, because the copy-set blob
read fails first on a gitlink and names the same path. The scenario asserts the
observable refusal (the path is named, the trial stays absent) and does not
claim the reason; the nested-marker sibling carries refusal 9's marker half.
Review closure F5: the submodule half is proven by the unit test
`a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link` in
`src/ise/mod.rs`, which calls `require_copy_set` directly with a `Submodule`
entry and fails when that branch is removed.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch:
target-and-copy-refusals'`.

- [x] Scenario: `Preparation refuses a target inside the checkout`
  - Design constraints:
    - Fixture: a clean Git project with a valid declaration; the
      Given names `--into ./trial` relative to the checkout.
    - When: `givn ise replace --full --into <checkout>/trial`.
    - Then: exit 1; stderr names the target; the checkout holds no trial
      directory and no receipt.
    - Contract: containment is decided on canonicalized paths before any write.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a target inside the checkout"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        IMPLEMENTATION: `require_write_targets` resolves the target through its
          deepest existing ancestor and refuses a path under the canonical
          checkout before any write.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses an existing target directory`
  - Design constraints:
    - Fixture: an existing empty directory outside the checkout named by
      `--into`.
    - When: `givn ise replace --full --into <existing dir>`.
    - Then: exit 1; stderr names the existing target; the directory is left
      unchanged (still empty).
    - Contract: the absence check precedes `create_dir`; nothing is removed.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        Observed first run: the scenario passed with no preflight, because
        `create_dir` failed on the existing path and the write error happened to
        name it. The Then now requires the named absence refusal, so the
        observable is create-new semantics, not a write failure.
        DISCRIMINATION (not historical RED): with `require_write_targets`
        removed, the same command fails with
          stderr "... create the trial at '/tmp/.tmpGJCGom/trial': File exists
          (os error 17)\n" does not report the existing path
        GREEN: with the preflight → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: the absence check runs before `create_dir`; a write
          failure is no longer what protects the scenario.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses an existing target file`
  - Design constraints:
    - Fixture: `--into` names an existing regular file holding known bytes.
    - When: `givn ise replace --full --into <existing file>`.
    - Then: exit 1; stderr names the existing target; the file's bytes are
      unchanged.
    - Contract: `create_dir` would fail anyway; the refusal is the named target
      check, before the write.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        Observed first run: the scenario passed with no preflight, because
        `create_dir` failed on the existing file and the write error happened to
        name it. The Then now requires the named absence refusal.
        DISCRIMINATION (not historical RED): with `require_write_targets`
        removed, the same command fails with
          stderr "... create the trial at '/tmp/.tmpmG3OiM/trial': File exists
          (os error 17)\n" does not report the existing path
        GREEN: with the preflight → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: the absence check runs before `create_dir`.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses a receipt inside the trial directory`
  - Design constraints:
    - Fixture: trial `<base>/trial` (absent), receipt `<base>/trial/record.yaml`.
    - When: `givn ise replace --full --into <base>/trial --receipt
      <base>/trial/record.yaml`.
    - Then: exit 1; stderr names the receipt; the trial and the receipt remain
      absent.
    - Contract: the receipt is compared against the trial path by prefix; the
      builder must not control the trust anchor (D2).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a receipt inside the trial directory"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        IMPLEMENTATION: `require_write_targets` refuses a receipt path that
          starts with the resolved trial path.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses a receipt inside the checkout`
  - Design constraints:
    - Fixture: trial outside the checkout, receipt `<checkout>/record.yaml`.
    - When: `givn ise replace --full --into <outside> --receipt
      <checkout>/record.yaml`.
    - Then: exit 1; stderr names the receipt; the checkout holds no receipt.
    - Contract: the receipt's outside-checkout check uses the same containment
      rule as the trial; the checkout is never written.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a receipt inside the checkout"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        IMPLEMENTATION: the receipt passes the same canonical containment check
          as the trial; the checkout is never written.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses a receipt that is an ancestor of the trial directory`
  - Design constraints:
    - Fixture: receipt `<base>/parent` (absent), trial `<base>/parent/trial`
      (absent), both outside the checkout, so the ancestor rule and not the
      existing-path rule produces the refusal.
    - When: `givn ise replace --full --into <base>/parent/trial --receipt
      <base>/parent`.
    - Then: exit 1; stderr names the receipt; both paths remain absent.
    - Contract: the ancestor check treats the receipt path as a directory
      prefix of the trial path (D2).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a receipt that is an ancestor of the trial directory"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 4 steps (4 passed).
        IMPLEMENTATION: `require_write_targets` refuses a receipt path that the
          resolved trial path starts with.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses a symlinked target that resolves inside the checkout`
  - Design constraints:
    - Fixture: `<tmp>/link` is a symlink to the checkout root; `--into` is
      `<tmp>/link/trial`.
    - When: `givn ise replace --full --into <tmp>/link/trial`.
    - Then: exit 1; stderr names the target; the checkout holds no trial
      directory.
    - Contract: the deepest existing ancestor (`<tmp>/link`) canonicalizes into
      the checkout, so the appended missing component is judged against the
      canonical root (design.md "Local Runnability", last obstacle).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a symlinked target that resolves inside the checkout"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        GREEN: the same command → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `resolved_target` canonicalizes `<tmp>/link` and appends
          the missing `trial` component, so the prefix test sees the checkout.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses an existing receipt`
  - Design constraints:
    - Fixture: a receipt file outside the checkout that already exists with
      known bytes; an absent trial path.
    - When: `givn ise replace --full --into <absent> --receipt <existing>`.
    - Then: exit 1; stderr names the existing receipt; its bytes are unchanged;
      the trial remains absent.
    - Contract: `create_new` semantics; an earlier trial's evidence is never
      overwritten (D2).
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses an existing receipt"
        → exit 101: assertion `left == right` failed: stderr=
          left: Some(0)
         right: Some(1)
        DISCRIMINATION (not historical RED): with `require_write_targets`
        removed, the same command fails on the missing named refusal, because
        the receipt path is not written yet.
        GREEN: with the preflight → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: the receipt absence check refuses before the trial is
          created and leaves the existing record untouched.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses a repository link in the copy set`
  - Design constraints:
    - Fixture: `structure: ["vendor/lib"]`, `source: ["src/**"]`; the revision
      carries a gitlink at `vendor/lib` written with
      `git update-index --add --cacheinfo 160000,<sha>,vendor/lib`.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `vendor/lib`; both targets absent.
    - Contract: `RevisionEntry::kind == Submodule` refuses (refusal 9); the
      check precedes the write.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/git.rs`, `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        Observed first run: the scenario passed before refusal 9 existed,
        because the copy-set blob read fails first on a gitlink and names the
        same path; the trial stays absent either way.
        DISCRIMINATION (not historical RED): with `require_copy_set` removed,
        the same command still passes (exactly the blob-read masking above), so
        this scenario cannot distinguish the submodule half of refusal 9. The
        nested-marker sibling fails without the check and carries that half.
        GREEN: with refusal 9 → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `require_copy_set` refuses a `Submodule` entry in the
          copy set before any content read; reported as a coverage limit below.
        REVIEW CLOSURE (F5): this end-to-end scenario is redundant with the
          blob-read refusal for the submodule half. The branch is proven by the
          unit test
          `a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link`
          in `src/ise/mod.rs` (COMMIT: 23ad9e6f -- Batch: `review-closure`),
          which calls `require_copy_set` with a `Submodule` entry directly and
          fails when the submodule branch is removed. The scenario title states
          the observable refusal, not the reason, so it is not retitled.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

- [x] Scenario: `Preparation refuses a nested repository marker in the copy set`
  - Design constraints:
    - Fixture: `structure: ["nested/_FOSSIL_"]`, `source: ["src/**"]`; the
      revision commits `nested/_FOSSIL_` as a regular file.
    - When: `givn ise replace --full --into <dir> --receipt <file>`.
    - Then: exit 1; stderr names `nested/_FOSSIL_`; both targets absent.
    - Contract: any path component `.git`, `.fslckout`, or `_FOSSIL_` refuses
      (refusal 9); the check covers every copied path, at any depth.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED: GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a nested repository marker in the copy set"
        → exit 101: the command succeeded; stderr=""
        DISCRIMINATION (not historical RED): with `require_copy_set` removed,
        the same command fails with the same succeeded/stderr="" output, so the
        scenario discriminates refusal 9.
        GREEN: with refusal 9 → exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `require_copy_set` refuses a copied path whose component
          is `.git`, `.fslckout`, or `_FOSSIL_`, before the trial is created.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Batch-wide: the copy-set check reads the revision entries without a
        per-entry map, and the partition moved to `src/ise/partition.rs` when
        `src/ise/mod.rs` passed the 500-code-line limit (374 + 212 code
        lines). The re-run stayed green; cargo fmt --check and clippy
        -D warnings are clean; ./run-tests.sh --non-e2e is 980/980.
        ```
  - [x] COMMIT: `1164935743437acbf720889ae2bf56afdd89f447` -- Batch: `target-and-copy-refusals`

---

### Batch 5: `copy-kinds`

Entry kinds, the executable bit, symlink targets, plain-directory absence of
metadata, and the Fossil adapter.

**Files modified:** `src/scm.rs` (the Fossil adapter's `revision_files`),
`src/fossil.rs` (check-in manifest reader), `src/ise/mod.rs` (kind-aware write
and the unix/cfg symlink split), `tests/steps/ise_clean_room_support.rs` (the
Fossil fixture and the kind fixtures),
`tests/steps/clean_room_preparation_steps.rs`.

**Design constraints (batch-wide):**
- Fossil reads `fossil artifact <revision_arg>` (with `HEAD` mapped to
  `current`) and parses each `F <path> <hash> [flags]` line: `\s` decodes to a
  space; the hash must be 40-hex or 64-hex; `x` sets `executable`; `l` sets
  `Symlink`. An `F` line without a valid hash, or with an unknown flag, refuses
  with the line and the revision. `revision_blobs` stays
  `fossil cat -r <revision> <path>`.
- Git derives the same facts from `ls-tree` modes (`100644`, `100755`,
  `120000`, `160000`).
- Write: a regular file is written and its executable bit set on unix; a
  symlink is created with its recorded target on unix; on a host without
  symlink support a symlink entry refuses by name before the record. The
  re-read compares kind, executable bit, and bytes or target.
- The trial carries no version-control metadata: the copy set comes from
  `revision_files`, which yields no `.git`/`.fslckout` entry, and refusal 9
  rejects those components in any case.
- **Fossil fixture requirement:** run `fossil settings allow-symlinks on`
  (a repository setting) **before** `fossil add` of the symlink; without it
  Fossil records no `l` entry and the scenario would assert a symlink that the
  fixture never committed. Commit the executable file after `chmod +x`, and
  assert both kinds in the trial.

**Batch verification:** the single-scenario command below for each scenario,
then `./run-tests.sh --non-e2e` for the `scm` change.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: copy-kinds'`.

- [x] Scenario: `Preparation preserves the executable bit and the symlink target`
  - Design constraints:
    - Fixture (Contradictions 1): `structure: ["src/run.sh", "src/link"]`,
      `source: ["src/**"]`; the revision commits `src/run.sh` (mode `100755`)
      and `src/link` (mode `120000`, target `run.sh`).
    - When: `givn ise replace --full --into <dir>`.
    - Then: exit 0; `<trial>/src/run.sh` has an executable mode bit;
      `<trial>/src/link` is a symlink whose target reads `run.sh`.
    - Contract: `RevisionEntry.kind` and `.executable` drive the write; the
      re-read compares both.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/git.rs`, `src/scm.rs`, `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        Observed first run: RED as the bound not-implemented Given,
          "not implemented: a committed revision with an executable script
          \"src/run.sh\" and a symlink \"src/link\" to \"run.sh\"" ;
          1 scenario (1 failed), 2 steps (1 passed, 1 failed).
        The first green attempt exposed a fixture contradiction: the shared
          Given builds `source_root_project`, so `--full` replaced the kind
          files. The kind Given now rewrites the declaration to
          `structure: ["src/run.sh", "src/link"]` before committing them,
          which is the design constraint's fixture.
        GREEN: the same command -> exit 0;
          1 scenario (1 passed), 6 steps (6 passed).
        No production diff: the kind-aware write and the re-read landed in
          bundle 1; this scenario protects that behavior.
        DISCRIMINATION (not historical RED): with `set_executable` and
          `executable_matches` made no-ops, the command fails at `the trial's
          "src/run.sh" is executable` ("the trial's src/run.sh is not
          executable (mode 644)"). With `EntryKind::Symlink` written as a
          regular file and the re-read relaxed to match, it fails at `the
          trial's "src/link" is a symlink to "run.sh"` ("the trial's src/link
          is not a symlink"). Restored; green again.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; ./run-tests.sh --non-e2e is 980/980 and ./run-tests.sh --e2e is
        111/111. Re-ran the scenario: 1 scenario (1 passed), 6 steps (6
        passed).
        ```
  - [x] COMMIT: `42def2eebb31612ea1052821c98ff50d76ef7198` -- Batch: `copy-kinds`

- [x] Scenario: `Preparation writes no version-control metadata into the trial`
  - Design constraints:
    - Fixture: `behavior: ["givn/specs/demo.feature"]`, `source: ["src/**"]`;
      the revision commits `givn/specs/demo.feature` and `src/kept.rs`. The
      retained role keeps the trial non-empty under `--full`.
    - When: `givn ise replace --full --into <dir>`.
    - Then: exit 0; a recursive walk of the trial finds no entry named `.git`
      and none named `.fslckout`.
    - Contract: the copy set is built from `revision_files`; no SCM command
      creates the target and no metadata is copied.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        First run after binding: exit 0; 1 scenario (1 passed), 5 steps
          (5 passed). Existing behavior protected; no production diff. The
          fixture's `structure: ["src/kept.rs"]` keeps the trial non-empty
          under `--full`, and the Then walks the trial without following
          links.
        DISCRIMINATION (not historical RED): with `write_trial` creating
          `<trial>/.git` after the file writes, the command fails at the same
          Then: "the trial at .../trial holds the version-control entry
          .../trial/.git". Restored; green again.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; ./run-tests.sh --non-e2e is 980/980 and ./run-tests.sh --e2e is
        111/111. Re-ran the scenario: 1 scenario (1 passed), 5 steps (5
        passed).
        ```
  - [x] COMMIT: `42def2eebb31612ea1052821c98ff50d76ef7198` -- Batch: `copy-kinds`

- [x] Scenario: `Preparation reads a Fossil revision and preserves an executable file and a symlink`
  - Design constraints:
    - Fixture: a real Fossil checkout with `fossil settings allow-symlinks on`,
      `givn/config.yaml` with both add-ons on, and a declaration that retains
      `givn/specs/**` with `source: ["src/**"]`. The revision commits retained
      `givn/specs/demo/spec.feature`, kept `src/other.rs`, replaced
      `src/keys/parser.rs`, executable `src/run.sh`, and symlink `src/link` to
      `run.sh`. Requires `fossil` on PATH; a missing executable surfaces the
      backend's named refusal.
    - When: `givn ise replace --path src/keys/parser.rs --into <dir>`.
    - Then: exit 0; the report prints `Backend: fossil`; the trial holds the
      retained and the kept file, omits `src/keys/parser.rs`, `src/run.sh` is
      executable, and `src/link` is a symlink to its target.
    - Contract: `src/fossil.rs` parses the check-in manifest for path, kind,
      and executable bit (D6); one partition and one writer serve both
      backends.
  - [x] Implement the scenario and its tests. Files: `src/fossil.rs`,
        `src/scm.rs`, `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`,
        `tests/steps/clean_room_preparation_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        First run after binding: exit 0; 1 scenario (1 passed), 8 steps
          (8 passed). Existing behavior protected; no production diff. The
          manifest reader landed in bundle 1. The fixture runs
          `fossil settings allow-symlinks on` before the symlink is added and
          commits an executable script and the symlink.
        DISCRIMINATION (not historical RED): with the manifest's `x` flag
          ignored, the command fails at `the trial's executable script is
          executable` ("the trial's src/run.sh is not executable (mode
          644)"); with the `l` flag ignored, it fails at `the trial's symlink
          is a symlink to its target` ("the trial's src/link is not a
          symlink"). Restored; green again.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; ./run-tests.sh --non-e2e is 980/980 and ./run-tests.sh --e2e is
        111/111. Re-ran the scenario: 1 scenario (1 passed), 8 steps (8
        passed).
        ```
  - [x] COMMIT: `42def2eebb31612ea1052821c98ff50d76ef7198` -- Batch: `copy-kinds`

---

### Batch 6: `trial-record`

The `ise-trial/v1` record, the per-path identities, the remaining report lines,
the dry run, the stdout form, the record-write failure, and the prepare
end-to-end scenario.

**Files created:** `src/ise/record.rs` (the record type, the counts, the digest
helpers).
**Files modified:** `src/ise/mod.rs` (record assembly, dry run), `src/cli/ise.rs`
(the full report, the `Would write:` block, `Receipt path`), `src/e2e_boundary/mod.rs`
(`e2e_file_set`), `src/gherkin/parse.rs` or `src/gherkin/content.rs` (the
`@e2e` scenario digest over a feature string), `src/ise/verify.rs` (shell only;
the checks land in `verify`), `tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/clean_room_preparation_e2e_steps.rs`,
`tests/steps/ise_clean_room_support.rs`, and the committed transcript under
`givn/changes/add-ise-clean-room-commands/evidence/visual/`.

**Design constraints (batch-wide):**
- `TrialRecord` fields, order, and YAML names are design.md "Trial record
  (`ise-trial/v1`)": `schema: 1`, `content_scope: ise-trial/v1`, `backend`,
  `revision`, `named_paths`, `replaced_paths`, `counts {retained, kept_source,
  replaced, outside_declaration, not_replaced}`, `retained_digest`,
  `e2e_digest`, `replaced_digest`, `retained_files`, `e2e_files`,
  `e2e_scenarios`, `written_path`, `givn_version`. Serialize with
  `serde_yaml_ng`; read with `deny_unknown_fields`. A schema other than `1`, a
  content scope other than `ise-trial/v1`, an unknown field, or a missing field
  refuses naming the record and the reason. No timestamp.
- Identity: `digest(path, kind, executable, bytes_or_target) = sha256(path NUL
  kind NUL ("1"|"0") NUL (kind == l ? target : bytes))`; `rollup(entries) =
  sha256(join(sorted(path NUL digest NUL "\n")))`; report abbreviation is the
  first 12 lowercase hex characters plus `…`.
- `e2e_file_set`: each path matched by the `e2e` role resolves to its nearest
  `Cargo.toml` at or above it **in the revision tree**; every tracked file
  under each resolved root is in the set; a matched path resolving no manifest
  is its own root. Reuse `Glob`, the declaration normalizer, and
  `nearest_manifest`; read no working tree.
- Scenario digest: parse each `.feature` file matched by the `behavior` role at
  the revision, select the scenarios tagged `@e2e`, and hash
  `sha256(feature_path NUL title + executable_content per scenario in file
  order)` using `gherkin::content::executable_content`. Nested `Rule:`
  scenarios are outside the selection, same as `has_e2e_scenarios` (R-146).
- `e2e_digest` is the rollup over `e2e_files` and `e2e_scenarios` together;
  `retained_digest` over `retained_files`; `replaced_digest` over the removed
  paths' revision entries. No path appears twice.
- Record write uses `create_new`; a write failure removes the trial it created
  and names the receipt path, exit 1. An omitted `--receipt` prints the YAML
  record to stdout after the report and prints no `Receipt path` line.
- The report prints, in order, `Backend`, `Revision`, `Retained`, `Kept source`,
  `Replaced`, `Outside the declaration`, `Not replaced`, `Retained digest`,
  `E2E digest`, `Written path`, then `Receipt path` only when given.
  `--dry-run` stops after the copy-set check and prints the same lines plus
  `Would write:` and the sorted union of the retained and kept-source paths.

**Batch verification:** the single-scenario command below for each scenario;
for the `@e2e` scenario, `./run-tests.sh --e2e` on the delta scope
(`GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs`) as well.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: trial-record'`.

- [x] Scenario: `Preparation collapses a repeated path`
  - Design constraints:
    - Fixture: `source: ["src/**"]`, empty retained roles; a committed revision
      with `src/one.rs` and `src/two.rs`.
    - When: `givn ise replace --path src/one.rs --path src/one.rs --into <dir>
      --receipt <file>`.
    - Then: exit 0; `Replaced: 1`; the record's `named_paths` holds one entry
      and `replaced_paths` holds one entry.
    - Contract: `named_paths` is sorted and deduplicated; `counts.replaced ==
      replaced_paths.len()`.
  - [x] Implement the scenario and its tests. Files: `src/ise/record.rs`,
        `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED replay (not chronological; the implementation preceded the first
          run): the bound not-implemented Given fails with
          "not implemented: a committed revision with \"src/one.rs\" and
          \"src/two.rs\""; 1 scenario (1 failed), 2 steps (1 passed, 1 failed).
        GREEN: the same command -> exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `named_paths` normalizes, sorts, and deduplicates the
          `--path` input; the partition already collapses the repeat, so one
          path is replaced and one is recorded.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; see the batch-wide REFACTOR evidence in the `@e2e` scenario.
        Re-ran the scenario: 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `0eb1c514cf5af97e4e808c84835ea0602369727e` -- Batch: `trial-record`

- [x] Scenario: `Preparation prints the trial record to stdout when no receipt path is given`
  - Design constraints:
    - Fixture: a clean Git project with a valid declaration and one committed
      file.
    - When: `givn ise replace --full --into <dir>` (no `--receipt`).
    - Then: exit 0; stdout carries the report and then a YAML record whose
      `content_scope` is `ise-trial/v1`; no record file exists; stdout holds no
      `Receipt path` line.
    - Contract: D4 — the receipt line is conditional; the record is printed
      after the report on the same stream.
  - [x] Implement the scenario and its tests. Files: `src/ise/record.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED replay (not chronological): the bound not-implemented When fails
          with "not implemented: the developer prepares a trial from \"HEAD\"
          with a full replacement and no receipt";
          1 scenario (1 failed), 2 steps (1 passed, 1 failed).
        GREEN: the same command -> exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `replace` returns the rendered YAML in the report when
          `--receipt` is absent and leaves `receipt` empty, so `render` prints
          no `Receipt path` line and writes no file.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; the batch-wide suite evidence is in the `@e2e` scenario. Re-ran
        the scenario: 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `0eb1c514cf5af97e4e808c84835ea0602369727e` -- Batch: `trial-record`

- [x] Scenario: `A dry run performs every read and writes no trial or record`
  - Design constraints:
    - Fixture: a clean Git project with a declaration that retains one file and
      declares one source root with two tracked files, so `Would write:` has
      three entries.
    - When: `givn ise replace --full --into <dir> --receipt <file> --dry-run`.
    - Then: exit 0; stdout holds the ten fixed lines, the `Receipt path` line,
      and a `Would write:` block whose entries are the sorted union of the
      retained and kept-source relative paths; the trial and the receipt remain
      absent.
    - Contract: the dry run performs every read, the partition, every refusal,
      the counts, and the identities, and stops before `create_dir`.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `src/cli/ise.rs`, `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED replay (not chronological): the bound not-implemented When fails
          with "not implemented: the developer runs a dry run from \"HEAD\"
          with a full replacement and a receipt";
          1 scenario (1 failed), 2 steps (1 passed, 1 failed).
        GREEN: the same command -> exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        RESOLVED CONTRADICTION: this scenario's shared Given (`a Git project
          with ise enabled and a declaration`) builds the default fixture, so
          the block holds its five retained files
          (docs/arc42/02-architecture-constraints.md, docs/glossary.md,
          docs/reference.md, givn/specs/demo.feature, tests/e2e/Cargo.toml),
          not the three entries this task's Fixture note predicts. The step
          asserts that fixture's sorted union; a dedicated three-entry fixture
          would need a new Given phrase, which the feature text does not have.
        IMPLEMENTATION: the dry run performs the blob reads and identities
          before returning; `create_dir` and the record write are skipped.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; the batch-wide suite evidence is in the `@e2e` scenario. Re-ran
        the scenario: 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `0eb1c514cf5af97e4e808c84835ea0602369727e` -- Batch: `trial-record`

- [x] Scenario: `Preparation refuses when the record cannot be written`
  - Design constraints:
    - Fixture: `--receipt` names a path whose parent is a regular file, so
      `create_new` fails after the trial exists.
    - When: `givn ise replace --full --into <dir> --receipt <file-under-regular-file>`.
    - Then: exit 1; stderr names the receipt path; the trial directory the run
      created is removed.
    - Contract: design-review blocker 3 — the refusal is reachable only after
      the write, so the created trial is removed and no record exists.
  - [x] Implement the scenario and its tests. Files: `src/ise/mod.rs`,
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED replay (not chronological): the bound not-implemented Given fails
          with "not implemented: a receipt path whose parent directory is a
          regular file"; 1 scenario (1 failed), 2 steps (1 passed, 1 failed).
        GREEN: the same command -> exit 0;
          1 scenario (1 passed), 5 steps (5 passed).
        IMPLEMENTATION: `TrialRecord::write` opens with `create_new`; `replace`
          removes the trial it created and returns the error that names the
          receipt path.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        No production edit. `cargo fmt` and
        `cargo clippy --locked --workspace --all-targets -- -D warnings` are
        clean; the batch-wide suite evidence is in the `@e2e` scenario. Re-ran
        the scenario: 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `0eb1c514cf5af97e4e808c84835ea0602369727e` -- Batch: `trial-record`

- [x] @e2e Scenario: `Developer prepares a clean-room trial from a recorded revision`
  - Design constraints:
    - Fixture: a temp Git project with `ise` enabled; a declaration that
      retains `givn/specs/**`, declares `tests/e2e`, and replaces `src/**`; a
      committed revision with `givn/specs/demo.feature`,
      `tests/e2e/Cargo.toml`, `tests/e2e/steps/demo.rs`,
      `src/keys/parser.rs`, `src/keys/mod.rs`, `src/other.rs`, `notes.md`.
    - When: `givn ise replace --path src/keys/parser.rs --into <dir> --receipt
      <file>`.
    - Then: exit 0; the ten lines with `Backend: git`, a `Revision:` line
      holding the resolved 40-hex commit, `Retained: 3` (the feature file and
      the two `tests/e2e` files), `Kept source: 2` (`src/keys/mod.rs`,
      `src/other.rs`), `Replaced: 1`, `Outside the declaration: 1`
      (`notes.md`), `Not replaced: 1` (`src/keys/mod.rs`); both digests
      abbreviated to 12 lowercase hex characters and `…`; `Written path` and
      `Receipt path`; the trial holds the retained and kept-source files
      including `src/keys/mod.rs`; it omits `src/keys/parser.rs` and
      `notes.md`; the record carries the revision, `backend: git`,
      `replaced_paths`, the counts, `replaced_digest`, `written_path`, and
      `givn_version`.
    - E2E mechanics: drive the compiled binary as a real subprocess
      (`env!("CARGO_BIN_EXE_givn")`) in a temp project; primary assertions are
      stdout and the exit code; trial and record contents are secondary
      evidence. Commit the transcript at
      `givn/changes/add-ise-clean-room-commands/evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`.
    - Step file: `tests/steps/clean_room_preparation_e2e_steps.rs` (separate
      from the regular steps).
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/clean_room_preparation_e2e_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`, the committed transcript. Run
        the single-scenario command under the e2e scope, then
        `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs
        ./run-tests.sh --e2e`. Must pass.
        ```
        RED replay (not chronological): the bound not-implemented Given fails
          with "not implemented: a Git project with ise enabled and a
          declaration that retains \"givn/specs/**\", declares \"tests/e2e\",
          and replaces \"src/**\""; 1 scenario (1 failed).
        GREEN: the single-scenario command -> exit 0;
          1 scenario (1 passed), 11 steps (11 passed).
        E2E SCOPE: GIVN_FEATURES=.../clean-room-preparation.feature
          ./run-tests.sh --e2e -> 1 test run: 1 passed, 41 skipped.
          ./run-tests.sh --e2e (permanent) -> 111 passed, 980 skipped.
        E2E SCOPE LIMIT: the whole change delta
          (GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs)
          selects both `@e2e` scenarios and fails on the still-unimplemented
          `trial-input-verification` one (1 passed, 1 failed, 57 skipped);
          batch `verify` owns that scenario. The owning-feature scope above is
          this batch's clean e2e evidence.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run both, must still pass.
        ```
        OUTLINE (advisory; src/ise, src/e2e_boundary, src/gherkin): 17 files,
          53 units, 3 hints, all pre-existing method-length/multi-type
          hints in files this batch did not change structurally. The new
          units read by declaration: `TrialRecord` owns the record, its
          `identities`/`scenario_digests` helpers own one identity set each,
          `e2e_file_set` resolves the declared packages over a revision tree,
          and `abbreviate` owns the report's digest rendering.
        CODE LINES: src/ise/mod.rs 437, partition.rs 238, record.rs 132,
          verify.rs 10; all within the 500 limit.
        No production edit in REFACTOR. `cargo fmt` and
          `cargo clippy --locked --workspace --all-targets -- -D warnings` are
          clean; `./run-tests.sh --non-e2e` is 980/980 and
          `./run-tests.sh --e2e` is 111/111. Re-ran the single scenario:
          1 scenario (1 passed), 11 steps (11 passed); the owning-feature e2e
          scope: 1 passed, 41 skipped.
        ```
  - [x] COMMIT: `0eb1c514cf5af97e4e808c84835ea0602369727e` -- Batch: `trial-record`

---

### Batch 7: `verify`

The `ise verify` re-check: the trust anchor, the retained and `e2e` identities
per path, the drift lines, the add-on precedence, and the verify end-to-end
scenario.

**Files created:** none (the `src/ise/verify.rs` shell landed in
`trial-record`).
**Files modified:** `src/ise/verify.rs`, `src/cli/ise.rs` (verify output and
exit mapping), `tests/steps/trial_input_verification_steps.rs`,
`tests/steps/trial_input_verification_e2e_steps.rs`,
`tests/steps/ise_clean_room_support.rs` (the prepared-trial helper and the
record rewriters), and the committed transcript under
`givn/changes/add-ise-clean-room-commands/evidence/visual/`.

**Design constraints (batch-wide):**
- Order: effective manifest (`ise`, `architecture_principles`) → strict record
  read → trial directory exists → trust anchor → retained digests → current
  declaration and tree (`Backend::content_paths`) → current `e2e_file_set` and
  scenario map → compare → print → exit.
- Trust anchor (D9): the canonicalized `written_path` equals canonicalized
  `--at`; `backend` equals the selected backend; the record path lies outside
  the trial; `replaced_digest` is a 64-hex value.
- Drift reasons, exactly as design.md's table: `the path is missing from the
  trial`, `the trial differs from the record`, `the path is missing from the
  checkout's e2e package`, `the checkout's e2e package gained this path`, `the
  checkout differs from the record`, `the selected @e2e scenarios differ in the
  checkout`.
- Output: clean prints `Trial: <abs path>`, `Retained: clean`, `E2E: clean` and
  exits 0. Drift prints one `Retained drift: <path> (<reason>)` or
  `E2E drift: <path> (<reason>)` line per path, then
  `Verification failed: N drifting path(s)`, and exits 1. Every reason is
  collected in one `BTreeMap` and rendered after all checks.
- The add-on check runs before the record read, so an `ise`-off project never
  reads the record (proved by a corrupted record).
- The prepared-trial fixture helper: a temp Git project whose declaration
  retains `givn/specs/**` (behavior) and `docs/**` (contracts), declares
  `tests/e2e`, and sources `src/**`; the revision commits
  `givn/specs/demo/spec.feature` (carrying an `@e2e`-tagged scenario),
  `docs/reference.md`, `tests/e2e/Cargo.toml`, `tests/e2e/steps/keys.rs`,
  kept `src/keys/steps.rs`, and replaced `src/keys/parser.rs`. The helper
  prepares with `--path src/keys/parser.rs --receipt <file>` so the kept-source
  file stays in the trial.

**Batch verification:** the single-scenario command below for each scenario;
for the `@e2e` scenario, `./run-tests.sh --e2e` on the delta scope as well.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: verify'`.

- [x] Scenario: `Verification names a retained file changed in the trial`
  - Design constraints:
    - Fixture: the prepared-trial helper; the verifier appends bytes to
      `<trial>/givn/specs/demo/spec.feature`.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `givn/specs/demo/spec.feature`; the reason is
      `the trial differs from the record`.
    - Contract: the retained check recomputes the per-path digest from the
      trial by kind, executable bit, and bytes.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `src/cli/ise.rs`, `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs cargo test --locked --test features_runner -- --exact "prepare-an-ise-test/trial-input-verification.feature::Verification names a retained file changed in the trial"`
             -> `✘ Then verification fails and names "givn/specs/demo/spec.feature"`; `Step panicked. Captured output: stderr "ise verify: the trial re-check is not implemented yet\n" does not name "givn/specs/demo/spec.feature"`
        GREEN same command -> `1 scenario (1 passed)`, `4 steps (4 passed)`

        Evidence: the observed failure is the assertion on the named path; the fixture prepared a real trial and the binary refused with the unimplemented re-check.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification names an e2e package file changed in the trial`
  - Design constraints:
    - Fixture: the prepared-trial helper; the verifier appends bytes to
      `<trial>/tests/e2e/steps/keys.rs`.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `tests/e2e/steps/keys.rs`; the reason is
      `the trial differs from the record` (the retained check covers e2e files
      inside the trial; Contradictions 5).
    - Contract: e2e files are retained files, so one walk covers both.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification names an e2e package file changed in the trial`
             -> `1 scenario (1 passed)`, `4 steps (4 passed)` (the batch's first increment walks the recorded
             retained set, and every resolved `e2e` file is a retained file)

        Discrimination probe (the scenario passed once the retained check existed, because e2e files are retained files): skipping every recorded retained path that is also in `e2e_files` made it fail with `✘ Then verification fails and names "tests/e2e/steps/keys.rs"` / `assertion left == right failed: stderr=`. Probe reverted; the retained walk is the coverage.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification names an e2e package file changed in the checkout`
  - Design constraints:
    - Fixture: the prepared-trial helper; the developer edits
      `<checkout>/tests/e2e/steps/keys.rs` after preparation.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `tests/e2e/steps/keys.rs`; the reason is
      `the checkout differs from the record`.
    - Contract: the current tree comes from `Backend::content_paths`, so a
      modified tracked file is read from the working tree (D8).
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `src/e2e_boundary/mod.rs`, `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  same command, scenario `Verification names an e2e package file changed in the checkout`
             -> `✘ Then verification fails and names "tests/e2e/steps/keys.rs"`; `assertion left == right failed: stderr=` (no current-tree check existed)
        GREEN same command -> `1 scenario (1 passed)`

        Implementation: `e2e_boundary::e2e_file_set` now takes a path slice, so the current tracked-plus-untracked tree is resolved from `Backend::content_paths` without copying strings.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification names an e2e scenario changed in the checkout`
  - Design constraints:
    - Fixture: the prepared-trial helper; the developer edits the step text of
      the `@e2e`-tagged scenario in `<checkout>/givn/specs/demo/spec.feature`
      after preparation (adding a step, or changing a step's text).
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `givn/specs/demo/spec.feature`; the reason is
      `the selected @e2e scenarios differ in the checkout`.
    - Contract: the per-feature scenario digest is recomputed from the
      checkout's `behavior` files; the frozen set is the `@e2e` selection only.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `src/gherkin/parse.rs` (or `content.rs`), `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  same command, scenario `Verification names an e2e scenario changed in the checkout`
             -> `✘ Then verification fails and names "givn/specs/demo/spec.feature"`; `assertion left == right failed: stderr=` (exit 0)
        GREEN same command -> `1 scenario (1 passed)`

        Setup defect found and fixed: the setup batch's step regex read `"givn/specs/demo\.feature"` and never matched the phrase `"givn/specs/demo/spec.feature"`, so the first run failed with `Step doesn't match any function` (`Step failed` on the When). Corrected the binding to `demo/spec\.feature`; the RED above is the corrected run.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification reports no drift after a kept source file changes`
  - Design constraints:
    - Fixture: the prepared-trial helper; the verifier edits
      `<trial>/src/keys/steps.rs` (kept source, inside `src/**`, outside every
      frozen input).
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 0; stdout prints `Retained: clean` and `E2E: clean`.
    - Contract: only the `E2E` package and the selected `@e2e` scenarios are
      frozen; kept source carries no focused identity.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification reports no drift after a kept source file changes`
             -> `1 scenario (1 passed)`, `4 steps (4 passed)`
        Probe injecting `src/keys/steps.rs` as a frozen retained path with an impossible digest
             -> `✘ Then verification reports the retained and e2e identities clean`; `Captured output: stdout "" does not print the line "Retained: clean"`. Probe reverted.

        Requirement protected: only the recorded retained set and the `e2e` inputs are frozen, so kept source carries no focused identity.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification names a recorded path missing from the trial`
  - Design constraints:
    - Fixture: the prepared-trial helper; the verifier deletes
      `<trial>/givn/specs/demo/spec.feature`.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `givn/specs/demo/spec.feature`; the reason is
      `the path is missing from the trial`.
    - Contract: a missing retained path is drift, not a read error.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification names a recorded path missing from the trial`
             -> `1 scenario (1 passed)`
        PROBE (a missing path is drift, not silence): `Err(IdentityGap::Missing) => continue`
             -> `✘ Then verification fails and names "givn/specs/demo/spec.feature"`; `assertion left == right failed: stderr=`. Probe reverted.

        Requirement protected: an absent recorded retained path is drift with its own reason.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses while the ise addon is off`
  - Design constraints:
    - Fixture: the prepared-trial helper, then the record is corrupted (for
      example truncated to `not: a-trial-record`), then
      `givn/config.yaml` sets `addons.ise: false` after preparation.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `ise`; stderr carries no record error (the
      record was not read).
    - Contract: the add-on check precedes the record read (D10); the corrupted
      record is the proof.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification refuses while the ise addon is off`
             -> `1 scenario (1 passed)`; `verification reports no record error` holds on a corrupted record.
        PROBE (the add-on check precedes the record read): reading the record before `require_addons`
             -> `✘ And verification reports no record error`; `stderr "ise verify: read the trial record '/tmp/.tmpXuGnzT/record.yaml': unknown field `not`, expected one of ...\n" reports a record error`. Probe reverted.

        Requirement protected: D10, the effective manifest decides before the record is read (batch 1 behaviour, now covered by a verify scenario).
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses while architecture principles are off`
  - Design constraints:
    - Fixture: the prepared-trial helper, then the record is corrupted
      (`not: a-trial-record`) and `givn/config.yaml` sets
      `addons.architecture_principles: false` with `ise: true` after
      preparation.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names `architecture_principles` and reports no
      record error.
    - Contract: the same prerequisite pair as prepare; read before the record.
  - [x] Implement the scenario and its tests. Files: `src/cli/ise.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification refuses while architecture principles are off`
             -> `1 scenario (1 passed)`
        PROBE: disabling the `architecture_principles` branch of `require_addons`
             -> `✘ Then verification fails and names "architecture_principles"`; `assertion left == right failed: stderr=`. Probe reverted.

        Requirement protected: the same prerequisite pair as replace, read before the record.
        REVIEW CLOSURE (F6): the record is now corrupted, so only the addon check
        can produce the refusal. DISCRIMINATION (not historical RED): with
        `read_record` moved before `require_addons` in `verify`, the scenario
        fails:
          stderr "ise verify: read the trial record '…': unknown field `not` …"
          does not name "architecture_principles"
        Probe reverted with `git checkout -- src/ise/verify.rs`. COMMIT:
        23ad9e6f -- Batch: `review-closure`.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses a record it cannot read`
  - Design constraints:
    - Fixture (Contradictions 4): the prepared-trial helper, then the record is
      rewritten as readable YAML whose `content_scope` is `other/v1`.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names the record and `ise-trial/v1`.
    - Contract: the strict record loader names the record path and the expected
      scope.
  - [x] Implement the scenario and its tests. Files: `src/ise/record.rs`,
        `src/ise/verify.rs`, `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  same command, scenario `Verification refuses a record it cannot read`
             -> `✘ Then verification fails and names the record`; `assertion left == right failed: stderr=` (the foreign `content_scope` parsed and the run exited 0)
        GREEN same command -> `1 scenario (1 passed)`

        Implementation: `read_record` asserts `schema == 1`, `content_scope == ise-trial/v1`, and a 64-hex `replaced_digest`, each naming the record. Fixture follows Contradictions 4: readable YAML with `content_scope: other/v1`, which produces both the record name and the expected scope.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses a record with an unknown field`
  - Design constraints:
    - Fixture: the prepared-trial helper, then the record gains a top-level
      `extra_field: 1`.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names the record and `extra_field`.
    - Contract: `deny_unknown_fields` on the record; the reason names the field.
  - [x] Implement the scenario and its tests. Files: `src/ise/record.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification refuses a record with an unknown field`
             -> `1 scenario (1 passed)` (the record's `deny_unknown_fields` from `trial-record`)
        PROBE: removing `#[serde(deny_unknown_fields)]` from `TrialRecord`
             -> `✘ Then verification fails and names the record and the unknown field`; `assertion left == right failed: stderr=`. Probe reverted.

        Requirement protected: an unknown top-level key refuses with the field name.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses a record with a missing field`
  - Design constraints:
    - Fixture: the prepared-trial helper, then the record's `counts` key is
      removed.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names the record and `counts`.
    - Contract: a missing required field refuses with the field name; no
      default is invented.
  - [x] Implement the scenario and its tests. Files: `src/ise/record.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification refuses a record with a missing field`
             -> `1 scenario (1 passed)`
        PROBE: `#[serde(default)]` on `counts`
             -> `✘ Then verification fails and names the record and the missing field`; `assertion left == right failed: stderr=`. Probe reverted.

        Requirement protected: a missing required field refuses with its name; no default is invented.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses a record placed inside the trial`
  - Design constraints:
    - Fixture: the prepared-trial helper; the record is copied to
      `<trial>/record.yaml` and verified from there.
    - When: `givn ise verify --trial <trial>/record.yaml --at <trial>`.
    - Then: exit 1; stderr names that record path and reports that the record is
      inside the trial.
    - Contract: the trust anchor's placement check; the builder must not own the
      record (D9).
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `tests/steps/trial_input_verification_steps.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  same command, scenario `Verification refuses a record placed inside the trial`
             -> `✘ Then verification fails and names the record path`; `assertion left == right failed: stderr=` (exit 0)
        GREEN same command -> `1 scenario (1 passed)`

        Implementation: the trust anchor's placement check (D9). The copied record names the same `written_path` and backend, so only the placement check can refuse it.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses a record whose written path is not the trial`
  - Design constraints:
    - Fixture: the prepared-trial helper; the record's `written_path` is
      rewritten to a different directory, and verification runs against the
      real trial.
    - When: `givn ise verify --trial <rewritten record> --at <trial>`.
    - Then: exit 1; stderr names the record and the trial path and reports the
      written-path mismatch.
    - Contract: canonicalized `written_path` must equal canonicalized `--at`.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  same command, scenario `Verification refuses a record whose written path is not the trial`
             -> `✘ Then verification fails and names the record and the trial path`; `assertion left == right failed: stderr=` (exit 0)
        GREEN same command -> `1 scenario (1 passed)`

        Fixture: `written_path` rewritten to an existing sibling directory, so the refusal is the canonicalized comparison, not a resolution failure.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification refuses a record whose backend differs from the selected backend`
  - Design constraints:
    - Fixture: the prepared-trial helper (Git); the record's `backend` is
      rewritten to `fossil`.
    - When: `givn ise verify --trial <rewritten record> --at <trial>`.
    - Then: exit 1; stderr names the record and `fossil` and reports the
      backend mismatch.
    - Contract: the record's backend must equal the selected backend; no
      backend is inferred from the directory contents.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        RED  same command, scenario `Verification refuses a record whose backend differs from the selected backend`
             -> `✘ Then verification fails and names the record and the backend`; `assertion left == right failed: stderr=` (exit 0)
        GREEN same command -> `1 scenario (1 passed)`
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] Scenario: `Verification names every drifting path in one run`
  - Design constraints:
    - Fixture: the prepared-trial helper; the verifier changes
      `<trial>/givn/specs/demo/spec.feature`, `<trial>/docs/reference.md`, and
      `<trial>/tests/e2e/steps/keys.rs`.
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 1; stderr names all three paths; the summary line reports 3
      drifting paths.
    - Contract: one `BTreeMap` of reasons rendered after every check, so one
      run reports all drift.
  - [x] Implement the scenario and its tests. Files: `src/ise/verify.rs`,
        `src/cli/ise.rs`, `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`. Run the single-scenario
        command. Must pass.
        ```
        Run  same command, scenario `Verification names every drifting path in one run`
             -> `1 scenario (1 passed)`; stderr names all three paths and `Verification failed: 3 drifting path(s)`
        PROBE (one `BTreeMap` rendered after every check): `break` after the first retained drift
             -> stderr `Trial: ...\nRetained drift: docs/reference.md (the trial differs from the record)\nVerification failed: 1 drifting path(s)`; `✘ Then verification fails and names all three paths`. Probe reverted.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [8.219s] 57 tests run: 57 passed, 2 skipped` (the two `@e2e` scenarios of the delta).
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

- [x] @e2e Scenario: `Verifier re-checks a prepared trial and receives the frozen-input report`
  - Design constraints:
    - Fixture: a prepared Git trial with a receipt, produced by a real
      `givn ise replace` subprocess against the fixture project (the same
      fixture as the other verify scenarios).
    - When: `givn ise verify --trial <receipt> --at <trial>`.
    - Then: exit 0; stdout prints `Trial: <abs path>`, `Retained: clean`, and
      `E2E: clean`.
    - E2E mechanics: drive the compiled binary as a real subprocess
      (`env!("CARGO_BIN_EXE_givn")`); primary assertions are stdout and the exit
      code. Commit the transcript at
      `givn/changes/add-ise-clean-room-commands/evidence/visual/verifier-re-checks-a-prepared-trial-and-receives-the-frozen-input-report/transcript.txt`.
    - Step file: `tests/steps/trial_input_verification_e2e_steps.rs`.
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/trial_input_verification_e2e_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`, the committed transcript. Run
        the single-scenario command under the e2e scope, then
        `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs
        ./run-tests.sh --e2e`. Must pass.
        ```
        Run  `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs cargo test --locked --test features_runner -- --exact "prepare-an-ise-test/trial-input-verification.feature::Verifier re-checks a prepared trial and receives the frozen-input report"`
             -> `1 scenario (1 passed)`, `4 steps (4 passed)`; the step captured the transcript.
        PROBE (clean versus drift): inverting the retained digest comparison
             -> `✘ Then verification reports the retained and e2e identities clean`; `Captured output: stdout "" does not print the line "Retained: clean"`. Probe reverted.
        E2E scope: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --e2e`
             -> `Summary [0.987s] 2 tests run: 2 passed, 57 skipped`
        Transcript: `givn/changes/add-ise-clean-room-commands/evidence/visual/verifier-re-checks-a-prepared-trial-and-receives-the-frozen-input-report/transcript.txt`
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run both, must still pass.
        ```
        Re-ran the scenario command after the batch refactor (reason constants hoisted, the support import
        normalised, `cargo fmt`): `1 scenario (1 passed)`. The batch-level re-run covers every scenario in this
        batch: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e`
        -> `Summary [9.111s] 62 tests run: 62 passed, 2 skipped` (the two `@e2e` scenarios of the delta, after
        the review-closure batch). The e2e scope: `GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs
        ./run-tests.sh --e2e` -> `Summary [0.995s] 2 tests run: 2 passed, 62 skipped`.
        ```
  - [x] COMMIT: `6f179102a312a48d979451ac6d52e635789494d9` -- Batch: `verify`

---

### Batch 8: `durable-docs`

No scenario task. The durable documentation obligations of design.md "Guidance
Amendments", verified against the implemented behavior and completed where the
text does not exist yet.

**Files modified:** `docs/reference.md` (the two CLI rows and a new
`### Clean-room trial` subsection under `## Verification and recovery`),
plus any entry in `## Durable documentation placement` whose committed text the
implementation contradicts (each correction named in the commit).

**Design constraints (batch-wide):**
- `docs/reference.md` documents delivered behavior: `--at` defaulting to `HEAD`;
  exactly one of `--full` and repeated `--path`; `--into` required and outside
  the checkout; `--receipt` placement (outside the checkout, outside the trial,
  not its ancestor, absent); `--dry-run` output; the ten fixed lines plus
  `Receipt path`; the `Would write:` block; the `ise-trial/v1` record fields and
  the per-path identity rule; the nine refusals in condition order and the exit
  codes `0`/`1`/`2`; the verify clean and drift lines with their reasons; the
  Git `ls-tree` and Fossil `F`-line entry-kind sources, and that Fossil records
  a symlink only with `allow-symlinks on`.
- The already-committed statements in `## Durable documentation placement` are
  re-read against the code: a statement that reality contradicts is corrected
  here and the correction is named in the commit body. No speculative behavior
  is documented; `docs/domain-model.md` and `assets/instructions/ise.md` stay
  unchanged and the reason is recorded.
- No Gherkin and no key-sequence narration in `docs/reference.md`; mappings go
  into tables (project communication policy).

**Batch verification:** `./run-tests.sh --non-e2e` (the suite checks the docs
gates) plus a manual re-read of every placement row against the code.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: durable-docs'`.

- [x] Write the `docs/reference.md` command reference and verify every
      placement-table statement against the implemented behavior.
      ```
      Added: two `## CLI commands` rows (`givn ise replace`, `givn ise verify`) and a `### Clean-room trial`
      subsection under `## Verification and recovery`: the two invocations, the replacement form (`--at`
      defaulting to `HEAD`, exactly one of `--full` and repeated `--path`, exit 2 on a usage error), trial and
      receipt placement, the ten report lines plus the conditional `Receipt path`, the dry-run `Would write:`
      block, the nine refusals in condition order, the `ise-trial/v1` field table, the per-path identity rule
      and rollup abbreviation, the Git `ls-tree` and Fossil `F`-line entry-kind sources (a Fossil symlink only
      with `allow-symlinks on`), the verify clean and drift lines with the six reasons, the trust anchor, and
      the `0`/`1`/`2` exit codes. Every statement was read back from the implemented command, not copied from
      design.md.

      Rows of `## Durable documentation placement` re-read against the code; none corrected:
      - `docs/glossary.md:73-76` (Trial directory, Trial record, Trial E2E digest, the scoped `E2E identity`
        anti-term): the record fields, the printed `E2E digest` line, and the alias-free identity all match.
      - `docs/arc42/02-architecture-constraints.md:54-55` (TC-46, TC-47): the preflight order, both readers,
        the drift check, and the add-on pair match `src/ise/{mod,partition,record,verify}.rs`.
      - `docs/arc42/03-context-and-scope.md:32,62-63`: both CLI rows and the independent-verifier entity match
        the clap surface and the re-check.
      - `docs/arc42/04-solution-strategy.md:24`: the clean-room trial row matches the revision read and the
        plain-directory write.
      - `docs/arc42/05-building-block-view.md:62-63,90`: the `cli::ise` and `ise` rows match the module split;
        the `e2e_file_set` row's "supplied path tree" matches the path-slice signature the verify batch reads
        from `Backend::content_paths`.
      - `docs/arc42/06-runtime-view.md:982` (6.21): the replace steps 1-10 and the verify steps 1-7 match the
        implemented order, including the placement-before-backend trust anchor.
      - `docs/arc42/08-crosscutting-concepts.md:1464` (8.29): the record table, the identity rule, the
        two-target placement rule, the trust anchor, and the revision-resolved e2e set match.
      - `docs/arc42/11-risks-and-technical-debt.md:129-133` (R-142..R-146), with R-143, R-144, and R-146
        implemented in this batch's sibling batches, match the recorded-set limit, the target containment
        check, and the `Rule:`-nested selection.
      - `docs/arc42/adr/ADR-076-clean-room-trial-record.md` and `docs/arc42/adr/README.md:63,80`: status
        `accepted`, the register row present, and the qualification block unchanged.
      - `docs/arc42/README.md`: both commands are named in the current-state list.

      `docs/domain-model.md` and `assets/instructions/ise.md` stay unchanged: the model scopes itself to
      domain units and excludes command parsing and file formats, and the five served `ise` obligations
      already name the Specification boundary and the frozen package. Both are unmodified in this commit.

      Batch verification: `./run-tests.sh --non-e2e` -> `Summary [168.045s] 980 tests run: 980 passed,
      111 skipped`; `./run-tests.sh --e2e` -> `Summary [39.669s] 111 tests run: 111 passed, 980 skipped`;
      `cargo nextest run` -> `Summary [191.072s] 1644 tests run: 1644 passed, 0 skipped`.
      ```
- [x] COMMIT: `b9e577fa0d97be734ff9494d0a05288fa3bdb0f9` -- Batch: `durable-docs`

---

### Batch 9: `ADR-076`

The delta's one `@givn.modified` unit, written by `givn/changes/add-ise-clean-room-commands/specs/fragments/adr-qualification-gate.feature`
and merged into the permanent `givn/specs/fragments/adr-qualification-gate.feature`.
It moves the register assertion with ADR-076: the register holds 28 accepted
decisions and no proposed decision.

**Design constraints (batch-wide):**
- Artifact: the fragment scenario replaces the permanent 27-accepted assertion
  with 28 accepted; the capability `adr-qualification-gate` and every other
  assertion stay as they were.
- Authority: `docs/arc42/adr/README.md` is the register; accepting ADR-076 adds
  one row, so the count moves with it.
- Route: `EXTEND adr-qualification-gate` in the proposal's Capability Routing
  table; the delta lives under `specs/fragments/`.
- Step binding: the permanent `tests/steps/adr_qualification_gate_steps.rs`
  binds `the final register has 28 accepted decisions and no proposed
  decisions`; the delta step copy binds the same phrase.

- [x] Scenario: `The migration preserves independent architecture history across three proposed decisions`
  (`@givn.modified`, capability `adr-qualification-gate`)
  - Design constraints:
    - Fixture: the permanent ADR register and the canonical sources, as the
      scenario's Given already establishes.
    - When: a maintainer compares the register with the canonical sources.
    - Then: 28 accepted decisions, 0 proposed; the reserved, absorbed,
      superseded, and permanent-specification dispositions unchanged.
    - Contract: one authoritative register; the fragment asserts its count.
  - [x] Implement the modified scenario and its evidence. Files:
        `givn/changes/add-ise-clean-room-commands/specs/fragments/adr-qualification-gate.feature`,
        `givn/specs/fragments/adr-qualification-gate.feature`,
        `docs/arc42/adr/ADR-076-clean-room-trial-record.md`,
        `docs/arc42/adr/README.md`.
        ```
        GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e
        → the modified scenario runs through the permanent binding:
          62 passed, 2 skipped (the two @e2e scenarios)
        ./run-tests.sh --non-e2e (permanent corpus) → exit 0;
          980 passed, 111 skipped; the permanent scenario asserts 28 accepted, 0 proposed
        ```
  - [x] REFACTOR: none; a count correction has no behaviour to restructure.
        ```
        ./run-tests.sh --non-e2e stayed at 980 passed, 111 skipped.
        ```
  - [x] COMMIT: `d5fa08ec20a934921bad9df8bdffb79e05750f37` -- Batch: `ADR-076`

---

### Batch 10: `review-closure`

The review's five coverage and discrimination findings (F3, F4, F7) plus the
unit test for F5 and the strengthened fixtures for F6. No production behaviour
changes: every branch already exists; the batch adds the regression evidence
that distinguishes it.

**Design constraints (batch-wide):**
- Step files: `tests/steps/clean_room_preparation_steps.rs`,
  `tests/steps/trial_input_verification_steps.rs`; fixtures in
  `tests/steps/ise_clean_room_support.rs`.
- Fixtures: the verify fixtures gain one untracked file per scenario through
  `add_checkout_file`; `git ls-files --others` lists it, so the re-check sees
  the checkout's current tree.
- Probe rule: each new assertion is proven by disabling its production branch,
  observing the failure, and restoring the tree. The branch exists, so the
  scenario passes on its first run; the probe, not a manufactured production
  edit, is the discrimination evidence.

- [x] Scenario: `A full replacement counts a retained file under a source root as not replaced`
  - Design constraints:
    - Fixture: `structure: ["src/keep.rs"]`, `source: ["src/**"]`; the revision
      commits `src/keep.rs` (retained) and `src/other.rs`.
    - When: `givn ise replace --at HEAD --full --into <trial>`.
    - Then: the report prints `Replaced: 1` and `Not replaced: 1`; the trial
      holds `src/keep.rs`.
    - Contract: retained precedence under `--full`; `Not replaced` counts the
      direct non-replaced sibling.
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/clean_room_preparation_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`.
        ```
        GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::A full replacement counts a retained file under a source root as not replaced"
        → 1 scenario (1 passed), 4 steps (4 passed)
        DISCRIMINATION: the retained-role test precedes the source-root test; with
        the retained check inverted the scenario reports Replaced: 2 and fails the
        Then. Probe reverted.
        ```
  - [x] REFACTOR: none; the scenario reuses the shipped steps and one new Given.
  - [x] COMMIT: `23ad9e6fd75f30110508aef7f15aabda59016fd0` -- Batch: `review-closure`

- [x] Scenario: `Verification names an e2e package path the record does not list`
  - Design constraints:
    - Fixture: the prepared verify trial; the developer writes
      `tests/e2e/steps/extra.rs` into the checkout untracked, so
      `Backend::content_paths` lists it and `e2e_file_set` resolves it under
      `tests/e2e`.
    - When: `givn ise verify --trial <record> --at <trial>`.
    - Then: exit 1; stderr prints `E2E drift: tests/e2e/steps/extra.rs (the
      checkout's e2e package gained this path)`.
    - Contract: `CHECKOUT_GAINED_PATH` in `src/ise/verify.rs`.
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`.
        ```
        GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/trial-input-verification.feature::Verification names an e2e package path the record does not list"
        → 1 scenario (1 passed), 4 steps (4 passed)
        DISCRIMINATION (not historical RED): with the extra-path insert in
        `check_checkout` disabled, the same command fails with
          assertion `left == right` failed: stderr= left: Some(0) / right: Some(1)
        Probe reverted with `git checkout -- src/ise/verify.rs`.
        ```
  - [x] REFACTOR: none; the scenario reuses the prepared-trial fixture.
  - [x] COMMIT: `23ad9e6fd75f30110508aef7f15aabda59016fd0` -- Batch: `review-closure`

- [x] Scenario: `Verification names an e2e scenario the record does not list`
  - Design constraints:
    - Fixture: the prepared verify trial; the developer writes
      `givn/specs/demo/extra.feature` with one `@e2e` scenario into the
      checkout untracked.
    - When: `givn ise verify --trial <record> --at <trial>`.
    - Then: exit 1; stderr prints `E2E drift: givn/specs/demo/extra.feature
      (the selected @e2e scenarios differ in the checkout)`.
    - Contract: the extra-feature branch of `check_scenarios` in
      `src/ise/verify.rs`.
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/trial_input_verification_steps.rs`,
        `tests/steps/ise_clean_room_support.rs`.
        ```
        GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/trial-input-verification.feature::Verification names an e2e scenario the record does not list"
        → 1 scenario (1 passed), 4 steps (4 passed)
        DISCRIMINATION (not historical RED): with the extra-feature insert in
        `check_scenarios` disabled, the same command fails with
          assertion `left == right` failed: stderr= left: Some(0) / right: Some(1)
        Probe reverted with `git checkout -- src/ise/verify.rs`.
        ```
  - [x] REFACTOR: none; one new Given and one new When, no duplicated fixture.
  - [x] COMMIT: `23ad9e6fd75f30110508aef7f15aabda59016fd0` -- Batch: `review-closure`

- [x] Scenario: `Verification refuses a record with another schema`
  - Design constraints:
    - Fixture: the prepared receipt rewritten with `schema: 2`, every other
      field intact.
    - When: `givn ise verify --trial <record> --at <trial>`.
    - Then: exit 1; stderr names the record, `schema`, and
      `a trial record is schema 1`.
    - Contract: `read_record` asserts `schema == 1` before any identity check.
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/trial_input_verification_steps.rs`.
        ```
        GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/trial-input-verification.feature::Verification refuses a record with another schema"
        → 1 scenario (1 passed), 3 steps (3 passed)
        DISCRIMINATION (not historical RED): with the schema bail narrowed, the
        same command fails with
          assertion `left == right` failed: stderr= left: Some(0) / right: Some(1)
        Probe reverted with `git checkout -- src/ise/verify.rs`.
        ```
  - [x] REFACTOR: none; the record rewrite reuses `rewrite_record`.
  - [x] COMMIT: `23ad9e6fd75f30110508aef7f15aabda59016fd0` -- Batch: `review-closure`

- [x] Scenario: `Verification refuses a record without a replaced digest`
  - Design constraints:
    - Fixture: the prepared receipt rewritten with
      `replaced_digest: abc123`, six characters and not a SHA-256 value.
    - When: `givn ise verify --trial <record> --at <trial>`.
    - Then: exit 1; stderr names the record, `replaced_digest`, and
      `the record is no trial evidence`.
    - Contract: `is_sha256_hex` rejects any value shorter than 64 hex
      characters.
  - [x] Implement the scenario and its tests. Files:
        `tests/steps/trial_input_verification_steps.rs`.
        ```
        GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/trial-input-verification.feature::Verification refuses a record without a replaced digest"
        → 1 scenario (1 passed), 3 steps (3 passed)
        DISCRIMINATION (not historical RED): with the digest bail narrowed, the
        same command fails with
          assertion `left == right` failed: stderr= left: Some(0) / right: Some(1)
        Probe reverted with `git checkout -- src/ise/verify.rs`.
        ```
  - [x] REFACTOR: none; the fixture differs from the schema case in one field.
  - [x] COMMIT: `23ad9e6fd75f30110508aef7f15aabda59016fd0` -- Batch: `review-closure`

- [x] Unit test: `a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link` (F5)
  - Design constraints:
    - Fixture: a `RevisionEntry` with `kind: Submodule` at `vendor/lib`, plus a
      regular entry in the companion test.
    - When: `require_copy_set(&paths, &entries)` is called directly.
    - Then: the submodule entry refuses with `'vendor/lib' is a repository
      link; ...`; the regular entry passes.
    - Contract: refusal 9's submodule branch, which no end-to-end fixture can
      isolate because the blob read fails first.
  - [x] Implement the unit test. Files: `src/ise/mod.rs`.
        ```
        cargo test --locked --lib ise::tests → 4 passed
        DISCRIMINATION: with the submodule branch's `EntryKind::Submodule`
          comparison replaced by `EntryKind::Symlink`, the unit test fails:
          "a repository link is refused: ()". Probe reverted with
          `git checkout -- src/ise/mod.rs`; the test was then re-added verbatim.
        ```
  - [x] REFACTOR: none.
  - [x] COMMIT: `23ad9e6fd75f30110508aef7f15aabda59016fd0` -- Batch: `review-closure`

---

## Closeout

- [x] Every delta phrase is bound in a permanent per-capability step file and no
      delta step copy exists under this change's `specs/`:
      `find givn/changes/add-ise-clean-room-commands/specs -name '*steps*'`
      prints nothing, and every binding lives in
      `tests/steps/clean_room_preparation_steps.rs`,
      `tests/steps/clean_room_preparation_e2e_steps.rs`,
      `tests/steps/trial_input_verification_steps.rs`,
      `tests/steps/trial_input_verification_e2e_steps.rs`, or the
      fixture-only `tests/steps/ise_clean_room_support.rs`.
      ```
      $ find givn/changes/add-ise-clean-room-commands/specs -name '*steps*'
      (no output)
      $ for f in tests/steps/clean_room_preparation_steps.rs \
          tests/steps/clean_room_preparation_e2e_steps.rs \
          tests/steps/trial_input_verification_steps.rs \
          tests/steps/trial_input_verification_e2e_steps.rs \
          tests/steps/ise_clean_room_support.rs; do
          printf '%s ' "$f"; grep -cE '^#\[(given|when|then)' "$f"; done
      tests/steps/clean_room_preparation_steps.rs 121
      tests/steps/clean_room_preparation_e2e_steps.rs 10
      tests/steps/trial_input_verification_steps.rs 41
      tests/steps/trial_input_verification_e2e_steps.rs 2
      tests/steps/ise_clean_room_support.rs 0
      ```
- [x] No empty, no-op, or stubbed step body remains:
      `grep -rn 'not implemented:' tests/steps/` prints nothing, and
      `grep -rn 'unimplemented!\|todo!()' tests/steps/` prints no step body.
      ```
      $ grep -rn 'not implemented:' tests/steps/
      (no output)
      $ grep -rn 'unimplemented!\|todo!()' tests/steps/
      tests/steps/outline_report_support.rs:581: "... todo!() ..."
      The one match is a fixture source string the outline-report scenario
      writes and parses; it is not a step body and not this change's file.
      ```
- [x] No delta scenario still carries `@wip`, and no `@e2e` tag was removed:
      `grep -rn '@wip' givn/changes/add-ise-clean-room-commands/specs/` prints
      nothing and the two `@e2e` tags remain on
      `Developer prepares a clean-room trial from a recorded revision` and
      `Verifier re-checks a prepared trial and receives the frozen-input report`.
      ```
      $ grep -rn '@wip' givn/changes/add-ise-clean-room-commands/specs/
      (no output)
      $ grep -rn '@e2e' givn/changes/add-ise-clean-room-commands/specs/
      clean-room-preparation.feature:5:  @givn.added @e2e
      trial-input-verification.feature:5:  @givn.added @e2e
      usecase.md:65,105,107 mention the tag in rules and examples
      ```
- [x] Both `@e2e` scenarios committed their transcript under
      `givn/changes/add-ise-clean-room-commands/evidence/visual/<scenario-slug>/transcript.txt`.
      ```
      givn/changes/add-ise-clean-room-commands/evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt
        Scenario: Developer prepares a clean-room trial from a recorded revision
      givn/changes/add-ise-clean-room-commands/evidence/visual/verifier-re-checks-a-prepared-trial-and-receives-the-frozen-input-report/transcript.txt
        Scenario: Verifier re-checks a prepared trial and receives the frozen-input report
      ```
- [x] `givn lint --change add-ise-clean-room-commands` reports no finding and
      exits 0: the `@wip` findings are gone with the batches, the delta carries
      two `@e2e` scenarios and this file names them, so
      `has_e2e_intent_in_tasks` is satisfied. `[SHAPE]`/`[SUBST]` advisories are
      advisory and every pair is dispositioned in review.
      ```
      $ givn lint --change add-ise-clean-room-commands → exit 0
      3 file(s) checked — clean
      22 [SHAPE] advisories and 118 [SUBST] advisories, 0 [LONG]; the advisories
      are advisory and the review dispositions every delta pair.
      ```
- [ ] Whole-change verification, in this order, each exiting 0 with zero
      failures and zero in-scope skips:
      ```
      cargo fmt --all -- --check
      cargo clippy --locked --workspace --all-targets -- -D warnings
      cargo nextest run
      ./run-tests.sh --non-e2e
      ./run-tests.sh --e2e
      GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e
      GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --e2e
      givn lint --change add-ise-clean-room-commands
      ```
      ```
      cargo fmt --all -- --check → exit 0
      cargo clippy --locked --workspace --all-targets -- -D warnings → exit 0
      cargo nextest run → exit 0; Summary 1646 tests run: 1646 passed, 0 skipped
      ./run-tests.sh --non-e2e → exit 0; Summary 980 tests run: 980 passed, 111 skipped
      ./run-tests.sh --e2e → exit 0; Summary 111 tests run: 111 passed, 980 skipped
      GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --non-e2e
        → exit 0; Summary 62 tests run: 62 passed, 2 skipped
      GIVN_FEATURES=givn/changes/add-ise-clean-room-commands/specs ./run-tests.sh --e2e
        → exit 0; Summary 2 tests run: 2 passed, 62 skipped
      givn lint --change add-ise-clean-room-commands → exit 0; 3 file(s) checked — clean
      The delta holds 64 scenarios, 2 of them @e2e.
      ```
- [x] `givn check tasks --change add-ise-clean-room-commands` exits 0 and
      records the change verification receipt. This change's own
      `config-lock.yaml` has `ise: false`, so the receipt is ise-off shaped (no
      `e2e_identity`, no `e2e_scope`); the delta e2e scope is proven by the
      explicit `--e2e` run above.
      ```
      $ git status --short
      ?? givn/changes/add-ise-clean-room-commands/verification/
      $ givn check tasks --change add-ise-clean-room-commands → exit 0
      [Summary]
      3 features
      62 scenarios (62 passed)
      296 steps (296 passed)
        ✓ change verification receipt: 62 scenario(s), 0 failed, 0 skipped at 14973c13b75fab8322045a11fe98dcad57df28f8
        ✓ verify passed
      ```
- [ ] `givn check review --change add-ise-clean-room-commands` passes the
      scenario-commit, integrity, routing, and add-on lock checks. The review
      verdict is the reviewer's, not the author's.
      ```
      <!-- paste the review check output; the reviewer records REVIEW: PASS in review.md -->
      ```

**Done when:** every box is checked with its batch `COMMIT:` revision, all 58
delta scenarios pass in the non-`@e2e` and `@e2e` scopes, the two `@e2e`
transcripts are committed, `givn check tasks` exits 0, and the contradictions
in `## Contradictions` are reported rather than silently repaired.
