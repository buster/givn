# Tasks: givn-self-rebuild-under-the-updated-rules

Executes `design.md` `## Declaration` (the exact declaration and the pin),
`## Trial protocol under the updated rules`, `## Neighbor record`,
`## Gap triage`, `## Evidence artifacts`, and `## Verification`. The change adds
no production code: `givn/specification.yaml` and `rust-toolchain.toml` are
data and build configuration, the two delta scenarios drive the unchanged CLI
and protect settled behavior, and the change's new evidence is the trial
re-run.

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both already set in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` on both cucumber
builders (`tests/features_runner.rs`, `e2e/tests/e2e_runner.rs`) and is
inherited unchanged by the design's `## Strict Mode`. Both delta scenarios are
regular: `GIVN_FEATURES=givn/changes/givn-self-rebuild-under-the-updated-rules/specs
cargo nextest list --locked --test features_runner` lists exactly two entries
and no End-to-end one.

Release note (subject of every behaviour batch commit, from `proposal.md`):

```text
Givn declares every build input its whole-product rebuild reads, so its next Dry-Run Ise rebuilds the product from the declared Specification and the recorded build inputs instead of inventing the build.
```

**Commit rule.** One atomic `givn commit` with explicit paths per batch. A
behaviour batch uses the release note as its subject; a docs or evidence batch
uses `docs(<change>): ...` or `chore(<change>): ...`. The body is
`Batch: <batch-name>`, plus the scenario title for a single-scenario batch.
Every task of a batch records the same revision as
`COMMIT: <hash> -- Batch: <batch-name>`. `@wip` is removed from a scenario when
its batch starts, never in bulk.

**Tick discipline.** A batch's boxes and its `COMMIT:` lines are committed in
the next commit, after the batch revision exists: no commit can record its own
hash. C1 therefore holds the unticked list, C2 holds the C1 tick, C3 holds the
C2 ticks, and so on. `givn check tasks` refuses a dirty tree outside evidence
paths, so the pre-trial ticks are committed before it runs and the boundary
receipt is committed after it. `givn archive` refuses an incomplete list, so
the two archive tasks are ticked before it runs (C7) and their identifiers are
corrected in C9 and C10.

**Placeholders.** Every `COMMIT:` line starts with `___`. A checked line with a
placeholder, a revision that does not exist, or a revision not reachable from
`HEAD` fails the review and archive gates; fill the real identifier, never a
reword.

**Evidence fields.** Per the served `tasks` instruction this file records no
test-first order and no RED/GREEN checkbox. Each batch map entry names the
planned discrimination proof, and each batch section carries one evidence slot
where the implementer pastes the observed run. Both scenarios protect a
requirement the tree already implements (`design.md` `## Verification` item 1:
"Both added scenarios pass immediately because they protect settled behavior; no
RED is claimed"), so the proofs below are mutation probes of the binding, not a
production RED.

**Commit numbering.**

| Commit | Content |
|---|---|
| C1 | plan commit: the change directory and the three arc42 chapter edits; the task list is unticked |
| C2 | batch `declaration-and-copy`: the declaration, the pin, both bindings, both scenarios untagged; the C1 tick |
| C3 | batch `pre-trial`: lint and declaration diff, dependency tree, coverage and inventory, clean-room scan; the C2 ticks |
| C4 | batch `pre-trial`: the `givn check tasks` transcript and receipt; the four pre-trial ticks. C4 is the trial revision |
| C5 | batch `trial`: every `attempt-1` file, `trial-revision.txt`, the boundary task tick |
| C6 | batch `verify-and-review`: both suites, `review.md`, the review transcript; the trial ticks |
| C7 | batch `archive-and-dogfood`: the review ticks and the two archive box ticks that make the list complete and the tree clean before publication |
| C8 | batch `archive-and-dogfood`: the archive candidate revision `givn archive` publishes |
| C9 | batch `archive-and-dogfood`: the post-archive suites and the self-dogfood transcript; the archive task identifier corrected to C8 |
| C10 | batch `archive-and-dogfood`: the final tick commit correcting the self-dogfood identifier to C9 |

## Batch map

| Batch | Tasks | Scenarios | Commit | Purpose |
|---|---|---|---|---|
| `setup` | 1 | 0 | C1 | Land the plan and the three chapter edits so the first scenario commit stays atomic. |
| `declaration-and-copy` | 2 | 2 | C2 | Apply the exact declaration and `rust-toolchain.toml`, bind both scenarios, untag both. |
| `pre-trial` | 6 | 0 | C3, C4 | Declaration lint and diff, package independence, coverage and inventory, clean-room scan, the boundary preflight receipt, the trial revision. |
| `trial` | 10 | 0 | C5 | Copy, owner check, compare-before, sandbox build, compare-after and version, frozen acceptance, gap triage, Neighbor record, the five resolved rows, the attempt evidence commit. |
| `verify-and-review` | 3 | 0 | C6, C7 | Both suites on the final revision, the independent review, `givn check review`, then the tick commit. |
| `archive-and-dogfood` | 2 | 0 | C7, C8, C9, C10 | The archive tick commit, `givn archive`, the post-archive suites and the self-dogfood transcript, the final identifier tick. |
| **Total** | **24** | **2** | | |

Planned discrimination proof for batch `declaration-and-copy`, declaration
scenario: the scenario owns its own temporary project, so the checkout's
declaration is never an input to it and no pre-edit behavior failure exists. The
proof is a mutation probe: with the fixture's `Cargo.lock` deleted, `givn lint`
prints `[SPEC ] advisory  Cargo.lock  the 'contracts' role declares 'Cargo.lock',
which matches no path in the project` and the third `Then` (no declared-path
advisory for the build inputs) fails; with the file restored the scenario
passes. The only pre-change failure of the scenario itself is the unbound
`Then`, which the strict runner reports as `Step doesn't match any function` and
never as PASS.

Planned discrimination proof for batch `declaration-and-copy`, carrier scenario:
the checkout's declaration is likewise not an input, and a prepared copy cannot
silently miss a declared build input (`rebuild-all` writes the copy or refuses
before any write). The proof is a mutation probe: with
`assets/templates/glossary.md` declared but absent at the selected revision,
`rebuild-all` exits 1, names the path and the revision, and writes no copy, so
the `Then` never sees a short copy; with the file committed the scenario passes.

Both scenarios share the batch commit, so the batch's full delta scope must be
green before C2: the runner executes a `@wip` scenario rather than skipping it
(the tag expression excludes only `@e2e` and `@givn.removed`), and an unbound
step fails strict mode. That is why the two bindings land in one commit.

## Interaction coverage pre-flight

The three owning inventories hold five entries; each has a row in `design.md`
`## Interaction Coverage Matrix` and an existing End-to-end scenario. This
change adds no user action and no End-to-end scenario.

| Inventory entry (owning document) | Matrix row | Existing End-to-end scenario |
|---|---|---|
| `specification-declaration`: declare the five roles and the replaceable source roots (`givn/specs/declare-the-specification/usecase.md`) | yes | `The declared roles are read by the project-health pass` |
| `clean-room-preparation`: write the spec copy (`givn/specs/prepare-an-ise-test/usecase.md`) | yes | `Developer rebuilds the whole product from its Specification` |
| `trial-input-verification`: compare the spec copy and the checkout (`givn/specs/prepare-an-ise-test/usecase.md`) | yes | `Verifier compares the spec copy with the manifest` |
| `change-strategy`: read the design guidance (`givn/specs/create-change/usecase.md`) | yes | `The design guidance requires one recorded change strategy` |
| `spec-gap`: read the review guidance (`givn/specs/create-change/usecase.md`) | yes | `The review guidance requires one spec gap outcome before any fix` |

No gap. The other `create-change` inventory rows are not touched.

## Domain constraints

Copied from the owning use-case documents and the proposal; every task below
carries the rows it needs.

| Capability | Guarantee or rule | Persona |
|---|---|---|
| `specification-declaration` | The declaration holds exactly five roles and one `source:` list; a build input sits under the role that owns its content class; a missing, unblobbed, non-UTF-8, or malformed declaration is refused by name before any reader acts; `givn lint` reads it only while ise is effective. | `agent--technical-role`, `developer--primary-user` |
| `clean-room-preparation` | The copy is written from the selected revision, never the working tree; every declared Specification file is copied; no End-to-end test enters it; a refusal is checked before the first write, names its condition and subject, exits 1, and leaves the checkout and every target unchanged. | `developer--primary-user`, `givn-maintainer--owner` |
| `trial-input-verification` | The comparison prints two subject lines, names every difference once, and exits 0 only when both subjects say `match`; an added file under a declared Specification path is refused. | `agent--technical-role`, `developer--primary-user` |
| `change-strategy` | Exactly one `Decides D` entry records the change strategy, the affected consumers, the persisted state, and the rebuild boundary. | `developer--primary-user` |
| `spec-gap` | One difference receives exactly one outcome before any fix; only `specification defect` is a gap in the Specification; an `E2E test defect` restarts the trial at the correcting revision. | `givn-maintainer--owner` |

Carried by every task: no Rust module, struct, or field name in user-facing
output; exit 0 is success or match, 1 is a refusal or difference, 2 is a usage
error; the spec copy, the manifest, and the rebuilt binary are evidence, never
acceptance authority; nothing is promoted and the `S14` decision stays outside
this change; no file under `givn/specs/**`, `e2e/**`, `assets/**`,
`docs/reference.md`, or `src/**` changes.

---

## Setup

- [x] Land the plan on the change branch
  - Paths: `givn/changes/givn-self-rebuild-under-the-updated-rules/` (proposal,
    specs, design, design-review, arc42, config-lock, tasks),
    `docs/arc42/02-architecture-constraints.md`,
    `docs/arc42/04-solution-strategy.md`,
    `docs/arc42/11-risks-and-technical-debt.md`.
  - Run:
    ```sh
    givn commit givn/changes/givn-self-rebuild-under-the-updated-rules \
      docs/arc42/02-architecture-constraints.md \
      docs/arc42/04-solution-strategy.md \
      docs/arc42/11-risks-and-technical-debt.md \
      -m 'docs(givn-self-rebuild-under-the-updated-rules): plan the change with the declared build inputs

Batch: setup'
    git rev-parse HEAD
    ```
  - Acceptance: exit 0; the commit holds the change directory and the three
    chapter edits; `git status --porcelain` prints nothing; the 40-character
    revision is C1.
  - Design constraints: served `tasks` commit rule (the planning layer lands
    before implementation); `arc42.md` rows 2, 4, and 11 are already realized by
    the three chapter diffs the design review hashed. No production file
    changes.
  - [x] COMMIT: `ee16f1f` -- Batch: setup

## Batch: `declaration-and-copy`

Files modified: `givn/specification.yaml`, `rust-toolchain.toml` (new),
`tests/steps/specification_declaration_steps.rs`,
`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`, and the two delta features
(`@wip` removed from both). Both bindings land together because the batch's
delta scope runs both scenarios and a `@wip` scenario still executes.

- [x] Scenario: A declaration that names build inputs under the four content roles is accepted
  - Design constraints: D1 (pin `1.97.1` with `clippy` and `rustfmt` under
    `limits`), D4 (build inputs under the four content-class roles with the
    design's exact entries), and the design's role table. The fixture writes
    the build-input declaration and every file it names, with `e2e: ["tests/e2e"]`
    and `tests/e2e/Cargo.toml` carrying a minimal `[package]`, disjoint from
    `src/**`; it holds no `.feature` file outside the `behavior` globs. The two
    `Then` bodies are non-empty and assert on `stderr`; the scenario drives
    `givn lint` through the built binary.
  - Paths: `givn/specification.yaml`, `rust-toolchain.toml`,
    `tests/steps/specification_declaration_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`,
    `givn/changes/givn-self-rebuild-under-the-updated-rules/specs/declare-the-specification/specification-declaration.feature`.
  - Run:
    ```sh
    GIVN_FEATURES=givn/changes/givn-self-rebuild-under-the-updated-rules/specs \
      cargo test --locked --test features_runner -- \
      --name '^A declaration that names build inputs under the four content roles is accepted$'
    ```
  - Acceptance: the single-scenario run prints `1 feature`, `1 scenario (1
    passed)`, `5 steps (5 passed)`, and exits 0. The mutation probe fails on the
    advisory named in the batch map and the restored fixture passes.
  - Evidence: paste the single-scenario run and the mutation-probe failure line.
    ```text
    Feature: specification-declaration
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    --- mutation probe: Cargo.lock not written into the fixture ---
    ✘  And stderr contains no declared-path advisory for the build inputs
    [SPEC ] advisory  givn/specification.yaml & Cargo.lock  the 'contracts' role declares 'Cargo.lock', which matches no path in the project
    ```
  - [x] COMMIT: `a524d14` -- Batch: declaration-and-copy

- [x] Scenario: Preparation carries every declared build input into the spec copy
  - Design constraints: D4 and D6; the design's `## Delta Step Bindings` rows
    for `clean-room-preparation`. The shared build-input declaration fixture
    writes the five build-input paths (`Cargo.toml`, `Cargo.lock`,
    `rust-toolchain.toml`, `assets/templates/glossary.md`,
    `assets/instructions/ise.md`) plus `givn/specs/demo.feature`; it takes the
    `e2e` role path as one argument, and this scenario passes `e2e` (the
    declaration scenario passes `tests/e2e`). The second `Given` commits those
    paths, `src/main.rs`, and `e2e/Cargo.toml` with `e2e: ["e2e"]`, so the
    asserted omission of `e2e/Cargo.toml` proves the End-to-end role is never
    copied. The existing `When` (whole product from `HEAD` into a spec copy),
    `Then` report-line, `the spec copy holds "X" and "Y"`, and
    `the spec copy omits "X" and "Y"` bindings are reused unchanged.
  - Paths: `tests/steps/clean_room_preparation_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`,
    `givn/changes/givn-self-rebuild-under-the-updated-rules/specs/prepare-an-ise-test/clean-room-preparation.feature`.
  - Run:
    ```sh
    GIVN_FEATURES=givn/changes/givn-self-rebuild-under-the-updated-rules/specs \
      cargo test --locked --test features_runner -- \
      --name '^Preparation carries every declared build input into the spec copy$'
    GIVN_FEATURES=givn/changes/givn-self-rebuild-under-the-updated-rules/specs \
      ./run-tests.sh --non-e2e
    ```
  - Acceptance: the single-scenario run prints `1 feature`, `1 scenario (1
    passed)`, exit 0, and the report line `Specification files: 7`; the delta
    scope prints `2 scenarios (2 passed)`, exit 0, with no `@wip` left;
    `cargo fmt --check` and clippy (both part of `run-tests.sh`) stay green. The
    mutation probe fails as described in the batch map (exit 1, names
    `assets/templates/glossary.md` and the revision, writes no copy) and the
    restored fixture passes.
  - Evidence: paste the single-scenario run, the delta summary, and the
    mutation-probe refusal.
    ```text
    Feature: Clean-room preparation
    1 scenario (1 passed)
    8 steps (8 passed)
    Specification files: 7
    --- delta scope ---
    Summary [   1.278s] 2 tests run: 2 passed, 0 skipped
    --- mutation probe: assets/templates/glossary.md not committed ---
    ✘  Then the command succeeds and prints "Specification files: 7"
    stderr=ise rebuild-all: the declared Specification entry 'assets/templates/glossary.md' in the structure role matches nothing at revision '0369a968ce2265a280af46847a11875e4d8b7f86'
    ```
  - [x] COMMIT: `a524d14` -- Batch: declaration-and-copy

## Batch: `pre-trial`

Every command runs from the repository root. Evidence paths are relative to
`givn/changes/givn-self-rebuild-under-the-updated-rules/`.

- [x] Record the accepted declaration
  - Paths: `evidence/ise/declaration-lint.txt`, `evidence/ise/declaration.diff`.
  - Run:
    ```sh
    set -o pipefail
    { givn lint; printf 'exit: %s\n' "$?"; } 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/declaration-lint.txt
    git diff <C1> -- givn/specification.yaml rust-toolchain.toml \
      > givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/declaration.diff
    ```
  - Acceptance: `givn lint` exits 0; the transcript prints 52 `[SPEC ]`
    advisories and no `[WIP ]` line: the one expected
    `specification_feature_unretained` for `assets/templates/spec.feature` plus
    51 for the undeclared `test-corpus/specs/**` features (recorded, not
    silenced); no `specification_path_missing`,
    `specification_role_duplicate`, or `specification_e2e_boundary` line names
    `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `assets/`, or `e2e`; no
    declaration-schema error. The diff names the build-input entries and the
    pin, and no unrelated hunk.
  - Design constraints: design `## Pre-trial checks`; the proposal's accepted
    declaration bullet. The declaration and the pin are the only files the diff
    may report.
  - [x] COMMIT: `6799399` -- Batch: pre-trial

- [x] Prove the frozen package reaches no product crate
  - Paths: `evidence/ise/e2e-dependencies.txt`.
  - Run:
    ```sh
    set -o pipefail
    cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/e2e-dependencies.txt
    ```
  - Acceptance: exit 0; the transcript names no `givn`, `givn-config`, or
    `givn-core` package (`grep -E '(givn|givn-config|givn-core) v' <transcript>`
    prints nothing); `e2e` is its own workspace root.
  - Design constraints: D3 and D6; the frozen package is authority outside the
    trial and links no product crate.
  - [x] COMMIT: `6799399` -- Batch: pre-trial

- [x] Export the coverage and the End-to-end inventory
  - Paths: `evidence/ise/spec-coverage.json`, `evidence/ise/e2e-inventory.md`.
  - Run:
    ```sh
    givn spec coverage --format json \
      > givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/spec-coverage.json
    ```
  - Acceptance: the JSON parses; the selected set counts 115 entries over 56
    distinct feature paths at the authoring revision:
    ```sh
    jq '[.entries[] | select(.e2e)] | length' <coverage.json>                         # 115
    jq '[.entries[] | select(.e2e) | .feature_path] | unique | length' <coverage.json> # 56
    ```
    `e2e-inventory.md` holds one row per selected entry with capability, feature
    path, and scenario title, and its row count equals the coverage count. The
    trial revision recomputes both values.
  - Design constraints: design `## Evidence artifacts`; the inventory is what
    the report's `end_to_end_files` and the `S14` comparison read.
  - [x] COMMIT: `6799399` -- Batch: pre-trial

- [x] Scan the Cargo stores for a published copy
  - Paths: `evidence/ise/clean-room-scan.txt`.
  - Run:
    ```sh
    set -o pipefail
    for store in "$HOME/.cargo/registry/src" "$HOME/.cargo/registry/cache" \
      "$HOME/.cargo/git"; do
      if [ -d "$store" ]; then
        find "$store" -maxdepth 4 \( -name 'givn' -o -name 'givn-*' \) -print
      else
        printf 'absent: %s\n' "$store"
      fi
    done | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/clean-room-scan.txt
    ```
  - Acceptance: exit 0. All three stores exist on this host and the transcript
    is empty; a store absent on another host records one `absent: <path>` line
    and has no hit. Any hit is quarantined under
    `/tmp/givn-ise-quarantine-<stamp>/` and its new location is recorded, or the
    trial uses a fresh `CARGO_HOME` populated from the declared lockfile. The
    result is carried into `attempt-1/neighbor-record.md`.
  - Design constraints: D6 and the design's carried clean-room-scan decision;
    `~/.cargo/registry/git` is
    not a store, and the sandbox binds `~/.cargo/registry` and `~/.cargo/git`.
  - [x] COMMIT: `6799399` -- Batch: pre-trial

- [x] Commit the pre-trial inputs
  - Paths: `evidence/ise/declaration-lint.txt`,
    `evidence/ise/declaration.diff`, `evidence/ise/e2e-dependencies.txt`,
    `evidence/ise/spec-coverage.json`, `evidence/ise/e2e-inventory.md`,
    `evidence/ise/clean-room-scan.txt`,
    `givn/changes/givn-self-rebuild-under-the-updated-rules/tasks.md`.
  - Change: tick the setup task and the two scenario tasks, write their
    `COMMIT:` lines (`C1`, `C2`, `C2`), then commit the change directory.
  - Run:
    ```sh
    givn commit givn/changes/givn-self-rebuild-under-the-updated-rules \
      -m 'chore(givn-self-rebuild-under-the-updated-rules): record the pre-trial inputs

Batch: pre-trial'
    git status --porcelain
    git rev-parse HEAD
    ```
  - Acceptance: exit 0; `git status --porcelain` prints nothing; the
    40-character revision is C3 and holds the six evidence files and the ticks.
    Do not write `evidence/ise/trial-revision.txt` yet: it cannot belong to the
    revision it names, and the trial reads the checkout. The `trial` batch
    writes it after the run.
  - Design constraints: D5 (the trial reads the C4 commit, passed explicitly);
    the design's evidence table (author checks stay under `evidence/ise/`). The
    commit moves no declaration entry and no chapter content.
  - [x] COMMIT: `6799399` -- Batch: pre-trial

- [x] Prove the boundary preflight and write the receipt
  - Paths: `evidence/ise/boundary-check.txt`,
    `verification/tasks-verification.json`, `verification/tasks-result.json`.
  - Precondition: C3 exists and the tree is clean; `givn check tasks` refuses a
    dirty tree outside evidence paths. The four pre-trial boxes stay unticked
    while it runs: it reads the ticked scenario lines, which C3 records.
  - Run:
    ```sh
    set -o pipefail
    givn check tasks --change givn-self-rebuild-under-the-updated-rules 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/boundary-check.txt
    givn commit givn/changes/givn-self-rebuild-under-the-updated-rules \
      -m 'chore(givn-self-rebuild-under-the-updated-rules): record the boundary receipt

Batch: pre-trial'
    git rev-parse HEAD
    ```
  - Acceptance: the gate exits 0; the transcript prints `Mandatory checks for
    'tasks':`, the declared-package preflight with no refusal,
    `✓ scenario commits passed (2 scenario(s))`, the delta suite summary with
    2 scenarios and 0 failed or skipped, and `✓ verify passed`. The receipt
    records the tested revision (C3), the runner counts, and the content digest;
    `verification/tasks-result.json` holds `total` 2 with `failed` and `skipped`
    both 0. The commit is C4 and holds the transcript, the receipt, and the four
    pre-trial ticks; this task's own tick and its `COMMIT:` line citing C4 are
    committed in C5. C4's 40-character revision is the trial revision carried
    into every command below. A `@wip` refusal or a non-zero suite is a setup
    error, not a task defect: repair and rerun.
  - Design constraints: design `## Pre-trial checks` (boundary row) and
    `## Verification` item 3; the receipt is suite evidence, the transcript is
    the boundary evidence.
  - [x] COMMIT: `746e660` -- Batch: pre-trial

## Batch: `trial`

Every command runs from the repository root. `attempt-1` is the only attempt
unless a difference reroutes to the builder or the tests; a rerun writes a new
`attempt-<N>` directory and never overwrites an earlier one.

- [x] Write the spec copy from the trial revision
  - Paths: `evidence/ise/attempt-1/rebuild-report.txt`,
    `evidence/ise/attempt-1/manifest.yaml`. The copy and the manifest stay
    outside the checkout at `/tmp/givn-ise-self-rebuild-<stamp>` and
    `/tmp/givn-ise-self-rebuild-<stamp>.yaml`, with
    `<stamp>` = `date -u +%Y%m%dT%H%M%SZ`.
  - Run:
    ```sh
    attempt=attempt-1
    mkdir -p givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/$attempt
    set -o pipefail
    givn ise rebuild-all \
      --at <revision> \
      --into /tmp/givn-ise-self-rebuild-<stamp> \
      --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/$attempt/rebuild-report.txt
    cp /tmp/givn-ise-self-rebuild-<stamp>.yaml \
      givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/$attempt/manifest.yaml
    ```
  - Acceptance: exit 0; `Revision:` equals the trial revision; `Source code
    kept: 0`; `Source code removed:` equals
    `git ls-tree -r --name-only <revision> -- src tests | wc -l` (187 at
    authoring); `Specification files:` equals the manifest's
    `specification_files` length (188 at authoring); `End-to-end tests: not
    copied (<n> files)` names the package count (100 at authoring); `Copy
    directory:` and `Manifest:` equal the two targets. The manifest is
    `ise-manifest/v1`; every checksum is 64 lowercase hex and both rollups
    recompute. A disagreement, usage error, or refusal is a setup error and
    stops the trial.
  - Design constraints: D5 (explicit `--at`), D6 (whole-product rebuild,
    `Dry-Run Ise`); `--dry-run` is not passed because it writes no copy for a
    builder.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Verify the copy carries every declared build input
  - Paths: the copy only; record the checks in
    `evidence/ise/attempt-1/builder-notes.md` under `## Copy ownership`.
  - Run:
    ```sh
    set -eu
    test ! -e <copy>/src
    test ! -e <copy>/tests
    test ! -e <copy>/e2e
    test ! -e <copy>/.git
    for path in givn/specification.yaml Cargo.toml Cargo.lock rust-toolchain.toml \
      assets/templates/glossary.md assets/instructions/ise.md \
      givn/specs/declare-the-specification/usecase.md; do
      test -e "<copy>/$path"
    done
    jq '.counts.specification_files' <manifest.yaml>
    find <copy> -type f | wc -l
    ```
  - Acceptance: the copy holds the declaration, the manifest, the lockfile,
    the toolchain pin, every `assets/**` path the declaration names, and the
    `givn/specs/**` corpus; the manifest's `specification_files` names every one
    of them; the report's `Specification files` count, the manifest's
    `counts.specification_files`, and `find <copy> -type f | wc -l` agree (188 at
    authoring); the copy holds no `e2e` file. A missing declared build input
    here is a setup error reported by `givn ise compare`, never a builder task:
    record it and stop before the builder starts.
  - Design constraints: design `## Rebuild and copy checks` and
    `## External boundary cases` (`A declared build input is missing from the
    copy`); the preparing attempt owns `rebuild-report.txt`, `manifest.yaml`,
    and `compare-before.txt`.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Compare the spec copy with the manifest before the build
  - Paths: `evidence/ise/attempt-1/compare-before.txt`.
  - Run:
    ```sh
    set -o pipefail
    givn ise compare --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml \
      --copy /tmp/givn-ise-self-rebuild-<stamp> 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/compare-before.txt
    ```
  - Acceptance: exit 0; `Specification files: match`; `End-to-end tests:
    match`; no added or removed path line.
  - Design constraints: D3 and D6; the comparison prints two subject lines and
    refuses an added file under a declared Specification path.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Build the replacement in the clean-room sandbox
  - Paths: `evidence/ise/attempt-1/builder-notes.md`; the copy's `src/**` and
    `tests/**`.
  - Run: start a fresh agent whose only workspace is the copy and whose only
    repository path is `<copy>` mounted at `/work`. Use the design's `bwrap`
    command with the copy bound at `/work`, `--tmpfs /home`, the three rustup
    binaries and `~/.rustup`, `~/.cargo/registry`, and `~/.cargo/git`
    read-only, `--setenv HOME /home/builder`, `RUSTUP_HOME`, `CARGO_HOME`, and
    `PATH` as the design states, and `cargo build --release --offline` as the
    builder command.
  - Acceptance: exit 0 and `<copy>/target/release/givn --version` prints the
    version the declared `Cargo.toml` names (`0.9.20`). `builder-notes.md`
    lists every file authored (only under `src/**` and `tests/**`), every crate
    version resolved, every command run, and a statement that the checkout,
    `e2e/`, and the network were not read. Both `tests/features_runner.rs` and
    `tests/ready_fixture.rs` exist with a minimal `fn main() {}` before the
    first build; if a first build still fails at manifest parse, record exit
    101 and the two authored target files, and continue: that is a setup error,
    never a Spec gap. A missing pinned toolchain or a locked crate missing from
    the cache is a setup error recorded in the notes; install or refresh
    outside the sandbox and restart the trial.
  - Design constraints: D6 (the builder reads no product source outside the
    copy and the declared inputs), the design's test-target rule, and the
    design's `## External boundary cases` rows.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Compare after the build and record the rebuilt version
  - Paths: `evidence/ise/attempt-1/compare-after.txt`,
    `evidence/ise/attempt-1/rebuilt-version.txt`.
  - Run:
    ```sh
    set -o pipefail
    givn ise compare --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml \
      --copy /tmp/givn-ise-self-rebuild-<stamp> 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/compare-after.txt
    { <copy>/target/release/givn --version
      sha256sum <copy>/target/release/givn; } \
      | tee -a givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/rebuilt-version.txt
    ```
  - Acceptance: `compare` exits 0 with both subjects `match` (the build adds
    only files under the replaced unit, which `compare` does not verify); the
    version line names `0.9.20` (the declared manifest version, not `0.9.10`);
    the SHA-256 is 64 lowercase hex.
  - Design constraints: design `## Frozen acceptance`; the version is the
    former product-version row's discriminator.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Run the frozen End-to-end acceptance
  - Paths: `evidence/ise/attempt-1/e2e-run.log`,
    `evidence/ise/attempt-1/e2e-result.json`.
  - Run:
    ```sh
    set -o pipefail
    GIVN_BIN=<copy>/target/release/givn \
    GIVN_RESULT_FILE=$PWD/givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/e2e-result.json \
    ./run-tests.sh --e2e --package e2e 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/e2e-run.log
    ```
  - Acceptance: the log and the result file record one set of counts for this
    attempt; `skipped` is 0 and `total` matches the frozen package's selected
    set (120 at authoring: 115 scenarios plus 5 Example rows). A zero-scenario
    run, a skipped step, or an unset `GIVN_BIN` is a setup error, never a green
    run. A non-zero `failed` is the input to the next task.
  - Design constraints: design `## Frozen acceptance` and `## Strict Mode`;
    `GIVN_BIN` names the rebuilt binary and the checkout supplies the fixture
    and the repository files the scenarios read.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Classify every difference with one outcome
  - Paths: `evidence/ise/attempt-1/gap-table.md`,
    `evidence/ise/attempt-1/spec-gaps.md`.
  - Acceptance: one row per difference with the frozen scenario title, the
    failing step, the observed and expected result, the rebuilt file, exactly
    one outcome (`E2E test defect`, `implementation defect`,
    `specification defect`, or `acceptable undocumented change`), the wrong
    artifact, and the evidence path; a trial with no difference records
    `No difference observed` and names the result file and counts. `spec-gaps.md`
    names the attempt it classifies and lists every attempt's counts. An
    `implementation defect` returns to the builder inside this trial with no
    corpus or Specification edit and a new `attempt-<N>` directory; an
    `E2E test defect` corrects the test and restarts at the correcting
    revision; `specification defect` and `acceptable undocumented change` are
    recorded with no edit in this change. No difference is closed as drift.
  - Design constraints: D8 (classify before repair); the design's
    `## Gap triage` table and routes.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Record the Neighbor record and the scan disposition
  - Paths: `evidence/ise/attempt-1/neighbor-record.md`.
  - Acceptance: one row per file changed outside the declared Specification or
    the replaced unit and one row per consolidation, with the reason; an empty
    table with the design's `(empty)` row is the expected result because the
    builder authors only `src/**` and `tests/**`. The file also records the
    clean-room scan result and the three store paths
    (`~/.cargo/registry/src`, `~/.cargo/registry/cache`, `~/.cargo/git`), every
    quarantined path, and the verifier's test-infrastructure inputs (`GIVN_BIN`
    naming the rebuilt binary, `GIVN_READY_FIXTURE` and `GIVN_ROOT` naming the
    checkout). It states that no rebuilt file was promoted. It is a review
    record, not a gate.
  - Design constraints: D7; the spelling `Neighbor record` follows
    `docs/glossary.md` and the served `ise` instruction.
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Confirm the five former Specification defects are resolved
  - Paths: `evidence/ise/attempt-1/spec-gaps.md` and `gap-table.md`.
  - Acceptance: the final table holds no `specification defect` row for the
    four attribution-asset surfaces (OpenCode, Claude, omp, Maki embedded
    integration text) and none for the product version (`0.9.10` versus the
    declared `0.9.20`); a remaining failure on any of those five surfaces is
    classified `implementation defect`, not `specification defect`. The residual
    `implementation defect` rows are recorded as classification evidence and
    are not repaired in this change. State the count of resolved rows and the
    attempt it was read from.
  - Design constraints: the proposal's `## Spec Gap` and `## Proposed Solution`;
    D8; the prior gap record
    `givn/archive/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/spec-gaps.md`
    (5 specification defects, 64 implementation defects, 120 total).
  - [x] COMMIT: `508e776` -- Batch: trial

- [x] Commit the attempt evidence and the trial revision
  - Paths: `givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/trial-revision.txt`,
    `evidence/ise/attempt-1/`, and the tick edits.
  - Change: write `trial-revision.txt` with the 40-character revision and the
    commands that resolve it (`git rev-parse HEAD` at the pre-trial commit,
    `git log -1 --format=%H -- givn/specification.yaml`); then commit the whole
    evidence directory and the ticks.
  - Acceptance: exit 0; the commit holds `attempt-1` and `trial-revision.txt`;
    `git status --porcelain` prints nothing; the trial revision recorded in the
    file equals the report's `Revision:` line.
  - Repair round: every `implementation defect` returned to the builder in the
    same trial with no corpus or Specification edit (design D8). The builder
    edited 13 files under `src/**` in the spec copy and added none; `attempt-2`
    records the rebuild, the frozen rerun (120 total, 41 passed, 79 failed, 0
    skipped), the gap table, and the neighbor record. Attempt 1 stays on disk
    with its own counts (33 passed, 87 failed). `attempt-2/spec-gaps.md` names
    the classified attempt and lists both.
  - Evidence: `attempt-1/` and `attempt-2/` in commit `78dfde5` and `508e776`.
  - Design constraints: the design's evidence table (`trial-revision.txt` is
    written after the run, never in the revision it names) and the served
    `tasks` evidence rule.
  - [x] COMMIT: `508e776` -- Batch: trial

## Batch: `verify-and-review`

- [ ] Run both suites on the final revision
  - Paths: `evidence/ise/attempt-1/verify-non-e2e.log`,
    `evidence/ise/attempt-1/verify-e2e.log`.
  - Run:
    ```sh
    set -o pipefail
    ./run-tests.sh --non-e2e 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/verify-non-e2e.log
    ./run-tests.sh --e2e --package e2e 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/verify-e2e.log
    ```
  - Acceptance: both exit 0 with 0 failed and 0 skipped, and the runs select
    their scopes explicitly (`verify.command` for the regular suite,
    `verify.e2e_command` for the package). The regular suite includes the two
    scenarios under the change's delta path only through `givn check tasks`; the
    permanent scope is green after archive in the next batch.
  - Design constraints: design `## Verification` item 6; no suite command
    changes.
  - [ ] COMMIT: `___` -- Batch: verify-and-review

- [ ] Write the independent review
  - Paths: `givn/changes/givn-self-rebuild-under-the-updated-rules/review.md`.
  - Change: one fresh review context reads the declaration, the two scenarios,
    the trial evidence, and the arc42 chapters, and writes `review.md` with the
    markers the gate reads: `Review kind: independent`; `Author:` and
    `Reviewer context:` copied byte for byte from `design-review.md`;
    `README-IMPACT:` as one of the two allowed values (the author's reading:
    `none`, because the README documents commands and not the declaration or a
    toolchain version; update it and record `updated - <sections>` if a README
    statement is made false by the pin); `## Overlap dispositions` with
    `variant` for each of the two added scenarios and the distinct invariant;
    `## Split-or-keep` with `keep` and the reason for each; the final
    `REVIEW: PASS` line outside fences and comments.
  - Acceptance: the review names the five former specification defects'
    resolution, the Neighbor record, the clean-room scan, the unpromoted trial,
    and the merged-step closeout (every phrase bound by the delta features is
    bound in the permanent `tests/steps/` tree, which holds both new bindings).
    Its checklist records the design's primitive inventory: the change adds no
    production primitive, and the two new bindings and the shared fixture are
    listed in `design.md` `## Step Definitions` and `## Delta Step Bindings`. If
    the reviewer judges the missing `## Internal Primitives` section a defect,
    update `design.md`, re-run design-review, and record the re-read; do not
    tick the checklist item without that record. No finding is excused with an
    invented classification.
  - Design constraints: the proposal's `## Review Context` (independent review,
    one fresh context owns design review and final review); served `review`
    instruction; `givn-review-gate-artifact-formats` for the exact marker
    formats.
  - [ ] COMMIT: `___` -- Batch: verify-and-review

- [ ] Record the review gate
  - Paths: `evidence/ise/boundary-review-check.txt`.
  - Run:
    ```sh
    set -o pipefail
    givn check review --change givn-self-rebuild-under-the-updated-rules 2>&1 \
      | tee givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/boundary-review-check.txt
    ```
  - Acceptance: exit 0; the transcript prints `✓ integrity checks passed`,
    `✓ run declaration passed`, the scoped lint with no `[WIP ]` and no error,
    the overlap and routing checks, and the change receipt check. Commit the
    transcript together with `review.md`. A refusal is repaired in the artifact
    it names; a `design.md` edit reopens design-review.
  - Design constraints: design `## Verification` item 7; the review gate runs
    integrity and evidence checks, not the suites.
  - [ ] COMMIT: `___` -- Batch: verify-and-review

## Batch: `archive-and-dogfood`

Ordering: `givn archive` refuses an incomplete task list
(`archive blocked: tasks not complete`) and refuses a dirty tree outside
evidence paths, so the three review ticks and both archive boxes are committed
as C7 before the command runs. The two archive `COMMIT:` lines cite C6 at that
point, because C8 and C9 do not exist yet and a checked line may not carry a
placeholder. C8 is the archive's own candidate revision. C9 carries the
self-dogfood transcript and corrects the archive task's identifier to C8; C10
corrects this task's identifier to C9.

- [ ] Archive the change
  - Paths: `givn/archive/givn-self-rebuild-under-the-updated-rules/`;
    `givn/specs/declare-the-specification/specification-declaration.feature`,
    `givn/specs/prepare-an-ise-test/clean-room-preparation.feature`.
  - Change: tick the three `verify-and-review` tasks with their `COMMIT:` lines
    citing C6, tick both tasks of this batch with their lines citing C6, and
    commit the task file alone. Nothing else may be dirty.
  - Run:
    ```sh
    givn commit givn/changes/givn-self-rebuild-under-the-updated-rules/tasks.md \
      -m 'chore(givn-self-rebuild-under-the-updated-rules): check the archive batch before publication

Batch: archive-and-dogfood'
    givn archive --change givn-self-rebuild-under-the-updated-rules
    ```
  - Acceptance: the tick commit is C7 and the tree is clean; `givn archive`
    exits 0 and publishes C8 with the move to `givn/archive/<id>`, the merged
    delta, and the archive receipt; the two merged scenarios keep their titles
    with no `@wip`; both scopes run against C8 and report 0 failed and 0
    skipped; the coverage gate runs `./measure-coverage.sh` and
    `./merge-coverages.sh` over the regular suite and passes; no trial file
    enters the checkout; the spec copy, the manifest, and the rebuilt binary
    remain outside it; nothing is installed, tagged, released, or promoted.
  - Design constraints: D7 (a `Dry-Run Ise` trial promotes nothing); the
    merged-step rule (the delta adds no step file, so every new phrase is
    already bound in the permanent `tests/steps/` tree); `S14` owns the
    promotion decision.
  - [ ] COMMIT: `___` -- Batch: archive-and-dogfood

- [ ] Record the post-archive suites and the self-dogfood transcript
  - Paths: `givn/archive/givn-self-rebuild-under-the-updated-rules/evidence/ise/self-dogfood.txt`.
  - Run:
    ```sh
    set -o pipefail
    { cargo install --path . --locked --force
      givn skills sync
      givn instructions
      ./run-tests.sh
      ./run-tests.sh --e2e --package e2e
    } 2>&1 | tee givn/archive/givn-self-rebuild-under-the-updated-rules/evidence/ise/self-dogfood.txt
    givn commit -m 'chore(givn-self-rebuild-under-the-updated-rules): record the post-archive suites and the self-dogfood

Batch: archive-and-dogfood'
    givn commit givn/archive/givn-self-rebuild-under-the-updated-rules/tasks.md \
      -m 'chore(givn-self-rebuild-under-the-updated-rules): record the post-archive revision identifiers'
    ```
  - Acceptance: every command exits 0; the installed `$HOME/.cargo/bin/givn`
    reports the checkout's `Cargo.toml` version; the permanent suite runs the
    two merged scenarios and both scopes report 0 failed and 0 skipped; the
    transcript commit is C9; the final tick commit is C10 and sets this task's
    `COMMIT:` line to C9 and the archive task's line to C8, so the archived
    file names the revisions that hold each task's evidence and
    `git status --porcelain` prints nothing.
  - Design constraints: `AGENTS.md` `## Self-dogfood after each change` and
    design `## Verification` item 8; the sequence runs after archive so the
    installed binary and the regenerated skills are current before the next
    change.
  - [ ] COMMIT: `___` -- Batch: archive-and-dogfood

---

**Done when:** every box is checked with its batch commit, `givn check tasks`,
`givn check review`, and `givn archive` exit 0, and both
`./run-tests.sh --non-e2e` and `./run-tests.sh --e2e --package e2e` exit 0 on
the post-archive revision.
