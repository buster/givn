# Tasks: keep-frozen-verifier-out-of-the-trial

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`), both already set in `givn/commands.yaml`. Strict mode:
`.fail_on_skipped()` on both cucumber builders in `tests/features_runner.rs`; an
undefined step is a failed `NotFound` step. Empty-body ban: no step body is `{}`,
a bare `pass`, a bare `return`, or whitespace. This change installs no runner and
changes no command, so strictness is proven once in `## Setup`.

Single-scenario run command (`<capability>/<feature>` is the path under the
change's `specs/`, design.md "Step Definitions"):

```sh
GIVN_FEATURES=givn/changes/keep-frozen-verifier-out-of-the-trial/specs \
  cargo test --locked --test features_runner -- \
  "<capability>/<feature>.feature::<scenario title>" --exact --nocapture
```

Release note (every batch commit subject, verbatim from `proposal.md`):

```text
givn ise replace keeps the frozen End-to-end (E2E) test package out of the clean-room trial while still recording its identity, and givn ise verify refuses a trial that holds a path the frozen package covers.
```

Batch commit rule: one atomic
`givn commit -m "<release note>"$'\n\n'"Batch: <batch-name>"$'\n'"Scenarios: <titles>" -- <paths>`
per batch with explicit paths. Every scenario of the batch records the same
revision as `COMMIT: <hash> -- Batch: <batch-name>`; the gate reads the first
`COMMIT:` line under the scenario line, so a scenario section carries exactly one.
`givn check review` passes a scenario commit only when the commit message
contains the batch name.

**Evidence slots.** Each scenario carries the RED slot (the observed failing
assertion for this batch) and the GREEN slot (the passing run). New phrases are
bound with their assertions in the same edit as the fixture, so the RED is a
failed step body naming the missed value, never an unbound step. No stub survives
its batch.

**`@wip`.** Each batch removes `@wip` from the scenarios it starts, never in bulk.
`givn check tasks` refuses the change while any selected delta scenario still
carries `@wip`.

**Permanent-corpus reconciliation.** Three delta scenarios are `@givn.modified`
over scenarios that already exist in `givn/specs/prepare-an-ise-test/` and
`givn/specs/fragments/`: `Developer prepares a clean-room trial from a recorded
revision`, `A dry run performs every read and writes no trial or record`,
`Verification names an e2e package file changed in the trial`, `Verification
names every drifting path in one run`, and `The migration preserves independent
architecture history across three proposed decisions`. Each batch moves the
permanent body, its bindings, and the delta body to the same text, so the
permanent suite and the delta suite both pass before archive; archive then
replaces the body with an identical assertion.

**Base digest recompute rule (mandatory from design.md).** The delta
`specs/prepare-an-ise-test/usecase.md` line 1 records
`givn:base-sha256:7953c190a04eed4909012c16d586293f8872ff778c779ee0e508e1c26cad51cb`,
the digest of `givn/specs/prepare-an-ise-test/usecase.md` at authoring time.
`add-delivery-identity-to-trial-record` is in review on this branch and carries
its own delta of the same document with the same base digest; whichever change
archives second must recompute this digest against the merged permanent document:

```sh
sha256sum givn/specs/prepare-an-ise-test/usecase.md
head -1 givn/changes/keep-frozen-verifier-out-of-the-trial/specs/prepare-an-ise-test/usecase.md
```

## Batch map

| Batch | Scenario entries | Executed rows | Owning capability |
|---|---|---|---|
| `setup` | 0 | 0 | none |
| `frozen-package-exclusion` | 4 | 4 | `clean-room-preparation` |
| `frozen-trial-refusal` | 5 | 5 | `trial-input-verification` |
| `pending-register-reconciliation` | 1 | 1 | `adr-qualification-gate` |
| `prepare-trial-frozen-package-e2e` | 1 (`@e2e`) | 1 | `clean-room-preparation` |
| **Total** | **11** | **11** | 5 `clean-room-preparation` + 5 `trial-input-verification` + 1 `adr-qualification-gate` |

The delta holds eleven scenario declarations, each with one raw scenario (no
Outline, no `Examples`). Every title appears in exactly one batch by exact title
below.

## Interaction coverage pre-flight

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/prepare-an-ise-test/usecase.md` | 2 (`clean-room-preparation`, `trial-input-verification`) | 2 | yes: the permanent prepare `@e2e` scenario (modified here) and the permanent verify `@e2e` scenario (unchanged) |

No gap: both consumer actions have their matrix row and their `@e2e` scenario.
Every new prepare scenario is a variant of the prepare action and every new
verify scenario is a variant of the re-check action, so neither adds an
inventory entry, an interaction, or an `@e2e` scenario; the change modifies the
content of the prepare `@e2e` scenario and adds no second action. No `@e2e` tag
is added or removed.

## Domain constraints carried by every batch

| Capability | Use-case rule or guarantee | Persona impact |
|---|---|---|
| `clean-room-preparation` | Rule: the frozen package is never copied. The frozen set is every tracked file of the `e2e` role's resolved package at the selected revision, the declaration excepted; the trial holds no path it covers, and a frozen path is neither retained, kept source, replaced, nor outside the declaration. | `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` |
| `clean-room-preparation` | Rule: the declaration is never frozen. A frozen package root that covers `givn/specification.yaml` leaves the declaration in the retained set. | `agent--technical-role` |
| `clean-room-preparation` | Rule: `Retained` equals the number of the record's `retained_files` entries and counts no frozen path; the report prints `Frozen package` only when the frozen set is non-empty, counting the frozen set and not `e2e_files`. | `developer--primary-user` |
| `clean-room-preparation` | Rule: a record written before this rule carries no `excluded_roles` and names no excluded role; `ise-trial/v1` and `schema: 1` stay unchanged. | `givn-maintainer--owner` |
| `trial-input-verification` | Main flow 6 / Extension 6: the re-check refuses before comparing any identity a trial that holds a frozen path, on a record whose `excluded_roles` names `e2e`, except `givn/specification.yaml` and every path the record's `retained_files` lists; the refusal names at most twenty sorted paths and then the remainder count. | `developer--primary-user`, `givn-maintainer--owner` |
| `trial-input-verification` | Rule: a frozen path is one the record's `e2e_files` lists or one under a root the current checkout's declared `e2e` role resolves; the `e2e` identity is compared against the checkout, never the trial. | `agent--technical-role` |

Carried by every batch: one record scope (`ise-trial/v1`, `schema: 1`); one
frozen-set source (`crate::e2e_boundary::e2e_file_set`) so the exclusion and the
recorded identity cannot disagree; the one containment contract
(`E2eBoundary::contains`, resolved root or below); exit codes unchanged
(`0` success, `1` refusal or drift, `2` usage); the report keeps its fixed
lines and gains no line when nothing is frozen.

## Setup

- [x] Read `specs/prepare-an-ise-test/usecase.md` (preconditions, main flow 1-6,
      extensions 3/5/6, rules, success guarantee, Personas, Interactions),
      `design.md` (Overview, Use-Case Traceability, Architecture Impact,
      Decisions D1-D6, Data Model Changes, Partition and Replace Flow, Verify
      Flow, Refusals, External Boundary Cases, Step Definitions, Strict Mode),
      and `design-review.md`. Domain constraints are in
      `## Domain constraints` above.
      ```
      read: usecase.md Preconditions, Main flow 1-6, Extensions 3/5/6,
        Rules (frozen package, declaration never frozen, five placement
        counts, record excluded_roles, trial-side refusal, bounded list),
        Success guarantee, Personas, Interactions;
        design.md all sections; design-review.md Scope..Deferrals.
      base digest verified:
      $ sha256sum givn/specs/prepare-an-ise-test/usecase.md
      7953c190a04eed4909012c16d586293f8872ff778c779ee0e508e1c26cad51cb  givn/specs/prepare-an-ise-test/usecase.md
      $ head -1 givn/changes/keep-frozen-verifier-out-of-the-trial/specs/prepare-an-ise-test/usecase.md
      <!-- givn:base-sha256:7953c190a04eed4909012c16d586293f8872ff778c779ee0e508e1c26cad51cb -->
      ```

- [x] Confirm the runner and both commands; change neither. `cucumber-rs 0.23.0`
      from `Cargo.lock`, `.fail_on_skipped()` on both builders in
      `tests/features_runner.rs`, step files registered in `tests/steps/mod.rs`.
      ```
      verify.command:     ./run-tests.sh --non-e2e
      verify.e2e_command: ./run-tests.sh --e2e
      $ grep -n 'fail_on_skipped' tests/features_runner.rs
      56:        .fail_on_skipped()
      107:        .fail_on_skipped()
      ```

- [x] Proof-of-strictness. Run one selected delta scenario before its phrase is
      bound; confirm a failed `NotFound` step and a non-zero exit. The probe is
      the delta tree itself, so there is no scratch feature to delete.
      ```
      $ GIVN_FEATURES=givn/changes/keep-frozen-verifier-out-of-the-trial/specs \
          cargo test --locked --test features_runner -- \
          "prepare-an-ise-test/clean-room-preparation.feature::The trial record names the frozen role its trial omits" \
          --exact --nocapture
       ✔  Given a Git project with ise enabled and a declaration that retains "givn/specs/**", declares "tests/e2e", and replaces "src/**"
       ✘  And a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", and "src/kept.rs"
          Step failed:
          Defined: .../clean-room-preparation.feature:24:5
          Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      2 steps (1 passed, 1 failed)
      error: test failed, to rerun pass `--test features_runner`; exit 101
      ```
      After the probe the permanent suite remains the passing baseline.

- [x] Fixture inventory (design.md "Step Definitions"). New helpers in
      `tests/steps/ise_clean_room_support.rs`: `e2e_declaration(entry, source)`
      (only the `e2e` role populated, so the frozen set is the scenario's own
      package), a twenty-five-file `tests/e2e` package whose manifest resolves
      the directory to one root, and a record rewrite that drops
      `excluded_roles` after moving `e2e_files` into `retained_files`. Reused:
      `verify_trial_project`, `commit_verify_trial_revision`, `commit_files`,
      `record_mapping`, `write_record_mapping`, `append_bytes`, `ise_project`,
      `ise_project_with_declaration`, `source_root_project`.

## Batch: frozen-package-exclusion

Files: `src/ise/partition.rs` (frozen set, four filters, accessor, unit test),
`src/ise/mod.rs` (`Report::excluded_roles`/`frozen_files`, record field, copy-set
union), `src/ise/record.rs` (`excluded_roles`), `src/cli/ise.rs` (conditional
frozen line), `tests/steps/clean_room_preparation_steps.rs` (bindings, dry-run
expectations, generic single-path trial assertions),
`tests/steps/ise_clean_room_support.rs` (`e2e_declaration`),
`givn/specs/prepare-an-ise-test/clean-room-preparation.feature` (dry-run body).

- [x] Scenario: The trial record names the frozen role its trial omits
  - Design constraints: `partition()` computes
    `frozen = e2e_file_set(declaration, revision_paths) - {DECLARATION_PATH}`
    before the retained set and filters it out of retained, candidates, kept
    source, and outside. `TrialRecord::excluded_roles` is `Vec<String>` with
    `#[serde(default)]`, built as `[Role::E2e.as_str()]` when the frozen set is
    non-empty and `[]` otherwise; `SCHEMA` stays `1` and the content scope stays
    `ise-trial/v1`. `retained_files` is built from `retained_paths()`, which
    holds no frozen path, so `Retained` equals the map length. The fixture is
    `e2e_trial_project()` with the declaration committed first and a revision of
    `givn/specs/demo.feature`, `tests/e2e/Cargo.toml`, `tests/e2e/steps/demo.rs`,
    `src/kept.rs`; the resolved package is the two `tests/e2e` files.
  - RED: first run of the bound scenario against the pre-change writer:
    `And the receipt's excluded roles hold "e2e"` failed with
    `the record has no 'excluded_roles' list`.
  - GREEN: 1 scenario / 7 steps passed; `excluded_roles: [e2e]`,
    `retained_files` holds no `tests/e2e` path, and `e2e_files` holds the two
    resolved package paths with 64-hex identities.
  - [x] COMMIT: 6df78e6dc1 -- Batch: frozen-package-exclusion

- [x] Scenario: A declaration with no frozen package excludes no role
  - Design constraints: `e2e_file_set` returns an empty set for a declaration
    whose `e2e` role is empty, so the frozen set is empty,
    `Report::excluded_roles` is empty, `render` prints no `Frozen package` line,
    the record's `excluded_roles` is an empty list, and the trial holds every
    declared retained path. The fixture is `source_root_project()` (one source
    root `src/**`, every retained role empty) and a full replacement with a
    receipt; the fixed report lines are unchanged.
  - PROBE: the scenario passed against the change because the line is
    conditional, so a discrimination probe printed it unconditionally
    (`if true || !excluded_roles.is_empty()`): the report read
    `Frozen package: , not copied (0 files)` and
    `And the report holds no "Frozen package" line` failed. The empty
    `excluded_roles` list was covered by the S2 RED (the field did not exist).
  - GREEN: 1 scenario / 5 steps passed after restoring the condition.
  - [x] COMMIT: 6df78e6dc1 -- Batch: frozen-package-exclusion

- [x] Scenario: A frozen package root that covers the declaration keeps it in the trial
  - Design constraints: the frozen set drops `DECLARATION_PATH` after
    resolution, so a resolved package that covers `givn/specification.yaml`
    leaves the declaration a retained entry: `Retained: 1`, the declaration in
    the trial, `givn/specs/a.md` out of it. `Report::frozen_files` is
    `partition.frozen.len()`, so the line prints `(1 file)` although `e2e_files`
    records two identities (the declaration and `givn/specs/a.md`). Fixture:
    `ise_project_with_declaration(e2e_declaration("\"givn/**\"", "\"src/**\""))`
    plus a committed revision with `givn/specs/a.md` and `src/kept.rs`.
  - RED: the first run against the pre-change writer printed
    `Frozen package: e2e, not copied (3 files)` and `Retained: 1` failed to
    match, because the scenario's first fixture (`ise_project`) tracks the
    scaffolded `givn/config.yaml` and `givn/commands.yaml`; that is a fixture
    failure, not behavioral RED. The fixture now excludes both files, and the
    behavioral RED is the probe below.
  - PROBE: with the retained-set filter removed, the run printed
    `Retained: 2` (the declaration and `givn/specs/a.md`) and
    `preparation succeeds and prints "Retained: 1" and ...` failed.
  - GREEN: 1 scenario / 6 steps passed; `(1 file)` printed while `e2e_files`
    holds two identities.
  - [x] COMMIT: 6df78e6dc1 -- Batch: frozen-package-exclusion

- [x] Scenario: A dry run performs every read and writes no trial or record
  - Design constraints: `would_write` is `copy_paths()` (`retained ∪ kept
    source`, both frozen-free), so the block lists the sorted paths the trial
    would hold with no path under `tests/e2e`; the report prints the fixed lines,
    the receipt line, and `Frozen package: e2e, not copied (1 file)`; no
    directory and no record is written. Fixture: `ise_project()` whose revision
    commits `tests/e2e/Cargo.toml`, so the frozen set is that one file and
    `DEFAULT_TRIAL_PATHS` loses it.
  - PROBE: the scenario passed against the change, so the retained-set filter
    was removed again: `And the "Would write:" block lists the sorted paths the
    trial would hold` failed because the block listed `tests/e2e/Cargo.toml`.
  - GREEN: 1 scenario / 6 steps passed after restoring the filter; the block
    holds the five sorted paths and no path under `tests/e2e`.
  - [x] COMMIT: 6df78e6dc1 -- Batch: frozen-package-exclusion

## Batch: frozen-trial-refusal

Files: `src/ise/verify.rs` (`require_frozen_absent`, `trial_paths`, declaration
load moved out of `check_checkout`), `src/e2e_boundary/mod.rs`
(`E2eBoundary::contains`), `tests/steps/trial_input_verification_steps.rs`
(bindings and fixtures), `tests/steps/ise_clean_room_support.rs` (twenty-five
file package, record rewrite),
`givn/specs/prepare-an-ise-test/trial-input-verification.feature` (two
reconciled bodies).

- [x] Scenario: Verification names an e2e package file changed in the trial
  - Design constraints: `require_frozen_absent` runs after
    `require_record_belongs` and before `check_retained`, returns early unless
    `record.excluded_roles` names `e2e`, and walks every trial path with
    `read_dir` without following symlinks. A path the record's `e2e_files` lists
    or `E2eBoundary::contains` covers refuses with
    `the trial '<dir>' holds N path(s) the frozen e2e package covers; the
    builder must not receive the acceptance suite: <paths>`. The step copies the
    checkout's bytes at `tests/e2e/steps/keys.rs` into the trial, because
    preparation refuses to write one.
  - RED: first run of the bound scenario before the refusal existed:
    `Then verification fails and names "tests/e2e/steps/keys.rs"` failed with
    `assertion left == right failed` because the run exited 0 (the planted path
    was invisible to the old verifier).
  - GREEN: 1 scenario / 5 steps passed; the refusal names the path and the
    acceptance-suite sentence.
  - [x] COMMIT: 7759bf5f99 -- Batch: frozen-trial-refusal

- [x] Scenario: Verification names every drifting path in one run
  - Design constraints: the two retained drifts are compared by
    `check_retained` against `record.retained_files`; the frozen identity is
    compared against the current checkout by `check_checkout`, never against the
    trial. The trial-side walk finds nothing (no frozen path is present), so the
    run reports the two `Retained drift` lines and the one `E2E drift` line and
    `3 drifting path(s)`. The e2e change is made in the checkout because the
    trial does not hold that path.
  - RED: none claimed. The scenario passed against the pre-change verifier and
    still passes: it protects a distinct requirement (per-path drift is named on
    both sides, and the three paths are reported once) that the frozen refusal
    must not swallow. The trial-side walk finds no frozen path, so the run
    reaches both comparisons.
  - GREEN: 1 scenario / 5 steps passed; the run reports two `Retained drift`
    lines, one `E2E drift` line, and `3 drifting path(s)`.
  - [x] COMMIT: 7759bf5f99 -- Batch: frozen-trial-refusal

- [x] Scenario: Verification bounds the frozen paths it refuses
  - Design constraints: `FROZEN_REPORT_LIMIT = 20`; the refusal prints the first
    twenty sorted paths and `(and N more)`. The fixture commits
    `tests/e2e/Cargo.toml` beside twenty-four step files, so `e2e_file_set`
    resolves the directory to that manifest and returns exactly twenty-five
    paths; copying the package into the trial plants twenty-five covered paths,
    so the refusal names twenty and reports five remaining.
  - RED: the first bound run failed as `Step doesn't match any function`: the
    binding had dropped the feature's `fails and` words. That is a binding
    defect, not behavioral RED; the phrase was corrected to the feature text.
  - PROBE: with `FROZEN_REPORT_LIMIT` widened to 25, the refusal printed all
    twenty-five paths (`... f19.rs, f20.rs, f21.rs, f22.rs, …`) and
    `Then verification fails and names the first twenty frozen paths in sorted
    order` failed because `f19.rs` was named. Restoring 20 made it pass.
  - GREEN: 1 scenario / 5 steps passed; stderr reports `25 path(s)` and
    `(and 5 more)`.
  - [x] COMMIT: 7759bf5f99 -- Batch: frozen-trial-refusal

- [x] Scenario: Verification keeps a trial prepared before the package was excluded re-checkable
  - Design constraints: `excluded_roles` is `#[serde(default)]`, so a record
    without it deserializes to an empty list and the refusal is skipped. The
    fixture prepares a trial, re-plants the frozen files from the checkout, and
    rewrites the receipt with `serde_yaml_ng`: every `e2e_files` entry moves into
    `retained_files` (identities carried, bytes identical) and
    `excluded_roles` is dropped, which is exactly the older record content. The
    retained comparison then verifies the package files in the trial and the
    checkout comparison reports no drift.
  - RED: none claimed against the pre-change verifier (it had no refusal to
    skip). PROBE: the fixture was mutated to keep `excluded_roles: [e2e]` on the
    rewritten record; `And the record names no excluded role` failed with
    `the record still carries an excluded role`. The same probe stayed clean on
    the retained/e2e lines, so the recorded `retained_files` exemption covers
    the planted package paths even when the role is named.
  - GREEN: 1 scenario / 6 steps passed; the trial holds
    `tests/e2e/steps/keys.rs` and the re-check exits 0.
  - [x] COMMIT: 7759bf5f99 -- Batch: frozen-trial-refusal

- [x] Scenario: A frozen role that covers the declaration re-checks clean
  - Design constraints: two exemptions, both before the frozen test:
    `DECLARATION_PATH` and every key of `record.retained_files`. A declaration
    with `e2e: ["givn/**"]` resolves `givn/specification.yaml` as part of the
    package, so the record's `e2e_files` lists the declaration the trial
    deliberately retains; without the exemption the re-check would refuse the
    trial for holding its own declaration. The widened checkout root still
    reports `CHECKOUT_GAINED_PATH` drift for a package path the record does not
    list, so the exemption does not weaken the checkout comparison.
  - RED: none claimed against the pre-change verifier (it had no refusal).
    PROBE: removing both exemption filters made the run refuse the trial for
    holding its own declaration: stdout was empty and
    `Then verification reports the retained and e2e identities clean` failed.
    Removing only the `DECLARATION_PATH` clause changed nothing, because every
    record this binary writes lists the declaration in `retained_files`; the
    clause states the use-case rule and the recorded-set exemption carries it.
  - GREEN: 1 scenario / 4 steps passed; `Retained: clean`, `E2E: clean`, exit 0.
  - [x] COMMIT: 7759bf5f99 -- Batch: frozen-trial-refusal

## Batch: pending-register-reconciliation

Files: `givn/specs/fragments/adr-qualification-gate.feature`,
`tests/steps/adr_qualification_gate_steps.rs`.

- [x] Scenario: The migration preserves independent architecture history across three proposed decisions
  - Design constraints: ADR-078 moves the active register to 30 accepted
    records; the register already counts 30 while the permanent scenario and its
    binding asserted 29, so the permanent suite was red before this batch. The
    delta `@givn.modified` scenario asserts 30 and the binding asserts the
    accepted `ADR-078` row, matching the delta body so archive replaces the
    permanent assertion with the same one. Existing behavior protected: the
    scenario passed against the reconciled corpus before any production edit, so
    this batch claims no historical RED and carries a discrimination probe
    instead.
  - PROBE: flipping the ADR-078 register row to `proposed` made
    `Then the final register has 30 accepted decisions and no proposed
    decisions` fail with `assertion left == right failed: unexpected accepted
    ADR count`. Restoring the row made both the delta and the permanent
    scenario pass.
  - GREEN: 1 scenario / 11 steps passed from the delta specs and from
    `givn/specs`. The reconciliation body landed in `500d78df8b`, whose message
    did not name the batch, so the gate rejected it; `84651394c7` re-issues the
    binding with the batch label and adds the prose/table assertion (the
    register's summary sentence must say 30 accepted records), which the same
    scenario now protects.
  - [x] COMMIT: 84651394c7 -- Batch: pending-register-reconciliation

## Batch: prepare-trial-frozen-package-e2e

Files: `tests/steps/clean_room_preparation_e2e_steps.rs` (counts, frozen line,
four-path omission, transcript), `tests/steps/ise_clean_room_support.rs`
(fixture counts),
`givn/specs/prepare-an-ise-test/clean-room-preparation.feature` (reconciled
`@e2e` body), `givn/changes/keep-frozen-verifier-out-of-the-trial/evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`.

- [x] Scenario: Developer prepares a clean-room trial from a recorded revision
  - Design constraints: the resolved `e2e` package `tests/e2e` (manifest plus
    one step file) leaves the retained count: `Retained: 2` (the declaration and
    `givn/specs/demo.feature`), `Kept source: 2` (`src/keys/mod.rs`,
    `src/other.rs`), `Replaced: 1` (`src/keys/parser.rs`), `Outside the
    declaration: 1` (`notes.md`), `Not replaced: 1`. The report prints
    `Frozen package: e2e, not copied (2 files)` between `E2E digest` and
    `Written path`; the trial holds the retained and kept source files and the
    declaration, and omits the replaced file, `notes.md`, and both package
    files. The receipt keeps one identity per resolved package path and the
    counts. The scenario drives the compiled binary as a subprocess and commits
    its terminal transcript as the visual evidence.
  - RED: none claimed; the production behavior is the one batch
    `frozen-package-exclusion` changed. PROBE: the step was run with the
    pre-change `Retained: 4` expectation; it failed with
    `does not print the line "Retained: 4"` while the report printed
    `Retained: 2` and `Frozen package: e2e, not copied (2 files)`.
  - GREEN: 1 scenario / 13 steps passed from the delta specs (writing the
    transcript) and from `givn/specs` (which writes only the gitignored
    `target/terminal-evidence/`).
  - [x] COMMIT: 1bd97ce030 -- Batch: prepare-trial-frozen-package-e2e

## Closeout: merged step definitions

No delta step file exists: every delta phrase is bound in the permanent
per-capability files (`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/clean_room_preparation_e2e_steps.rs`,
`tests/steps/trial_input_verification_steps.rs`,
`tests/steps/adr_qualification_gate_steps.rs`). The archive check therefore finds
every delta phrase already bound in the permanent test tree.

## Done

- [x] Every scenario box above is checked and records its batch `COMMIT:`.
- [x] `./run-tests.sh --non-e2e` exits 0 on the permanent and delta corpus.
      ```
      $ ./run-tests.sh --non-e2e
      cargo fmt --all -- --check                 (clean)
      cargo clippy --locked --all-targets -- -D warnings
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.71s
          Summary [ 195.608s] 1075 tests run: 1075 passed, 114 skipped
      EXIT: 0
      ```
      The 114 skipped tests are the `@e2e` scenarios; the regular scope excludes
      them by tag.
- [x] `./run-tests.sh --e2e` exits 0; the e2e count is strictly less than the
      full count (114 of 1189 scenarios), so both scopes are honestly isolated.
      ```
      $ ./run-tests.sh --e2e
          Summary [  40.693s] 114 tests run: 114 passed, 1075 skipped
      EXIT: 0
      ```
- [x] No empty step body, no `@wip`, no removed `@e2e` tag:
      ```
      $ grep -n "@wip" givn/changes/keep-frozen-verifier-out-of-the-trial/specs/prepare-an-ise-test/*.feature
      (no output)
      $ git diff 1dd78e4234...HEAD -- givn/specs/prepare-an-ise-test/*.feature | grep -E "^[-+].*@e2e"
      (no removed or added @e2e tag; the two modified scenarios keep their tags)
      ```
- [x] Change-scoped gate evidence:
      ```
      $ givn check tasks --change keep-frozen-verifier-out-of-the-trial
      ```
      (output in the change-scoped receipt; the delta runs rewrite the committed
      transcript with a fresh fixture revision, which was restored to the
      committed bytes afterwards.)

