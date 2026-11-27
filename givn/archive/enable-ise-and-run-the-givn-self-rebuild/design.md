# Design: enable-ise-and-run-the-givn-self-rebuild

## Scope

This change is the rebuild half of `S13`. The boundary half is archived as
`enforce-ise-e2e-boundary`, and the declaration, the `e2e` package, and the
clean-room commands are already in place. The change does four things:

1. Records the `ise` enablement that is already effective on this checkout.
2. Proves the declared E2E package boundary on Givn itself.
3. Runs one whole-product `Dry-Run Ise` from the declared Specification plus
   the non-Specification build inputs disclosed in D7.
4. Records every difference between the frozen End-to-end (E2E) result and the
   rebuilt binary as one `Spec gap` outcome.

No production code and no permanent scenario change. `specs` is skipped
because the change adds no product behavior; `.givn-skip` lists `specs`, and
the proposal declares `Behavior change: none`.

The plan's phase table gates `S13` on `S3`, `S4`, `S5`, and `S12`
(`plan.md:165`); its section 0 prose names only `S12` (`plan.md:67`) and
records `S10` to `S12` as outstanding as of 2026-11-16. `S3` to `S5` are
recorded as delivered in section 0, leaving `S12` outstanding. This change can
be designed and tasked before that evidence exists; the trial step starts only
after the `S12` full-product `Dry-Run Ise` evidence is recorded. The tasks list
names that gate before the trial batch.

| Input | Value |
|---|---|
| Change strategy | `whole-product rebuild` |
| `Ise Test` mode | `Dry-Run Ise` |
| Rebuild command | `givn ise rebuild-all --at <revision> --into <dir> --manifest <file>` |
| Trial revision | the change's pre-trial commit, passed explicitly |
| Rebuild boundary | every tracked file under `src/**` and `tests/**` is removed |
| Frozen authority | the `e2e` package and every `@e2e` feature path |
| Build inputs | `Cargo.toml`, `Cargo.lock`, `assets/**`, the Rust toolchain, and the cargo registry cache, none declared by the Specification; the builder supplies them (D7) |
| Promotion | none; the spec copy and rebuilt binary stay outside the checkout |

## Trial Inputs

The declaration at the authoring revision resolves to these paths:

| Role | Entries | Trial use |
|---|---|---|
| `behavior` | `givn/specs/**` | Copied as Specification files; the `@e2e` feature paths stay in the copy. |
| `structure` | `docs/glossary.md`, `docs/domain-model.md` | Copied as Specification files. |
| `contracts` | `docs/reference.md` | Copied as a Specification file. |
| `limits` | `docs/arc42/02-architecture-constraints.md` | Copied as a Specification file. |
| `e2e` | `e2e` | Frozen; never copied; checksummed. |
| `source` | `src/**`, `tests/**` | Removed from the copy; the builder recreates the Source code. |
| Declaration | `givn/specification.yaml` | Copied as an implicit Specification entry. |

The authoring revision's counts, used as a sanity check against the trial's
report and manifest:

| Measure | Value |
|---|---|
| Tracked `src/**` and `tests/**` files | 187 |
| Tracked `e2e` package files | 100 |
| Tracked `givn/specs/**` files | 85 |
| Declared non-behavior documents plus declaration | 5 |
| Specification files | 90 |
| Feature paths carrying a selected `@e2e` scenario | 56 |
| `@e2e` scenarios in the permanent corpus | 115 |
| `ise-manifest/v1` `end_to_end_files` entries | 156 (100 package files plus 56 feature paths) |

The trial revision recomputes every value. The authoring counts are not
pinned: the change directory and its evidence are outside the declaration and
only move the `Outside the spec copy` count. An `E2E test defect` correction
changes the `e2e` package or feature count, and the trial restarts at the
revision that records it with the counts recomputed. `S14` compares the
recorded `e2e-inventory.md` total at the trial revision, not the plan's older
108 (`plan.md:165,981`) or the S14 table's 119 (`plan.md:1140`); a correction
that retires a scenario changes the total.

## Exact Commands

The trial runs from the checkout after the change's pre-trial commit, which
records the boundary-check transcript, the inventory, the coverage export, the
dependency tree, and every task completed before the trial. `<revision>` is
that commit's 40-character hash. `<stamp>` is a UTC timestamp. `<copy>` and
`<manifest>` must both lie outside the checkout.

`trial-revision.txt` and the post-trial evidence cannot belong to the revision
they name. Commit the pre-trial files first, run the trial, then commit the
evidence and `trial-revision.txt` after the run. A restart after an `E2E test
defect` commits the first attempt's evidence, the corrected test, and the
regenerated inventory and coverage; the restarted trial reads that commit.

```sh
givn ise rebuild-all \
  --at <revision> \
  --into /tmp/givn-ise-self-rebuild-<stamp> \
  --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml
```

`--dry-run` is not passed. The domain mode is `Dry-Run Ise`: the copy is
written, the replacement is built and accepted, and nothing is promoted. The
command's `--dry-run` flag only prints the write plan and would leave no copy
for a builder.

Before the builder starts and after it finishes:

```sh
givn ise compare --manifest <manifest> --copy <copy>
```

The rebuilt binary is built in the copy and accepted from the checkout:

```sh
# inside the copy, under the builder's sandbox
cargo build --release --offline

# from the checkout, after the build
GIVN_BIN=<copy>/target/release/givn \
GIVN_RESULT_FILE=$PWD/givn/changes/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/e2e-result.json \
./run-tests.sh --e2e --package e2e
```

`run-tests.sh` keeps `GIVN_BIN` when it names an executable and builds
`ready_fixture` from the checkout. `GIVN_ROOT` defaults to the checkout. Both
are test infrastructure, not product construction. The E2E runner selects
`@e2e` and refuses a zero-scenario, failed, or skipped run.

## Clean-Room Builder Boundary

The builder is a fresh agent whose workspace is the spec copy; the checkout is
not mounted into that workspace. The agent's commands and reads are recorded in
`builder-notes.md`, and the build runs inside the sandbox below. The concrete
build control is a `bwrap` sandbox. The mount set was probed with
`cargo --version` on 2026-11-17:

```sh
bwrap \
  --unshare-all --die-with-parent \
  --ro-bind /usr /usr --ro-bind /bin /bin --ro-bind /lib /lib \
  --ro-bind /lib64 /lib64 --ro-bind /etc /etc \
  --dev /dev --proc /proc \
  --bind <copy> /work \
  --tmpfs /home \
  --ro-bind "$HOME/.cargo/bin/rustup" /home/builder/.cargo/bin/rustup \
  --ro-bind "$HOME/.cargo/bin/cargo" /home/builder/.cargo/bin/cargo \
  --ro-bind "$HOME/.cargo/bin/rustc" /home/builder/.cargo/bin/rustc \
  --ro-bind "$HOME/.rustup" /home/builder/.rustup \
  --ro-bind "$HOME/.cargo/registry" /home/builder/.cargo/registry \
  --ro-bind "$HOME/.cargo/git" /home/builder/.cargo/git \
  --tmpfs /tmp \
  --setenv HOME /home/builder \
  --setenv RUSTUP_HOME /home/builder/.rustup \
  --setenv CARGO_HOME /home/builder/.cargo \
  --setenv PATH "/home/builder/.cargo/bin:/usr/bin:/bin" \
  --chdir /work \
  <builder-command>
```

The only repository path in the sandbox is `<copy>`, mounted at `/work`. The
builder has no `e2e/`, no `src/` or `tests/` from the checkout, no `.git`, and
no installed `givn` binary: the three rustup shims are bound alone, not the
whole `~/.cargo/bin` directory. It has no network. It reads the cargo registry
cache through a read-only bind, and the cache is populated before the sandbox
starts by the host's existing build against the checkout's lockfile.
`cargo build --offline` is the only resolution mode. A crate missing from the
cache stops the trial as a setup error: the operator refreshes the cache
outside the sandbox and restarts the trial. `builder-notes.md` records the
sandbox command, every source and file the builder read or authored, every
crate version it chose, and the build command.

Before the sandbox starts, the operator checks that the cargo registry holds no
`givn` package source, because a published crate is a copy of the Source code:

```sh
find "$HOME/.cargo/registry/src" "$HOME/.cargo/registry/cache" \
  "$HOME/.cargo/registry/git" -maxdepth 2 -name 'givn-*' -print
```

The command must print nothing. If it prints a path, that path is removed from
the cache before the sandbox starts, or the trial uses a fresh `CARGO_HOME`
populated from the required crates only. `registry/cache` holds the `.crate`
archives and `registry/git` holds git checkouts, so both are scanned with the
extracted sources.

### Non-Specification build inputs

The copy carries neither a package manifest nor the embedded asset tree. The
declaration does not name them, and `docs/reference.md` does not state their
contents. The builder supplies these build inputs from outside the
Specification:

- **`Cargo.toml`.** Withheld from the copy; the builder authors the package,
  binary, and test targets.
- **`Cargo.lock`.** Withheld from the copy; the builder authors the lockfile,
  so every crate version is its choice.
- **`assets/**`.** Withheld from the copy; 95 files, 764 KB. `src/config/embedded.rs`
  embeds the tree with `rust_embed` (`#[folder = "assets/"]`), and five paths
  are `include_str!` compile inputs (`assets/commands.yaml.tmpl`,
  `assets/config.yaml.tmpl`, `assets/run.yaml.tmpl`,
  `assets/timeline/report.html`,
  `assets/agent-integrations/maki/plugin-before-attribution.toml`). The builder
  invents every served instruction, skill, and template the product emits.
- **Rust toolchain.** `rustc`, `rustup`, the edition, and the profile come from
  the sandbox mounts, not from the copy.
- **Cargo registry cache.** Read-only in the sandbox and populated before the
  trial; crate versions resolve from it under `cargo build --offline`.

The trial therefore does not test whether these inputs are derivable from the
Specification; it records that they are not. `builder-notes.md` lists every
file the builder authored and every crate version it chose. `spec-gaps.md`
holds one `specification defect` row against `givn/specification.yaml` for the
withheld surface; the served corpus is product behavior the declaration does
not retain.

The builder must not modify or delete a file the copy was written with. It
creates `Cargo.toml`, `Cargo.lock`, `src/**`, and any other file the build
needs. A file created under a declared Specification path is refused by
`givn ise compare`; the trial restarts after the builder corrects the tree.

## Rebuild Boundary

| Set | Paths | Fate |
|---|---|---|
| Specification files | the four content roles and the declaration | Copied unchanged. |
| Source code kept | a tracked file under a source root that the command did not remove and that is not a Specification file | None expected; `Source code kept: 0`. |
| Source code removed | every tracked file under `src/**` and `tests/**` | Absent from the copy; the builder recreates it. |
| End-to-end (E2E) tests | every tracked file of the `e2e` package, plus the feature paths carrying selected `@e2e` scenarios | The package is never copied; the feature paths are copied as `behavior` files and checksummed as E2E. |
| Outside the spec copy | `Cargo.toml`, `Cargo.lock`, `assets/**`, `README.md`, `CHANGELOG.md`, `givn/**` outside the declaration, undeclared `docs/**`, `.github/**` | Neither copied nor removed. `Cargo.toml`, `Cargo.lock`, and `assets/**` are compile inputs the builder authors because the Specification does not provide them (D7); scenarios that read `README.md` or `CHANGELOG.md` read the checkout through `GIVN_ROOT` (D3). |

The declaration, the `e2e` package, and the checkout stay where they are. No
trial file enters the checkout except the recorded evidence. Nothing is
installed, tagged, or released.

## Manifest and Checksum Expectations

The manifest is `ise-manifest/v1` and carries every field the clean-room
contract names: `scope`, `revision`, `removed_paths`, `specification_files`,
`source_kept_files`, `end_to_end_files`, `counts`, `checksums`,
`specification_checksum`, `end_to_end_checksum`, `copy_directory`, and
`givn_version`.

| Invariant | Check |
|---|---|
| `revision` equals the `--at` value | Compare `trial-revision.txt`, the report's `Revision:` line, and the manifest. |
| `removed_paths` equals every tracked file under `src/**` and `tests/**` at the revision | Compare against `git ls-tree -r --name-only <revision> -- src tests`. |
| `specification_files` equals the union of the four content roles and the declaration, minus `e2e` matches | Compare against `git ls-tree` and the declaration. |
| `source_kept_files` is empty | The report prints `Source code kept: 0`. |
| `end_to_end_files` is the union of the package paths and the selected `@e2e` feature paths | Compare `checksums.end_to_end`, `checksums.scenarios`, and the package's tracked files. |
| `checksums.specification` covers every `specification_files` path | `givn ise compare` reports `Specification files: match`. |
| `checksums.end_to_end` covers every `end_to_end_files` path and `checksums.scenarios` covers every selected `@e2e` feature path | `givn ise compare` reports `End-to-end tests: match`. |
| `specification_checksum` rolls up `checksums.specification` | Recompute the rollup from the manifest. |
| `end_to_end_checksum` rolls up the union of `checksums.end_to_end` and `checksums.scenarios`, scenario checksum winning | Recompute the rollup from the manifest. |
| `copy_directory` is the `--into` path | The report's `Copy directory:` line matches. |
| `givn_version` is the running binary's version | `givn --version` matches. |

The rebuild command records the eleven fixed report lines, plus `Manifest` when
`--manifest` is given, before the builder starts:

```text
Backend: git
Revision: <revision>
Specification files: 90
Source code kept: 0
Source code removed: 187
Outside the spec copy: <n>
Files beside removed paths: <n>
Specification checksum: <12 hex>…
End-to-end checksum: <12 hex>…
End-to-end tests: not copied (100 files)
Copy directory: <copy>
Manifest: <manifest>
```

The abbreviated checksums are the report's first characters; the manifest keeps
the full 64 lowercase hex values. The counts are the trial revision's values,
not the authoring values. A difference between the report and the manifest is a
setup error; the trial does not continue.

## Gap Triage

A difference is one observable mismatch between the frozen E2E result and the
rebuilt binary. Each difference receives exactly one outcome before any edit is
made.

| Outcome | Meaning | Artifact that is wrong | Route |
|---|---|---|---|
| `E2E test defect` | The frozen test asserts a wrong expectation, or depends on an artifact the Specification does not declare. | The specific `e2e/**` file. | Correct the test, re-freeze it, and restart the trial at the revision that records the correction. |
| `implementation defect` | The rebuilt binary does not satisfy a behavior the Specification states. | The rebuilt file in the spec copy. | Return to the builder inside the same trial; no corpus or Specification edit; rerun the suite. |
| `specification defect` | The Specification is silent, ambiguous, or wrong about the observed behavior. | `givn/specification.yaml`, `docs/reference.md`, the owning `givn/specs/**` use case, or the declaration entry that should have covered the path. | Record the outcome; the correction belongs to a later change. No edit in this change. |
| `acceptable undocumented change` | The rebuilt behavior is a legitimate implementation choice the Specification does not constrain, and no acceptance test rejects it. | The rebuilt file, which is not a gap. | Record the outcome; no edit. |

`spec-gaps.md` records one row per difference with the frozen scenario title,
the failing step, the observed and expected result, the rebuilt file, the
outcome, the wrong artifact, and the evidence path. A trial with no difference
records `No difference observed` and names the result file and counts. No
difference is closed as drift.

The undisclosed build inputs (D7) are not an E2E difference, but they are a gap
in the Specification. `spec-gaps.md` always holds one `specification defect`
row against `givn/specification.yaml` for `Cargo.toml`, `Cargo.lock`,
`assets/**`, the Rust toolchain, and the registry cache, even when the E2E run
is green. The row names the surface the builder supplied and the artifact that
should declare it.

## Anticipated Coverage Risks

These are candidates for a gap, not pre-assigned outcomes. The trial records
whether each produces a difference.

| Surface | Why it may differ | Artifact to name if it is a gap |
|---|---|---|
| `assets/**` (served instructions, skills, templates) | Outside the declaration; exact served text is asserted by `@e2e` scenarios. | `givn/specification.yaml` or `docs/reference.md` when the Specification should declare it; the `e2e/**` file when the test asserts an undeclared implementation string. |
| `Cargo.toml` version | Outside the declaration; scenarios read the version from `GIVN_ROOT/Cargo.toml`. | `docs/reference.md` or `givn/specification.yaml`. |
| `Cargo.lock` | The builder authors it; a crate version or feature set differs from the checkout's. | `givn/specification.yaml`; the build inputs are undeclared (D7). |
| `README.md`, `CHANGELOG.md` | Outside the declaration; read-only repository files some scenarios assert. | The `e2e/**` file, or `acceptable undocumented change`. |
| `tests/ready_fixture.rs` | A test fixture from the checkout, not product behavior. | The `e2e/**` file when the frozen suite cannot run without an undeclared fixture. |
| `givn/` project configuration | Outside the declaration; the rebuilt binary embeds defaults. | `givn/specification.yaml` when a scenario depends on a project file the Specification does not declare. |

The trial does not fix a `specification defect`. It records the missing
declaration and leaves the correction to the change that `S14` opens.

## Evidence Artifacts

All paths are under
`givn/changes/enable-ise-and-run-the-givn-self-rebuild/`, which is exempt from
the change check's clean-tree requirement and archived with the change.

| Path | Produced by | Content |
|---|---|---|
| `evidence/ise/trial-revision.txt` | author | The 40-character revision and the commands that resolve it; committed after the run, not in the revision it names. |
| `evidence/ise/e2e-dependencies.txt` | author | `cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e`; no `givn`, `givn-config`, or `givn-core`. |
| `evidence/ise/boundary-check.txt` | author | Pre-trial `givn check tasks --change <id>` transcript: exit 0, `Mandatory checks for 'tasks':`, the declared-package preflight (no refusal), `changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)`, and the expected `⚠ task preflight: review.md is missing` warning. It is not a clean preflight. |
| `evidence/ise/boundary-review-check.txt` | author | `givn check review --change <id>` transcript after `review.md` exists: exit 0 and the same preflight on the not-applicable path. |
| `evidence/ise/spec-coverage.json` | author | `givn spec coverage --format json` at the trial revision. |
| `evidence/ise/e2e-inventory.md` | author | One row per `@e2e` entry: capability, feature path, scenario title; the count equals the runner's `total`. |
| `evidence/ise/rebuild-report.txt` | author | The `givn ise rebuild-all` stdout, all fixed lines. |
| `evidence/ise/manifest.yaml` | author | The `--manifest` file copied verbatim after the command. |
| `evidence/ise/compare-before.txt` | author | `givn ise compare` before the builder starts. |
| `evidence/ise/builder-notes.md` | builder | Sandbox command, every source and file read or authored, every crate version chosen, build command, and a statement that the checkout and `e2e/` were not read. |
| `evidence/ise/compare-after.txt` | verifier | `givn ise compare` after the build. |
| `evidence/ise/rebuilt-version.txt` | verifier | `<copy>/target/release/givn --version` and the binary's checksum. |
| `evidence/ise/e2e-result.json` | verifier | The runner's scoped result file with `total`, `passed`, `failed`, and `skipped`. |
| `evidence/ise/spec-gaps.md` | verifier | The gap list and the triage table, including the disclosed-input `specification defect` row. |

The change has no `verification/tasks-verification.json`. `src/cli/check.rs`
returns before `write_change_receipt` on the not-applicable path;
`givn check tasks` and `givn check review` accept an `ise`-effective change
with no behavior change through `preflight_e2e_boundary`, and those two
transcripts are the boundary evidence.

The archive's own result files under `results/` are written by the archive
later. The self-dogfood sequence runs after archive; its transcript is recorded
as a post-archive maintenance commit under the archived change's
`evidence/ise/self-dogfood.txt`, and the review names the commands.

## Architecture Impact

| Artifact | Kind | Effect |
|---|---|---|
| `givn/config.yaml` | project configuration | `addons.ise: true` is already recorded by revision `a9b54140`; no edit in this change. |
| `givn/changes/<id>/config-lock.yaml` | change lock | `ise: true`; no edit. |
| `givn/specification.yaml` | declaration | Unchanged; declared by `enforce-ise-e2e-boundary`. |
| `docs/arc42/02-architecture-constraints.md` | durable architecture constraint | Records that this project is `ise`-effective and that its rebuild boundary is the declared source roots. |
| `docs/arc42/04-solution-strategy.md` | durable strategy | Records the self-enablement and the trial's no-promotion rule. |
| `docs/arc42/11-risks-and-technical-debt.md` | risk register | Adds the Specification coverage risk the trial tests. |
| `src/**`, `tests/**` | production and component source | No edit; the trial rebuilds them only inside the spec copy. |
| `e2e/**` | frozen acceptance package | No edit; an `E2E test defect` would correct one file before the trial result. |
| `givn/specs/**` | permanent corpus | No edit; `specs` is skipped. |
| Trial copy and manifest | ephemeral evidence | Outside the checkout; only the copies under `evidence/ise/` are committed. |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `SpecCopy` (rebuild-all, rebuild-component, compare) | prepare-an-ise-test | the maintainer, the independent verifier | `crate::ise` | A second copy-resolution rule or a second manifest reader appears. |
| `SpecificationDeclaration` | declare-the-specification | `crate::ise`, `crate::e2e_boundary`, `crate::cli::lint` | `crate::config::declaration` | A second declaration format appears. |
| `E2eBoundary` | verify-change, prepare-an-ise-test | `crate::cli::check`, `crate::ise` | `crate::e2e_boundary` | A second package-resolution strategy appears. |
| `givn instructions design` / `givn instructions review` (served change guidance) | create-change | authors reading the `change-strategy` and `spec-gap` guidance | `crate::cli::instructions` over `assets/instructions/**` | A second served change-work surface or a second gap-record shape appears. |

The change adds no contract and no component. It exercises the four existing
contracts on this repository and records their output; the served guidance is
read, not modified.

## Visual Design Contract

```text
- Interface: terminal
- Viewports: not applicable (terminal text surface)
- Design system: none (CLI)
```

The trial's user-visible surface is terminal text: the report lines, the
comparison subjects, refusal messages, and exit codes. The delta adds no screen
and no `@e2e` scenario, so no visual evidence file is committed.

Baseline asserted by the trial transcripts:

- every report line is one of the fixed lines above;
- every refusal names the condition and its subject path, revision, or flag;
- no Rust module, struct, or field name appears in user-facing output;
- exit codes stay stable: `givn ise rebuild-all` 0 on success, 1 on a refusal,
  2 on a usage error; `givn ise compare` 1 on any difference; `givn check
  tasks` 0 on a green run.

### Persona Disposition

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Runs the trial and reads the report, the comparison, and the gap list. | `rebuild-report.txt`, `compare-before.txt`, `compare-after.txt`, `spec-gaps.md` |
| `agent--technical-role` | Builds the product from the copy and records its inputs. | `builder-notes.md`, `rebuilt-version.txt`, `spec-gaps.md` |
| `givn-maintainer--owner` | Owns `S14` and decides promotion from the evidence. | `boundary-check.txt`, `boundary-review-check.txt`, `spec-gaps.md`, `e2e-result.json` |

## Step Definitions

none. No step definition is added, moved, or copied. `specs` is skipped, so archive
copies no feature and the delta step-file preflight has no phrase to resolve.
The trial runs the permanent `@e2e` scenarios through the existing
`e2e/tests/steps/**` files and the component scenarios through
`tests/steps/**`; both stay unchanged.

## Interaction Coverage Matrix

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism | Disposition |
|---|---|---|---|---|
| `prepare-an-ise-test` / `clean-room-preparation`: write the spec copy, wholly with `rebuild-all` or for named component paths with `rebuild-component` | `Developer rebuilds the whole product from its Specification` (`givn/specs/prepare-an-ise-test/clean-room-preparation.feature`) | CLI | Real `givn ise rebuild-all --at HEAD --into <copy> --manifest <file>` subprocess; assert the fixed report lines, the copy contents, and the manifest. | `unchanged`: the trial runs the archived command; no step, surface, or assertion moves. |
| `prepare-an-ise-test` / `trial-input-verification`: compare the spec copy and the checkout with the manifest | `Verifier compares the spec copy with the manifest` (`givn/specs/prepare-an-ise-test/trial-input-verification.feature`) | CLI | Real `givn ise compare --manifest <file> --copy <dir>` subprocess; assert the two subject lines and exit 0. | `unchanged`: `givn ise compare` runs before and after the build; its subjects and assertions stay. |
| `declare-the-specification` / `specification-declaration`: declare the five roles and the replaceable source roots | `The declared roles are read by the project-health pass` (`givn/specs/declare-the-specification/specification-declaration.feature`) | CLI | Real `givn lint` subprocess; assert exit 0, no declaration-schema error, and the contracts-role advisory. | `unchanged`: the declaration is unchanged and the trial reads it. |
| `create-change` / `change-strategy`: read the design guidance for a rebuilding change | `The design guidance requires one recorded change strategy` (`givn/specs/create-change/change-strategy.feature`) | CLI | Real `givn instructions design --change <id>` subprocess; assert the served text. | `unchanged`: the design records D5 and no served text changes. |
| `create-change` / `spec-gap`: read the review guidance after an Ise Test | `The review guidance requires one spec gap outcome before any fix` (`givn/specs/create-change/spec-gap.feature`) | CLI | Real `givn instructions review --change <id>` subprocess; assert the served text. | `unchanged`: `spec-gaps.md` follows the served outcomes; no served text changes. |

`create-change`'s other inventory rows are not touched by this change.

## Strict Mode

Strict mode is inherited unchanged: `tests/features_runner.rs` calls
`.fail_on_skipped()`, and `e2e/tests/e2e_runner.rs` calls
`.fail_on_skipped()` and exits nonzero when `failed > 0`, `skipped > 0`, or
`total == 0`. No step body is added, so no empty-body check is needed for this
change. The trial treats a skipped or zero-scenario E2E result as a setup
failure and records it in `spec-gaps.md`, not as a green run.

## Local Runnability and Digital Twins

The trial runs locally on one host:

```sh
# copy write
givn ise rebuild-all --at <revision> --into /tmp/givn-ise-self-rebuild-<stamp> \
  --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml

# builder
bwrap ... --chdir /work cargo build --release --offline

# acceptance
GIVN_BIN=<copy>/target/release/givn \
GIVN_RESULT_FILE=$PWD/givn/changes/<id>/evidence/ise/e2e-result.json \
./run-tests.sh --e2e --package e2e
```

The product is a CLI; it starts no server, database, queue, or browser. The
builder resolves crates from the local cargo cache with `--offline` and has no
network. No digital twin is required because the product talks to no
third-party service. The `ready_fixture` binary is local test infrastructure
built from the checkout and named in the builder notes.

## External Boundary Cases

The trial's external boundary is the CLI and the frozen runner. These cases are
recorded before the trial; each is a setup error, not a `Spec gap`:

| Boundary | Case | Expected |
|---|---|---|
| `givn ise rebuild-all` | `--into` omitted | Exit 2, usage error, no write. |
| `givn ise rebuild-all` | `--into` inside the checkout or already existing | Exit 1, refusal names the target, no write. |
| `givn ise rebuild-all` | `--manifest` inside the copy | Exit 1, refusal names the manifest, no write. |
| `givn ise rebuild-all` | Dirty checkout or missing backend | Exit 1, refusal names every dirty path or the backend selector. |
| `givn ise rebuild-all` | Declaration missing, unreadable, non-UTF-8, or malformed at the revision | Exit 1, refusal names the path, the revision, and the condition, no write. |
| `givn ise compare` | Manifest not belonging to the copy, or a file added under a Specification path | Exit 1, names the path or mismatch. |
| Builder | A dependency is missing from the offline cargo cache | The build fails; record a setup error, refresh the cache outside the sandbox, and restart the trial. |
| E2E runner | `GIVN_BIN` unset or not executable | Exit 2, names `GIVN_BIN`. |
| E2E runner | `GIVN_READY_FIXTURE` unset or not executable | `run-tests.sh` exits 2 when it cannot resolve the built fixture; a step that needs the fixture fails by name, and the runner exits 1. |
| E2E runner | Zero scenarios, failed, or skipped | Exit 1; the result file records the counts. |
| E2E runner | `GIVN_RESULT_FILE` unset | The run still exits by counts; the trial records no result and restarts with the variable set. |

## Coverage Process Boundaries

The change adds no production code and no measured process. The archive
coverage gate uses the existing `measure-coverage.sh` and
`merge-coverages.sh` over the regular suite. The trial's rebuilt binary is
evidence, not the measured build, so it is not instrumented. No new row is
needed in the coverage process table.

## Use-Case Traceability

| Use-case guarantee or rule | Decision and artifact | Evidence |
|---|---|---|
| The declaration names the five roles and the replaceable source roots | declaration; D5 | `rebuild-report.txt`, `manifest.yaml` |
| The spec copy carries the Specification files and the declaration, and no E2E file | D5; `givn ise rebuild-all` | `compare-before.txt`, the report's `End-to-end tests: not copied` line |
| The builder reads neither the replaced Source code nor the End-to-end (E2E) tests | D3; sandbox and builder notes | `builder-notes.md`, `compare-before.txt` |
| The frozen End-to-end (E2E) package accepts the rebuilt binary | D3 | `e2e-result.json`, `rebuilt-version.txt` |
| Every difference receives one `Spec gap` outcome before a fix | D2 | `spec-gaps.md` |
| The change records the change strategy and the `Ise Test` mode | D5 | this design's Decisions ledger |
| The declared E2E package boundary passes on Givn itself | enablement; D6 | `boundary-check.txt`, `boundary-review-check.txt` |
| The scenario inventory accounts for every `@e2e` scenario | D4 | `spec-coverage.json`, `e2e-inventory.md` |

## Decisions

Decides D1: the trial reads the change's pre-trial commit, passed explicitly
with `--at <40 hex>`. That commit records the boundary transcript, the
inventory, the coverage export, the dependency tree, and every task completed
before the trial. `trial-revision.txt` and the post-trial evidence are
committed after the run, because they cannot belong to the revision they name.
A restart after an `E2E test defect` reads a commit that records the first
attempt's evidence, the corrected test, and the regenerated inventory and
coverage. Realized by `trial-revision.txt`, the report's `Revision:` line, and
the manifest's `revision`. Rejected: `HEAD` (a moving input no transcript can
reproduce) and the change's base revision (which ignores the corrected test
and the evidence paths).

Decides D2: classify before repair. One difference receives one outcome and one
wrong artifact before any edit. An `implementation defect` returns to the
builder inside the same trial; an `E2E test defect` is corrected and the trial
restarts at the correcting revision; a `specification defect` and an
`acceptable undocumented change` are recorded without a corpus edit in this
change. Realized by `spec-gaps.md` and the triage table above. Rejected:
fix-first (the outcome is chosen to fit the fix) and stop-on-first (S13
requires every difference classified).

Decides D3: the trial takes `GIVN_BIN` from the rebuilt binary and
`GIVN_READY_FIXTURE` and `GIVN_ROOT` from the checkout. The fixture and the
repository-file assertions are test infrastructure, not product construction.
The clean-room claim covers the product behavior, not the fixture bytes or the
checkout files those scenarios read. The build inputs the copy lacks
(`Cargo.toml`, `Cargo.lock`, `assets/**`, the toolchain, and the registry
cache) are disclosed in D7, not covered by this claim. Realized by
`builder-notes.md`, `e2e-result.json`, and the run command above. Rejected:
rebuilding the fixture (the Specification does not declare it; its source is
in the replaced `tests/` set) and excluding the run-execution scenarios (the
frozen suite would no longer be whole).

Decides D4: no delta spec. The change adds no product behavior; the enablement
is project configuration and the trial evidence belongs to capabilities that
already have scenarios. `.givn-skip` lists `specs`. The routing check sees no
delta and no narrative and returns success; review records the E2E scope as not
applicable and has no scenario commit to bind. Rejected: a delta scenario
stating the enablement (it would assert project configuration, not product
behavior) and duplicate clean-room scenarios (they would restate archived
behavior).

Decides D5: the change strategy is `whole-product rebuild` with the `Ise Test`
mode `Dry-Run Ise`. Affected consumers: the `developer--primary-user` runs the
trial and reads the report; the `agent--technical-role` builds in the copy; the
`givn-maintainer--owner` decides `S14` from the evidence; the independent
verifier runs `givn ise compare` and the frozen suite. Persisted state: none.
The spec copy, manifest, and rebuilt binary are ephemeral and outside the
checkout; Givn keeps no product state in the trial. Rebuild boundary: every
tracked file under `src/**` and `tests/**` is removed; the `e2e` package, the
declaration, the four content roles, and every file outside the declaration
stay as described above. Rejected: `incremental modification` (an in-place edit
proves no rebuild), `Constant Ise` (the scope is the whole product, not one
component), and `Full replacement` (nothing is promoted; `S14` owns that).

Decides D6: the enablement is already recorded by revision `a9b54140` and the
change lock, and this change does not rewrite either. The change's own
`givn check tasks` and `givn check review` prove the boundary is effective
through `preflight_e2e_boundary`; a missing or rejected declaration is a setup
failure, not a trial difference. The not-applicable path writes no
`verification/tasks-verification.json`. Realized by `boundary-check.txt` and
`boundary-review-check.txt`. Rejected: re-enabling the addon in this change (it
would rewrite a committed value with no change) and treating the enablement as
a delta scenario (D4).

Decides D7 (design decision): the builder supplies five non-Specification
build inputs: `Cargo.toml`, `Cargo.lock`, `assets/**`, the Rust toolchain, and
the cargo registry cache. The copy carries none of them and the declaration
names none of them, so the builder authors the package manifest and lockfile,
invents every embedded asset the served corpus needs, chooses the crate
versions it resolves from the read-only cache, and takes the compiler from the
sandbox mounts. The trial therefore does not test whether these inputs are
derivable from the Specification, and `spec-gaps.md` records one
`specification defect` row against `givn/specification.yaml` for the withheld
surface. Realized by `builder-notes.md` (every authored file and chosen crate
version) and `spec-gaps.md`. Rejected: declaring `Cargo.toml`, `Cargo.lock`,
and `assets/**` before the trial (it changes the trial inputs and the counts
and needs its own change) and leaving the inputs undisclosed (it would report a
rebuild from the Specification alone that did not happen).

## Verification

The trial's checks, in order:

1. `cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e`
   names no `givn`, `givn-config`, or `givn-core` package.
2. `givn spec coverage --format json` reports the `@e2e` set; its count equals
   the E2E runner's `total`.
3. `givn check tasks --change enable-ise-and-run-the-givn-self-rebuild` exits
   0, passes the declared package boundary preflight, and prints
   `changed-scenario verification: not applicable (Behavior change: none;
   specs explicitly skipped)`. The not-applicable path returns before
   `write_change_receipt`, so no receipt exists. `givn check review` repeats
   the preflight after `review.md` exists.
4. `givn ise rebuild-all ...` prints the fixed report and writes the manifest;
   the manifest matches the report and the revision.
5. `givn ise compare` reports `Specification files: match` and
   `End-to-end tests: match` before the build.
6. `find "$HOME/.cargo/registry/src" "$HOME/.cargo/registry/cache"
   "$HOME/.cargo/registry/git" -maxdepth 2 -name 'givn-*' -print` prints
   nothing, so the builder's cache holds no product source or archive.
7. The builder builds the binary in the sandbox; `givn ise compare` still
   reports both subjects as match.
8. The E2E run exits 0 with `failed = 0`, `skipped = 0`, and
   `total = <@e2e count>`, or every difference is in `spec-gaps.md` with one
   outcome. The disclosed-input `specification defect` row is recorded in both
   cases.
9. The change's own `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e
   --package e2e` are green on the final revision.
10. After archive, the `AGENTS.md` self-dogfood sequence runs:
    `cargo install --path . --locked --force`, `givn skills sync`,
    `givn instructions`, `./run-tests.sh`, `./run-tests.sh --e2e`.

## Open Technical Questions

None.
