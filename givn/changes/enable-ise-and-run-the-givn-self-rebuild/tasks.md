# Tasks: enable-ise-and-run-the-givn-self-rebuild

Procedure task list. The change adds no product behavior, no delta scenario, and
no step body: `.givn-skip` lists `specs`, `proposal.md` declares
`Behavior change: none`, and `design.md` D4 records why. The list executes
`design.md` (`## Exact Commands`, `## Clean-Room Builder Boundary`,
`## Rebuild Boundary`, `## Manifest and Checksum Expectations`, `## Gap Triage`,
`## Evidence Artifacts`, `## Verification`).

## Setup and runner

No runner setup task is authored. `design.md` `## Strict Mode` records that
strict mode is inherited and that no step body is added, so the
proof-of-strictness procedure has nothing to prove here; a probe would add a
step file the trial's rebuilt Source must reproduce, and no permanent scenario
change belongs to this change.

| Contract | Value | Source |
|---|---|---|
| `verify.command` | `./run-tests.sh --non-e2e` | `givn/commands.yaml` |
| `verify.e2e_command` | `./run-tests.sh --e2e --package e2e` | `givn/commands.yaml` |
| Regular strictness | `.fail_on_skipped()` | `tests/features_runner.rs` |
| End-to-end strictness | `.fail_on_skipped()`, exit 1 when `failed > 0`, `skipped > 0`, or `total == 0` | `e2e/tests/e2e_runner.rs` |
| `ise` effectiveness | `addons.ise: true`; the change lock records `ise: true` | `givn/config.yaml`, `givn/changes/enable-ise-and-run-the-givn-self-rebuild/config-lock.yaml` |
| Rebuild command | `givn ise rebuild-all --at <revision> --into <dir> --manifest <file>` | `givn ise rebuild-all --help` |

**Commit discipline.** One `givn commit` per batch. Immediately after a batch
commit, set that batch's boxes and write its revision identifier into every
`COMMIT:` line of the batch; that tick edit is committed with the next batch.
The pre-trial commit is the trial revision, so its own ticks and
`trial-revision.txt` are committed after the run: `design.md` `## Exact
Commands` states that `trial-revision.txt` cannot belong to the revision it
names, and writing it before `givn ise rebuild-all` would dirty the checkout
that command refuses. A tick line never carries a placeholder once set.

**Placeholders.** Every `COMMIT:` line starts with the placeholder `___`. A
checked line with a placeholder, with a revision that does not exist, or with a
revision not reachable from `HEAD` fails the shared gates; fill the real
identifier, never a reword.

## Batch map

| Batch | Tasks | Commit | Purpose |
|---|---|---|---|
| `pre-trial-inputs` | 5 | C1 | Boundary transcript, package independence, coverage export and inventory, the `S12` prerequisite gate, and the pre-trial commit. |
| `spec-copy` | 3 | C2 | Write the copy, prove the rebuild boundary, compare before the build. |
| `clean-room-build` | 4 | C2 | Registry scan, sandbox, the fresh-agent rebuild, the after-build comparison and version. |
| `frozen-acceptance` | 1 | C2 | Run the frozen `e2e` package against the rebuilt binary. |
| `gap-triage` | 4 | C2 | One outcome per difference, both repair routes, the disclosed-input row. |
| `evidence-commit` | 2 | C2 | `trial-revision.txt`, the evidence commit, both suite scopes on the final revision. |
| `review-and-archive` | 2 | C3 | The review-time boundary transcript (committed with `review.md`) and `givn archive`. |
| `self-dogfood` | 2 | C4 | The corrected `AGENTS.md` command and the post-archive self-dogfood transcript. |
| **Total** | **23** | | |

C1 is the trial revision; C2, C3, and C4 are later commits. C2 carries the four
trial batches, which is allowed: a shared batch revision is valid when every
line names its `Batch:`.

## Trial inputs at the authoring revision

Every value is recomputed at the trial revision; these are the authoring
values, used only as a sanity check.

| Measure | Value | Resolve with |
|---|---|---|
| Tracked `src/**` and `tests/**` files | 187 | `git ls-tree -r --name-only <rev> -- src tests \| wc -l` |
| Tracked `e2e` package files | 100 | `git ls-tree -r --name-only <rev> -- e2e \| wc -l` |
| Tracked `givn/specs/**` files | 85 | `git ls-tree -r --name-only <rev> -- givn/specs \| wc -l` |
| Declared documents plus the declaration | 5 | `docs/glossary.md`, `docs/domain-model.md`, `docs/reference.md`, `docs/arc42/02-architecture-constraints.md`, `givn/specification.yaml` |
| Specification files | 90 | 85 behavior files plus the 5 above |
| Feature paths carrying a selected End-to-end scenario | 56 | `givn spec coverage --format json` |
| Selected End-to-end scenarios | 115 | `givn spec coverage --format json` |
| `end_to_end_files` manifest entries | 156 | 100 package files plus 56 feature paths |
| `assets/**` files | 95 | `git ls-tree -r --name-only <rev> -- assets \| wc -l` |
| `assets/**` tracked-blob bytes | 491,382 | `git ls-tree -r -l <rev> -- assets` |

The `assets/**` figure is the tracked-blob total. The `du -sk` value names disk
usage, not content size, and is not recorded.

## Interaction coverage pre-flight

The three owning use-case documents hold five inventory entries; each has a row
in `design.md` `## Interaction Coverage Matrix` and an existing End-to-end
scenario in the permanent corpus. No delta feature is authored, so no matrix
row depends on a new scenario.

| Inventory entry (owning document) | Matrix row | Permanent scenario and tag |
|---|---|---|
| `clean-room-preparation`: write the spec copy (`prepare-an-ise-test/usecase.md`) | yes | `Developer rebuilds the whole product from its Specification` (`givn/specs/prepare-an-ise-test/clean-room-preparation.feature:231`, `@e2e` at line 230) |
| `trial-input-verification`: compare the copy and the checkout (`prepare-an-ise-test/usecase.md`) | yes | `Verifier compares the spec copy with the manifest` (`givn/specs/prepare-an-ise-test/trial-input-verification.feature:5`, `@e2e` at line 4) |
| `specification-declaration`: declare the five roles and the source roots (`declare-the-specification/usecase.md`) | yes | `The declared roles are read by the project-health pass` (`givn/specs/declare-the-specification/specification-declaration.feature:5`, `@e2e` at line 4) |
| `change-strategy`: read the design guidance (`create-change/usecase.md`) | yes | `The design guidance requires one recorded change strategy` (`givn/specs/create-change/change-strategy.feature:5`, `@e2e` at line 4) |
| `spec-gap`: read the review guidance (`create-change/usecase.md`) | yes | `The review guidance requires one spec gap outcome before any fix` (`givn/specs/create-change/spec-gap.feature:5`, `@e2e` at line 4) |

No gap: every inventory entry has a matrix row and an existing tagged scenario.
`create-change`'s other inventory rows are not touched by this change.

## Domain constraints

Copied from the owning use-case documents and the proposal; every task below
carries the rows it needs.

| Capability | Guarantee or rule | Persona |
|---|---|---|
| `clean-room-preparation` | The copy is a plain directory outside the checkout, written from the selected revision and never the working tree; no End-to-end test enters it, and the checkout is never written, including the manifest. | `developer--primary-user`, `givn-maintainer--owner` |
| `clean-room-preparation` | Every refusal is checked before the first write, names its condition and its subject, exits 1, and leaves the checkout and every target unchanged; a usage error exits 2. | `agent--technical-role` |
| `clean-room-preparation` | A Specification role match wins over a `source:` root, the declaration is an implicit Specification entry counted once, and an End-to-end match is neither copied nor removed. | `givn-maintainer--owner` |
| `clean-room-preparation` | The copy carries no `Cargo.toml`, `Cargo.lock`, `assets/**`, toolchain, or crate cache: the declaration names none of them (D7). The builder supplies them, and the trial records that they are not derivable from the Specification. | `agent--technical-role` |
| `trial-input-verification` | The comparison prints two subject lines, names every difference once in one run, and exits 0 only when both subjects say `match`. An added file under a declared Specification path is refused. | `developer--primary-user`, `agent--technical-role` |
| `specification-declaration` | The declaration holds exactly five roles and one `source:` list; the rebuild commands read the revision's blob and refuse a missing, unblobbed, non-UTF-8, or malformed declaration by name. | `agent--technical-role` |
| `change-strategy` | Exactly one `Decides D` entry records the change strategy, the affected consumers, the persisted state, and the rebuild boundary. | `developer--primary-user` |
| `spec-gap` | One difference receives exactly one outcome before any fix; only `specification defect` is a gap in the Specification; an `E2E test defect` stops the Ise Test until the test is corrected. | `givn-maintainer--owner` |

Carried by every task: no Rust module, struct, or field name appears in
user-facing output; exit 0 is success or match, 1 is a refusal or difference, 2
is a usage error; the spec copy, the manifest, and the rebuilt binary are
evidence, never acceptance authority; nothing is promoted and `S14` owns that
decision.

---

## Batch: pre-trial-inputs

- [x] Prove the declared End-to-end package boundary on Givn itself
  - Paths: `givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/boundary-check.txt`
  - Run:
    ```sh
    set -o pipefail
    givn check tasks --change enable-ise-and-run-the-givn-self-rebuild 2>&1 \
      | tee givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/boundary-check.txt
    ```
  - Acceptance: exit 0. The transcript holds `Mandatory checks for 'tasks':`,
    the expected `⚠ task preflight: review.md is missing` warning, the
    declared-package preflight with no refusal, and
    `changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)`.
    Append one line to the transcript stating that its `✓ verify passed` line is
    the not-applicable path's line and is not `verify.command` evidence. This is
    not a clean preflight: `review.md` does not exist yet.
  - Design constraints: D6. `src/cli/check.rs` returns on the not-applicable
    path before `write_change_receipt`, so no `verification/tasks-verification.json`
    exists and this transcript is the boundary evidence. The declared-package
    preflight is `preflight_e2e_boundary`.
  - [x] COMMIT: `9822ffbb29effb0de320789c2882f9a6915714b0` -- Batch: pre-trial-inputs

- [x] Prove the frozen package reaches no product crate
  - Paths: `givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/e2e-dependencies.txt`
  - Run:
    ```sh
    set -o pipefail
    cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e 2>&1 \
      | tee givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/e2e-dependencies.txt
    ```
  - Acceptance: exit 0; the transcript names no `givn`, `givn-config`, or
    `givn-core` package: `grep -E '(givn|givn-config|givn-core) v' <transcript>`
    prints nothing. The `e2e` package is its own workspace root.
  - Design constraints: D5 (the frozen package is authority outside the trial);
    the `e2e` package links no product crate and the runner refuses a product
    package in `e2e/Cargo.lock`.
  - [x] COMMIT: `9822ffbb29effb0de320789c2882f9a6915714b0` -- Batch: pre-trial-inputs

- [x] Export the trial inputs and author the scenario inventory
  - Paths: `evidence/ise/spec-coverage.json`, `evidence/ise/e2e-inventory.md`
  - Run:
    ```sh
    givn spec coverage --format json \
      > givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/spec-coverage.json
    ```
  - Acceptance: the JSON parses; the selected set counts 115 entries and 56
    distinct feature paths at the authoring revision:
    ```sh
    jq '[.entries[] | select(.e2e)] | length' <spec-coverage.json>                        # 115
    jq '[.entries[] | select(.e2e) | .feature_path] | unique | length' <spec-coverage.json> # 56
    ```
    `e2e-inventory.md` holds one row per selected entry with the capability, the
    feature path, and the scenario title, and its row count equals the coverage
    count. The trial revision recomputes both values.
  - Design constraints: D4 (no delta) and D5 (the frozen authority). The
    inventory total is what `S14` compares, not the plan's older 108 or 119.
  - [x] COMMIT: `9822ffbb29effb0de320789c2882f9a6915714b0` -- Batch: pre-trial-inputs

- [x] Verify the `S12` evidence row exists and is recorded
  - Paths: `givn/ideation/ise-rebuildability/plan.md` (§0 and the phase table);
    `evidence/ise/spec-gaps.md` (`## Setup failures`) only when the row is absent.
  - Steps: read §0's `Outstanding` list and the `S12` phase row; resolve the
    evidence `S12` names (the trial record, both `givn ise verify` outputs, the
    frozen suite result, the grade, and the Spec gap records).
  - Acceptance: §0 no longer records `S10` to `S12` as blocked or outstanding,
    and a `Done` row or an explicit evidence pointer records `S12` delivered.
    Paste the resolving lines into this task.
  - If the row is absent: write `S12 evidence absent` under
    `evidence/ise/spec-gaps.md` `## Setup failures` with the date and the
    `plan.md` lines read. The trial is blocked, never skipped; do not run
    `givn ise rebuild-all`.
  - Override applied 2026-11-17: the operator's S13 execution instruction
    starts the trial with the row outstanding, recorded in `proposal.md`
    `## Operator Override: S12 Prerequisite`. On this run the absence is
    recorded under `## Setup failures` and the four trial batches proceed
    under that override. `S12` is not satisfied and is not claimed as evidence.
  - Design constraints: the plan's phase table gates `S13` on `S3`, `S4`, `S5`,
    and `S12`; §0 records `S3` to `S5` delivered and `S10` to `S12`
    outstanding as of 2026-11-16. A missing prerequisite is a setup failure, not
    a Spec gap.
  - [x] COMMIT: `9822ffbb29effb0de320789c2882f9a6915714b0` -- Batch: pre-trial-inputs

- [x] Commit the planning layer and the pre-trial inputs, and resolve the trial revision
  - Paths: `givn/changes/enable-ise-and-run-the-givn-self-rebuild/` (proposal,
    design, design-review, arc42, tasks, `.givn-skip`, `config-lock.yaml`, and the
    four evidence files above), `docs/arc42/02-architecture-constraints.md`,
    `docs/arc42/04-solution-strategy.md`,
    `docs/arc42/11-risks-and-technical-debt.md`,
    `givn/ideation/ise-rebuildability/changes/enable-ise-on-givn-and-run-a-dry-run.md`
    (the seed status).
  - Run:
    ```sh
    givn commit givn/changes/enable-ise-and-run-the-givn-self-rebuild \
      docs/arc42/02-architecture-constraints.md \
      docs/arc42/04-solution-strategy.md \
      docs/arc42/11-risks-and-technical-debt.md \
      givn/ideation/ise-rebuildability/changes/enable-ise-on-givn-and-run-a-dry-run.md \
      -m 'docs(enable-ise-and-run-the-givn-self-rebuild): plan the change with the trial inputs

Batch: pre-trial-inputs'
    git rev-parse HEAD
    ```
  - Acceptance: the commit holds the change directory and the three arc42
    chapter edits; the tree is clean afterward (`git status --porcelain` prints
    nothing), which `givn ise rebuild-all` requires. The 40-character revision
    printed by `git rev-parse HEAD` is the trial revision carried into every
    command below. Do not write `evidence/ise/trial-revision.txt` yet: it would
    dirty the checkout before the rebuild reads it, and it cannot belong to the
    revision it names. The `evidence-commit` batch writes it.
  - Design constraints: D1 (the trial reads this commit, passed explicitly);
    D6 (the enablement is already recorded and this change rewrites neither
    `givn/config.yaml` nor the change lock).
  - [x] COMMIT: `9822ffbb29effb0de320789c2882f9a6915714b0` -- Batch: pre-trial-inputs

## Batch: spec-copy

- [ ] Write the spec copy from the trial revision
  - Paths: `evidence/ise/rebuild-report.txt`, `evidence/ise/manifest.yaml`; the
    copy and the manifest stay outside the checkout at
    `/tmp/givn-ise-self-rebuild-<stamp>` and `/tmp/givn-ise-self-rebuild-<stamp>.yaml`,
    where `<stamp>` is `date -u +%Y%m%dT%H%M%SZ`.
  - Run:
    ```sh
    set -o pipefail
    givn ise rebuild-all \
      --at <revision> \
      --into /tmp/givn-ise-self-rebuild-<stamp> \
      --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml 2>&1 \
      | tee givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/rebuild-report.txt
    cp /tmp/givn-ise-self-rebuild-<stamp>.yaml \
      givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/manifest.yaml
    ```
  - Acceptance: exit 0; the report holds the eleven fixed lines plus
    `Manifest:`; `Revision:` equals the trial revision; `Source code kept: 0`;
    `Source code removed:` equals
    `git ls-tree -r --name-only <revision> -- src tests | wc -l` (187 at the
    authoring revision); `Specification files:` equals the manifest's
    `specification_files` count (90 at the authoring revision);
    `End-to-end tests: not copied (<n> files)` names the `e2e` package count
    (100 at the authoring revision); `Copy directory:` and `Manifest:` equal the
    two targets. The manifest is `ise-manifest/v1` with every field; its
    `revision`, counts, `copy_directory`, and `givn_version` agree with the
    report and with `givn --version`; every checksum is 64 lowercase hex and
    both rollups recompute from the per-path maps, the scenario checksum
    winning for a path in both maps. A disagreement, a usage error, or a
    refusal is a setup error: record it under `spec-gaps.md` `## Setup
    failures` and do not continue.
  - Design constraints: D1 (explicit `--at`); D5 (whole-product rebuild,
    `Dry-Run Ise`, every tracked file under `src/**` and `tests/**` removed);
    D7 (the five withheld build inputs are absent from the copy); `--dry-run` is
    not passed, because it writes no copy for a builder.
  - [ ] COMMIT: `___` -- Batch: spec-copy

- [ ] Verify the rebuild boundary in the spec copy
  - Paths: the copy only.
  - Acceptance (all hold before the builder starts):
    ```sh
    test ! -e <copy>/src
    test ! -e <copy>/tests
    test ! -e <copy>/e2e
    test ! -e <copy>/.git
    test -f <copy>/givn/specification.yaml
    find <copy> -name '*.rs' -o -name Cargo.toml      # prints nothing
    find <copy> -type f | wc -l                         # equals the report's Specification files value
    ```
    The declaration and the five matrix feature paths are present, and the copy
    holds no `Cargo.toml`, `Cargo.lock`, `assets/**`, or End-to-end file.
  - Design constraints: D5's rebuild-boundary table (Specification files
    copied unchanged; no Source code kept; every `src/**` and `tests/**` file
    removed; the `e2e` package never copied; `Cargo.toml`, `Cargo.lock`, and
    `assets/**` outside the copy).
  - [ ] COMMIT: `___` -- Batch: spec-copy

- [ ] Compare the spec copy with the manifest before the build
  - Paths: `evidence/ise/compare-before.txt`
  - Run:
    ```sh
    set -o pipefail
    givn ise compare --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml \
      --copy /tmp/givn-ise-self-rebuild-<stamp> 2>&1 \
      | tee givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/compare-before.txt
    ```
  - Acceptance: exit 0; `Specification files: match`; `End-to-end tests: match`;
    no `+ <path>` and no `- <path>` line.
  - Design constraints: the comparison prints two subject lines, names every
    difference once, and exits 0 only when both say `match`; a file added under
    a declared Specification path is refused before any count line.
  - [ ] COMMIT: `___` -- Batch: spec-copy

## Batch: clean-room-build

- [ ] Prove the cargo cache holds no product source or archive
  - Paths: the transcript goes into `evidence/ise/builder-notes.md` (the builder
    records its inputs there).
  - Run:
    ```sh
    find "$HOME/.cargo/registry/src" "$HOME/.cargo/registry/cache" \
      "$HOME/.cargo/registry/git" -maxdepth 2 -name 'givn-*' -print
    ```
  - Acceptance: prints nothing, because a published crate is a copy of the
    Source code. If it prints a path, remove that path from the cache or use a
    fresh `CARGO_HOME` populated from the required crates only, rerun the scan,
    and restart from the sandbox task. Hand the command and its output to the
    builder.
  - Design constraints: D7 and the clean-room boundary: `registry/cache` holds
    `.crate` archives and `registry/git` holds git checkouts, so all three roots
    are scanned. The registry is read-only in the sandbox and populated before
    the sandbox starts by the host build against the checkout's lockfile.
  - [ ] COMMIT: `___` -- Batch: clean-room-build

- [ ] Start the builder sandbox with the probed mount set
  - Paths: `evidence/ise/builder-notes.md`; the copy mounted at `/work`.
  - Acceptance: the sandbox starts with the exact `bwrap` invocation from
    `design.md` `## Clean-Room Builder Boundary`; inside it `cargo --version`
    and `rustc --version` exit 0; `/work` is the copy; `/work/e2e`,
    `/work/src`, `/work/tests`, and `/work/.git` do not exist; `command -v givn`
    prints nothing; the three rustup shims, `~/.rustup`, `registry`, and `git`
    are read-only; `--unshare-all` leaves no network. Save the exact command in
    `builder-notes.md`.
  - Design constraints: D3 (the builder's workspace is the copy; the checkout,
    `.git`, the installed binary, and `e2e/` are not builder inputs); the
    builder's own file tools run outside the sandbox, so the mount set and the
    builder notes are the control, and the design does not claim the mount
    enforces read isolation.
  - [ ] COMMIT: `___` -- Batch: clean-room-build

- [ ] Rebuild the binary in the copy from the Specification and the disclosed inputs
  - Paths: the copy only; `evidence/ise/builder-notes.md`.
  - Acceptance: the builder authors `Cargo.toml`, `Cargo.lock`, `src/**`, and
    `assets/**` from the Specification plus the five disclosed inputs, and any
    other file the build needs; it modifies or deletes no file the copy was
    written with; `cargo build --release --offline` exits 0 inside the sandbox;
    `<copy>/target/release/givn --version` runs. `assets/**` is 95 files and
    491,382 tracked-blob bytes at the authoring revision; the trial revision
    recomputes it. `builder-notes.md` records the sandbox command, the registry
    scan and its output, every file and source read or authored, every crate
    version chosen, the build command, the test-infrastructure inputs the
    verifier later reads from the checkout (`GIVN_READY_FIXTURE`, `GIVN_ROOT`),
    and the statement that the checkout, `.git`, and `e2e/` were not read. A
    crate missing from the offline cache is a setup error: record it under
    `spec-gaps.md` `## Setup failures`, refresh the cache outside the sandbox,
    and restart from the registry scan.
  - Design constraints: D7 (five withheld inputs; the trial does not test
    whether they are derivable from the Specification, it records that they are
    not); D3 (the fixture and repository-file assertions are test
    infrastructure, not product construction); the embedded tree is
    `rust_embed`'s `#[folder = "assets/"]` plus five `include_str!` compile
    inputs.
  - [ ] COMMIT: `___` -- Batch: clean-room-build

- [ ] Verify the copy after the build and record the rebuilt version
  - Paths: `evidence/ise/compare-after.txt`, `evidence/ise/rebuilt-version.txt`
  - Run:
    ```sh
    set -o pipefail
    givn ise compare --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml \
      --copy /tmp/givn-ise-self-rebuild-<stamp> 2>&1 \
      | tee givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/compare-after.txt
    { /tmp/givn-ise-self-rebuild-<stamp>/target/release/givn --version
      sha256sum /tmp/givn-ise-self-rebuild-<stamp>/target/release/givn
    } > givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/rebuilt-version.txt
    ```
  - Acceptance: the comparison exits 0 with both subjects `match`; the authored
    `src/**`, `Cargo.toml`, `Cargo.lock`, `assets/**`, and `target/` are not
    differences; no `+ <path>` names a Specification path. A `+ <path>` refusal
    means the tree holds a file under a declared Specification path: remove it,
    rebuild, and rerun the comparison. `rebuilt-version.txt` holds the version
    line and the binary checksum.
  - Design constraints: the comparison allows added source and refuses an added
    Specification path or a deleted kept path; the rebuilt binary is evidence and
    is never installed or promoted.
  - [ ] COMMIT: `___` -- Batch: clean-room-build

## Batch: frozen-acceptance

- [ ] Run the frozen End-to-end package against the rebuilt binary
  - Paths: `evidence/ise/e2e-result.json`
  - Run:
    ```sh
    GIVN_BIN=/tmp/givn-ise-self-rebuild-<stamp>/target/release/givn \
    GIVN_RESULT_FILE=$PWD/givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/e2e-result.json \
    ./run-tests.sh --e2e --package e2e
    ```
  - Acceptance: exit 0; the result file records `total` 120 at the authoring
    revision, `passed` equal to `total`, `failed` 0, and `skipped` 0. The 120
    is 115 selected `@e2e` definitions plus 5 Scenario Outline examples; the
    inventory names the three outlines. The baseline observation on 2026-11-17
    against the checkout binary recorded `total` 120, `passed` 120, `failed` 0,
    `skipped` 0. A `total` other than the recomputed 120 is a setup failure.
    Record the exact command in `spec-gaps.md` under
    `## Trial inputs read from the checkout`, stating that `GIVN_BIN` is the
    rebuilt binary while `GIVN_READY_FIXTURE` and `GIVN_ROOT` come from the
    checkout and that the clean-room claim covers product behavior, not the
    fixture bytes or the checkout files those assertions read.
  - A failed scenario is a difference: continue to `gap-triage` and record it
    before any edit. A zero-scenario result, a skipped result, or a `GIVN_BIN`
    that is unset or not executable is a setup failure: record it under
    `## Setup failures` and never as a Spec gap.
  - Design constraints: D3; the runner selects the tier and the package
    explicitly, refuses a zero-scenario, failed, or skipped run, and writes the
    scoped result file.
  - [ ] COMMIT: `___` -- Batch: frozen-acceptance

## Batch: gap-triage

- [ ] Classify every difference before any edit
  - Paths: `evidence/ise/spec-gaps.md`
  - Acceptance: one row per failing scenario, each with the frozen scenario
    title, the failing step, the observed result, the expected result, the
    rebuilt file, exactly one outcome, the wrong artifact, and the evidence
    path. The outcomes are exactly:

    | Outcome | Wrong artifact | Route |
    |---|---|---|
    | `E2E test defect` | the specific `e2e/**` file | correct the test, re-freeze it, restart at the revision that records the correction |
    | `implementation defect` | the rebuilt file in the spec copy | return to the builder inside the same trial, no corpus or Specification edit |
    | `specification defect` | `givn/specification.yaml`, `docs/reference.md`, the owning `givn/specs/**` use case, or the declaration entry that should have covered the path | record the outcome; the correction belongs to a later change |
    | `acceptable undocumented change` | the rebuilt file, which is not a gap | record the outcome; no edit |

    No row is closed as drift and no row carries two outcomes. A trial with no
    difference states `No difference observed` and names the result file and
    counts.
  - Design constraints: D2 (classify before repair, one outcome and one wrong
    artifact before any edit); `create-change/spec-gap` guidance; only
    `specification defect` is a gap in the Specification.
  - [ ] COMMIT: `___` -- Batch: gap-triage

- [ ] Route each implementation defect back to the builder inside the same trial
  - Paths: the copy; `evidence/ise/spec-gaps.md`,
    `evidence/ise/compare-after.txt`, `evidence/ise/e2e-result.json`.
  - Acceptance: no edit to `givn/specs/**`, `givn/specification.yaml`,
    `docs/**`, or `e2e/**`; the builder corrects the rebuilt file in the copy;
    `cargo build --release --offline` exits 0 again; `givn ise compare` still
    reports both subjects as `match`; the frozen package is rerun and its counts
    are recorded. The row stays `implementation defect` and names the rebuilt
    file.
  - Design constraints: D2's `implementation defect` route; D7 (a defect in the
    builder's authored file is repaired in the copy, never by changing the
    Specification).
  - [ ] COMMIT: `___` -- Batch: gap-triage

- [ ] Restart the trial after an `E2E test defect`
  - Paths: the corrected `e2e/**` file; regenerated
    `evidence/ise/e2e-inventory.md` and `evidence/ise/spec-coverage.json`; a new
    `/tmp` copy and manifest; `evidence/ise/trial-revision.txt`.
  - Acceptance: correct the frozen test and commit it together with the first
    attempt's evidence and the regenerated inventory and coverage; that commit
    is the new trial revision; rerun `rebuild-all` with `--at <new revision>`
    into a new copy and manifest; the manifest's `revision`, `removed_paths`,
    `specification_files`, `end_to_end_files`, and checksums are recomputed at
    the new revision; the first attempt's evidence stays reachable in history.
    `trial-revision.txt` names the new revision. When no difference is an
    `E2E test defect`, record `No E2E test defect` in `spec-gaps.md` under
    `## Spec gaps` and complete this task with that note; never leave the box
    open and never run a restart that the evidence does not justify.
  - Design constraints: the `prepare-an-ise-test` rule that a mid-rebuild test
    defect stops the rebuild as a `Spec gap: E2E test defect` and the rebuild
    restarts after the test is corrected; a scenario is frozen only after it was
    observed failing against the pre-change implementation for the intended
    reason; D1's restart rule.
  - [ ] COMMIT: `___` -- Batch: gap-triage

- [ ] Record the disclosed-input specification defect
  - Paths: `evidence/ise/spec-gaps.md`
  - Acceptance: exactly one `specification defect` row naming
    `givn/specification.yaml` as the wrong artifact and the surface the builder
    supplied: `Cargo.toml`, `Cargo.lock`, `assets/**`, the Rust toolchain, and
    the cargo registry cache. The row is recorded even when the frozen suite is
    green, because the undisclosed inputs are a gap in the Specification and not
    an End-to-end difference.
  - Design constraints: D7 (the trial records that the five inputs are not
    derivable from the Specification); the row names the artifact that should
    declare the surface, never who found it.
  - [ ] COMMIT: `___` -- Batch: gap-triage

## Batch: evidence-commit

- [ ] Record the trial revision and commit the trial evidence
  - Paths: `givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/`
    (every file the trial produced, `trial-revision.txt` included), the corrected
    test if an `E2E test defect` occurred, and `tasks.md` ticks.
  - Run:
    ```sh
    printf 'revision: <revision>\ncommit: <subject>\ngit rev-parse HEAD\ngit ls-tree -r --name-only <revision> -- src tests | wc -l\n' \
      > givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/trial-revision.txt
    givn commit givn/changes/enable-ise-and-run-the-givn-self-rebuild \
      -m 'test(enable-ise-and-run-the-givn-self-rebuild): record the self-rebuild trial'
    ```
  - Acceptance: `trial-revision.txt` holds the 40-character revision, the commit
    subject, and the commands that resolve it; the commit holds every evidence
    file, the corrected test, and the ticks with their real revision
    identifiers; the tree is clean after the commit except the ignored
    `givn/history/`.
  - Design constraints: D1 (the revision is recorded explicitly and cannot
    live in the revision it names); D2 and the proposal's evidence list.
  - [ ] COMMIT: `___` -- Batch: evidence-commit

- [ ] Verify both suite scopes on the final revision
  - Paths: none; this gate runs the configured commands.
  - Run:
    ```sh
    ./run-tests.sh --non-e2e
    ./run-tests.sh --e2e --package e2e
    ```
  - Acceptance: both exit 0 with no failed and no skipped scenario. The second
    command names the tier and the package explicitly; `--e2e` alone is refused
    with exit 2.
  - Design constraints: `design.md` Verification item 9; the two commands are
    `verify.command` and `verify.e2e_command`; the change adds no production
    code, so the suite protects existing behavior and no new scenario is
    required.
  - [ ] COMMIT: `___` -- Batch: evidence-commit

## Batch: review-and-archive

- [ ] Produce the review-time boundary transcript
  - Paths: `evidence/ise/boundary-review-check.txt`
  - Precondition: `review.md` exists with `Review kind:`, `Reviewer context:`,
    the actual verdict, `README-IMPACT:`, and `PERSONAS:`.
  - Run:
    ```sh
    set -o pipefail
    givn check review --change enable-ise-and-run-the-givn-self-rebuild 2>&1 \
      | tee givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/boundary-review-check.txt
    ```
  - Acceptance: exit 0; the transcript repeats the declared-package preflight
    and prints
    `changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)`.
    Commit the transcript together with `review.md`.
  - Known blocker, verified on 2026-11-17: `givn check review` currently fails
    on `design.md` `## Step Definitions` with
    ``integrity check failed: `## Step Definitions` section must use a markdown table (found prose/backticks instead)``.
    The section states that no step definition is added but names neither `n/a`
    nor `none` and carries no table. Correct the section in `design.md`, then
    re-run design-review (a `design.md` edit reopens it) before this task can
    pass.
  - Design constraints: D6 (the not-applicable path writes no receipt; the
    boundary preflight is the acceptance check review can make); the transcript
    is boundary evidence, not suite evidence.
  - [ ] COMMIT: `___` -- Batch: review-and-archive

- [ ] Run `givn archive` and record the receipts
  - Paths: `givn/archive/enable-ise-and-run-the-givn-self-rebuild/`
  - Run:
    ```sh
    givn archive --change enable-ise-and-run-the-givn-self-rebuild
    ```
  - Acceptance: exit 0; the archive holds `verification.json` and the archive
    result files; `givn/specs/**` is unchanged (specs is skipped, so no
    feature is copied or merged); the coverage gate runs
    `./measure-coverage.sh` and `./merge-coverages.sh` over the regular suite
    and passes; the spec copy, the manifest, and the rebuilt binary remain
    outside the checkout; the archive result is committed.
  - Run the commit with no path so the change-directory deletion and the new
    archive directory stage together:
    ```sh
    givn commit -m 'docs(enable-ise-and-run-the-givn-self-rebuild): archive the change with the trial evidence'
    ```
  - Design constraints: `specs` is skipped (`.givn-skip`) and the proposal
    declares `Behavior change: none`; no trial file enters the checkout except
    the recorded evidence; nothing is installed, tagged, or released.
  - [ ] COMMIT: `___` -- Batch: review-and-archive

## Batch: self-dogfood

- [ ] Correct the stale self-dogfood command in `AGENTS.md`
  - Paths: `AGENTS.md` `## Self-dogfood after each change`.
  - Change: `./run-tests.sh --e2e` becomes
    `./run-tests.sh --e2e --package e2e`. `run-tests.sh` refuses the former with
    exit 2 and `--e2e names the tier, not the package`.
  - Acceptance: the section's command list runs as written; the next task
    exercises it. `givn commit` never stages `AGENTS.md`, so stage and commit it
    with `git add AGENTS.md && git commit -m '<subject>'`; the checked-in
    commit-msg hook stamps the attribution.
  - Design constraints: `design.md` Verification item 10 with the
    design-review advisory applied; `AGENTS.md` is project-owned content outside
    the generated `givn` block, so no served guidance changes and no delta
    scenario belongs to this edit.
  - [ ] COMMIT: `___` -- Batch: self-dogfood

- [ ] Run the self-dogfood sequence and commit its transcript
  - Paths: `givn/archive/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/self-dogfood.txt`
  - Run, from the corrected `AGENTS.md`:
    ```sh
    set -o pipefail
    { cargo install --path . --locked --force
      givn skills sync
      givn instructions
      ./run-tests.sh
      ./run-tests.sh --e2e --package e2e
    } 2>&1 | tee givn/archive/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/self-dogfood.txt
    ```
  - Acceptance: every command exits 0; the installed binary reports the
    checkout's `Cargo.toml` version; the transcript is committed as a
    post-archive maintenance commit under the archived change.
  - Design constraints: `design.md` Verification item 10 and the post-archive
    record rule; the sequence runs after archive, so the installed binary and
    the regenerated skills are current before the next change starts.
  - [ ] COMMIT: `___` -- Batch: self-dogfood
