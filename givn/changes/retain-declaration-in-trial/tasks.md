# Tasks: retain-declaration-in-trial

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`), both already set in `givn/commands.yaml`. Strict mode:
`.fail_on_skipped()` on the cucumber builder in `tests/features_runner.rs`; an
undefined step is a failed `NotFound` step. Empty-body ban: no step body is
`{}`, a bare `pass`, a bare `return`, or whitespace. This change installs no
runner and changes no command, so strictness is proven once in `## Setup`.

Single-scenario run command (`<capability>/<feature>` is the path under the
change's `specs/`, design.md "Step Definitions"):

```sh
GIVN_FEATURES=givn/changes/retain-declaration-in-trial/specs \
  cargo test --locked --test features_runner -- \
  "<capability>/<feature>.feature::<scenario title>"
```

Release note (every batch commit subject, verbatim from `proposal.md`):

```text
givn ise replace copies the project's Specification declaration into every clean-room trial, counts and records it as retained, and givn ise verify re-checks it per path.
```

Batch commit rule: one atomic
`givn commit -m "<release note>"$'\n\n'"Batch: <batch-name>"$'\n'"Scenarios: <titles>" -- <paths>`
per batch with explicit paths. Every scenario of the batch records the same
revision as `COMMIT: <hash> -- Batch: <batch-name>`; the gate reads the first
`COMMIT:` line under the scenario line, so a scenario section carries exactly
one. `givn check review` passes a scenario commit only when the commit message
contains the batch name.

**Evidence slots.** Each scenario carries the RED slot (the observed failing
assertion for this batch) and the GREEN slot (the passing run). New phrases were
bound with their assertions in the same edit as the fixture, so the observed RED
is a failed step body naming the phrase, never an unbound step. No stub survived
its batch.

**`@wip`.** Each batch removes `@wip` from the scenarios it starts, never in
bulk. `givn check tasks` refuses the change while any selected delta scenario
still carries `@wip`.

**Base digest recompute rule (mandatory from design.md).** The delta
`specs/prepare-an-ise-test/usecase.md` line 1 records
`givn:base-sha256:7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6`,
the digest of `givn/specs/prepare-an-ise-test/usecase.md` at authoring time.
After any rebase or any revision that replaces the permanent
`givn/specs/prepare-an-ise-test/usecase.md`, recompute it and update delta
line 1 before archive:

```sh
sha256sum givn/specs/prepare-an-ise-test/usecase.md
head -1 givn/changes/retain-declaration-in-trial/specs/prepare-an-ise-test/usecase.md
```

## Batch map

| Batch | Scenario entries | Executed rows | Owning capability |
|---|---|---|---|
| `setup` | 0 | 0 | none |
| `declaration-retained-partition` | 6 | 6 | `clean-room-preparation` |
| `revision-declaration-authority` | 4 | 4 | `clean-room-preparation` |
| `verification-qualified-drift` | 4 | 4 | `trial-input-verification` |
| `served-guidance-declaration-rule` | 1 | 1 | `prose-consolidation` |
| `prepare-trial-declaration-e2e` | 1 (`@e2e`) | 1 | `clean-room-preparation` |
| **Total** | **16** | **16** | 11 `clean-room-preparation` + 4 `trial-input-verification` + 1 `prose-consolidation` |

The delta holds sixteen scenario declarations, each with one raw scenario (no
Outline, no `Examples`). Every title appears in exactly one batch by exact
title below.

## Interaction coverage pre-flight

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/prepare-an-ise-test/usecase.md` | 2 (`clean-room-preparation`, `trial-input-verification`) | 2 | yes: the permanent prepare `@e2e` scenario (modified here) and the permanent verify `@e2e` scenario |
| `givn/specs/create-change/usecase.md` | 1 (`prose-consolidation`) | 0 by design.md "Interaction Coverage Matrix": the change adds a served-text assertion, not a consumer action | not applicable; the capability's existing `@e2e` scenario is unchanged |

No gap: both consumer actions have their matrix row and their `@e2e` scenario.
Every new prepare scenario is a variant of the prepare action; the modified
`@e2e` prepare scenario carries the new smoke assertion (`Retained: 4` and the
trial's declaration). The four verify scenarios are variants of the verify
action.

## Domain Constraints

Copied from `specs/prepare-an-ise-test/usecase.md` (preconditions, main flow,
extensions, rules, success guarantee, Personas, Interactions) and design.md
(Decisions D1–D6, Data Model Changes, Admission and Replace Flow).

| Capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| `clean-room-preparation` | Rule: the declaration is an implicit retained entry, not a sixth role. Every trial retains `givn/specification.yaml` under `--full` and under `--path`; the copy recorded at the selected revision is the one the partition reads. The retained set is a union, so a role that also matches the declaration counts it once and `Retained` equals the record's `retained_files` entries. | `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` |
| `clean-room-preparation` | Rule: preparation reads the selected revision, never the working tree. A revision that does not record the declaration refuses; a malformed declaration at the revision refuses with the loader's reason. | `agent--technical-role` |
| `clean-room-preparation` | Rule: a path matched by a role or by the declaration is never replaced; naming the declaration, or a directory whose tracked files include it, refuses and names the declaration and the revision. A `source:` root that covers it replaces nothing. | `developer--primary-user` |
| `clean-room-preparation` | Main flow 5 and rule: the record's retained set includes `givn/specification.yaml`, so `Retained` counts the declaration and every role-retained file once, and `Retained digest` covers it. | `agent--technical-role`, `givn-maintainer--owner` |
| `trial-input-verification` | Rule and extension 6: the trial's declaration is re-checked against the record like every retained entry; the checkout's declaration is consulted only where it derives the frozen `e2e` package and the `behavior` feature selection, so moving `e2e:` or `behavior:` is drift and moving only `structure:` or `limits:` is not. | `developer--primary-user`, `agent--technical-role` |
| `prose-consolidation` | The served `ise` instruction states that the trial always carries the declaration itself and that it is an implicit retained entry, not a sixth role. | `agent--technical-role` |

Carried by every batch: no new record field and no schema change (D5); the
declaration is read through the one strict loader (`from_content`, same schema
and errors); `ise verify` keeps its working-tree declaration read only for the
`e2e` set and the `behavior` selection (D6); the nine refusals keep their
numbers and the exit codes stay `0`/`1`/`2` (D4). The report keeps its ten
fixed lines; no line is added.

## Setup

- [x] Read `specs/prepare-an-ise-test/usecase.md` (preconditions, main flow,
      extensions, rules, success guarantee, Personas, Interactions),
      `design.md` (Overview, Use-Case Traceability, Architecture Impact,
      Decisions D1–D6, Data Model Changes, Admission and Replace Flow, Verify
      Flow, Partition Rules, Refusals, External Boundary Cases, E2E
      infrastructure, Step Definitions, Strict Mode), and `design-review.md`.
      Domain constraints are in `## Domain Constraints` above.
      ```
      read: usecase.md Preconditions, Main flow 1-6, Extensions 3/5/6,
        Rules (declaration retained, deletion arithmetic, re-check),
        Success guarantee, Personas, Interactions;
        design.md all sections; design-review.md Q1-Q2 and B1-B6.
      base digest verified:
      $ sha256sum givn/specs/prepare-an-ise-test/usecase.md
      7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6  givn/specs/prepare-an-ise-test/usecase.md
      $ head -1 givn/changes/retain-declaration-in-trial/specs/prepare-an-ise-test/usecase.md
      <!-- givn:base-sha256:7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6 -->
      ```

- [x] Confirm the runner and both commands; change neither. cucumber 0.23.0
      from `Cargo.lock`, `.fail_on_skipped()` in `tests/features_runner.rs`,
      step files registered in `tests/steps/mod.rs`.
      ```
      verify.command:     ./run-tests.sh --non-e2e
      verify.e2e_command: ./run-tests.sh --e2e
      $ grep -n 'fail_on_skipped' tests/features_runner.rs
      56:        .fail_on_skipped()
      ```

- [x] Proof-of-strictness. Run one selected delta scenario before its phrase is
      bound; confirm a failed `NotFound` step and a non-zero exit. Then confirm
      `givn check tasks` refuses the change while a selected delta scenario is
      still `@wip`. The probe is the delta tree itself, so there is no scratch
      feature to delete.
      ```
      $ GIVN_FEATURES=givn/changes/retain-declaration-in-trial/specs \
          cargo test --locked --test features_runner -- --exact \
          "prepare-an-ise-test/clean-room-preparation.feature::The trial carries the declaration when no role names it"
       ✔  Given a Git project with ise enabled and a declaration with one source root "src/**"
       ✔  When the developer prepares a trial from "HEAD" with a full replacement
       ✔  Then preparation succeeds
       ✘  And the trial holds "givn/specification.yaml"
          Step failed:
          Defined: .../clean-room-preparation.feature:25:5
          Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      4 steps (3 passed, 1 failed)
      error: test failed, to rerun pass `--test features_runner`; exit 101

      $ givn check tasks --change retain-declaration-in-trial
      Mandatory checks for 'tasks':
      error: scenario 'Served ise guidance states that the trial always carries the declaration' is still tagged @wip in '.../specs/create-change/prose-consolidation.feature' — remove @wip before marking tasks complete
      exit 1
      ```
      After the probe the permanent suite remains the passing baseline:
      `./run-tests.sh --non-e2e` on `givn/specs` exits 0.

- [x] Fixture inventory (design.md "Step Definitions"). `e2e_trial_project` and
      `verify_trial_project` drop `givn/specification.yaml` from
      `.git/info/exclude` and commit it; `commit_e2e_trial_revision` and
      `commit_verify_trial_revision` add the declaration to their file lists.
      New fixtures: a `givn/**` source-root declaration (roles empty), a
      `contracts: ["givn/**"]` project whose `givn/` holds only the declaration
      and `givn/specs/a.md`, a `structure: ["docs/glossary.md"]` declaration, a
      first revision that predates the declaration, a malformed first revision
      with a valid `HEAD`, and a later revision that drops the `structure`
      entry. `DEFAULT_TRIAL_PATHS` gains `givn/specification.yaml`.

## Batch: declaration-retained-partition

Files: `src/ise/partition.rs` (implicit retained entry, declaration candidate
refusal, unit test), `tests/steps/ise_clean_room_support.rs` (fixtures),
`tests/steps/clean_room_preparation_steps.rs` (bindings and
`DEFAULT_TRIAL_PATHS`).

- [x] Scenario: The trial carries the declaration when no role names it
  - Design constraints: `partition()` seeds the retained set with
    `DECLARATION_PATH` (`givn/specification.yaml`) and unions the five role
    matches into it, so the declaration is retained with no role naming it and
    `copy_paths()` copies it. The fixture is
    `source_only_declaration("\"src/**\"")` (`ise_project_with_declaration`);
    every role list is empty and no source file is tracked, so the trial's only
    content is the declaration. The declaration is the revision's copy, not the
    working tree's (D1).
  - RED: Probe (the production edit preceded the run, so this is a discrimination run,
    not a historical RED): `git stash push -- src/ise/partition.rs`, same scope ->
    `Then the trial holds "givn/specification.yaml"` fails with
    `the trial at ... does not hold givn/specification.yaml`; the report printed
    `Retained: 0`. `git stash pop` restores the union seed.
  - GREEN: 6 scenarios / 29 steps passed; the trial holds the declaration.
  - [x] COMMIT: e2fb7e240d535c57f004070b75c7a90de1982721 -- Batch: declaration-retained-partition

- [x] Scenario: The trial record lists the declaration in its retained set
  - Design constraints: `record::identities(placement.retained_paths(), ...)`
    hashes every retained path with the same entry-kind/executable/bytes rule,
    so an implicit retained entry appears in `retained_files` with its 64-hex
    identity and `retained_digest` covers it. No record field and no scope
    change (D5). The receipt is parsed as YAML; the step looks the path up in
    `retained_files` and does not depend on field order.
  - RED: Probe (same reverted `partition()`): `Then the receipt lists
    "givn/specification.yaml" in its retained set` fails with `the record does
    not list givn/specification.yaml in retained_files`; the map held only
    role-matched paths.
  - GREEN: passed; `retained_files` holds the declaration with a 64-hex identity and
    `counts.retained` equals the map length.
  - [x] COMMIT: e2fb7e240d535c57f004070b75c7a90de1982721 -- Batch: declaration-retained-partition

- [x] Scenario: Preparation refuses a replacement path that names the declaration
  - Design constraints: `require_replaceable_paths` runs a declaration
    candidate test over the whole candidate set before the retained-role test
    and before the source-root test, and refuses with
    `'givn/specification.yaml' is retained by the Specification declaration at
    revision '<revision>'; a retained path is never replaced` (D3, refusal 7).
    `--path givn/specification.yaml` normalizes to the tracked declaration, so
    the refusal is deterministic whatever roles the project declares. Nothing
    is written: the trial and the receipt stay absent.
  - RED: Probe (same reverted `partition()`): exit 1, but stderr reads
    `ise replace: 'givn/specification.yaml' is outside every declared source root
    at revision '<r>'`; the declaration refusal does not exist.
  - GREEN: exit 1, stderr carries `'givn/specification.yaml' is retained by the
    Specification declaration` and `a retained path is never replaced`; both
    targets absent.
  - [x] COMMIT: e2fb7e240d535c57f004070b75c7a90de1982721 -- Batch: declaration-retained-partition

- [x] Scenario: Preparation refuses a directory that contains the declaration
  - Design constraints: `named_candidates` expands directory `givn` to every
    tracked file beneath it; the declaration candidate test over that expansion
    refuses the whole run with the same message as the exact path (D3). The
    test precedes the role test, so a role that also matches the directory
    cannot change the reported condition.
  - RED: Probe (same reverted `partition()`): the command exits 0. The directory
    expansion replaces `givn/notes.md` and silently keeps the declaration.
  - GREEN: exit 1 with the same declaration refusal; both targets absent.
  - [x] COMMIT: e2fb7e240d535c57f004070b75c7a90de1982721 -- Batch: declaration-retained-partition

- [x] Scenario: A full replacement keeps the declaration under a source root
  - Design constraints: the declaration is in the retained set before the
    source-root test, and `replaced` filters every retained path, so
    `--full` with `source: ["givn/**"]` replaces `givn/config.yaml`,
    `givn/commands.yaml`, and `givn/notes.md` (`Replaced: 3`) and keeps the
    declaration. `Not replaced` counts direct tracked siblings of the removed
    files, the declaration included; it is a count, never a refusal.
  - RED: Probe (same reverted `partition()`): `Then preparation succeeds and prints
    "Replaced: 3"` fails; the report prints `Replaced: 4`, because `--full`
    also replaces the declaration.
  - GREEN: `Replaced: 3`; the trial holds the declaration and omits `givn/notes.md`.
  - [x] COMMIT: e2fb7e240d535c57f004070b75c7a90de1982721 -- Batch: declaration-retained-partition

- [x] Scenario: A role that matches the declaration counts it once
  - Design constraints: the retained set is a set union, never a
    concatenation, so `contracts: ["givn/**"]` matching the declaration adds no
    second entry: `Retained: 2` for the declaration and `givn/specs/a.md`. The
    unit test `the_retained_count_equals_the_recorded_retained_entries` asserts
    `counts().retained == retained_paths().len()` for this declaration. The
    fixture excludes `givn/config.yaml` and `givn/commands.yaml` from the
    commit, so `givn/` holds only the declaration and `givn/specs/a.md`.
  - RED: Passes before the change, so it is a protected requirement, not a RED.
    Discrimination probe: replace the set union with
    `vec![DECLARATION_PATH.to_string()]` plus the role matches (a concatenation)
    -> the scenario fails with `Retained: 3` and the unit test fails with
    `left: 3 / right: 2`; restore the union -> both pass.
  - GREEN: `Retained: 2`; the unit test
    `ise::partition::tests::the_retained_count_equals_the_recorded_retained_entries`
    passes.
  - [x] COMMIT: e2fb7e240d535c57f004070b75c7a90de1982721 -- Batch: declaration-retained-partition

## Batch: revision-declaration-authority

Files: `src/config/declaration.rs` (`from_content`), `src/ise/mod.rs`
(`admit` order, revision declaration read), `tests/steps/ise_clean_room_support.rs`
(four fixtures), `tests/steps/clean_room_preparation_steps.rs` (bindings).

- [x] Scenario: Preparation partitions with the declaration recorded at the selected revision
  - Design constraints: `admit` reads `revision_files(revision, "")` first,
    then reads that revision's `givn/specification.yaml` blob and parses it with
    `SpecificationDeclaration::from_content`. The working-tree load is removed
    from `ise replace`; `ise verify` keeps its own working-tree read (D6). The
    partition and the trial therefore use the selected revision's roles and
    bytes. The fixture's first revision declares
    `structure: ["docs/glossary.md"]` with `source: ["src/**"]`; a later
    revision drops the `structure` entry, so a working-tree read would retain
    nothing and a revision read retains the document (refusal order: addon,
    backend, dirty, revision, declaration, partition).
  - RED: Probe (the production edit preceded the run): `git stash push -- src/ise/mod.rs
    src/config/declaration.rs`, same scope -> `Then the trial holds
    "docs/glossary.md"` fails with `the trial at ... does not hold
    docs/glossary.md`; the working-tree declaration has no `structure` entry.
    `git stash pop` restores the revision read.
  - GREEN: The batch scope (9 scenarios) passed; the trial holds `docs/glossary.md`
    and its declaration bytes equal the first revision's copy.
  - [x] COMMIT: 735498933db54b2ee38cecaf43154260dc88c54e -- Batch: revision-declaration-authority

- [x] Scenario: Preparation refuses a revision that does not record the declaration
  - Design constraints: refusal 5 absorbs the declaration's absence (D4).
    `admit` refuses when the entry list holds no `givn/specification.yaml`,
    naming the revision and the path:
    `the revision '<revision>' does not record 'givn/specification.yaml'; a
    clean-room trial always carries the declaration`. The check precedes the
    partition and every write. The fixture's first revision predates the
    declaration and `HEAD` records it, so the working tree is clean and a
    working-tree read would succeed with the wrong evidence.
  - RED: Probe (same reverted production): the run exits 1, but stderr reads
    `read the trial inputs at revision '<r>': 'givn/specification.yaml' cannot
    be read from revision '<r>'`. The Then fails with `does not carry "does not
    record"`: the old admission accepted the working-tree declaration and the
    blob read refused later.
  - GREEN: exit 1; stderr carries `the revision '<r>' does not record
    'givn/specification.yaml'`; both targets absent.
  - [x] COMMIT: 735498933db54b2ee38cecaf43154260dc88c54e -- Batch: revision-declaration-authority

- [x] Scenario: Preparation refuses a malformed declaration at the selected revision
  - Design constraints: `from_content` is the strict loader's byte-level entry
    point (same key scan, role order, normalizer, and `DeclarationError`
    variants); `load` delegates to it. A malformed declaration at the selected
    revision refuses as refusal 1 with `givn/specification.yaml` and the
    loader's reason, in the same wording as a working-tree read. The fixture's
    first revision carries the unknown key `component` while `HEAD` is valid.
  - RED: Probe (same reverted production): exit 0 (`left: Some(0) / right: Some(1)`);
    the old admission parsed the working tree's valid declaration and prepared
    the trial.
  - GREEN: exit 1; stderr carries the declaration and `unknown key 'component'`; both
    targets absent.
  - [x] COMMIT: 735498933db54b2ee38cecaf43154260dc88c54e -- Batch: revision-declaration-authority

- [x] Scenario: Preparation refuses a declaration that is not UTF-8
  - Design constraints: `admit` decodes the selected revision's declaration
    bytes as UTF-8 before the strict loader; a decoding failure refuses as
    refusal 1 and names `givn/specification.yaml` with the decoding error. The
    fixture's first revision commits `NON_UTF8_DECLARATION_BYTES` while `HEAD`
    records the valid declaration, so the working tree is clean and a
    working-tree read would prepare the trial.
  - RED: Probe (production restored after the run): replacing
    `std::str::from_utf8(&bytes)` with `String::from_utf8_lossy(&bytes)` made
    the scope fail at the Then with stderr `ise replace:
    givn/specification.yaml: missing role 'structure'` (the lossy bytes reached
    the loader) and no `invalid utf-8`; `git checkout -- src/ise/mod.rs`
    restored the decoding refusal.
  - GREEN: exit 1; stderr carries `givn/specification.yaml` and `invalid utf-8`;
    both targets absent. The delta scope passed 16 scenarios / 83 steps.
  - [x] COMMIT: 4df994238d8d6ee1be6896575a9119b65be33daf -- Batch: revision-declaration-authority

## Batch: verification-qualified-drift

Files: `tests/steps/trial_input_verification_steps.rs` (three bindings),
`tests/steps/ise_clean_room_support.rs` (reuse the committed declaration and
checkout rewrite helpers). No `src/ise/verify.rs` change: `check_retained`
already iterates `record.retained_files`, and `check_checkout` already derives
only the `e2e` set and the `behavior` selection.

- [x] Scenario: Verification names a declaration changed in the trial
  - Design constraints: the record's `retained_files` now holds the
    declaration, so `check_retained` reads the trial's copy and an appended
    byte yields `Retained drift: givn/specification.yaml (the trial differs from
    the record)` and exit 1. No new branch: the declaration travels the same
    path as every retained entry (D6).
  - RED: Passes after batches 1-2: `ise verify` has no production change in this
    batch, so the requirement is protected. Discrimination probe: skip
    `DECLARATION_PATH` inside `check_retained` -> exit 0 and `Retained: clean`,
    so the Then fails; restore -> the declaration is named.
  - GREEN: batch scope 13 scenarios / 63 steps passed; exit 1 with the retained drift
    line naming the declaration.
  - [x] COMMIT: 5ce1931586321c0f32d624e6f303d5e800bdd629 -- Batch: verification-qualified-drift

- [x] Scenario: Verification names a declaration missing from the trial
  - Design constraints: the same `check_retained` iteration maps a missing
    trial path to `the path is missing from the trial` on the retained drift
    line. The declaration's presence is a set membership in the record, not a
    synthesized check (D5: an older record simply lacks the entry).
  - RED: Same protected requirement; the same `check_retained` skip probe -> exit 0
    and the Then fails on the missing `left: Some(0)` exit.
  - GREEN: passed; exit 1 with the retained drift line naming the declaration.
  - [x] COMMIT: 5ce1931586321c0f32d624e6f303d5e800bdd629 -- Batch: verification-qualified-drift

- [x] Scenario: Verification reports no drift after only the checkout's structure role changes
  - Design constraints: `check_checkout` loads the checkout's declaration only
    to derive the current `e2e` package (`e2e_file_set`) and the `behavior`
    feature selection (`behavior_features`); the retained identities come from
    the record. Rewriting only `structure:` to `docs/glossary.md` moves no
    compared input, so the run prints `Retained: clean` and `E2E: clean` and
    exits 0 (D6).
  - RED: Passes before the batch; the split probe is the sibling scenario's: replace
    `current_e2e = e2e_file_set(&declaration, ...)` with the recorded
    `e2e_files` keys (removing the checkout declaration's `e2e` derivation) ->
    this scenario stays green while the added-root scenario fails. Restoring
    the derivation keeps both green.
  - GREEN: passed; `Retained: clean`, `E2E: clean`, exit 0.
  - [x] COMMIT: 5ce1931586321c0f32d624e6f303d5e800bdd629 -- Batch: verification-qualified-drift

- [x] Scenario: Verification names the e2e package a checkout declaration's added root brings
  - Design constraints: adding `tests/extra-e2e` to the checkout declaration's
    `e2e:` role and committing `tests/extra-e2e/Cargo.toml` makes
    `e2e_file_set` resolve that manifest into the current package; a path the
    record does not list is reported as `E2E drift: tests/extra-e2e/Cargo.toml
    (the e2e package gained this path)` and exit 1 (D6).
  - RED: Split probe: with the `e2e` derivation replaced by the recorded set, the run
    exits 0 and prints `E2E: clean`, so the Then fails (`left: Some(0)`); the
    structure-only sibling stays green.
  - GREEN: passed; exit 1, stderr names `tests/extra-e2e/Cargo.toml` and reports
    `e2e package gained this path`.
  - [x] COMMIT: 5ce1931586321c0f32d624e6f303d5e800bdd629 -- Batch: verification-qualified-drift

## Batch: served-guidance-declaration-rule

Files: `tests/steps/prose_consolidation_steps.rs` (two `Then` bindings). The
guidance itself is the design commit's `assets/instructions/ise.md` sentence;
this batch only asserts it.

- [x] Scenario: Served ise guidance states that the trial always carries the declaration
  - Design constraints: `givn instructions ise` serves
    `assets/instructions/ise.md`; the `Specification completeness` bullet ends
    with "The trial always carries the declaration itself:
    `givn/specification.yaml` is an implicit retained entry, not a sixth role,
    so a project need not list it and a builder reads the declared roles from
    the copy the selected revision records." The scenario reads the real served
    output through the CLI (`normalized_served(env, "ise")`) and asserts both
    sentences.
  - RED: Passes after the design commit: the sentence and the requirement are one
    shipped asset, so the requirement is protected and no production edit
    belongs to this batch. Discrimination probe: delete the sentence from
    `assets/instructions/ise.md`, rebuild -> `Then the guidance states that
    the trial always carries the declaration itself` fails; `git checkout --
    assets/instructions/ise.md`, rebuild -> green.
  - GREEN: batch scope 14 scenarios / 67 steps passed; the served text carries
    `The trial always carries the declaration itself` and `implicit retained
    entry, not a sixth role`.
  - [x] COMMIT: 19346249573d61115ae1f664d4489df3931189d9 -- Batch: served-guidance-declaration-rule

## Batch: prepare-trial-declaration-e2e

Files: `tests/steps/clean_room_preparation_e2e_steps.rs` (count binding change,
new trial-holds binding, transcript capture), `tests/steps/ise_clean_room_support.rs`
(the two e2e/verify fixtures commit the declaration). The `@e2e` scenario runs
the real `givn ise replace` subprocess against a temp Git project and asserts
the report, the trial, and the record.

- [x] Scenario: Developer prepares a clean-room trial from a recorded revision
  - Design constraints: the fixture commits the declaration with the seven
    files (`givn/config.yaml` stays excluded and untracked), so the trial's
    retained set is the declaration, `givn/specs/demo.feature`, and the two
    `tests/e2e` files: `Retained: 4`, with `Kept source: 2`, `Replaced: 1`,
    `Outside the declaration: 1`, and `Not replaced: 1` unchanged. The trial
    holds the declaration; the record's `counts.retained` is 4; typing stays
    the record's full digest while the report abbreviates. Transcript evidence
    is written to
    `givn/changes/retain-declaration-in-trial/evidence/visual/developer-prepares-a-clean-room-trial-from-a-recorded-revision/transcript.txt`
    from the real run; a permanent-corpus run writes to
    `target/terminal-evidence/` and never recreates the change directory.
  - RED: With the commits landed and the fixture unchanged: `GIVN_FEATURES=...
    ./run-tests.sh --e2e` -> `Then preparation succeeds` fails with
    `ise replace: the revision '<r>' does not record
    'givn/specification.yaml'; a clean-room trial always carries the
    declaration` (`left: Some(1) / right: Some(0)`), because the stale
    fixture excluded the declaration from the commit. After committing the
    declaration the stale `Retained: 3` binding no longer matches the
    scenario's `Retained: 4` phrase (`Step doesn't match any function`).
  - GREEN: delta e2e scope -> 1 scenario passed / 15 skipped; the report prints
    `Retained: 4`, the trial holds `givn/specification.yaml`, and the
    transcript is written with the receipt path line.
  - [x] COMMIT: 8568ee272eab55111d03aff40391eeaaccdcbe38 -- Batch: prepare-trial-declaration-e2e

## Closeout

- [x] Merged step definitions. No delta step copy exists: every phrase the delta
      features bind is bound in the permanent per-capability files
      (`tests/steps/clean_room_preparation_steps.rs`,
      `clean_room_preparation_e2e_steps.rs`,
      `trial_input_verification_steps.rs`, `prose_consolidation_steps.rs`), so
      the archive's phrase comparison has nothing to merge.
      ```
      $ grep -c '#\[given\|#\[when\|#\[then' tests/steps/clean_room_preparation_steps.rs \
          tests/steps/clean_room_preparation_e2e_steps.rs \
          tests/steps/trial_input_verification_steps.rs \
          tests/steps/prose_consolidation_steps.rs
      tests/steps/clean_room_preparation_steps.rs:144
      tests/steps/clean_room_preparation_e2e_steps.rs:11
      tests/steps/trial_input_verification_steps.rs:46
      tests/steps/prose_consolidation_steps.rs:69
      ```

- [x] Empty-body audit. No step body is `{}`, a bare `pass`, a bare `return`,
      or whitespace-only.
      ```
      $ grep -nE '^\s*\{\s*\}\s*$|^\s*pass\s*$|^\s*return\s*$' \
          tests/steps/clean_room_preparation_steps.rs \
          tests/steps/clean_room_preparation_e2e_steps.rs \
          tests/steps/trial_input_verification_steps.rs \
          tests/steps/prose_consolidation_steps.rs
      (no match; grep exit 1)
      ```

- [x] Full suites.
      ```
      $ cargo fmt --all -- --check                      exit 0
      $ cargo clippy --locked --all-targets -- -D warnings
      Finished `dev` profile; exit 0
      $ ./run-tests.sh --non-e2e
      Summary [190.693s] 1060 tests run: 1060 passed, 114 skipped; exit 0
      $ ./run-tests.sh --e2e
      Summary [40.526s] 114 tests run: 114 passed, 1060 skipped; exit 0
      ```
      The e2e run rewrites six archived screenshots in
      `givn/archive/development-timeline/evidence/visual/`; they were restored
      with `git checkout --` so no archived evidence changed.

- [x] Base digest rechecked after the implementation commits: the permanent
      `givn/specs/prepare-an-ise-test/usecase.md` is unchanged, so delta line 1
      still matches.
      ```
      $ sha256sum givn/specs/prepare-an-ise-test/usecase.md
      7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6  givn/specs/prepare-an-ise-test/usecase.md
      $ head -1 givn/changes/retain-declaration-in-trial/specs/prepare-an-ise-test/usecase.md
      <!-- givn:base-sha256:7539c79c70eaa4293a57b6a743ab051eeb6cd165098733828feed442d3b39fa6 -->
      ```

- [x] Discrepancies resolved while implementing.
      - design.md's "New bindings" table omits `Then the trial holds
        "docs/glossary.md"` (used by `Preparation partitions with the
        declaration recorded at the selected revision`). The permanent step
        file bound other trial-holds phrases literally, so a binding was added
        to the same file. No design decision changed.
      - The design's "Not-implemented stub pattern" was not used: each new
        phrase was bound with its assertion in the same edit, so the observed
        failure was a failed assertion in the step body, never an unbound step,
        a skip, or a compile error. The user-facing RED/GREEN discipline is
        unchanged.
      - The batch commits share one subject (the release note). The
        `declaration-retained-partition` commit also carries the batch's unit
        test; the `revision-declaration-authority` commit carries the verify
        fixture change that the new admission rule requires (the fixture's
        revision must record the declaration).
