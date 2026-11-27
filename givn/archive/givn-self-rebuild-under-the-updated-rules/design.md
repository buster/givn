# Design: givn-self-rebuild-under-the-updated-rules

## Purpose and approach

Apply the updated `ise` build-input rule to Givn itself and re-run the
whole-product `Dry-Run Ise` under it. The change does four things:

1. Declares Givn's build inputs under the content-class role.
2. Adds the toolchain pin those declarations name.
3. Guards the declaration and the spec-copy carrier with two delta scenarios.
4. Re-runs the trial and records the frozen result, the gap table, the
   neighbor record, the clean-room scan, and the declaration diff.

No production code changes. `givn/specification.yaml` and
`rust-toolchain.toml` are data and build configuration; the rebuild commands,
the declaration loader, the manifest shape, and the E2E runner stay unchanged.
The two delta scenarios drive the existing CLI and protect settled behavior;
they pass immediately. The change's new evidence is the trial re-run.

| Input | Value |
|---|---|
| Change strategy | `whole-product rebuild` |
| `Ise Test` mode | `Dry-Run Ise` |
| Rebuild command | `givn ise rebuild-all --at <revision> --into <dir> --manifest <file>` |
| Trial revision | the change's pre-trial commit, passed explicitly |
| Rebuild boundary | every tracked file under `src/**` and `tests/**` is removed |
| Frozen authority | the `e2e` package and every `@e2e` feature path |
| Build inputs | declared under `contracts`, `limits`, `structure`, and `behavior`; the `e2e` package's own manifest and lockfile stay the test build's inputs |
| Promotion | none; the spec copy and rebuilt binary stay outside the checkout |

## Declaration

### Role split

The updated rule places a build input under the role that owns its content
class. Givn's `assets/**` tree is partitioned whole, so no embedded asset is
left outside the Specification. The `e2e` package's manifest and lockfile are
not product build inputs and stay under the `e2e` role, excluded from the
copy.

| Role | Entry | Content class | Files at authoring |
|---|---|---|---|
| `behavior` | `givn/specs/**` | The behavior corpus and the `@e2e` scenario text. | 85 |
| `behavior` | `assets/instructions/**` | Served instruction text. | 28 |
| `behavior` | `assets/skills/**` | Served skill text. | 16 |
| `behavior` | `assets/agent-integrations/**` | Served OpenCode, Claude, omp, and Maki integration text, including the four rows the prior trial classified `specification defect`. | 10 |
| `behavior` | `assets/addons/arc42-docs/instructions.md` | Served addon instruction. | 1 |
| `behavior` | `assets/addons/communication-policy/policy.md` | Served policy text. | 1 |
| `behavior` | `assets/addons/coverage/instructions.md` | Served addon instruction. | 1 |
| `behavior` | `assets/agents-project-hint.md` | Served project hint. | 1 |
| `behavior` | `assets/hooks/**` | Served commit-attribution hook text. | 2 |
| `structure` | `docs/glossary.md` | Vocabulary. | 1 |
| `structure` | `docs/domain-model.md` | Structural model. | 1 |
| `structure` | `assets/templates/**` | Document templates, including the domain-model and glossary templates the rule names. | 9 |
| `structure` | `assets/addons/arc42-docs/chapters/**` | Arc42 chapter and ADR templates. | 14 |
| `structure` | `assets/addons/arc42-docs/impact-assessment-template.md` | Impact-assessment template. | 1 |
| `structure` | `assets/addons/coverage/template.md` | Coverage-section template. | 1 |
| `structure` | `assets/addons/readme/README.md.tmpl` | README scaffold. | 1 |
| `structure` | `assets/timeline/report.html` | Development-report template. | 1 |
| `contracts` | `docs/reference.md` | Reference contract. | 1 |
| `contracts` | `Cargo.toml` | Build manifest; fixes the package version and targets. The prior product-version row is this entry. | 1 |
| `contracts` | `Cargo.lock` | Lockfile; fixes the resolved dependencies. | 1 |
| `contracts` | `assets/commands.yaml.tmpl` | Commands configuration schema. | 1 |
| `contracts` | `assets/config.yaml.tmpl` | Project configuration schema. | 1 |
| `contracts` | `assets/run.yaml.tmpl` | Run-declaration schema. | 1 |
| `contracts` | `assets/default-manifest.yaml` | Embedded default manifest. | 1 |
| `contracts` | `assets/specification.yaml.tmpl` | Declaration scaffold. | 1 |
| `contracts` | `assets/migrations/**` | Migration catalog and fixtures. | 3 |
| `limits` | `docs/arc42/02-architecture-constraints.md` | Architecture constraints. | 1 |
| `limits` | `rust-toolchain.toml` | Toolchain pin. | 1 |
| `e2e` | `e2e` | The frozen acceptance package; never copied. | 100 |
| `source` | `src/**`, `tests/**` | Replaceable Source code. | 187 |
| implicit | `givn/specification.yaml` | The declaration. | 1 |

### Exact declaration

```yaml
# The project's declared Specification. The four content roles retain the
# documents and build inputs the whole-product rebuild reads; the `e2e` role
# names the frozen acceptance package, and the `source` roots are replaced.
#
# A build input sits under the role that owns its content class: the build
# manifest and lockfile under `contracts`, the toolchain pin under `limits`,
# document and report templates under `structure`, and served instruction,
# skill, integration, and hook text under `behavior`.
behavior:
  - "givn/specs/**"
  - "assets/instructions/**"
  - "assets/skills/**"
  - "assets/agent-integrations/**"
  - "assets/addons/arc42-docs/instructions.md"
  - "assets/addons/communication-policy/policy.md"
  - "assets/addons/coverage/instructions.md"
  - "assets/agents-project-hint.md"
  - "assets/hooks/**"
structure:
  - "docs/glossary.md"
  - "docs/domain-model.md"
  - "assets/templates/**"
  - "assets/addons/arc42-docs/chapters/**"
  - "assets/addons/arc42-docs/impact-assessment-template.md"
  - "assets/addons/coverage/template.md"
  - "assets/addons/readme/README.md.tmpl"
  - "assets/timeline/report.html"
contracts:
  - "docs/reference.md"
  - "Cargo.toml"
  - "Cargo.lock"
  - "assets/commands.yaml.tmpl"
  - "assets/config.yaml.tmpl"
  - "assets/run.yaml.tmpl"
  - "assets/default-manifest.yaml"
  - "assets/specification.yaml.tmpl"
  - "assets/migrations/**"
limits:
  - "docs/arc42/02-architecture-constraints.md"
  - "rust-toolchain.toml"
e2e:
  - "e2e"
source:
  - "src/**"
  - "tests/**"
```

### Toolchain pin

`rust-toolchain.toml` at the repository root:

```toml
[toolchain]
channel = "1.97.1"
components = ["clippy", "rustfmt"]
```

1.97.1 is the installed stable toolchain on this host, verified on 2026-11-17
with `rustup toolchain list` and `cargo +1.97.1 check --locked`; the check
finished green in 10.62 s. The current default `stable` is 1.99.1, but no
`1.99.1` toolchain is installed, so pinning it would add a download before the
first build.

The pin binds the rustup-based build and the sandbox, which reads the copied
file through the read-only `~/.rustup`. `flake.nix` builds with nixpkgs'
`rustPlatform` and its devShell uses nixpkgs' rustc; `rust-toolchain.toml` does
not bind those. `nix build` is out of scope for this change and the trial. A
`rustToolchain` override is a later change if nix builds must follow the pin.

### Authoring counts

These are sanity checks, not pinned values. The trial recomputes every count at
the selected revision and records the result in the report and the manifest.

| Measure | Value |
|---|---|
| `behavior` files | 145 (85 `givn/specs/**` plus 60 assets) |
| `structure` files | 29 |
| `contracts` files | 11 |
| `limits` files | 2 |
| Specification files | 188 (the four roles plus the declaration) |
| Tracked files under `src/**` and `tests/**` | 187 |
| Tracked `e2e` package files | 100 |
| Feature paths carrying a selected `@e2e` scenario | 56 |
| `@e2e` coverage entries | 115 |
| `ise-manifest/v1` `end_to_end_files` | 156 (100 package files plus 56 feature paths) |
| Outside the spec copy, excluding the change directory | 2509 |

The trial revision adds the change directory and its evidence to `Outside the
spec copy`; it does not change the Specification or the replaced set. The
report's `Specification files` must equal the manifest's
`specification_files` length; a difference is a setup error.

## Trial protocol under the updated rules

Every command block in this section and in `## Local runnability and digital
twins` runs from the repository root, the directory that holds `run-tests.sh`
and `givn/changes/<id>/`. The `tee` and `cp` targets carry that prefix, so
`$PWD` and `./run-tests.sh` resolve from the same directory.

### Pre-trial checks

The trial runs from the checkout after the change's pre-trial commit, which
records the declaration, the pin, the delta scenarios, the step bindings, the
arc42 chapter edits, and every task completed before the trial. `<revision>` is
that commit's 40-character hash.

| Check | Command | Evidence |
|---|---|---|
| Declared build inputs are accepted | `givn lint` in the checkout | `evidence/ise/declaration-lint.txt` |
| Declaration diff | `git diff <base> -- givn/specification.yaml rust-toolchain.toml` | `evidence/ise/declaration.diff` |
| E2E package links no product crate | `cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e` | `evidence/ise/e2e-dependencies.txt` |
| Frozen scenario inventory | `givn spec coverage --format json` | `evidence/ise/spec-coverage.json`, `evidence/ise/e2e-inventory.md` |
| Boundary preflight | `givn check tasks --change <id>` | `evidence/ise/boundary-check.txt` |
| Pinned toolchain present | `rustup toolchain list` | `evidence/ise/attempt-<N>/builder-notes.md` |
| Clean-room scan | see below | `evidence/ise/clean-room-scan.txt` |

The clean-room scan reads the three Cargo stores the sandbox exposes: the
registry source, the registry cache, and the live git store `~/.cargo/git`.
`~/.cargo/registry/git` is not a store and does not exist on this host; the
sandbox binds `$HOME/.cargo/git` read-only, so that path is the one scanned.

```sh
mkdir -p givn/changes/<id>/evidence/ise
for store in "$HOME/.cargo/registry/src" "$HOME/.cargo/registry/cache" \
  "$HOME/.cargo/git"; do
  if [ -d "$store" ]; then
    find "$store" -maxdepth 4 \( -name 'givn' -o -name 'givn-*' \) -print
  else
    printf 'absent: %s\n' "$store"
  fi
done | tee givn/changes/<id>/evidence/ise/clean-room-scan.txt
```

All three stores exist on this host, so the command prints nothing. A store
that does not exist on another host is recorded as `absent: <path>` and has no
hit. A hit is quarantined outside the builder's mounts under
`/tmp/givn-ise-quarantine-<stamp>/`; if moving it would damage the cache, the
trial uses a fresh `CARGO_HOME` populated from the declared lockfile only. The
scan result is recorded in the neighbor record.

The pre-trial commit is made with `givn commit` (the project's explicit-path
commit path) or the configured equivalent. `trial-revision.txt` and the
post-trial evidence cannot belong to the revision they name; commit the
pre-trial files first, run the trial, then commit the evidence and
`trial-revision.txt` after the run.

### Rebuild and copy checks

```sh
attempt=attempt-1
mkdir -p givn/changes/<id>/evidence/ise/$attempt

givn ise rebuild-all \
  --at <revision> \
  --into /tmp/givn-ise-self-rebuild-<stamp> \
  --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml \
  | tee givn/changes/<id>/evidence/ise/$attempt/rebuild-report.txt

givn ise compare --manifest <manifest> --copy <copy> \
  | tee givn/changes/<id>/evidence/ise/$attempt/compare-before.txt

cp /tmp/givn-ise-self-rebuild-<stamp>.yaml \
  givn/changes/<id>/evidence/ise/$attempt/manifest.yaml
```

`--dry-run` is not passed. The mode is `Dry-Run Ise`: the copy is written, the
replacement is built and accepted, and nothing is promoted. Each attempt uses a
new `attempt-<N>` directory; a restart never overwrites a previous attempt's
files.

Before the builder starts, the verifier confirms the copy carries every
declared build input: `Specification files: match`, the manifest's
`specification_files` includes `Cargo.toml`, `Cargo.lock`,
`rust-toolchain.toml`, `assets/templates/**`, and the served text paths, and
the report's `Specification files` count equals the manifest's length. The copy
holds no `e2e` package file.

### Clean-room builder

The builder is a fresh agent whose workspace is the spec copy; the checkout is
not mounted into that workspace. The agent's commands and reads are recorded
in `attempt-<N>/builder-notes.md`. The concrete build control is a `bwrap` sandbox. The
mount set was probed with `cargo --version` on 2026-11-17 and reused here:

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

The only repository path in the sandbox is `<copy>` at `/work`. The builder has
no `e2e/`, no checkout `src/` or `tests/`, no `.git`, and no network. The
sandbox binds the two Cargo stores the clean-room scan reads:
`~/.cargo/registry` (source and cache) and `~/.cargo/git` (the git store). The
copied `rust-toolchain.toml` selects 1.97.1 from the read-only `~/.rustup`.
The copied `Cargo.lock` fixes the crates, resolved from the read-only registry
under `cargo build --release --offline`. The cache is populated before the
sandbox starts by the host's build against the same lockfile.

The builder reads no product source outside the spec copy and the declared
inputs. It does not modify or delete a file the copy was written with; it
authors only files under the removed `src/**` and `tests/**` roots, which are
the rebuilt Source code. A file it creates elsewhere is a neighbor and is
recorded. A declared build input it would have to author is a `specification
defect`, not rebuilt Source code.

The declared `Cargo.toml` carries two test targets, `[[test]] name =
"features_runner"` and `[[test]] name = "ready_fixture"`, both with
`harness = false`. Because the copy removes `tests/**`, Cargo parses those
targets before compiling and fails with exit 101 until both files exist. Before
the first build, the builder authors `tests/features_runner.rs` and
`tests/ready_fixture.rs` with a minimal `fn main() {}`; they are rebuilt Source
code, and the declared manifest and the `contracts` role name the targets. A
first build that fails on a missing target is a setup error, not an expected
red result and not a `Spec gap`; the trial neither classifies it as a
difference nor hides it, and the builder records the exit 101 and the two
authored target files in `attempt-<N>/builder-notes.md`.

### Frozen acceptance

```sh
attempt=attempt-<N>
mkdir -p givn/changes/<id>/evidence/ise/$attempt

givn ise compare --manifest <manifest> --copy <copy> \
  | tee givn/changes/<id>/evidence/ise/$attempt/compare-after.txt

<copy>/target/release/givn --version \
  | tee givn/changes/<id>/evidence/ise/$attempt/rebuilt-version.txt
sha256sum <copy>/target/release/givn \
  >> givn/changes/<id>/evidence/ise/$attempt/rebuilt-version.txt

GIVN_BIN=<copy>/target/release/givn \
GIVN_RESULT_FILE=$PWD/givn/changes/<id>/evidence/ise/$attempt/e2e-result.json \
./run-tests.sh --e2e --package e2e 2>&1 \
  | tee givn/changes/<id>/evidence/ise/$attempt/e2e-run.log
```

The expected rebuilt version is the version recorded in the declared
`Cargo.toml` at the trial revision (`0.9.20` at authoring). `GIVN_BIN` names
the rebuilt binary; `GIVN_READY_FIXTURE` and `GIVN_ROOT` name the checkout
because they are test infrastructure and read-only repository assertions.
`run-tests.sh` builds `ready_fixture` from the checkout. The clean-room claim
covers the product behavior, not the fixture bytes or the checkout files those
scenarios read.

Every attempt retains its own `attempt-<N>` directory and files. The final
`spec-gaps.md` names the attempt it classifies (the last attempt with a
completed E2E run) and records each attempt's result counts; a superseded
attempt stays on disk. A new attempt after an `E2E test defect` starts with a
new rebuild and copy; an `implementation defect` rerun uses the same copy and
writes its builder notes, comparison, version, run log, result, gap table, and
neighbor record in the new attempt directory without repeating `rebuild-all`.

## Neighbor record

`evidence/ise/attempt-<N>/neighbor-record.md` holds one row per file changed
outside the declared Specification or the replaced unit, and one row per
consolidation, with the reason. A rebuild with no neighbor or consolidation
records both tables empty; an empty table is the expected result because the
builder authors only `src/**` and `tests/**`. The spelling follows
`docs/glossary.md` and the served `ise` instruction: `Neighbor record`.

| Path or consolidation | Kind | Reason | Evidence |
|---|---|---|---|
| (empty) | | The builder changes only the replaced unit. | `attempt-<N>/builder-notes.md` |

The record also carries the clean-room scan result, the three scanned store
paths (`~/.cargo/registry/src`, `~/.cargo/registry/cache`, `~/.cargo/git`),
every quarantined path, and the verifier's test-infrastructure inputs
(`GIVN_BIN`, `GIVN_READY_FIXTURE`, `GIVN_ROOT`). It is a trial-review record,
never a gate.

## Gap triage

A difference is one observable mismatch between the frozen E2E result and the
rebuilt binary. Each difference receives exactly one outcome before any edit.

| Outcome | Meaning | Artifact that is wrong | Route |
|---|---|---|---|
| `E2E test defect` | The frozen test asserts a wrong expectation, or depends on an artifact the Specification does not declare. | The specific `e2e/**` file. | Correct the test, re-freeze it, and restart the trial at the revision that records the correction. |
| `implementation defect` | The rebuilt binary does not satisfy a behavior the Specification states. | The rebuilt file in the spec copy. | Return to the builder inside the same trial; no corpus or Specification edit; rerun the suite in a new `attempt-<N>` directory. |
| `specification defect` | The Specification is silent, ambiguous, or wrong about the observed behavior. | `givn/specification.yaml`, `docs/reference.md`, the owning `givn/specs/**` use case, or the declaration entry that should have covered the path. | Record the outcome; the correction belongs to a later change. No edit in this change. |
| `acceptable undocumented change` | The rebuilt behavior is a legitimate implementation choice the Specification does not constrain, and no acceptance test rejects it. | The rebuilt file, which is not a gap. | Record the outcome; no edit. |

`evidence/ise/attempt-<N>/spec-gaps.md` and
`evidence/ise/attempt-<N>/gap-table.md` record one row per difference with the
frozen scenario title, the failing step, the observed and expected result, the
rebuilt file, the outcome, the wrong artifact, and the evidence path. The five
prior `specification defect` rows are expected to disappear because the build
inputs are declared; a remaining failure on those surfaces is an
`implementation defect`. The 64 prior `implementation defect` rows remain
classification evidence, not gaps in the Specification. A trial with no
difference records `No difference observed` and names the result file and
counts. No difference is closed as drift. Every attempt keeps its own result,
log, gap table, and neighbor record; the final `spec-gaps.md` names the attempt
it classifies and lists the counts of every attempt.

A `Dry-Run Ise` trial promotes nothing: the rebuilt work stays in the copy, and
the trial review records that no rebuilt file was promoted.

## Evidence artifacts

All paths are under
`givn/changes/givn-self-rebuild-under-the-updated-rules/`, which is exempt from
the change check's clean-tree requirement and archived with the change. Author
checks stay under `evidence/ise/`; every trial attempt writes its own
`evidence/ise/attempt-<N>/` directory, so a restart never overwrites earlier
evidence. The attempt that prepared a copy owns its `rebuild-report.txt`,
`manifest.yaml`, and `compare-before.txt`; a rerun after an `implementation
defect` writes the other files in its own attempt directory and references the
preparing attempt's three files.

| Path | Produced by | Content |
|---|---|---|
| `evidence/ise/declaration.diff` | author | The declaration and pin diff from the change's base revision. |
| `evidence/ise/declaration-lint.txt` | author | `givn lint` on the checkout: exit 0, no declaration-schema error, no `specification_path_missing`, `specification_role_duplicate`, or `specification_e2e_boundary` advisory for a build-input entry, and the one expected `specification_feature_unretained` advisory for `assets/templates/spec.feature` with its text. |
| `evidence/ise/trial-revision.txt` | author | The 40-character revision and the commands that resolve it; committed after the run, not in the revision it names. |
| `evidence/ise/e2e-dependencies.txt` | author | `cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e`; no product crate. |
| `evidence/ise/spec-coverage.json` | author | `givn spec coverage --format json` at the trial revision. |
| `evidence/ise/e2e-inventory.md` | author | One row per `@e2e` entry: capability, feature path, scenario title. |
| `evidence/ise/clean-room-scan.txt` | author | The scan output for `~/.cargo/registry/src`, `~/.cargo/registry/cache`, and `~/.cargo/git`: no hit, plus one `absent: <path>` line per missing store, or the quarantined hits and their new location. |
| `evidence/ise/boundary-check.txt` | author | The pre-trial `givn check tasks --change <id>` transcript. |
| `evidence/ise/boundary-review-check.txt` | author | The `givn check review --change <id>` transcript after `review.md` exists. |
| `evidence/ise/attempt-<N>/rebuild-report.txt` | author | The `givn ise rebuild-all` stdout, all fixed lines, for the attempt that prepared this copy. |
| `evidence/ise/attempt-<N>/manifest.yaml` | author | The `--manifest` file copied verbatim for this copy. |
| `evidence/ise/attempt-<N>/compare-before.txt` | author | `givn ise compare` before the builder starts on this copy. |
| `evidence/ise/attempt-<N>/builder-notes.md` | builder | Sandbox command, every source and file read or authored, the exit 101 and the two authored `[[test]]` target files if the first build hit the missing-target parse error, every crate version chosen, the build command, and a statement that the checkout and `e2e/` were not read. |
| `evidence/ise/attempt-<N>/compare-after.txt` | verifier | `givn ise compare` after the build for this attempt. |
| `evidence/ise/attempt-<N>/rebuilt-version.txt` | verifier | `<copy>/target/release/givn --version` and the binary's SHA-256 for this attempt. |
| `evidence/ise/attempt-<N>/e2e-run.log` | verifier | The complete `./run-tests.sh --e2e --package e2e` output for this attempt. |
| `evidence/ise/attempt-<N>/e2e-result.json` | verifier | The runner's scoped result file with `total`, `passed`, `failed`, and `skipped` for this attempt. |
| `evidence/ise/attempt-<N>/spec-gaps.md` | verifier | The gap list and the triage table; the final one names the attempt it classifies and lists every attempt's counts. |
| `evidence/ise/attempt-<N>/gap-table.md` | verifier | This attempt's per-difference table. |
| `evidence/ise/attempt-<N>/neighbor-record.md` | verifier | Every file changed outside the Specification or the replaced unit, every consolidation, the scan result, and the test-infrastructure inputs for this attempt. |
| `evidence/ise/self-dogfood.txt` | maintainer, post-archive | The self-dogfood commands and their results. |

## Architecture impact

| Artifact | Kind | Effect |
|---|---|---|
| `givn/specification.yaml` | declaration | Adds the build inputs and the toolchain pin under their content-class roles. |
| `rust-toolchain.toml` | new build input | Pins Rust 1.97.1 and the clippy and rustfmt components. |
| `givn/changes/<id>/specs/**` | delta corpus | Two added scenarios; no modified or removed scenario. |
| `tests/steps/specification_declaration_steps.rs` | component test | Two new `Then` bindings and one `Given` for the build-input declaration. |
| `tests/steps/clean_room_preparation_steps.rs` | component test | Two new `Given` bindings for the build-input fixture. |
| `tests/steps/ise_clean_room_support.rs` | test support | One shared build-input declaration fixture. |
| `docs/arc42/02-architecture-constraints.md` | durable constraint | Records the declared build inputs and the pin in 2.3. |
| `docs/arc42/04-solution-strategy.md` | durable strategy | The `Self-rebuild` row names the updated-rule re-run and the resolved specification defects. |
| `docs/arc42/11-risks-and-technical-debt.md` | risk register | Updates `R-163` and adds the toolchain-pin risk. |
| `src/**` | production source | No edit. The trial rebuilds it only inside the spec copy. |
| `e2e/**` | frozen acceptance package | No edit. |
| `givn/specs/**` | permanent corpus | No edit; the delta merges at archive. |
| `assets/**`, `docs/reference.md` | served text and reference | No edit; the rule already exists. |
| Trial copy and manifest | ephemeral evidence | Outside the checkout; only the copies under `evidence/ise/` are committed. |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `SpecificationDeclaration` | declare-the-specification | `crate::ise`, `crate::e2e_boundary`, `crate::cli::lint`, the rebuild commands | `crate::config::declaration` | A second declaration format, a build-input key, or a per-component declaration appears. |
| `SpecCopy` (`rebuild-all`, `rebuild-component`, `compare`) | prepare-an-ise-test | the maintainer, the independent verifier, the builder | `crate::ise` | A second copy-resolution rule or a second manifest reader appears. |
| `E2eBoundary` | verify-change | `crate::cli::check`, `crate::ise` | `crate::e2e_boundary` | A second package-resolution strategy appears. |
| `Served change guidance` | `create-change` (`change-strategy`, `spec-gap`) | `givn instructions design`, `givn instructions review` | `crate::cli::instructions` over `assets/instructions/**` | A second guidance source or a per-stack rendering appears. |

The change adds no contract and no component. It changes the data the
declaration and copy contracts read and the test bindings that guard them.

## Use-case traceability

| Use-case guarantee or rule | Decision and artifact | Evidence |
|---|---|---|
| The declaration names the five roles and one source list; a build input is declared under the role that owns its content class. | D4; the exact declaration above; the delta scenario `A declaration that names build inputs under the four content roles is accepted`. | `declaration-lint.txt`, `declaration.diff` |
| The spec copy carries the declared Specification files and the declaration, and no End-to-end test. | D4, D6; `givn ise rebuild-all`; the delta scenario `Preparation carries every declared build input into the spec copy`. | `attempt-<N>/manifest.yaml`, `attempt-<N>/compare-before.txt` |
| The builder reads neither the replaced Source code nor the frozen E2E tests. | D6; sandbox and builder notes. | `attempt-<N>/builder-notes.md`, `attempt-<N>/compare-before.txt` |
| The frozen E2E package accepts the rebuilt binary. | D3, D5; trial protocol. | `attempt-<N>/e2e-result.json`, `attempt-<N>/rebuilt-version.txt` |
| Every difference receives one `Spec gap` outcome before a fix. | D8; gap triage. | `attempt-<N>/spec-gaps.md`, `attempt-<N>/gap-table.md` |
| The change records one change strategy and its `Ise Test` mode. | D3. | this design's `## Decisions` |
| Every file changed outside the Specification or the replaced unit, and every consolidation, is recorded. | D7. | `attempt-<N>/neighbor-record.md` |
| The trial promotes nothing. | D7; dry-run sentence. | `review.md`, `attempt-<N>/neighbor-record.md` |
| The scenario inventory accounts for every `@e2e` scenario. | `givn spec coverage`. | `spec-coverage.json`, `e2e-inventory.md` |

Confirmed Personas and the artifact each reads:

| Persona | Impact | Evidence |
|---|---|---|
| `developer--primary-user` | Declares the build inputs and reads the re-run's frozen result and gap table. | The declaration, `attempt-<N>/spec-gaps.md`, `attempt-<N>/gap-table.md`, `declaration-lint.txt` |
| `agent--technical-role` | Builds from a copy that already carries every build input and records its reads. | `attempt-<N>/builder-notes.md`, `attempt-<N>/manifest.yaml`, `attempt-<N>/rebuilt-version.txt` |
| `givn-maintainer--owner` | Decides `S14` from the declaration diff, the scan, and the residual gap table. | `declaration.diff`, `clean-room-scan.txt`, `attempt-<N>/neighbor-record.md`, `attempt-<N>/spec-gaps.md` |

## Step Definitions

| Capability | Step definition file |
|---|---|
| `specification-declaration` | `tests/steps/specification_declaration_steps.rs` |
| `clean-room-preparation` | `tests/steps/clean_room_preparation_steps.rs` |

The shared build-input declaration fixture lives in
`tests/steps/ise_clean_room_support.rs` (test support, not a capability file)
and is imported by both step files.

## Delta Step Bindings

New bindings:

| Capability | New phrase | Asserted behavior |
|---|---|---|
| `specification-declaration` | `Given an ise-enabled project selecting opencode whose declaration places build inputs under the four content roles` | Writes the build-input declaration and the files it names, including `tests/e2e/Cargo.toml` with a minimal `[package]` for the `e2e` role (disjoint from `src/**`), then runs `givn lint`. |
| `specification-declaration` | `Then stderr contains no declaration-schema error` | Stderr carries no declaration schema refusal. |
| `specification-declaration` | `Then stderr contains no declared-path advisory for the build inputs` | In the fixture, which holds no `.feature` template, no `[SPEC ] advisory` line names `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `assets/`, or `e2e`. The checkout's one expected `specification_feature_unretained` advisory for `assets/templates/spec.feature` is recorded in `declaration-lint.txt`, not asserted here. |
| `clean-room-preparation` | `Given a Git project with ise enabled and a declaration whose build inputs sit under the four content roles` | Writes and commits the build-input declaration with `ise` effective. |
| `clean-room-preparation` | `Given a committed revision with those build inputs, one source file, and an End-to-end manifest` | Commits the five build-input paths, `givn/specs/demo.feature`, `src/main.rs`, and `e2e/Cargo.toml`. |

Existing bindings reused: `the command exits 0`, `the command succeeds and
prints "Specification files: 7"`, `the spec copy holds "X" and "Y"`, and
`the spec copy omits "X" and "Y"`. The delta step bodies are non-empty and
drive the built binary.

Both added scenarios share the standard fixture setup with many permanent
scenarios, so `givn lint` prints `[SUBST]` subset advisories for them. The
overlap review records `keep` for each with its distinct invariant: the
declaration scenario proves build-input acceptance under the four content
roles, and the clean-room scenario proves the copy carries those inputs while
the End-to-end manifest stays out. No scenario is split or retagged.

## Interaction Coverage Matrix

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism | Change disposition |
|---|---|---|---|---|
| `specification-declaration` / declare the five roles and the replaceable source roots | `The declared roles are read by the project-health pass` | CLI | The E2E runner invokes `givn lint` in the fixture and reads stderr. | `unchanged`: the delta adds a regular scenario for the same action; no step or assertion moves. |
| `clean-room-preparation` / write the spec copy, wholly with `rebuild-all` or for named component paths with `rebuild-component` | `Developer rebuilds the whole product from its Specification` | CLI | The E2E runner invokes `givn ise rebuild-all --at HEAD --into <copy> --manifest <file>` and reads the report and copy. | `unchanged`: the trial runs the archived command; the delta adds a regular carrier scenario. |
| `trial-input-verification` / compare the spec copy and the checkout with the manifest | `Verifier compares the spec copy with the manifest` | CLI | The E2E runner invokes `givn ise compare --manifest <file> --copy <dir>`. | `unchanged`: the trial compares before and after the build. |
| `change-strategy` / read the design guidance | `The design guidance requires one recorded change strategy` | CLI | `givn instructions design --change <id>` | `unchanged`: D3 records the strategy and mode. |
| `spec-gap` / read the review guidance after an Ise Test | `The review guidance requires one spec gap outcome before any fix` | CLI | `givn instructions review --change <id>` | `unchanged`: the gap table follows the four outcomes. |

No row is added, removed, or retagged; the change adds no user action. The
other `create-change` inventory rows (prose-consolidation,
visual-design-contract, and change-release-note) are not touched.

## Strict Mode

Strict mode is inherited unchanged: `tests/features_runner.rs` calls
`.fail_on_skipped()`, and `e2e/tests/e2e_runner.rs` calls
`.fail_on_skipped()` and exits nonzero when `failed > 0`, `skipped > 0`, or
`total == 0`. The two new bindings have non-empty bodies; no empty-body check
is needed. A skipped or zero-scenario E2E result is a setup failure recorded in
`attempt-<N>/spec-gaps.md`, not a green run.

## Visual Design Contract

```text
- Interface: none
- Viewports: not applicable (no rendered screen)
- Design system: docs/design/design-system.md (unchanged)
```

The change renders no screen. Its user-visible surface is terminal text: the
`givn lint` findings, the `givn ise rebuild-all` report and refusals, the
`givn ise compare` subjects, and the E2E runner summary. The delta adds no
screen and no `@e2e` scenario, so no screenshot or transcript is committed.

Baseline asserted by the trial transcripts:

- every report line is one of the fixed lines the reference names;
- every refusal names the condition and its subject path, revision, or flag;
- no Rust module, struct, or field name appears in user-facing output;
- exit codes stay stable: `givn ise rebuild-all` 0 on success, 1 on a refusal,
  2 on a usage error; `givn ise compare` 1 on any difference; `givn lint` 0
  with advisories.

## Black-box-first policy

No new internal test. The two delta scenarios drive `givn lint` and
`givn ise rebuild-all` through the real CLI. No unit test is retained: the
declaration loader and the copy partition are exercised through those commands.

## External boundary cases

The external boundary is the declaration loader, the rebuild command, and the
frozen runner. Each case is a setup error, not a `Spec gap`.

| Boundary | Case | Expected |
|---|---|---|
| Declaration loader | Unknown key or role, repeated role, omitted role, absent source list, path outside the project, non-UTF-8, missing at the revision | Exit 1 or the lint refusal; the path and the condition are named; no partial role set. |
| `givn ise rebuild-all` | `--into` omitted, inside the checkout, or already existing; manifest inside the copy or checkout | Exit 2 or 1; the target is named; no write. |
| `givn ise rebuild-all` | Dirty checkout, missing backend, missing declaration, declared path missing at the revision | Exit 1; the condition and path are named; no write. |
| Builder | A declared build input is missing from the copy | A setup error; `compare` reports the missing Specification file. |
| Builder | `tests/features_runner.rs` or `tests/ready_fixture.rs` absent before the first build | Cargo exits 101 at manifest parse; the builder authors both minimal `fn main() {}` targets from the declared `[[test]]` entries. A setup error, not a `Spec gap` and not an expected red result. |
| Builder | The pinned toolchain 1.97.1 is absent | The build fails before compilation; record a setup error and install the toolchain outside the sandbox. |
| Builder | A locked dependency is missing from the offline cache | The build fails; record a setup error, refresh the cache outside the sandbox, and restart the trial. |
| Clean-room scan | A `givn` or `givn-*` path in `~/.cargo/registry/src`, `~/.cargo/registry/cache`, or `~/.cargo/git` | Quarantine it outside the builder's reach; record it. |
| E2E runner | `GIVN_BIN` unset or not executable | Exit 2, names `GIVN_BIN`. |
| E2E runner | Zero scenarios, failed, or skipped | Exit 1; the result file records the counts. |

## Local runnability and digital twins

The trial runs locally on one host:

```sh
givn ise rebuild-all --at <revision> --into /tmp/givn-ise-self-rebuild-<stamp> \
  --manifest /tmp/givn-ise-self-rebuild-<stamp>.yaml
bwrap ... --chdir /work cargo build --release --offline
GIVN_BIN=<copy>/target/release/givn \
GIVN_RESULT_FILE=$PWD/givn/changes/<id>/evidence/ise/attempt-<N>/e2e-result.json \
./run-tests.sh --e2e --package e2e
```

The product is a CLI; it starts no server, database, queue, or browser. The
builder resolves crates from the local cache with `--offline` and has no
network. No digital twin is required because the product talks to no
third-party service. The `ready_fixture` binary is local test infrastructure
built from the checkout and named in the builder notes.

## Version freshness

| Decision | Version | Source |
|---|---|---|
| Rust toolchain pin | 1.97.1 | `rustup toolchain list` on 2026-11-17; `cargo +1.97.1 check --locked` finished green in 10.62 s. The current stable alias is 1.99.1, but no `1.99.1` toolchain is installed. Binds rustup builds and the sandbox; `nix build` uses nixpkgs' rustc and is out of scope. |
| E2E runner | `cucumber-rs` 0.23 | `e2e/Cargo.toml` at authoring. |
| Browser driver | Playwright, unchanged | Not used by this change; `e2e/browser/package.json` keeps the existing pin. |

## Coverage process boundaries

The change adds no production process and no measured process. The archive
coverage gate uses the existing scripts over the regular suite. The trial's
rebuilt binary is evidence, not the measured build, so it is not instrumented.
No new row is needed.

## Verification

1. Run the delta batch with `GIVN_FEATURES=givn/changes/<id>/specs
   ./run-tests.sh --non-e2e`; record the summary and the exact counts. Both
   added scenarios pass immediately because they protect settled behavior; no
   RED is claimed.
2. Apply the declaration, the pin, the arc42 chapter edits, and the step
   bindings. Run `givn lint` in the checkout: exit 0, no declaration-schema
   error, no `specification_path_missing`,
   `specification_role_duplicate`, or `specification_e2e_boundary` advisory
   for a build-input entry, and the one expected
   `specification_feature_unretained` advisory for
   `assets/templates/spec.feature`. Record `declaration-lint.txt` and
   `declaration.diff`.
3. Run `givn check tasks --change <id>` after the task list is checked;
   record `boundary-check.txt` and the receipt.
4. Make the pre-trial commit, record the 40-hex revision, and run the rebuild,
   the comparison, the sandboxed build, and the frozen E2E run. Record every
   evidence path above.
5. Triage every difference with exactly one outcome before any edit; record
   `attempt-<N>/spec-gaps.md` and `attempt-<N>/gap-table.md`. The five prior
   `specification defect` rows must be absent; a remaining failure there is an
   `implementation defect`.
6. Run `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e --package e2e` on
   the final revision; both must report zero failed and zero skipped.
7. Run `givn check review --change <id>`; record `boundary-review-check.txt`.
8. After archive, run the AGENTS.md self-dogfood sequence (`cargo install
   --path . --locked --force`, `givn skills sync`, `givn instructions`,
   `./run-tests.sh`, `./run-tests.sh --e2e --package e2e`) and record
   `evidence/ise/self-dogfood.txt`.

## Decisions

Decides D1: pin Rust 1.97.1 in `rust-toolchain.toml` with the clippy and
rustfmt components, and declare it under `limits`. 1.97.1 is installed on the
host, passes `cargo +1.97.1 check --locked`, and is available to the sandbox
from the read-only `~/.rustup`. The pin binds the rustup build and the sandbox;
`nix build` is out of scope. Realized by `rust-toolchain.toml` and the
declaration's `limits` entry. Rejected: 1.99.1 (no installed toolchain; an
extra download before the first build) and the `stable` alias (no version is
fixed, so the build input changes without a revision).

Decides D2: partition the whole `assets/**` tree by content class: served
instruction, skill, integration, hook, and addon text under `behavior`;
document, chapter, report, and README templates under `structure`;
configuration, manifest, declaration, and migration assets under `contracts`.
The `e2e` package's manifest and lockfile stay under the `e2e` role. Realized
by the exact declaration above. Rejected: declaring only the five prior
surfaces (88 embedded assets stay undeclared) and one `assets/**` entry under
`behavior` (templates and schemas sit under a role that does not own their
content class).

Decides D3: `change strategy whole-product rebuild`, `Ise Test mode Dry-Run
Ise`. Affected consumers: `developer--primary-user` declares the build inputs
and reads the frozen result; `agent--technical-role` builds in the copy;
`givn-maintainer--owner` decides `S14` from the evidence; the independent
verifier runs `givn ise compare` and the frozen suite. Persisted state: none;
Givn keeps no product state in the trial. Rebuild boundary: every tracked file
under `src/**` and `tests/**` is removed; the `e2e` package, the declaration,
the four content roles, and every file outside the declaration stay as above.
Realized by the trial protocol and the evidence table. Rejected: `incremental
modification` (no rebuild), `Constant Ise` (the scope is the whole product),
and `Full replacement` (nothing is promoted; `S14` owns that).

Decides D4: the declaration carries the build inputs under the four
content-class roles with the exact entries above. The End-to-end package's own
manifest and lockfile stay the test build's inputs, never `contracts`.
Realized by `givn/specification.yaml` and the delta scenario `A declaration
that names build inputs under the four content roles is accepted`. Rejected:
one role for all assets, and excluding the toolchain pin from the declaration.

Decides D5: the trial reads the change's pre-trial commit, passed explicitly
with `--at <40 hex>`. That commit records the declaration, the pin, the delta,
the step bindings, the chapter edits, and every task completed before the
trial. A restart after an `E2E test defect` reads a commit that records the
first attempt's evidence, the corrected test, and the regenerated inventory
and coverage. Realized by `trial-revision.txt`, the report's `Revision:` line,
and the manifest's `revision`. Rejected: `HEAD` (a moving input no transcript
can reproduce) and the change's base revision (which ignores the corrected
test and the evidence paths).

Decides D6: the builder runs in the `bwrap` sandbox above, with the copy at
`/work`, the pinned toolchain and registry cache read-only, and no checkout
path. Before the rebuild, scan `~/.cargo/registry/src`,
`~/.cargo/registry/cache`, and `~/.cargo/git` for a `givn` or `givn-*` path and
quarantine every hit outside the builder's reach; an absent store has no hit.
Realized by the sandbox command, `clean-room-scan.txt`,
and `attempt-<N>/builder-notes.md`. Rejected: mounting the checkout read-only for
convenience (the builder could read the replaced Source code) and skipping the
scan (a published copy is a copy of the Source code).

Decides D7: the trial review records every file changed outside the declared
Specification or the replaced unit, every consolidation, the scan result, and
the verifier's test-infrastructure inputs in `attempt-<N>/neighbor-record.md`;
it is a review question, never a gate. A `Dry-Run Ise` trial promotes nothing;
the review records that no rebuilt file was promoted. Realized by
`attempt-<N>/neighbor-record.md`, `review.md`, and
`attempt-<N>/spec-gaps.md`. Rejected: a
consolidation gate (the review question owns the judgement) and treating a
green trial as a promotion authorization.

Decides D8: classify before repair. One difference receives one outcome and
one wrong artifact before any edit. An `implementation defect` returns to the
builder inside the trial; an `E2E test defect` is corrected and the trial
restarts at the correcting revision; a `specification defect` and an
`acceptable undocumented change` are recorded without a corpus edit in this
change. The five prior `specification defect` rows are expected to be resolved
by the declaration; the residual `implementation defect` rows remain
classification evidence. Realized by `attempt-<N>/spec-gaps.md` and
`attempt-<N>/gap-table.md`. Rejected: fix-first (the outcome is chosen to fit
the fix) and stop-on-first
(the rule requires every difference classified).

Decides D9: no served-text rebinding. The build-input rule already lives in
the served `ise` instruction and the reference; the change edits neither, adds
no `@givn.modified` scenario, and changes no permanent scenario whose assertion
names the declaration's content. Realized by the two `@givn.added` delta
features and the code-impact table. Rejected: restating the build-input rule in
the declaration scenario (it is not served text) and copying the new entries
into `docs/reference.md` (the rule is already stated there).

Carried decision C1 (toolchain fallback): default `1.97.1` in
`rust-toolchain.toml`; trigger: the copy fails to build on 1.97.1 for a
toolchain reason; switch: change the pin to the current stable `1.99.1`,
install that toolchain outside the sandbox, and restart the trial at a new
pre-trial commit; switch cost: one file line, one install, one commit, and one
rerun; owner: `givn-maintainer--owner`.

Carried decision C2 (clean-room scan hit): default: the scan prints nothing;
trigger: any `givn` or `givn-*` path in `~/.cargo/registry/src`,
`~/.cargo/registry/cache`, or `~/.cargo/git`; switch: move every hit to
`/tmp/givn-ise-quarantine-<stamp>/`, or use a fresh `CARGO_HOME` populated
from the declared lockfile, then rerun the scan and record the result; switch
cost: one setup round and a fresh pre-trial commit; owner:
`givn-maintainer--owner`.

Carried decision C3 (new specification defect in the re-run): default: record
one `specification defect` row with the wrong artifact and do not fix it in
this change; trigger: a frozen scenario failure whose expected behavior the
declaration or the Specification does not state; switch: open the correction
in a later change, or, only if the operator widens this change, amend the
declaration and restart the trial; switch cost: a new change and a new trial;
owner: `givn-maintainer--owner`.

## Risks and tradeoffs

- **Large spec copy.** The declaration adds 95 asset files; the copy and the
  report counts grow. Mitigation: the content-class split is explicit, the
  trial recomputes the counts, and `givn ise compare` checks the copy against
  the manifest.
- **Toolchain pin friction.** A rustup contributor without 1.97.1 downloads it
  on the first build; a sandbox or offline host fails before compilation.
  Mitigation: the pin names the installed stable, the README's
  stable-toolchain requirement is unchanged, and the trial records the
  installed toolchain.
- **Re-run cost.** The whole-product rebuild is expensive and may leave
  implementation defects. Mitigation: the trial is `Dry-Run Ise`, the four
  outcomes classify every difference, and nothing is promoted.
- **Specification silence remains possible.** A new `specification defect` is
  recorded, not fixed. Mitigation: the gap table names the wrong artifact and
  a later change carries the correction; the trial does not promote.
- **Independent review.** The change edits the project's own declaration and
  its guards. Mitigation: one fresh review context checks the exact entries,
  the counts, the scenario discrimination, and the trial evidence.

## Open Technical Questions

None.
